mod config_paths;
mod digging;
mod runner;
mod ui;

use libadwaita as adw;
use adw::prelude::*;

const APP_ID: &str = "tech.baxyz.ff-profiles";

fn main() -> glib::ExitCode {
    let app = adw::Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(ui::build_ui);
    app.run()
}
