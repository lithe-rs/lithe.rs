Here is the architectural summary of your **"Tri-brid" Rust Framework**.

This framework is designed to unify **Web (WASM)**, **Server (Cloud)**, and **Desktop (Native)** development into a single, type-safe Rust codebase without relying on HTML macros or Virtual DOM overhead.

### 1. Core Philosophy: "Pure Rust, No Magic"

* **No DSLs:** You do not use `rsx!` or `html!` macros. You use the **Builder Pattern** (Method Chaining) to construct UI trees.
* **Type-Safe:** Attributes (like `tabindex`) are strictly typed integers/enums, not strings.
* **IDE Friendly:** Since it is just standard Rust code, features like "Jump to Definition," "Refactor," and `rustfmt` work perfectly.

### 2. The Execution Model: "The Three Worlds"

Your framework orchestrates three distinct execution environments from one codebase:

| Environment | Compilation Target | Primary Role | Rendering Engine |
| --- | --- | --- | --- |
| **Client** | `wasm32-unknown-unknown` | UI Logic, Animation, State | System WebView (via Tauri) |
| **Native** | Native Binary (Win/Mac/Linux) | File System, OS Hardware, Shell | Tauri Core Process |
| **Server** | Native Binary (Linux) | Database, Auth, Secrets | Headless (Axum/Actix) |

### 3. The Reactivity System: "Direct & Copy-able"

* **No Virtual DOM:** The framework does not diff trees. It binds data directly to specific nodes.
* **Copy-able Signals:** State handles are lightweight integer IDs (Copy types).
* *Result:* You write `move || count.get()` without needing to `.clone()` the signal beforehand.


* **Fine-Grained:** Updates are surgical (O(1)). Changing a number only updates that specific text node, not the parent component.

### 4. The Unified Bridge (RPC & IPC)

This is your framework's superpower. You abstract network and system boundaries using attribute macros that rewrite code at compile time.

* **`#[server]` Macro:**
* **On Server:** Compiles to an HTTP route handler (e.g., `POST /api/save`).
* **On Client:** Compiles to a `window.fetch` call that hits that endpoint.


* **`#[native]` Macro:**
* **On Desktop:** Compiles to a registered Tauri Command (Native Rust).
* **On Client:** Compiles to a `tauri::invoke` IPC message.



**User Code Example:**

```rust
// One file, three execution contexts
#[server]
async fn save_to_db(data: String) -> Result { ... } // Runs in Cloud

#[native]
async fn save_to_disk(data: String) -> Result { ... } // Runs on OS

pub fn save_button() -> impl View {
    // Runs in WASM
    button().on_click(|_| {
        save_to_disk("data"); // Call Native
        save_to_db("data");   // Call Server
    })
}

```

### 5. Project Topology (Workspace)

To prevent dependency conflicts, the project is structured as a Cargo Workspace with feature-flag isolation.

```text
my-app/
├── Cargo.toml            # Workspace Root
├── shared/               # THE CORE (UI, Logic, Models)
│   ├── src/lib.rs        # Uses #[cfg(feature = "...")] to toggle logic
│   └── Cargo.toml        # Features: [hydrate, ssr, desktop]
├── client/               # WASM Wrapper (Compiles 'shared' with 'hydrate')
├── server/               # Cloud Wrapper (Compiles 'shared' with 'ssr')
└── native/               # Tauri Wrapper (Compiles 'shared' with 'desktop')

```

### 6. Rendering & Accessibility

* **Tauri Strategy:** You leverage Tauri to create the window and host the WebView.
* **Accessibility:** You get this for free. Because you render to the DOM (inside the WebView), standard Screen Readers (VoiceOver/NVDA) work out of the box.
* **Islands Architecture (Optional):** Since you have a server runtime, you can default to sending static HTML strings for performance and only "hydrate" the interactive buttons with WASM.