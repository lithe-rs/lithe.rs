# CLI Reference

The `lithe` command is the primary tool for managing Lithe.rs projects.

## Commands

### `lithe init`
Scaffolds a new Lithe.rs workspace.
```bash
lithe init my-app
```

### `lithe dev`
Starts the cross-platform development engine.
- Runs the Axum server (Cloud).
- Starts the WASM build watch (Client).
- Enables HMR (Hot Module Replacement) for all tiers.

### `lithe build`
Compiles the project for production.
- **`--target cloud`:** Outputs a Linux binary and optimized assets.
- **`--target desktop`:** Bundles into a Tauri app (.exe, .app, .deb).
- **`--target mobile`:** Bundles for Android or iOS.
- **`--target web`:** Outputs static HTML and WASM for traditional hosting.

## Global Flags
- **`--monolith`:** (Default) Builds everything into one binary.
- **`--distributed`:** Decouples the frontend from the server logic.
