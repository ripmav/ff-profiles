# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**ff-profiles** is a GNOME Shell extension + native Rust binary that adds a Firefox icon to the GNOME top bar, letting users launch Gecko-based browsers (Firefox, Waterfox, LibreWolf, Floorp, Zen Browser, IceCat, Palemoon) with specific profiles in one click.

Architecture in brief:
1. JavaScript extension (`extension/`) registers a panel indicator and spawns the Rust binary on click.
2. Rust binary (`src/`) discovers profiles across all supported browsers, renders a GTK4/libadwaita window, and launches the selected browser with `-P <profile> -no-remote`.

## Commands

```bash
# Build (debug)
cargo build

# Build (release)
cargo build --release

# Run tests
cargo test

# Run a single test
cargo test <test_name>

# Install (builds release, installs binary + extension + .desktop)
make install

# Uninstall
make uninstall

# Clean
make clean
```

## Architecture

### Source modules (`src/`)

| File | Responsibility |
|------|----------------|
| `main.rs` | GTK4/libadwaita app entry point; single-instance enforcement via `activate` signal |
| `ui.rs` | Builds the profile list window; groups profiles by browser |
| `config_paths.rs` | Returns candidate config directories per browser (XDG, classic `~/.mozilla`, Flatpak paths) |
| `digging.rs` | Parses `profiles.ini` line-by-line to extract `Name=` entries |
| `runner.rs` | Spawns browser with `-P <profile> -no-remote`; detaches via `thread::spawn` |

### Extension (`extension/`)
Minimal GJS: creates a panel button, calls `Gio.Subprocess` to run `ff-profiles`, single file (`extension.js`) + `metadata.json`.

### Key design decisions
- **GLib path helpers** (`glib::home_dir`, `glib::user_config_dir`) are used instead of `std::env` to stay XDG-compliant and match GJS behavior.
- **Multi-word browser commands** (e.g. `flatpak run org.mozilla.firefox`) are split with `split_whitespace()`—no shell quoting.
- **Missing `profiles.ini`** returns empty list silently, not an error.
- **Single-instance**: secondary `app.activate` calls re-present the existing window rather than opening a duplicate.
