use anyhow::{Context, Result};
use std::process::Command;

/// Spawns a Gecko-based browser with the given profile and returns immediately.
///
/// Equivalent to `<command> -P <profile> -no-remote`.
/// Uses std::process::Command so profile names with spaces are passed correctly
/// without shell quoting (unlike the GLib.spawn_command_line_async approach in the
/// original TypeScript extension, which could misparse profiles containing spaces).
pub fn open_browser_profile(command: &str, profile: &str) -> Result<()> {
    let mut parts = command.split_whitespace();
    let program = parts.next().ok_or_else(|| anyhow::anyhow!("command is empty"))?;
    let prefix_args: Vec<&str> = parts.collect();

    let mut child = Command::new(program)
        .args(&prefix_args)
        .args(["-P", profile, "-no-remote"])
        .spawn()
        .with_context(|| format!("failed to execute '{command}'"))?;

    // Detach so the browser outlives this process.
    std::thread::spawn(move || {
        let _ = child.wait();
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_binary_returns_error() {
        let result = open_browser_profile("this-binary-does-not-exist", "default");
        assert!(result.is_err());
    }

    #[test]
    fn empty_command_returns_error() {
        let result = open_browser_profile("", "default");
        assert!(result.is_err());
    }
}
