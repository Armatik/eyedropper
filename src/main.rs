mod application;
#[rustfmt::skip]
mod config;
mod colors;
mod model;
mod widgets;
mod window;

use gettextrs::{LocaleCategory, gettext};
use glib::ExitCode;
use gtk::{gio, glib};

use self::application::App;
use self::config::{GETTEXT_PACKAGE, LOCALEDIR, RESOURCES_FILE};

fn main() -> ExitCode {
    // Initialize logger
    pretty_env_logger::init();

    // ensure Adwaita will be used
    unsafe {
        std::env::remove_var("GTK_THEME");
    }

    // Prepare i18n
    gettextrs::setlocale(LocaleCategory::LcAll, "");
    gettextrs::bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    gettextrs::textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");

    glib::set_application_name(&gettext("Eyedropper"));

    let res = gio::Resource::load(RESOURCES_FILE).expect("Could not load gresource file");
    gio::resources_register(&res);

    let app = App::new();
    app.run()
}
