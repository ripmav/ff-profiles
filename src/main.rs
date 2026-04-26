mod config_paths;
mod digging;
mod runner;
mod ui;

use adw::prelude::*;
use libadwaita as adw;

const APP_ID: &str = "tech.baxyz.ff-profiles";

fn main() -> glib::ExitCode {
    let app = adw::Application::builder()
        .application_id(APP_ID)
        .build();

    // GTK4's single-instance mechanism sends a second `activate` to the already-running
    // process when the indicator is clicked again. Present the existing window instead of
    // opening a duplicate.
    app.connect_activate(|app| {
        if let Some(window) = app.windows().into_iter().next() {
            window.present();
        } else {
            ui::build_ui(app);
        }
    });

    app.run()
}
