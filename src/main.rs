mod config_paths;
mod digging;
mod runner;

use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Select};
use digging::BrowserProfiles;

fn main() -> Result<()> {
    let browsers = digging::get_firefox_profiles();

    if browsers.is_empty() {
        eprintln!("No browser configuration files found.");
        eprintln!("Make sure Firefox (or a compatible browser) is installed and has been launched at least once.");
        std::process::exit(1);
    }

    let entries = flatten_profiles(&browsers);

    if entries.is_empty() {
        eprintln!("Configuration files were found but contain no profiles.");
        std::process::exit(1);
    }

    let labels: Vec<String> = entries
        .iter()
        .map(|(label, _, profile)| format!("{label}: {profile}"))
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a profile to launch")
        .items(&labels)
        .default(0)
        .interact()?;

    let (_, command, profile) = &entries[selection];
    runner::open_browser_profile(command, profile)?;

    Ok(())
}

fn flatten_profiles(browsers: &[BrowserProfiles]) -> Vec<(String, String, String)> {
    browsers
        .iter()
        .flat_map(|b| {
            b.profiles
                .iter()
                .map(|p| (b.label.clone(), b.command.clone(), p.clone()))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flatten_empty_browsers() {
        assert!(flatten_profiles(&[]).is_empty());
    }

    #[test]
    fn flatten_combines_profiles() {
        let browsers = vec![
            BrowserProfiles {
                label: "Firefox".into(),
                command: "firefox".into(),
                profiles: vec!["default".into(), "Work".into()],
            },
            BrowserProfiles {
                label: "LibreWolf".into(),
                command: "librewolf".into(),
                profiles: vec!["personal".into()],
            },
        ];

        let flat = flatten_profiles(&browsers);
        assert_eq!(flat.len(), 3);
        assert_eq!(flat[0], ("Firefox".into(), "firefox".into(), "default".into()));
        assert_eq!(flat[2], ("LibreWolf".into(), "librewolf".into(), "personal".into()));
    }
}
