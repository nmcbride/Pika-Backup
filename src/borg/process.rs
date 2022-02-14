use super::prelude::*;
use super::{Borg, BorgRunConfig, Error, Result};

use std::any::TypeId;
use std::io::Write;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::IntoRawFd;
use std::process::{Command, Stdio};

use futures::prelude::*;
use futures::task::SpawnExt;
use zeroize::Zeroizing;

use futures::channel::oneshot;
use std::time::Duration;

use super::communication::*;
use super::log_json;
use super::status::*;
use super::utils;
use crate::config::Password;

use super::error::*;

#[derive(Default)]
pub struct BorgCall {
    command: Option<String>,
    options: Vec<String>,
    envs: std::collections::BTreeMap<String, String>,
    pub positional: Vec<String>,
    password: Password,
}

pub struct Process<T> {
    pub result: oneshot::Receiver<Result<T>>,
}

impl BorgCall {
    pub fn new(command: &str) -> Self {
        Self {
            command: Some(command.to_string()),
            options: vec![
                "--rsh".into(),
                // Avoid hangs from ssh asking for passwords via stdin
                // https://borgbackup.readthedocs.io/en/stable/usage/notes.html#ssh-batch-mode
                "ssh -o BatchMode=yes -o StrictHostKeyChecking=accept-new".into(),
            ],
            ..Self::default()
        }
    }

    pub fn new_raw() -> Self {
        Self::default()
    }

    pub fn add_envs<L, V>(&mut self, vars: L) -> &mut Self
    where
        L: std::iter::IntoIterator<Item = (V, V)>,
        V: ToString,
    {
        for (var, value) in vars {
            self.envs.insert(var.to_string(), value.to_string());
        }

        self
    }

    pub fn add_options<L>(&mut self, options: L) -> &mut Self
    where
        L: std::iter::IntoIterator,
        <L as std::iter::IntoIterator>::Item: ToString,
    {
        for option in options {
            self.options.push(option.to_string());
        }

        self
    }

    pub fn add_positional<A: ToString>(&mut self, pos_arg: &A) -> &mut Self {
        self.positional.push(pos_arg.to_string());
        self
    }

    pub fn add_include_exclude(&mut self, borg: &Borg) -> &mut Self {
        for exclude in &borg.config.exclude_dirs_internal() {
            self.add_options(vec![format!("--exclude={}", exclude.borg_pattern())]);
        }

        self.positional.extend(
            borg.config
                .include_dirs()
                .iter()
                .map(|d| d.to_string_lossy().to_string()),
        );

        self
    }

    pub fn add_archive(&mut self, borg: &Borg) -> &mut Self {
        let random_str = glib::uuid_string_random();
        let arg = format!(
            "{repo}::{archive_prefix}{archive}",
            repo = borg.config.repo,
            archive_prefix = borg.config.archive_prefix,
            archive = random_str.get(..8).unwrap_or(&random_str)
        );
        if let Some(first) = self.positional.first_mut() {
            *first = arg;
        } else {
            self.add_positional(&arg);
        }

        self
    }

    pub fn add_password<T: BorgRunConfig>(&mut self, borg: &T) -> Result<&mut Self> {
        if let Some(ref password) = borg.password() {
            debug!("Using password enforced by explicitly passed password");
            self.password = password.clone();
        } else if borg.is_encrypted() {
            debug!("Config says the backup is encrypted");
            if let Some(config_id) = borg.config_id() {
                let password: Zeroizing<Vec<u8>> =
                    secret_service::SecretService::new(secret_service::EncryptionType::Dh)?
                        .search_items(vec![
                            ("backup_id", config_id.as_str()),
                            ("program", env!("CARGO_PKG_NAME")),
                        ])?
                        .get(0)
                        .ok_or(Error::PasswordMissing)?
                        .get_secret()?
                        .into();

                self.password = password;
            } else {
                // TODO when is this happening?
                return Err(Error::PasswordMissing);
            }
        } else {
            trace!("Config says no encryption. Writing empty password.");
            self.password = Password::default();
        }

        Ok(self)
    }

    fn set_password(&self) -> Result<(String, String)> {
        // Password pipe
        let (pipe_reader, mut pipe_writer) = std::os::unix::net::UnixStream::pair()?;

        // Allow pipe to be passed to borg
        let mut flags = nix::fcntl::FdFlag::from_bits_truncate(nix::fcntl::fcntl(
            pipe_reader.as_raw_fd(),
            nix::fcntl::FcntlArg::F_GETFD,
        )?);

        flags.remove(nix::fcntl::FdFlag::FD_CLOEXEC);
        nix::fcntl::fcntl(
            pipe_reader.as_raw_fd(),
            nix::fcntl::FcntlArg::F_SETFD(flags),
        )?;

        pipe_writer.write_all(&self.password)?;

        Ok((
            String::from("BORG_PASSPHRASE_FD"),
            pipe_reader.into_raw_fd().to_string(),
        ))
    }

    pub fn add_basics_without_password<T: BorgRunConfig>(&mut self, borg: &T) -> &mut Self {
        self.add_options(&["--log-json"]);

        if self.positional.is_empty() {
            self.add_positional(&borg.repo().to_string());
        }

        self.add_options(
            &borg
                .repo()
                .settings()
                .and_then(|x| x.command_line_args)
                .unwrap_or_default(),
        );

        self
    }

    pub fn add_basics<T: BorgRunConfig>(&mut self, borg: &T) -> Result<&mut Self> {
        self.add_password(borg)?;
        self.add_basics_without_password(borg);
        Ok(self)
    }

    pub fn args(&self) -> Vec<String> {
        let mut args: Vec<String> = self.command.clone().into_iter().collect();
        args.extend(self.options.clone());
        args.push("--".to_string());
        args.extend(self.positional.clone());

        args
    }

    pub fn cmd(&self) -> Result<Command> {
        let mut cmd = Command::new("borg");

        cmd.envs([self.set_password()?]);

        cmd.args(self.args())
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .envs(self.envs.clone().into_iter());

        Ok(cmd)
    }

    pub fn output(&self) -> Result<std::process::Output> {
        info!("Running borg: {:#?}\nenv: {:#?}", &self.args(), &self.envs);
        Ok(self.cmd()?.output()?)
    }

    pub fn cmd_async(&self) -> Result<async_process::Command> {
        let mut cmd = async_process::Command::new("borg");

        cmd.envs([self.set_password()?]);

        cmd.args(self.args())
            .stderr(async_process::Stdio::piped())
            .stdout(async_process::Stdio::piped())
            .envs(self.envs.clone().into_iter());

        Ok(cmd)
    }

    pub fn spawn_async(&self) -> Result<async_process::Child> {
        info!(
            "Async running borg: {:#?}\nenv: {:#?}",
            &self.args(),
            &self.envs
        );
        Ok(self.cmd_async()?.spawn()?)
    }

    pub fn spawn_async_managed<
        T: std::fmt::Debug + serde::de::DeserializeOwned + Send + 'static,
    >(
        self,
        communication: super::Communication,
    ) -> Result<Process<T>> {
        let (result_sink, result) = futures::channel::oneshot::channel();

        let pool = futures::executor::ThreadPool::new()?;
        pool.spawn(async {
            result_sink
                .send(self.handle_disconnect(communication).await)
                .unwrap();
        })?;

        Ok(Process { result })
    }

    async fn handle_disconnect<T: std::fmt::Debug + serde::de::DeserializeOwned + 'static>(
        mut self,
        communication: super::Communication,
    ) -> Result<T> {
        communication.status.update(move |status| {
            status.started = Some(chrono::Local::now());
        });

        let mut retries = 0;
        let mut retried = false;

        loop {
            let result = self.managed_process(communication.clone()).await;
            match &result {
                Err(Error::Failed(ref failure)) if failure.is_connection_error() => {
                    if !retried {
                        debug!("First disconnect for this task");
                        retried = true;
                        self.add_options(&[
                            "--lock-wait",
                            &super::LOCK_WAIT_RECONNECT.as_secs().to_string(),
                        ]);
                    }

                    if !matches!(communication.status.load().run, Run::Reconnecting) {
                        debug!("Starting reconnect attempts");
                        retries = 0;
                        communication.status.update(|status| {
                            status.run = Run::Reconnecting;
                        });
                    }

                    if retries < super::MAX_RECONNECT {
                        retries += 1;
                        debug!("Reconnect attempt number {}", retries);
                        std::thread::sleep(super::DELAY_RECONNECT);
                        continue;
                    } else {
                        return result;
                    }
                }
                _ => {
                    return result;
                }
            }
        }
    }

    async fn managed_process<T: std::fmt::Debug + serde::de::DeserializeOwned + 'static>(
        &self,
        communication: super::Communication,
    ) -> Result<T> {
        let mut line = String::new();
        let mut process = self.spawn_async()?;
        let mut reader = futures::io::BufReader::new(
            process
                .stderr
                .take()
                .ok_or_else(|| String::from("Failed to get stderr."))?,
        );

        let mut unresponsive = Duration::ZERO;

        loop {
            // react to abort instruction before potentially listening for messages again
            if matches!(**communication.instruction.load(), Instruction::Abort) {
                communication.status.update(|status| {
                    status.run = Run::Stopping;
                });
                debug!("Sending SIGTERM to borg process");
                nix::sys::signal::kill(
                    nix::unistd::Pid::from_raw(process.id() as i32),
                    nix::sys::signal::Signal::SIGTERM,
                )?;
                process.status().await?;
                debug!("Process terminated");
                return Err(Error::Aborted(Abort::User));
            }

            line.clear();
            let read =
                async_std::io::timeout(super::MESSAGE_POLL_TIMEOUT, reader.read_line(&mut line))
                    .await;

            match read {
                // nothing new to read
                Err(err) if err.kind() == async_std::io::ErrorKind::TimedOut => {
                    unresponsive += super::MESSAGE_POLL_TIMEOUT;
                    if unresponsive > super::STALL_THRESHOLD
                        && !matches!(communication.status.load().run, Run::Reconnecting)
                    {
                        communication.status.update(|status| {
                            status.run = Run::Stalled;
                        });
                    }
                    continue;
                }
                Err(err) => return Err(err.into()),
                // end of stream
                Ok(0) => break,
                // one line read
                Ok(_) => {}
            }

            unresponsive = Duration::ZERO;

            debug!("borg output: {}", line);

            let msg = if let Ok(msg) = serde_json::from_str::<log_json::Progress>(&line) {
                if !matches!(communication.status.load().run, Run::Running) {
                    communication.status.update(|status| {
                        status.run = Run::Running;
                    });
                }
                log_json::Output::Progress(msg)
            } else {
                let msg = utils::check_line(&line);

                communication.status.update(|status| {
                    status.add_message(&msg);
                });
                log_json::Output::LogEntry(msg)
            };

            for mut sender in communication.sender.load().iter() {
                sender.send(msg.clone()).await?;
            }
        }

        for mut sender in communication.sender.load().iter() {
            sender.close().await?;
        }

        let output = process.output().await?;

        debug!("Process terminated");

        let result = if TypeId::of::<T>() == TypeId::of::<()>() {
            serde_json::from_slice(b"null")
        } else {
            serde_json::from_slice(&output.stdout)
        };

        if output.status.success() {
            Ok(result?)
        } else if let Ok(err) =
            Error::try_from(communication.status.load().last_combined_message_history())
        {
            Err(err)
        } else {
            Err(ReturnCodeError::new(output.status.code()).into())
        }
    }
}