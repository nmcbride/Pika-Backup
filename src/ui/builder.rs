#[derive(Clone)]
pub struct AppWindow {
    builder: gtk::Builder,
}

#[derive(Clone)]
pub struct AppWindowWeak {
    builder: glib::WeakRef<gtk::Builder>,
}

impl glib::clone::Downgrade for AppWindow {
    type Weak = AppWindowWeak;

    fn downgrade(&self) -> Self::Weak {
        Self::Weak {
            builder: self.builder.downgrade(),
        }
    }
}

impl glib::clone::Upgrade for AppWindowWeak {
    type Strong = AppWindow;

    fn upgrade(&self) -> Option<Self::Strong> {
        Some(Self::Strong {
            builder: self.builder.upgrade()?,
        })
    }
}

impl AppWindow {
    pub fn new() -> Self {
        Self {
            builder: gtk::Builder::from_string(include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/src/ui/app_window.ui"
            ))),
        }
    }

    fn get<T: glib::IsA<glib::object::Object>>(&self, id: &str) -> T {
        gtk::Builder::object(&self.builder, id)
            .unwrap_or_else(|| panic!("Object with id '{id}' not found in 'src/ui/app_window.ui'"))
    }

    pub fn add_backup(&self) -> gtk::Button {
        self.get("add_backup")
    }

    pub fn add_backup_empty(&self) -> gtk::Button {
        self.get("add_backup_empty")
    }

    pub fn add_exclude(&self) -> gtk::Button {
        self.get("add_exclude")
    }

    pub fn add_include(&self) -> gtk::Button {
        self.get("add_include")
    }

    pub fn archive_list(&self) -> gtk::ListBox {
        self.get("archive_list")
    }

    pub fn archive_list_placeholder(&self) -> gtk::ListBox {
        self.get("archive_list_placeholder")
    }

    pub fn archives_check_abort(&self) -> gtk::Button {
        self.get("archives_check_abort")
    }

    pub fn archives_check_now(&self) -> gtk::Button {
        self.get("archives_check_now")
    }

    pub fn archives_cleanup(&self) -> adw::ActionRow {
        self.get("archives_cleanup")
    }

    pub fn archives_eject_button(&self) -> gtk::Button {
        self.get("archives_eject_button")
    }

    pub fn archives_fs_usage(&self) -> gtk::LevelBar {
        self.get("archives_fs_usage")
    }

    pub fn archives_location_icon(&self) -> gtk::Image {
        self.get("archives_location_icon")
    }

    pub fn archives_location_subtitle(&self) -> gtk::Label {
        self.get("archives_location_subtitle")
    }

    pub fn archives_location_suffix_subtitle(&self) -> gtk::Label {
        self.get("archives_location_suffix_subtitle")
    }

    pub fn archives_location_suffix_title(&self) -> gtk::Label {
        self.get("archives_location_suffix_title")
    }

    pub fn archives_location_title(&self) -> gtk::Label {
        self.get("archives_location_title")
    }

    pub fn archives_prefix(&self) -> gtk::Label {
        self.get("archives_prefix")
    }

    pub fn archives_prefix_edit(&self) -> gtk::Button {
        self.get("archives_prefix_edit")
    }

    pub fn archives_reloading_spinner(&self) -> gtk::Spinner {
        self.get("archives_reloading_spinner")
    }

    pub fn archives_reloading_stack(&self) -> gtk::Stack {
        self.get("archives_reloading_stack")
    }

    pub fn archives_stack(&self) -> gtk::Stack {
        self.get("archives_stack")
    }

    pub fn backup_disk_disconnected(&self) -> gtk::Box {
        self.get("backup_disk_disconnected")
    }

    pub fn backup_disk_eject_button(&self) -> gtk::Button {
        self.get("backup_disk_eject_button")
    }

    pub fn backup_exclude(&self) -> gtk::ListBox {
        self.get("backup_exclude")
    }

    pub fn backup_run(&self) -> gtk::Button {
        self.get("backup_run")
    }

    pub fn check_status(&self) -> crate::ui::export::StatusRow {
        self.get("check_status")
    }

    pub fn detail_current_path(&self) -> gtk::Label {
        self.get("detail_current_path")
    }

    pub fn detail_deduplicated_size(&self) -> gtk::Label {
        self.get("detail_deduplicated_size")
    }

    pub fn detail_header_bar(&self) -> adw::HeaderBar {
        self.get("detail_header_bar")
    }

    pub fn detail_hint_icon(&self) -> gtk::Image {
        self.get("detail_hint_icon")
    }

    pub fn detail_info_error(&self) -> gtk::Label {
        self.get("detail_info_error")
    }

    pub fn detail_info_progress(&self) -> gtk::ProgressBar {
        self.get("detail_info_progress")
    }

    pub fn detail_info_status(&self) -> crate::ui::export::StatusRow {
        self.get("detail_info_status")
    }

    pub fn detail_nfiles(&self) -> gtk::Label {
        self.get("detail_nfiles")
    }

    pub fn detail_original_size(&self) -> gtk::Label {
        self.get("detail_original_size")
    }

    pub fn detail_path_row(&self) -> adw::ActionRow {
        self.get("detail_path_row")
    }

    pub fn detail_repo_icon(&self) -> gtk::Image {
        self.get("detail_repo_icon")
    }

    pub fn detail_repo_row(&self) -> adw::ActionRow {
        self.get("detail_repo_row")
    }

    pub fn detail_running_backup_info(&self) -> adw::Window {
        self.get("detail_running_backup_info")
    }

    pub fn detail_stack(&self) -> adw::ViewStack {
        self.get("detail_stack")
    }

    pub fn detail_stats(&self) -> gtk::ListBox {
        self.get("detail_stats")
    }

    pub fn detail_status_row(&self) -> crate::ui::export::StatusRow {
        self.get("detail_status_row")
    }

    pub fn detail_view_switcher(&self) -> adw::ViewSwitcher {
        self.get("detail_view_switcher")
    }

    pub fn detail_view_switcher_bar(&self) -> adw::ViewSwitcherBar {
        self.get("detail_view_switcher_bar")
    }

    pub fn dialog_check_result(&self) -> crate::ui::export::DialogCheckResult {
        self.get("dialog_check_result")
    }

    pub fn include(&self) -> gtk::ListBox {
        self.get("include")
    }

    pub fn main_backups(&self) -> gtk::ListBox {
        self.get("main_backups")
    }

    pub fn main_stack(&self) -> adw::ViewStack {
        self.get("main_stack")
    }

    pub fn navigation_page_detail(&self) -> adw::NavigationPage {
        self.get("navigation_page_detail")
    }

    pub fn navigation_page_overview(&self) -> adw::NavigationPage {
        self.get("navigation_page_overview")
    }

    pub fn navigation_view(&self) -> adw::NavigationView {
        self.get("navigation_view")
    }

    pub fn overview(&self) -> adw::ToolbarView {
        self.get("overview")
    }

    pub fn page_archives(&self) -> adw::PreferencesPage {
        self.get("page_archives")
    }

    pub fn page_backup(&self) -> adw::PreferencesPage {
        self.get("page_backup")
    }

    pub fn page_detail(&self) -> adw::ToolbarView {
        self.get("page_detail")
    }

    pub fn page_overview(&self) -> adw::PreferencesPage {
        self.get("page_overview")
    }

    pub fn page_overview_empty(&self) -> adw::StatusPage {
        self.get("page_overview_empty")
    }

    pub fn page_schedule(&self) -> adw::PreferencesPage {
        self.get("page_schedule")
    }

    pub fn pending_menu(&self) -> gtk::MenuButton {
        self.get("pending_menu")
    }

    pub fn pending_menu_spinner(&self) -> gtk::Spinner {
        self.get("pending_menu_spinner")
    }

    pub fn preferred_time_row(&self) -> adw::ActionRow {
        self.get("preferred_time_row")
    }

    pub fn preferred_weekday_row(&self) -> adw::ComboRow {
        self.get("preferred_weekday_row")
    }

    pub fn primary_menu_button(&self) -> gtk::MenuButton {
        self.get("primary_menu_button")
    }

    pub fn prune_detail(&self) -> adw::ExpanderRow {
        self.get("prune_detail")
    }

    pub fn prune_enabled(&self) -> gtk::Switch {
        self.get("prune_enabled")
    }

    pub fn prune_preset(&self) -> adw::ComboRow {
        self.get("prune_preset")
    }

    pub fn prune_save(&self) -> gtk::Button {
        self.get("prune_save")
    }

    pub fn prune_save_revealer(&self) -> gtk::Revealer {
        self.get("prune_save_revealer")
    }

    pub fn refresh_archives(&self) -> gtk::Button {
        self.get("refresh_archives")
    }

    pub fn schedule_active(&self) -> adw::ExpanderRow {
        self.get("schedule_active")
    }

    pub fn schedule_frequency(&self) -> adw::ComboRow {
        self.get("schedule_frequency")
    }

    pub fn schedule_keep_daily(&self) -> adw::SpinRow {
        self.get("schedule_keep_daily")
    }

    pub fn schedule_keep_hourly(&self) -> adw::SpinRow {
        self.get("schedule_keep_hourly")
    }

    pub fn schedule_keep_monthly(&self) -> adw::SpinRow {
        self.get("schedule_keep_monthly")
    }

    pub fn schedule_keep_weekly(&self) -> adw::SpinRow {
        self.get("schedule_keep_weekly")
    }

    pub fn schedule_keep_yearly(&self) -> adw::SpinRow {
        self.get("schedule_keep_yearly")
    }

    pub fn schedule_preferred_day(&self) -> adw::SpinRow {
        self.get("schedule_preferred_day")
    }

    pub fn schedule_preferred_hour(&self) -> gtk::SpinButton {
        self.get("schedule_preferred_hour")
    }

    pub fn schedule_preferred_minute(&self) -> gtk::SpinButton {
        self.get("schedule_preferred_minute")
    }

    pub fn schedule_preferred_time_button(&self) -> gtk::MenuButton {
        self.get("schedule_preferred_time_button")
    }

    pub fn schedule_preferred_time_popover(&self) -> gtk::Popover {
        self.get("schedule_preferred_time_popover")
    }

    pub fn schedule_status(&self) -> crate::ui::export::StatusRow {
        self.get("schedule_status")
    }

    pub fn schedule_status_list(&self) -> gtk::ListBox {
        self.get("schedule_status_list")
    }

    pub fn stop_backup_create(&self) -> gtk::Button {
        self.get("stop_backup_create")
    }

    pub fn toast(&self) -> adw::ToastOverlay {
        self.get("toast")
    }

    pub fn window(&self) -> adw::ApplicationWindow {
        self.get("window")
    }
}

#[derive(Clone)]
pub struct DialogAbout {
    builder: gtk::Builder,
}

#[derive(Clone)]
pub struct DialogAboutWeak {
    builder: glib::WeakRef<gtk::Builder>,
}

impl glib::clone::Downgrade for DialogAbout {
    type Weak = DialogAboutWeak;

    fn downgrade(&self) -> Self::Weak {
        Self::Weak {
            builder: self.builder.downgrade(),
        }
    }
}

impl glib::clone::Upgrade for DialogAboutWeak {
    type Strong = DialogAbout;

    fn upgrade(&self) -> Option<Self::Strong> {
        Some(Self::Strong {
            builder: self.builder.upgrade()?,
        })
    }
}

impl DialogAbout {
    pub fn new() -> Self {
        Self {
            builder: gtk::Builder::from_string(include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/src/ui/dialog_about.ui"
            ))),
        }
    }

    fn get<T: glib::IsA<glib::object::Object>>(&self, id: &str) -> T {
        gtk::Builder::object(&self.builder, id).unwrap_or_else(|| {
            panic!("Object with id '{id}' not found in 'src/ui/dialog_about.ui'")
        })
    }

    pub fn dialog(&self) -> adw::AboutWindow {
        self.get("dialog")
    }
}

#[derive(Clone)]
pub struct DialogArchivePrefix {
    builder: gtk::Builder,
}

#[derive(Clone)]
pub struct DialogArchivePrefixWeak {
    builder: glib::WeakRef<gtk::Builder>,
}

impl glib::clone::Downgrade for DialogArchivePrefix {
    type Weak = DialogArchivePrefixWeak;

    fn downgrade(&self) -> Self::Weak {
        Self::Weak {
            builder: self.builder.downgrade(),
        }
    }
}

impl glib::clone::Upgrade for DialogArchivePrefixWeak {
    type Strong = DialogArchivePrefix;

    fn upgrade(&self) -> Option<Self::Strong> {
        Some(Self::Strong {
            builder: self.builder.upgrade()?,
        })
    }
}

impl DialogArchivePrefix {
    pub fn new() -> Self {
        Self {
            builder: gtk::Builder::from_string(include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/src/ui/dialog_archive_prefix.ui"
            ))),
        }
    }

    fn get<T: glib::IsA<glib::object::Object>>(&self, id: &str) -> T {
        gtk::Builder::object(&self.builder, id).unwrap_or_else(|| {
            panic!("Object with id '{id}' not found in 'src/ui/dialog_archive_prefix.ui'")
        })
    }

    pub fn archive_prefix(&self) -> adw::EntryRow {
        self.get("archive_prefix")
    }

    pub fn cancel(&self) -> gtk::Button {
        self.get("cancel")
    }

    pub fn dialog(&self) -> adw::Window {
        self.get("dialog")
    }

    pub fn ok(&self) -> gtk::Button {
        self.get("ok")
    }
}

#[derive(Clone)]
pub struct DialogDeleteArchive {
    builder: gtk::Builder,
}

#[derive(Clone)]
pub struct DialogDeleteArchiveWeak {
    builder: glib::WeakRef<gtk::Builder>,
}

impl glib::clone::Downgrade for DialogDeleteArchive {
    type Weak = DialogDeleteArchiveWeak;

    fn downgrade(&self) -> Self::Weak {
        Self::Weak {
            builder: self.builder.downgrade(),
        }
    }
}

impl glib::clone::Upgrade for DialogDeleteArchiveWeak {
    type Strong = DialogDeleteArchive;

    fn upgrade(&self) -> Option<Self::Strong> {
        Some(Self::Strong {
            builder: self.builder.upgrade()?,
        })
    }
}

impl DialogDeleteArchive {
    pub fn new() -> Self {
        Self {
            builder: gtk::Builder::from_string(include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/src/ui/dialog_delete_archive.ui"
            ))),
        }
    }

    fn get<T: glib::IsA<glib::object::Object>>(&self, id: &str) -> T {
        gtk::Builder::object(&self.builder, id).unwrap_or_else(|| {
            panic!("Object with id '{id}' not found in 'src/ui/dialog_delete_archive.ui'")
        })
    }

    pub fn cancel(&self) -> gtk::Button {
        self.get("cancel")
    }

    pub fn date(&self) -> gtk::Label {
        self.get("date")
    }

    pub fn delete(&self) -> gtk::Button {
        self.get("delete")
    }

    pub fn dialog(&self) -> adw::Window {
        self.get("dialog")
    }

    pub fn name(&self) -> gtk::Label {
        self.get("name")
    }

    pub fn page_decision(&self) -> adw::ToolbarView {
        self.get("page_decision")
    }
}

#[derive(Clone)]
pub struct DialogDeviceMissing {
    builder: gtk::Builder,
}

#[derive(Clone)]
pub struct DialogDeviceMissingWeak {
    builder: glib::WeakRef<gtk::Builder>,
}

impl glib::clone::Downgrade for DialogDeviceMissing {
    type Weak = DialogDeviceMissingWeak;

    fn downgrade(&self) -> Self::Weak {
        Self::Weak {
            builder: self.builder.downgrade(),
        }
    }
}

impl glib::clone::Upgrade for DialogDeviceMissingWeak {
    type Strong = DialogDeviceMissing;

    fn upgrade(&self) -> Option<Self::Strong> {
        Some(Self::Strong {
            builder: self.builder.upgrade()?,
        })
    }
}

impl DialogDeviceMissing {
    pub fn new() -> Self {
        Self {
            builder: gtk::Builder::from_string(include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/src/ui/dialog_device_missing.ui"
            ))),
        }
    }

    fn get<T: glib::IsA<glib::object::Object>>(&self, id: &str) -> T {
        gtk::Builder::object(&self.builder, id).unwrap_or_else(|| {
            panic!("Object with id '{id}' not found in 'src/ui/dialog_device_missing.ui'")
        })
    }

    pub fn icon(&self) -> gtk::Box {
        self.get("icon")
    }

    pub fn name(&self) -> gtk::Label {
        self.get("name")
    }

    pub fn window(&self) -> adw::Window {
        self.get("window")
    }
}

#[derive(Clone)]
pub struct DialogEncryptionPassword {
    builder: gtk::Builder,
}

#[derive(Clone)]
pub struct DialogEncryptionPasswordWeak {
    builder: glib::WeakRef<gtk::Builder>,
}

impl glib::clone::Downgrade for DialogEncryptionPassword {
    type Weak = DialogEncryptionPasswordWeak;

    fn downgrade(&self) -> Self::Weak {
        Self::Weak {
            builder: self.builder.downgrade(),
        }
    }
}

impl glib::clone::Upgrade for DialogEncryptionPasswordWeak {
    type Strong = DialogEncryptionPassword;

    fn upgrade(&self) -> Option<Self::Strong> {
        Some(Self::Strong {
            builder: self.builder.upgrade()?,
        })
    }
}

impl DialogEncryptionPassword {
    pub fn new() -> Self {
        Self {
            builder: gtk::Builder::from_string(include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/src/ui/dialog_encryption_password.ui"
            ))),
        }
    }

    fn get<T: glib::IsA<glib::object::Object>>(&self, id: &str) -> T {
        gtk::Builder::object(&self.builder, id).unwrap_or_else(|| {
            panic!("Object with id '{id}' not found in 'src/ui/dialog_encryption_password.ui'")
        })
    }

    pub fn dialog(&self) -> adw::MessageDialog {
        self.get("dialog")
    }

    pub fn password(&self) -> gtk::PasswordEntry {
        self.get("password")
    }
}

#[derive(Clone)]
pub struct DialogExclude {
    builder: gtk::Builder,
}

#[derive(Clone)]
pub struct DialogExcludeWeak {
    builder: glib::WeakRef<gtk::Builder>,
}

impl glib::clone::Downgrade for DialogExclude {
    type Weak = DialogExcludeWeak;

    fn downgrade(&self) -> Self::Weak {
        Self::Weak {
            builder: self.builder.downgrade(),
        }
    }
}

impl glib::clone::Upgrade for DialogExcludeWeak {
    type Strong = DialogExclude;

    fn upgrade(&self) -> Option<Self::Strong> {
        Some(Self::Strong {
            builder: self.builder.upgrade()?,
        })
    }
}

impl DialogExclude {
    pub fn new() -> Self {
        Self {
            builder: gtk::Builder::from_string(include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/src/ui/dialog_exclude.ui"
            ))),
        }
    }

    fn get<T: glib::IsA<glib::object::Object>>(&self, id: &str) -> T {
        gtk::Builder::object(&self.builder, id).unwrap_or_else(|| {
            panic!("Object with id '{id}' not found in 'src/ui/dialog_exclude.ui'")
        })
    }

    pub fn dialog(&self) -> adw::Window {
        self.get("dialog")
    }

    pub fn exclude_file(&self) -> adw::ActionRow {
        self.get("exclude_file")
    }

    pub fn exclude_folder(&self) -> adw::ActionRow {
        self.get("exclude_folder")
    }

    pub fn exclude_pattern(&self) -> adw::ActionRow {
        self.get("exclude_pattern")
    }

    pub fn suggestions(&self) -> adw::PreferencesGroup {
        self.get("suggestions")
    }

    pub fn unreadable_paths(&self) -> adw::PreferencesGroup {
        self.get("unreadable_paths")
    }
}

#[derive(Clone)]
pub struct DialogExcludePattern {
    builder: gtk::Builder,
}

#[derive(Clone)]
pub struct DialogExcludePatternWeak {
    builder: glib::WeakRef<gtk::Builder>,
}

impl glib::clone::Downgrade for DialogExcludePattern {
    type Weak = DialogExcludePatternWeak;

    fn downgrade(&self) -> Self::Weak {
        Self::Weak {
            builder: self.builder.downgrade(),
        }
    }
}

impl glib::clone::Upgrade for DialogExcludePatternWeak {
    type Strong = DialogExcludePattern;

    fn upgrade(&self) -> Option<Self::Strong> {
        Some(Self::Strong {
            builder: self.builder.upgrade()?,
        })
    }
}

impl DialogExcludePattern {
    pub fn new() -> Self {
        Self {
            builder: gtk::Builder::from_string(include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/src/ui/dialog_exclude_pattern.ui"
            ))),
        }
    }

    fn get<T: glib::IsA<glib::object::Object>>(&self, id: &str) -> T {
        gtk::Builder::object(&self.builder, id).unwrap_or_else(|| {
            panic!("Object with id '{id}' not found in 'src/ui/dialog_exclude_pattern.ui'")
        })
    }

    pub fn add(&self) -> gtk::Button {
        self.get("add")
    }

    pub fn dialog(&self) -> adw::Window {
        self.get("dialog")
    }

    pub fn pattern(&self) -> adw::EntryRow {
        self.get("pattern")
    }

    pub fn pattern_type(&self) -> adw::ComboRow {
        self.get("pattern_type")
    }
}

#[derive(Clone)]
pub struct DialogPrune {
    builder: gtk::Builder,
}

#[derive(Clone)]
pub struct DialogPruneWeak {
    builder: glib::WeakRef<gtk::Builder>,
}

impl glib::clone::Downgrade for DialogPrune {
    type Weak = DialogPruneWeak;

    fn downgrade(&self) -> Self::Weak {
        Self::Weak {
            builder: self.builder.downgrade(),
        }
    }
}

impl glib::clone::Upgrade for DialogPruneWeak {
    type Strong = DialogPrune;

    fn upgrade(&self) -> Option<Self::Strong> {
        Some(Self::Strong {
            builder: self.builder.upgrade()?,
        })
    }
}

impl DialogPrune {
    pub fn new() -> Self {
        Self {
            builder: gtk::Builder::from_string(include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/src/ui/dialog_prune.ui"
            ))),
        }
    }

    fn get<T: glib::IsA<glib::object::Object>>(&self, id: &str) -> T {
        gtk::Builder::object(&self.builder, id).unwrap_or_else(|| {
            panic!("Object with id '{id}' not found in 'src/ui/dialog_prune.ui'")
        })
    }

    pub fn cancel(&self) -> gtk::Button {
        self.get("cancel")
    }

    pub fn delete(&self) -> gtk::Button {
        self.get("delete")
    }

    pub fn dialog(&self) -> adw::Window {
        self.get("dialog")
    }

    pub fn keep(&self) -> gtk::Label {
        self.get("keep")
    }

    pub fn page_decision(&self) -> adw::ToolbarView {
        self.get("page_decision")
    }

    pub fn prune(&self) -> gtk::Label {
        self.get("prune")
    }

    pub fn stack(&self) -> gtk::Stack {
        self.get("stack")
    }

    pub fn untouched(&self) -> gtk::Label {
        self.get("untouched")
    }
}

#[derive(Clone)]
pub struct DialogPruneReview {
    builder: gtk::Builder,
}

#[derive(Clone)]
pub struct DialogPruneReviewWeak {
    builder: glib::WeakRef<gtk::Builder>,
}

impl glib::clone::Downgrade for DialogPruneReview {
    type Weak = DialogPruneReviewWeak;

    fn downgrade(&self) -> Self::Weak {
        Self::Weak {
            builder: self.builder.downgrade(),
        }
    }
}

impl glib::clone::Upgrade for DialogPruneReviewWeak {
    type Strong = DialogPruneReview;

    fn upgrade(&self) -> Option<Self::Strong> {
        Some(Self::Strong {
            builder: self.builder.upgrade()?,
        })
    }
}

impl DialogPruneReview {
    pub fn new() -> Self {
        Self {
            builder: gtk::Builder::from_string(include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/src/ui/dialog_prune_review.ui"
            ))),
        }
    }

    fn get<T: glib::IsA<glib::object::Object>>(&self, id: &str) -> T {
        gtk::Builder::object(&self.builder, id).unwrap_or_else(|| {
            panic!("Object with id '{id}' not found in 'src/ui/dialog_prune_review.ui'")
        })
    }

    pub fn apply(&self) -> gtk::Button {
        self.get("apply")
    }

    pub fn dialog(&self) -> adw::Window {
        self.get("dialog")
    }

    pub fn keep(&self) -> gtk::Label {
        self.get("keep")
    }

    pub fn page_decision(&self) -> adw::ToolbarView {
        self.get("page_decision")
    }

    pub fn prune(&self) -> gtk::Label {
        self.get("prune")
    }

    pub fn stack(&self) -> gtk::Stack {
        self.get("stack")
    }

    pub fn untouched(&self) -> gtk::Label {
        self.get("untouched")
    }
}

#[derive(Clone)]
pub struct DialogSetup {
    builder: gtk::Builder,
}

#[derive(Clone)]
pub struct DialogSetupWeak {
    builder: glib::WeakRef<gtk::Builder>,
}

impl glib::clone::Downgrade for DialogSetup {
    type Weak = DialogSetupWeak;

    fn downgrade(&self) -> Self::Weak {
        Self::Weak {
            builder: self.builder.downgrade(),
        }
    }
}

impl glib::clone::Upgrade for DialogSetupWeak {
    type Strong = DialogSetup;

    fn upgrade(&self) -> Option<Self::Strong> {
        Some(Self::Strong {
            builder: self.builder.upgrade()?,
        })
    }
}

impl DialogSetup {
    pub fn new() -> Self {
        Self {
            builder: gtk::Builder::from_string(include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/src/ui/dialog_setup.ui"
            ))),
        }
    }

    fn get<T: glib::IsA<glib::object::Object>>(&self, id: &str) -> T {
        gtk::Builder::object(&self.builder, id).unwrap_or_else(|| {
            panic!("Object with id '{id}' not found in 'src/ui/dialog_setup.ui'")
        })
    }

    pub fn add_button(&self) -> gtk::Button {
        self.get("add_button")
    }

    pub fn add_local_row(&self) -> adw::ActionRow {
        self.get("add_local_row")
    }

    pub fn add_remote_row(&self) -> adw::ActionRow {
        self.get("add_remote_row")
    }

    pub fn add_repo_list(&self) -> gtk::ListBox {
        self.get("add_repo_list")
    }

    pub fn add_task(&self) -> crate::ui::export::AddConfigTask {
        self.get("add_task")
    }

    pub fn ask_password(&self) -> gtk::PasswordEntry {
        self.get("ask_password")
    }

    pub fn button_stack(&self) -> gtk::Stack {
        self.get("button_stack")
    }

    pub fn command_line_args_entry(&self) -> adw::EntryRow {
        self.get("command_line_args_entry")
    }

    pub fn creating_repository_spinner(&self) -> gtk::Spinner {
        self.get("creating_repository_spinner")
    }

    pub fn dialog(&self) -> adw::Window {
        self.get("dialog")
    }

    pub fn encryption_preferences_group(&self) -> crate::ui::export::EncryptionPreferencesGroup {
        self.get("encryption_preferences_group")
    }

    pub fn init_button(&self) -> gtk::Button {
        self.get("init_button")
    }

    pub fn init_dir(&self) -> adw::EntryRow {
        self.get("init_dir")
    }

    pub fn init_local_row(&self) -> adw::ActionRow {
        self.get("init_local_row")
    }

    pub fn init_path(&self) -> crate::ui::export::FolderButton {
        self.get("init_path")
    }

    pub fn init_remote_row(&self) -> adw::ActionRow {
        self.get("init_remote_row")
    }

    pub fn init_repo_list(&self) -> gtk::ListBox {
        self.get("init_repo_list")
    }

    pub fn location_group_local(&self) -> adw::PreferencesGroup {
        self.get("location_group_local")
    }

    pub fn location_group_remote(&self) -> adw::PreferencesGroup {
        self.get("location_group_remote")
    }

    pub fn location_local(&self) -> adw::ActionRow {
        self.get("location_local")
    }

    pub fn location_url(&self) -> adw::EntryRow {
        self.get("location_url")
    }

    pub fn navigation_view(&self) -> adw::NavigationView {
        self.get("navigation_view")
    }

    pub fn non_journaling_warning(&self) -> gtk::Box {
        self.get("non_journaling_warning")
    }

    pub fn page_creating(&self) -> adw::NavigationPage {
        self.get("page_creating")
    }

    pub fn page_detail(&self) -> adw::NavigationPage {
        self.get("page_detail")
    }

    pub fn page_detail_continue(&self) -> gtk::Button {
        self.get("page_detail_continue")
    }

    pub fn page_overview(&self) -> adw::NavigationPage {
        self.get("page_overview")
    }

    pub fn page_password(&self) -> adw::NavigationPage {
        self.get("page_password")
    }

    pub fn page_password_continue(&self) -> gtk::Button {
        self.get("page_password_continue")
    }

    pub fn page_password_input(&self) -> adw::ToolbarView {
        self.get("page_password_input")
    }

    pub fn page_password_pending(&self) -> gtk::WindowHandle {
        self.get("page_password_pending")
    }

    pub fn page_password_stack(&self) -> gtk::Stack {
        self.get("page_password_stack")
    }

    pub fn page_setup_encryption(&self) -> adw::NavigationPage {
        self.get("page_setup_encryption")
    }

    pub fn page_transfer(&self) -> adw::NavigationPage {
        self.get("page_transfer")
    }

    pub fn page_transfer_pending(&self) -> adw::ToolbarView {
        self.get("page_transfer_pending")
    }

    pub fn page_transfer_prefix(&self) -> adw::NavigationPage {
        self.get("page_transfer_prefix")
    }

    pub fn page_transfer_select(&self) -> adw::ToolbarView {
        self.get("page_transfer_select")
    }

    pub fn page_transfer_stack(&self) -> gtk::Stack {
        self.get("page_transfer_stack")
    }

    pub fn pending_spinner(&self) -> gtk::Spinner {
        self.get("pending_spinner")
    }

    pub fn prefix(&self) -> gtk::Entry {
        self.get("prefix")
    }

    pub fn prefix_submit(&self) -> gtk::Button {
        self.get("prefix_submit")
    }

    pub fn show_settings(&self) -> gtk::ToggleButton {
        self.get("show_settings")
    }

    pub fn transfer_pending_spinner(&self) -> gtk::Spinner {
        self.get("transfer_pending_spinner")
    }

    pub fn transfer_suggestions(&self) -> gtk::ListBox {
        self.get("transfer_suggestions")
    }
}

#[derive(Clone)]
pub struct DialogSetupTransferOption {
    builder: gtk::Builder,
}

#[derive(Clone)]
pub struct DialogSetupTransferOptionWeak {
    builder: glib::WeakRef<gtk::Builder>,
}

impl glib::clone::Downgrade for DialogSetupTransferOption {
    type Weak = DialogSetupTransferOptionWeak;

    fn downgrade(&self) -> Self::Weak {
        Self::Weak {
            builder: self.builder.downgrade(),
        }
    }
}

impl glib::clone::Upgrade for DialogSetupTransferOptionWeak {
    type Strong = DialogSetupTransferOption;

    fn upgrade(&self) -> Option<Self::Strong> {
        Some(Self::Strong {
            builder: self.builder.upgrade()?,
        })
    }
}

impl DialogSetupTransferOption {
    pub fn new() -> Self {
        Self {
            builder: gtk::Builder::from_string(include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/src/ui/dialog_setup_transfer_option.ui"
            ))),
        }
    }

    fn get<T: glib::IsA<glib::object::Object>>(&self, id: &str) -> T {
        gtk::Builder::object(&self.builder, id).unwrap_or_else(|| {
            panic!("Object with id '{id}' not found in 'src/ui/dialog_setup_transfer_option.ui'")
        })
    }

    pub fn exclude(&self) -> crate::ui::export::WrapBox {
        self.get("exclude")
    }

    pub fn hostname(&self) -> gtk::Label {
        self.get("hostname")
    }

    pub fn include(&self) -> crate::ui::export::WrapBox {
        self.get("include")
    }

    pub fn prefix(&self) -> gtk::Label {
        self.get("prefix")
    }

    pub fn transfer(&self) -> adw::ActionRow {
        self.get("transfer")
    }

    pub fn username(&self) -> gtk::Label {
        self.get("username")
    }

    pub fn widget(&self) -> gtk::ListBoxRow {
        self.get("widget")
    }
}

#[derive(Clone)]
pub struct DialogStorage {
    builder: gtk::Builder,
}

#[derive(Clone)]
pub struct DialogStorageWeak {
    builder: glib::WeakRef<gtk::Builder>,
}

impl glib::clone::Downgrade for DialogStorage {
    type Weak = DialogStorageWeak;

    fn downgrade(&self) -> Self::Weak {
        Self::Weak {
            builder: self.builder.downgrade(),
        }
    }
}

impl glib::clone::Upgrade for DialogStorageWeak {
    type Strong = DialogStorage;

    fn upgrade(&self) -> Option<Self::Strong> {
        Some(Self::Strong {
            builder: self.builder.upgrade()?,
        })
    }
}

impl DialogStorage {
    pub fn new() -> Self {
        Self {
            builder: gtk::Builder::from_string(include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/src/ui/dialog_storage.ui"
            ))),
        }
    }

    fn get<T: glib::IsA<glib::object::Object>>(&self, id: &str) -> T {
        gtk::Builder::object(&self.builder, id).unwrap_or_else(|| {
            panic!("Object with id '{id}' not found in 'src/ui/dialog_storage.ui'")
        })
    }

    pub fn device(&self) -> adw::ActionRow {
        self.get("device")
    }

    pub fn dialog(&self) -> adw::Window {
        self.get("dialog")
    }

    pub fn disk(&self) -> adw::PreferencesGroup {
        self.get("disk")
    }

    pub fn fs(&self) -> adw::PreferencesGroup {
        self.get("fs")
    }

    pub fn fs_free(&self) -> adw::ActionRow {
        self.get("fs_free")
    }

    pub fn fs_size(&self) -> adw::ActionRow {
        self.get("fs_size")
    }

    pub fn fs_usage(&self) -> gtk::LevelBar {
        self.get("fs_usage")
    }

    pub fn path(&self) -> adw::ActionRow {
        self.get("path")
    }

    pub fn remote(&self) -> adw::PreferencesGroup {
        self.get("remote")
    }

    pub fn uri(&self) -> adw::ActionRow {
        self.get("uri")
    }

    pub fn volume(&self) -> adw::ActionRow {
        self.get("volume")
    }
}

#[derive(Clone)]
pub struct OverviewItem {
    builder: gtk::Builder,
}

#[derive(Clone)]
pub struct OverviewItemWeak {
    builder: glib::WeakRef<gtk::Builder>,
}

impl glib::clone::Downgrade for OverviewItem {
    type Weak = OverviewItemWeak;

    fn downgrade(&self) -> Self::Weak {
        Self::Weak {
            builder: self.builder.downgrade(),
        }
    }
}

impl glib::clone::Upgrade for OverviewItemWeak {
    type Strong = OverviewItem;

    fn upgrade(&self) -> Option<Self::Strong> {
        Some(Self::Strong {
            builder: self.builder.upgrade()?,
        })
    }
}

impl OverviewItem {
    pub fn new() -> Self {
        Self {
            builder: gtk::Builder::from_string(include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/src/ui/overview_item.ui"
            ))),
        }
    }

    fn get<T: glib::IsA<glib::object::Object>>(&self, id: &str) -> T {
        gtk::Builder::object(&self.builder, id).unwrap_or_else(|| {
            panic!("Object with id '{id}' not found in 'src/ui/overview_item.ui'")
        })
    }

    pub fn include(&self) -> crate::ui::export::WrapBox {
        self.get("include")
    }

    pub fn location(&self) -> adw::ActionRow {
        self.get("location")
    }

    pub fn location_icon(&self) -> gtk::Image {
        self.get("location_icon")
    }

    pub fn location_subtitle(&self) -> gtk::Label {
        self.get("location_subtitle")
    }

    pub fn location_title(&self) -> gtk::Label {
        self.get("location_title")
    }

    pub fn schedule(&self) -> crate::ui::export::StatusRow {
        self.get("schedule")
    }

    pub fn status(&self) -> crate::ui::export::StatusRow {
        self.get("status")
    }

    pub fn widget(&self) -> gtk::ListBoxRow {
        self.get("widget")
    }
}
