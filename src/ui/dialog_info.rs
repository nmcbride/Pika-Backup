use adw::prelude::*;

use num_format::ToFormattedString;

use crate::borg;
use crate::config::history::*;
use crate::ui::backup_status;
use crate::ui::prelude::*;

fn is_visible() -> bool {
    main_ui().detail_running_backup_info().is_visible()
}

pub fn show() {
    main_ui().detail_running_backup_info().present();
    refresh_status();
}

pub fn refresh_status() {
    if is_visible() {
        if let Some(id) = ACTIVE_BACKUP_ID.load().as_ref().as_ref() {
            refresh_status_display(&backup_status::Display::new_from_id(id));
        }
    }
}

fn refresh_status_display(status: &backup_status::Display) {
    main_ui()
        .detail_info_status()
        .set_from_backup_status(status);

    if let Some(progress) = status.progress {
        main_ui().detail_info_progress().set_fraction(progress);
        main_ui().detail_info_progress().set_visible(true);
    } else {
        main_ui().detail_info_progress().set_visible(false);
    }

    if let Some(backup_status::Stats::Final(run_info)) = &status.stats {
        let mut message = String::new();

        if !matches!(run_info.outcome, borg::Outcome::Completed { .. }) {
            message.push_str(&run_info.outcome.to_string());
            message.push_str("\n\n");
        }

        message.push_str(&run_info.messages.clone().filter_hidden().to_string());

        main_ui().detail_info_error().set_text(&message);
        main_ui().detail_info_error().set_visible(true);
    } else {
        main_ui().detail_info_error().set_visible(false);
    }

    match &status.stats {
        Some(backup_status::Stats::Final(RunInfo {
            outcome: borg::Outcome::Completed { stats },
            ..
        })) => {
            main_ui().detail_stats().set_visible(true);
            main_ui().detail_path_row().set_visible(false);

            main_ui()
                .detail_original_size()
                .set_text(&glib::format_size(stats.archive.stats.original_size));
            main_ui()
                .detail_deduplicated_size()
                .set_text(&glib::format_size(stats.archive.stats.deduplicated_size));
            main_ui()
                .detail_nfiles()
                .set_text(&stats.archive.stats.nfiles.to_formatted_string(&*LC_LOCALE));
        }
        Some(backup_status::Stats::Progress(progress_archive)) => {
            main_ui().detail_stats().set_visible(true);
            main_ui().detail_path_row().set_visible(true);

            main_ui()
                .detail_original_size()
                .set_text(&glib::format_size(progress_archive.original_size));
            main_ui()
                .detail_deduplicated_size()
                .set_text(&glib::format_size(progress_archive.deduplicated_size));
            main_ui()
                .detail_nfiles()
                .set_text(&progress_archive.nfiles.to_formatted_string(&*LC_LOCALE));

            main_ui()
                .detail_current_path()
                .set_text(&format!("/{}", progress_archive.path));
            main_ui()
                .detail_current_path()
                .set_tooltip_text(Some(&format!("/{}", progress_archive.path)));
        }
        _ => {
            main_ui().detail_stats().set_visible(false);
        }
    }
}
