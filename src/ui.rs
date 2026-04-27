use adw::prelude::*;
use gtk4::gio;
use gtk4::prelude::*;
use libadwaita as adw;

use crate::{digging, runner};

pub fn build_ui(app: &adw::Application) {
    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("Firefox Profiles")
        .default_width(360)
        .build();

    let toast_overlay = adw::ToastOverlay::new();
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

    // Spinner shown while profile discovery runs on the background thread
    let spinner = gtk4::Spinner::new();
    spinner.set_spinning(true);
    content.append(&spinner);

    scrolled.set_child(Some(&content));
    toolbar_view.set_content(Some(&scrolled));
    toast_overlay.set_child(Some(&toolbar_view));
    window.set_content(Some(&toast_overlay));
    window.present();

    // Discover profiles off the main thread so slow/NFS home dirs don't freeze the shell panel
    glib::spawn_future_local(async move {
        let browsers = gio::spawn_blocking(digging::get_firefox_profiles)
            .await
            .unwrap_or_else(|_| Vec::new());
        content.remove(&spinner);
        populate_profiles(&content, &toast_overlay, &window, browsers);
    });
}

fn populate_profiles(
    content: &gtk4::Box,
    toast_overlay: &adw::ToastOverlay,
    window: &adw::ApplicationWindow,
    browsers: Vec<digging::BrowserProfiles>,
) {
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
        return;
    }

    for browser in browsers {
        let command = browser.command;
        let group = adw::PreferencesGroup::builder()
            .title(browser.label.as_str())
            .build();

        for profile in browser.profiles {
            let row = adw::ActionRow::builder()
                .title(profile.as_str())
                .activatable(true)
                .build();

            let command_clone = command.clone();
            let win = window.clone();
            let overlay = toast_overlay.clone();

            row.connect_activated(move |_| {
                match runner::open_browser_profile(&command_clone, &profile) {
                    Ok(()) => win.close(),
                    Err(e) => {
                        // Keep the window open so the user can try another profile
                        overlay.add_toast(adw::Toast::new(&e.to_string()));
                    }
                }
            });

            group.add(&row);
        }

        content.append(&group);
    }
}
