use crate::config;
pub use crate::globals::*;
use arc_swap::ArcSwap;
use once_cell::sync::Lazy;
use std::rc::Rc;

pub static BACKUP_CONFIG: Lazy<ArcSwap<config::Backups>> = Lazy::new(Default::default);
pub static BACKUP_HISTORY: Lazy<ArcSwap<config::Histories>> = Lazy::new(Default::default);

pub static SCHEDULE_STATUS: Lazy<ArcSwap<config::Writeable<config::ScheduleStatus>>> =
    Lazy::new(Default::default);

thread_local!(
    static GIO_APPLICATION: Rc<gio::Application> = Rc::new(gio::Application::new(
        Some(&crate::daemon_app_id()),
        gio::ApplicationFlags::IS_SERVICE,
    ));
);

pub fn gio_app() -> Rc<gio::Application> {
    GIO_APPLICATION.with(Clone::clone)
}
