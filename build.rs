fn main() {
    println!("cargo:rerun-if-changed=extension/metadata.json");

    let text = std::fs::read_to_string("extension/metadata.json").unwrap_or_default();
    let width: i32 = text
        .lines()
        .find(|l| l.contains("\"window-width\""))
        .and_then(|l| l.split(':').nth(1))
        .and_then(|v| v.trim().trim_end_matches(',').parse().ok())
        .unwrap_or(360);

    let out = std::env::var("OUT_DIR").unwrap();
    std::fs::write(
        format!("{out}/constants.rs"),
        format!("const DEFAULT_WINDOW_WIDTH: i32 = {width};\n"),
    )
    .unwrap();
}
