use crate::build;
use crate::generate;
use anyhow::{Context, Result};
use log::{info, warn};
use regex::Regex;
use std::path::Path;
use std::process::Command;
pub fn handle_dev(port: u16) -> Result<()> {
    let project_dir = std::env::current_dir()?;
    info!("Generating routes and server code...");

    generate::generate_all(&project_dir, port)?;
    ensure_cargo_bin_config(&project_dir)?;
    build::sync_public_assets(&project_dir)?;

    if let Err(e) = build::build_wasm_unified(&project_dir) {
        warn!("WASM build failed: {}", e);
    }

    info!("Starting development server on port {}...", port);
    let status = Command::new("cargo")
        .args(["run", "--bin", "lithe-app"])
        .current_dir(&project_dir)
        .status()
        .context("Failed to run cargo")?;
    if !status.success() {
        warn!("Development server exited with error");
    }
    Ok(())
}
pub fn ensure_cargo_bin_config(project_dir: &Path) -> Result<()> {
    let cargo_path = project_dir.join("Cargo.toml");
    let mut content = std::fs::read_to_string(&cargo_path).context("Failed to read Cargo.toml")?;
    let mut modified = false;

    if !content.contains("path = \".lithe/lib.rs\"") {
        if content.contains("[lib]") {
            let re = Regex::new(r#"\[lib\][^\[]*"#).unwrap();
            let lib_section = r#"[lib]
path = ".lithe/lib.rs"
crate-type = ["cdylib", "rlib"]

"#;
            content = re.replace(&content, lib_section).to_string();
        } else {
            let lib_section = r#"
[lib]
path = ".lithe/lib.rs"
crate-type = ["cdylib", "rlib"]
"#;
            content = content.replace(
                "[dependencies]",
                &format!("{}\n[dependencies]", lib_section),
            );
        }
        modified = true;
    }

    if !content.contains("tower-http") || !content.contains("features = [\"fs\"]") {
        if content.contains("tower-http") {
            let re = Regex::new(r#"tower-http\s*=\s*".*""#).unwrap();
            content = re
                .replace(
                    &content,
                    "tower-http = { version = \"0.6\", features = [\"fs\"] }",
                )
                .to_string();
        } else {
            content = content.replace(
                "axum = \"0.7\"",
                "axum = \"0.7\"\ntower-http = { version = \"0.6\", features = [\"fs\"] }",
            );
        }
        modified = true;
    }

    if !content.contains("[[bin]]") || !content.contains("lithe-app") {
        let bin_config = r#"
[[bin]]
name = "lithe-app"
path = ".lithe/main.rs"
"#;
        content = format!("{}{}", content, bin_config);
        modified = true;
        info!("Added [[bin]] configuration to Cargo.toml");
    }

    if modified {
        std::fs::write(&cargo_path, content).context("Failed to update Cargo.toml")?;
    }

    Ok(())
}
