use adw::prelude::*;
use gtk4::prelude::*;
use libadwaita as adw;

use crate::{digging, runner};

pub fn build_ui(app: &adw::Application) {
    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("Firefox Profiles")
        .default_width(360)
        .build();

    let toolbar_view = adw::ToolbarView::new();
    toolbar_view.add_top_bar(&adw::HeaderBar::new());

    let scrolled = gtk4::ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .propagate_natural_height(true)
        .max_content_height(640)
        .build();

    let content = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .spacing(18)
        .margin_top(18)
        .margin_bottom(18)
        .margin_start(18)
        .margin_end(18)
        .build();

    let browsers = digging::get_firefox_profiles();

    if browsers.is_empty() {
        let status = adw::StatusPage::builder()
            .title("No profiles found")
            .description(
                "Install Firefox (or a compatible browser) and launch it at least once \
                 to create a profile.",
            )
            .icon_name("web-browser-symbolic")
            .vexpand(true)
            .build();
        content.append(&status);
    } else {
        for browser in browsers {
            let label = browser.label;
            let command = browser.command;

            let group = adw::PreferencesGroup::builder()
                .title(label.as_str())
                .build();

            for profile in browser.profiles {
                let row = adw::ActionRow::builder()
                    .title(profile.as_str())
                    .activatable(true)
                    .build();

                let command_clone = command.clone();
                let win = window.clone();

                row.connect_activated(move |_| {
                    if let Err(e) = runner::open_browser_profile(&command_clone, &profile) {
                        eprintln!("Error launching browser: {e}");
                    }
                    win.close();
                });

                group.add(&row);
            }

            content.append(&group);
        }
    }

    scrolled.set_child(Some(&content));
    toolbar_view.set_content(Some(&scrolled));
    window.set_content(Some(&toolbar_view));
    window.present();
}
