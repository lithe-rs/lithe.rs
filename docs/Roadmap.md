# Lithe JS: Development Roadmap

This roadmap outlines the path from initial concept to a production-ready polyglot meta-framework.

## Phase 1: The Core Compiler & Parsing Foundation
*Goal: Establish the ability to parse polyglot files and generate basic build artifacts.*

- [ ] **Unified CLI Scaffolding**: Create the `lithe` CLI tool with `init`, `dev`, and `build` commands.
- [ ] **Rust AST Parser**: Implement a parser (using `syn` in Rust) to extract `structs` and `#[lithe::rpc]` functions from `.rs` files.
- [ ] **LZR Template Parser**: Create a parser for `.lzr` files that separates `<script>`, `<style>`, and HTML templates.
- [ ] **Dependency Graph**: Build a system to track relationships between `.rs` logic and `.lzr` frontend components.

## Phase 2: The Magic Bridge (RPC & Type Safety)
*Goal: Enable seamless communication between Rust and TypeScript.*

- [ ] **Auto-Generated RPC Routes**: Compiler generates a native binary (using Axum or Gin) that automatically maps Rust functions to HTTP endpoints.
- [ ] **Type Mirroring**: Implement the "Generator" that converts Rust `structs` to TypeScript `interfaces` in a hidden `.lithe/` directory.
- [ ] **Bridge Injection**: Configure the compiler to replace `import { fn } from "./logic.rs"` with a generated fetch client.
- [ ] **Serialization Layer**: Standardize JSON serialization/deserialization with robust error handling for the bridge.

## Phase 3: Frontend Runtime (Signals & Islands)
*Goal: Implement a lightweight, reactive frontend experience.*

- [ ] **Signal Core**: Build the proxy-based reactivity system (`$signal`) for fine-grained DOM updates.
- [ ] **Islands Runtime**: Implement the hydration logic that only executes JavaScript for components marked as `<island />`.
- [ ] **Template Directive Engine**: Build support for `{expression}`, `{#if}`, and `{#each}` in the `.lzr` compiler.
- [ ] **Scoped CSS**: Implement automated CSS scoping for styles defined within `.lzr` files.

## Phase 4: Hybrid Rendering Engine (SSR & SSG)
*Goal: Support multiple rendering strategies for performance.*

- [ ] **SSR Middleware**: Create the server-side logic to execute `.lzr` templates and inject initial data into the HTML.
- [ ] **Static Site Generation (SSG)**: Implement a crawler that pre-renders specific routes at build time.
- [ ] **Hydration Strategy**: Develop the logic to "re-attach" signals to server-rendered HTML without full re-renders.
- [ ] **Asset Pipeline**: Integrate a bundler (like Esbuild or Vite) to optimize the generated vanilla JS.

## Phase 5: Developer Experience (IDE & Tooling)
*Goal: Create a world-class development environment.*

- [ ] **VS Code Extension**:
    - Syntax highlighting via TextMate Grammar.
    - "Go to Definition" redirection from TS definition files back to original Rust source.
- [ ] **Hot Module Replacement (HMR)**: Implement instant updates for both `.lzr` templates and Rust logic changes.
- [ ] **Lithe Devtools**: Browser extension to inspect Signals and Island states.

## Phase 6: Optimization & Ecosystem
*Goal: Scale and polish the framework for production use.*

- [ ] **Multi-Backend Support**: Extend the RPC bridge to support Go, Python, and TypeScript (Node/Bun/Deno).
- [ ] **Bundle Minimization**: Optimize the runtime to stay under 5KB (gzipped).
- [ ] **Benchmark Suite**: Create automated performance comparisons against Next.js, Astro, and Qwik.
- [ ] **Documentation & Tutorials**: Comprehensive guides for building complex polyglot applications.
