use anyhow::{bail, Context, Result};
use std::process::Command;

/// Launch a Gecko-based browser with the given profile.
///
/// Translates to: `<command> -P <profile> -no-remote`
pub fn open_browser_profile(command: &str, profile: &str) -> Result<()> {
    let mut parts = command.split_whitespace();
    let program = parts.next().expect("command must not be empty");
    let args: Vec<&str> = parts.collect();

    let status = Command::new(program)
        .args(&args)
        .args(["-P", profile, "-no-remote"])
        .status()
        .with_context(|| format!("failed to execute '{command}'"))?;

    if !status.success() {
        bail!(
            "browser exited with {}: could not open profile \"{profile}\"",
            status
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bad_binary_returns_error() {
        let result = open_browser_profile("this-binary-does-not-exist", "default");
        assert!(result.is_err());
    }
}
