# Ecosystem and Distribution

Lithe.rs is designed to be easily distributed and integrated into existing workflows.

## 1. The CLI (Lithe-CLI)
The CLI is written in Rust but distributed via **Bun** and **NPM** using a platform-specific binary trick.

### Distribution Strategy:
1. **Platform Packages:** Separate packages for each target (e.g., `@lithe/cli-linux-x64`).
2. **Main Package:** A thin wrapper (`@lithe/cli`) that uses `optionalDependencies` to install the correct binary for the user's OS and CPU.

## 2. Crates.io
The core logic is split into several crates for maximum modularity:
- **`lithe`:** The primary user-facing library.
- **`lithe-macros`:** Procedural macros for `#[server]`, `#[native]`, etc.
- **`lithe-core`:** Shared runtime logic for the Signal Arena and Bridge.
- **`lithe-cli`:** The build tool logic (available as a library for custom build scripts).

## 3. Tooling Compatibility
Because Lithe.rs is "Pure Rust," it works out of the box with:
- `rust-analyzer`
- `rustfmt`
- `clippy`
- `cargo-dist` (for automated releases)
