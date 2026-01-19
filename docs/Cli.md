# The LitheCLI

Recommended Crate Stack

| Category | Recommendation | Description |
| :--- | :--- | :--- |
| **Argument Parsing** | `clap` (v4) | Handles flags, options, subcommands, and help messages. |
| **Error Handling** | `anyhow` / `thiserror` | Simplifies error propagation and reporting. |
| **Configuration** | `confy` / `config` | Manages user settings in YAML, TOML, or JSON. |
| **UI/UX** | `indicatif` & `dialoguer` | Adds progress bars, spinners, and interactive prompts. |
| **Logging** | `env_logger` & `log` | Configurable logging levels via environment variables. |

Distribution Strategy

We use the **Native Binary Wrapper** strategy. This allows users to run `npm install -g lithe-cli` and get a working binary immediately, avoiding local compilation or fragile post-install scripts.
 Architecture
- **Platform Packages**: Separate packages for each target (e.g., `lithe-cli-linux-x64`) containing only the pre-compiled binary.
- **Main Package**: A central package (`lithe-cli`) that lists platform packages in `optionalDependencies`.
- **Resolution**: Package managers (npm/yarn/pnpm) automatically install only the package matching the user's `os` and `cpu`.

Implementation (via cargo-dist)

Use [cargo-dist](https://opensource.axodotdev.host/cargo-dist/) to automate matrix builds and npm publishing.
 1. Install cargo-dist
cargo install cargo-dist
2. Initialize Project
cargo dist init
Note: Select npm when prompted for installers.
3. Configure Cargo.toml
Add the required metadata to your Cargo.toml:
[package.metadata.dist]
# The scope to publish under (e.g., @my-org)
npm-scope = "@lithe"
# CI will build binaries for these targets
targets = ["x86_64-unknown-linux-gnu", "aarch64-apple-darwin", "x86_64-pc-windows-msvc"]
4. CI/CD Pipeline
When a git tag is pushed (e.g., v1.0.0), the generated GitHub Action will:
1. Build binaries for Linux, macOS, and Windows.
2. Create npm platform packages (e.g., @lithe/cli-linux-x64).
3. Create the main npm package (@lithe/cli).
4. Automatically publish all packages to the registry.
Technical Details
Main Package Structure
The package.json for the main package delegates installation to platform-specific dependencies:
{
  name: @lithe/cli,
  version: 1.0.0,
  bin: bin.js,
  optionalDependencies: {
    @lithe/cli-linux-x64: 1.0.0,
    @lithe/cli-darwin-arm64: 1.0.0,
    @lithe/cli-win32-x64: 1.0.0
  }
}
Platform Package Structure
The platform-specific package.json uses os and cpu fields to ensure correct installation:
{
  name: @lithe/cli-linux-x64,
  version: 1.0.0,
  os: [linux],
  cpu: [x64],
  bin: {
    lithe: ./lithe
  }
}