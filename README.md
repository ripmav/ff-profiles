# Firefox Profiles

Launch Firefox and other Gecko-based browsers with a specific profile directly from the GNOME panel.

This is a Rust rewrite of the [baxyz/firefox-profiles](https://github.com/baxyz/firefox-profiles) GNOME Shell extension. The original was a pure TypeScript/GJS extension; this version replaces all logic and UI with a native GTK4 + libadwaita application while keeping a minimal JavaScript extension for the GNOME panel indicator.

## How it works

```
GNOME panel indicator (extension.js)
        │
        │  spawns
        ▼
  ff-profiles binary  ──►  GTK4 window (libadwaita)
                                    │
                             select a profile
                                    │
                            firefox -P <profile> -no-remote
```

Clicking the Firefox icon in the GNOME top bar opens a native window listing all detected profiles grouped by browser. Clicking a profile launches that browser with `-P <profile> -no-remote` and closes the window.

## Supported browsers

| Browser | Installation types |
|---|---|
| Firefox | native (XDG + classic), flatpak, snap |
| Waterfox | native (XDG + classic) |
| LibreWolf | native (XDG + classic) |
| Floorp | native (XDG + classic), flatpak |
| Zen Browser | native (XDG + classic), flatpak |
| IceCat | native |
| Palemoon | native |

Profiles are read from each browser's `profiles.ini`. A browser only appears in the list if its configuration file exists.

## Requirements

- GNOME Shell 46–49
- GTK4 ≥ 4.14 and libadwaita ≥ 1.6 (provided by most distros shipping GNOME 46+)
- Rust toolchain (for building from source)

## Installation

```sh
# 1. Build and install the binary + extension + .desktop file
make install

# 2. Enable the GNOME Shell extension
gnome-extensions enable ff-profiles@seblebs.de

# 3. Restart GNOME Shell to load the new extension
#    On Wayland: log out and back in
#    On X11:     press Alt+F2, type r, press Enter
```

`make install` places files in:

| File | Location |
|---|---|
| Binary | `~/.local/bin/ff-profiles` |
| Extension | `~/.local/share/gnome-shell/extensions/ff-profiles@seblebs.de/` |
| Desktop entry | `~/.local/share/applications/de.seblebs.ff-profiles.desktop` |

The binary must be on your `$PATH`. If `~/.local/bin` is not in your `$PATH`, add this to your shell profile:

```sh
export PATH="$HOME/.local/bin:$PATH"
```

## Uninstall

```sh
make uninstall
gnome-extensions disable ff-profiles@seblebs.de
```

## Building manually

```sh
# Debug build
cargo build

# Release build
cargo build --release
./target/release/ff-profiles
```

## Running tests

```sh
cargo test
```

## Project structure

```
ff-profiles/
├── src/
│   ├── main.rs           # GTK4 application entry point
│   ├── ui.rs             # libadwaita window and profile list
│   ├── config_paths.rs   # Browser config file locations (XDG + classic paths)
│   ├── digging.rs        # Reads profiles.ini, extracts profile names
│   └── runner.rs         # Spawns browser with -P <profile> -no-remote
├── extension/
│   ├── extension.js      # GNOME Shell panel indicator (JavaScript, required by GNOME)
│   └── metadata.json     # Extension metadata (UUID, supported shell versions)
├── data/
│   └── de.seblebs.ff-profiles.desktop
└── Makefile
```
