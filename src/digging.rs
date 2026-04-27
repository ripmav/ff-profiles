use crate::config_paths::config_paths;

pub struct BrowserProfiles {
    pub label: String,
    pub command: String,
    pub profiles: Vec<String>,
}

/// Returns every browser that has at least one profile on disk.
///
/// Browsers with an empty or name-less profiles.ini are skipped.
/// If the same file is reachable via two paths (e.g. XDG and classic are symlinked to the
/// same inode), only the first entry for that (canonical-path, command) pair is kept.
pub fn get_firefox_profiles() -> Vec<BrowserProfiles> {
    let mut seen: std::collections::HashSet<(std::path::PathBuf, &'static str)> =
        Default::default();
    let mut result = Vec::new();

    for b in config_paths() {
        if !b.path.exists() {
            continue;
        }
        let canonical = b.path.canonicalize().unwrap_or_else(|_| b.path.clone());
        if !seen.insert((canonical, b.command)) {
            continue;
        }
        let profiles = profiles_from_ini(&b.path);
        if profiles.is_empty() {
            continue;
        }
        result.push(BrowserProfiles {
            label: b.label.to_string(),
            command: b.command.to_string(),
            profiles,
        });
    }
    result
}

/// Extracts every non-empty `Name=` value from a profiles.ini file.
fn profiles_from_ini(path: &std::path::Path) -> Vec<String> {
    std::fs::read_to_string(path)
        .unwrap_or_default()
        .lines()
        .filter_map(|line| line.strip_prefix("Name="))
        .filter(|s| !s.is_empty())
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

    #[test]
    fn blank_name_entry_is_filtered() {
        let mut f = NamedTempFile::new().unwrap();
        writeln!(f, "[Profile0]\nName=\n[Profile1]\nName=real-profile").unwrap();
        let names = profiles_from_ini(f.path());
        assert_eq!(names, vec!["real-profile"]);
    }
}
