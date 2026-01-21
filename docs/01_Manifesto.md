# Lithe.rs Manifesto

## The Vision
Lithe.rs is a **Tri-tier Rust Framework** designed to bridge the gap between Cloud, Native, and the Browser. It enables developers to build high-performance, type-safe applications for the Web, Desktop, and Mobile from a single, unified Rust codebase.

## Core Pillars

### 1. Static-by-Default (Zero JS)
Every component in Lithe.rs is a static string generator by default. JavaScript (via WASM) is only introduced when a component is explicitly marked as an `#[island]`. This ensures that your application remains lightweight and fast, with zero unnecessary overhead.

### 2. Resumability (Beyond Hydration)
Unlike traditional frameworks that "hydrate" by re-running code to find event listeners, Lithe.rs **resumes**. The server serializes the state (Signal Arena) into the HTML. When the WASM boots, it reads this state and attaches listeners instantly without a full re-render.

### 3. The Tri-tier Bridge
Lithe.rs abstracts the boundary between execution environments:
- **Cloud (Server):** Direct DB access and SSR.
- **Native (OS):** Hardware and File System access via Tauri.
- **Client (Browser):** Surgical UI updates via WASM.

### 4. Pure Rust DX
No custom DSLs or fragile file formats. Lithe.rs uses standard Rust patterns, macros, and the Builder Pattern to provide a developer experience that is fully compatible with `rust-analyzer`, `rustfmt`, and the entire Rust ecosystem.

## Why Lithe.rs?
The modern web is bloated. Developers are forced to choose between the SEO of static sites and the interactivity of SPAs. Lithe.rs eliminates this trade-off, providing a "Tri-brid" architecture that is fast by default, type-safe by design, and truly universal.
