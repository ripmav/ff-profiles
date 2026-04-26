use crate::config_paths::config_paths;

pub struct BrowserProfiles {
    pub label: String,
    pub command: String,
    pub profiles: Vec<String>,
}

/// Returns all browsers that have a profiles.ini file on disk, together with their profiles.
pub fn get_firefox_profiles() -> Vec<BrowserProfiles> {
    config_paths()
        .into_iter()
        .filter(|b| b.path.exists())
        .map(|b| BrowserProfiles {
            label: b.label.to_string(),
            command: b.command.to_string(),
            profiles: profiles_from_ini(&b.path),
        })
        .collect()
}

/// Parses a profiles.ini file and returns every `Name=` value found.
fn profiles_from_ini(path: &std::path::Path) -> Vec<String> {
    std::fs::read_to_string(path)
        .unwrap_or_default()
        .lines()
        .filter_map(|line| line.strip_prefix("Name="))
        .map(str::to_string)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn parses_names_from_ini() {
        let mut f = NamedTempFile::new().unwrap();
        writeln!(
            f,
            "[Profile0]\nName=default-release\nIsRelative=1\n\n[Profile1]\nName=Work\nIsRelative=1"
        )
        .unwrap();
        let names = profiles_from_ini(f.path());
        assert_eq!(names, vec!["default-release", "Work"]);
    }

    #[test]
    fn missing_file_returns_empty() {
        let names = profiles_from_ini(std::path::Path::new("/nonexistent/profiles.ini"));
        assert!(names.is_empty());
    }

    #[test]
    fn ini_with_no_names_returns_empty() {
        let mut f = NamedTempFile::new().unwrap();
        writeln!(f, "[Profile0]\nIsRelative=1\nPath=something").unwrap();
        let names = profiles_from_ini(f.path());
        assert!(names.is_empty());
    }
}
