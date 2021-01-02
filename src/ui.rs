use std::error::Error;

use gdk_pixbuf::prelude::*;
use gio::prelude::*;
use gtk::prelude::*;

use crate::borg;
use crate::shared;
use crate::ui;
use crate::ui::globals::*;
use crate::ui::prelude::*;

mod backup_status;
#[allow(dead_code)]
mod builder;
mod dialog_about;
mod dialog_add_config;
mod dialog_device_missing;
mod dialog_encryption_password;
mod dialog_info;
mod dialog_storage;
mod globals;
mod headerbar;
mod page_archives;
mod page_detail;
mod page_overview;
mod page_pending;
pub mod prelude;
mod update_config;
mod utils;

pub fn main() {
    // suppress "gdk_pixbuf_from_pixdata()" debug spam
    glib::log_set_handler(
        Some("GdkPixbuf"),
        glib::LogLevels::LEVEL_DEBUG,
        false,
        false,
        |_, _, _| {},
    );

    // init gettext
    gettextrs::setlocale(gettextrs::LocaleCategory::LcAll, "");
    let localedir = option_env!("LOCALEDIR").unwrap_or(crate::DEFAULT_LOCALEDIR);
    gettextrs::bindtextdomain(env!("CARGO_PKG_NAME"), localedir);
    info!("bindtextdomain sets directory to {:?}", localedir);
    gettextrs::textdomain(env!("CARGO_PKG_NAME"));

    // init gtk and libhandy
    gtk::init().expect("Failed to gtk::init()");
    libhandy::init();
    let none: Option<&gio::Cancellable> = None;
    gtk_app()
        .register(none)
        .expect("Failed to gtk::Application::register()");

    gtk_app().connect_activate(init);
    gtk_app().connect_shutdown(on_shutdown);

    crate::globals::init();

    // Ctrl-C handling
    glib::unix_signal_add(nix::sys::signal::Signal::SIGINT as i32, on_ctrlc);

    init_check_borg();

    gtk_app().run(&[]);
}

fn on_ctrlc() -> Continue {
    gtk_app().release();
    Continue(true)
}

fn on_shutdown(app: &gtk::Application) {
    app.mark_busy();
    IS_SHUTDOWN.swap(std::sync::Arc::new(true));
    while !ACTIVE_MOUNTS.load().is_empty() {
        for backup_id in ACTIVE_MOUNTS.load().iter() {
            let config = &SETTINGS.load().backups[backup_id];
            if borg::Borg::new(config.clone()).umount().is_ok() {
                ACTIVE_MOUNTS.update(|mounts| {
                    mounts.remove(backup_id);
                });
            }
        }
    }

    debug!("Good bye!");
}

fn init(_app: &gtk::Application) {
    load_config();

    if let Some(screen) = gdk::Screen::get_default() {
        let provider = gtk::CssProvider::new();
        ui::utils::dialog_catch_err(
            provider.load_from_data(include_bytes!("../data/style.css")),
            "Could not load style sheet.",
        );
        gtk::StyleContext::add_provider_for_screen(
            &screen,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    let loader = gdk_pixbuf::PixbufLoader::new();
    loader
        .write(include_bytes!(concat!(data_dir!(), "/app.svg")))
        .unwrap_or_else(|e| error!("loader.write() failed: {}", e));
    loader
        .close()
        .unwrap_or_else(|e| error!("loader.close() failed: {}", e));
    if let Some(icon) = loader.get_pixbuf() {
        gtk::Window::set_default_icon(&icon);
    }

    init_actions();
    init_timeouts();

    ui::page_archives::init();
    ui::page_detail::init();
    ui::headerbar::init();
    ui::page_overview::init();
    ui::page_pending::init();
    ui::dialog_info::init();

    gtk_app().set_accels_for_action("app.quit", &["<Ctrl>Q"]);

    main_ui()
        .window()
        .connect_delete_event(|_, _| gtk::Inhibit(!is_quit_okay()));

    // decorate headerbar of pre-release versions
    if !option_env!("APPLICATION_ID_SUFFIX")
        .unwrap_or_default()
        .is_empty()
    {
        main_ui().window().get_style_context().add_class("devel");
    }

    gtk_app().add_window(&main_ui().window());

    main_ui().window().show_all();
    main_ui().window().present();

    ui::update_config::run();
}

fn init_timeouts() {
    glib::timeout_add_local(1000, move || {
        let inhibit_cookie = INHIBIT_COOKIE.get();

        if is_backup_running() {
            if inhibit_cookie.is_none() {
                INHIBIT_COOKIE.update(|c| {
                    *c = Some(gtk_app().inhibit(
                        Some(&main_ui().window()),
                        gtk::ApplicationInhibitFlags::LOGOUT
                            | gtk::ApplicationInhibitFlags::SUSPEND,
                        Some("Backup in Progress"),
                    ))
                });
            }
        } else if let Some(cookie) = inhibit_cookie {
            gtk_app().uninhibit(cookie);
            INHIBIT_COOKIE.update(|c| *c = None);
        }

        Continue(true)
    });
}

/// checks if there is any running backup
fn is_backup_running() -> bool {
    !BACKUP_COMMUNICATION.load().is_empty()
}

/// Checks if it's okay to quit and ask the user if necessary
fn is_quit_okay() -> bool {
    if is_backup_running() {
        ui::utils::confirmation_dialog(
            &gettext("Abort running backup creation?"),
            &gettext("The backup will remain incomlete if aborted now."),
            &gettext("Continue"),
            &gettext("Abort"),
        )
    } else {
        true
    }
}

fn init_actions() {
    let action = gio::SimpleAction::new("detail", glib::VariantTy::new("s").ok());
    action.connect_activate(|_, backup_id: _| {
        if let Some(backup_id) = backup_id.and_then(|v| v.get_str()) {
            ui::page_detail::view_backup_conf(&backup_id.to_string());
            main_ui().window().present();
        }
    });
    gtk_app().add_action(&action);

    let action = gio::SimpleAction::new("about", None);
    action.connect_activate(|_, _| ui::dialog_about::show());
    gtk_app().add_action(&action);

    let action = gio::SimpleAction::new("quit", None);
    action.connect_activate(|_, _| {
        if is_quit_okay() {
            gtk_app().quit()
        }
    });
    gtk_app().add_action(&action);
}

fn init_check_borg() {
    let version_result = borg::version();

    match version_result {
        Err(err) => ui::utils::show_error(
            gettext("Failed to run `borg`. Is borg-backup installed correctly?"),
            err,
        ),
        Ok(version_output) => {
            if let Some(version_string) = version_output.split(' ').nth(1) {
                let version_list = version_string
                    .split('.')
                    .map(str::parse)
                    .map(Result::ok)
                    .take(2);
                if vec![Some(crate::BORG_MIN_MAJOR), Some(crate::BORG_MIN_MINOR)]
                    .into_iter()
                    .cmp(version_list)
                    == std::cmp::Ordering::Greater
                {
                    ui::utils::show_error(
                    gettext("Borg version too old."),
                    gettextf(
                        "The installed version of borg-backup is too old. Some features requiring borg-backup version {}.{} will not work.",
                        &[
                            &crate::BORG_MIN_MAJOR.to_string(),
                            &crate::BORG_MIN_MINOR.to_string(),
                        ],
                    ));
                }
            }
        }
    }
}

fn load_config_e() -> Result<(), Box<dyn Error>> {
    let conf = shared::Settings::from_path(&shared::Settings::default_path()?)?;
    SETTINGS.update(|s| *s = conf.clone());
    Ok(())
}

fn load_config() {
    utils::dialog_catch_err(load_config_e(), gettext("Could not load config."));
}

fn write_config_e() -> Result<(), Box<dyn Error>> {
    let settings: &shared::Settings = &SETTINGS.load();
    let file = std::fs::File::create(&shared::Settings::default_path()?)?;
    serde_json::ser::to_writer_pretty(file, settings)?;
    Ok(())
}

fn write_config() {
    utils::dialog_catch_err(write_config_e(), gettext("Could not write config."));
}
