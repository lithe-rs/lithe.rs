use anyhow::{Context, Result};
use log::{info, warn};
use std::path::Path;
use std::process::Command;

use crate::generate;

pub fn handle_dev(port: u16) -> Result<()> {
    let project_dir = std::env::current_dir()?;
    info!("Generating routes and server code...");

    generate::generate_all(&project_dir, port)?;

    ensure_cargo_bin_config(&project_dir)?;

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
    let content = std::fs::read_to_string(&cargo_path).context("Failed to read Cargo.toml")?;

    if content.contains("[[bin]]") && content.contains("lithe-app") {
        return Ok(());
    }

    let bin_config = r#"

[[bin]]
name = "lithe-app"
path = ".lithe/main.rs"
"#;

    let new_content = format!("{}{}", content, bin_config);
    std::fs::write(&cargo_path, new_content).context("Failed to update Cargo.toml")?;

    info!("Added [[bin]] configuration to Cargo.toml");
    Ok(())
}
