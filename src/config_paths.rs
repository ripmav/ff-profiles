use std::path::PathBuf;

pub struct BrowserInfo {
    pub label: &'static str,
    pub path: PathBuf,
    pub command: &'static str,
}

/// Returns the list of known browser profile config locations, ordered most-specific first.
///
/// Uses GLib's path helpers so XDG_CONFIG_HOME and $HOME follow the same resolution as in
/// the original GJS extension (GLib.get_home_dir / GLib.getenv("XDG_CONFIG_HOME")).
pub fn config_paths() -> Vec<BrowserInfo> {
    let home = glib::home_dir();
    let xdg_config = glib::user_config_dir();

    vec![
        // Firefox (native) — XDG path first (supported since Firefox 147)
        BrowserInfo {
            label: "Firefox",
            path: xdg_config.join("mozilla/firefox/profiles.ini"),
            command: "firefox",
        },
        BrowserInfo {
            label: "Firefox (classic)",
            path: home.join(".mozilla/firefox/profiles.ini"),
            command: "firefox",
        },
        // Firefox (flatpak)
        BrowserInfo {
            label: "Firefox (flatpak)",
            path: home.join(".var/app/org.mozilla.firefox/.mozilla/firefox/profiles.ini"),
            command: "flatpak run org.mozilla.firefox",
        },
        // Firefox (snap)
        BrowserInfo {
            label: "Firefox (snap)",
            path: home.join("snap/firefox/common/.mozilla/firefox/profiles.ini"),
            command: "snap run firefox",
        },
        // Waterfox
        BrowserInfo {
            label: "Waterfox",
            path: xdg_config.join("waterfox/profiles.ini"),
            command: "waterfox",
        },
        BrowserInfo {
            label: "Waterfox (classic)",
            path: home.join(".waterfox/profiles.ini"),
            command: "waterfox",
        },
        // LibreWolf
        BrowserInfo {
            label: "LibreWolf",
            path: xdg_config.join("librewolf/profiles.ini"),
            command: "librewolf",
        },
        BrowserInfo {
            label: "LibreWolf (classic)",
            path: home.join(".librewolf/profiles.ini"),
            command: "librewolf",
        },
        // Floorp
        BrowserInfo {
            label: "Floorp",
            path: xdg_config.join("floorp/profiles.ini"),
            command: "floorp",
        },
        BrowserInfo {
            label: "Floorp (classic)",
            path: home.join(".floorp/profiles.ini"),
            command: "floorp",
        },
        BrowserInfo {
            label: "Floorp (flatpak)",
            path: home.join(".var/app/one.ablaze.floorp/.floorp/profiles.ini"),
            command: "flatpak run one.ablaze.floorp",
        },
        // Zen Browser
        BrowserInfo {
            label: "Zen",
            path: xdg_config.join("zen/profiles.ini"),
            command: "zen-browser",
        },
        BrowserInfo {
            label: "Zen (classic)",
            path: home.join(".zen/profiles.ini"),
            command: "zen-browser",
        },
        BrowserInfo {
            label: "Zen (flatpak)",
            path: home.join(".var/app/app.zen_browser.zen/.zen/profiles.ini"),
            command: "flatpak run app.zen_browser.zen",
        },
        // IceCat
        BrowserInfo {
            label: "IceCat",
            path: home.join(".icecat/profiles.ini"),
            command: "icecat",
        },
        // Palemoon — the vendor uses spaces in both the org dir and browser dir;
        // PathBuf::join handles them correctly, but the spaces are not a typo.
        BrowserInfo {
            label: "Palemoon",
            path: home.join(".moonchild productions/pale moon/profiles.ini"),
            command: "palemoon",
        },
    ]
}
