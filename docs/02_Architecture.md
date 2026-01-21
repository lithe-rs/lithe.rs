# Architecture: The Three Worlds

Lithe.rs coordinates three distinct execution contexts, ensuring that logic lives where it belongs while remaining accessible everywhere.

## 1. The Trinity of Tiers

| Tier | Environment | Role | Communication |
| --- | --- | --- | --- |
| **Cloud** | Linux Server | Auth, Database, Secrets | `#[server]` (RPC) |
| **Native** | OS (Win/Mac/Mobile) | File System, Bluetooth, Hardware | `#[native]` (IPC) |
| **Client** | WASM (Browser) | UI Logic, Animation, State | Direct Execution |

## 2. The Bridge Macros

Lithe.rs uses procedural macros to swap function implementations at compile time.

### `#[server]`
- **On Server:** Compiles to a route handler (e.g., Axum `post`).
- **On Client:** Compiles to a type-safe `fetch()` call.
- **On Native:** Compiles to an HTTP request to the Cloud binary.

### `#[native]`
- **On Native:** Compiles to a Tauri command in the main Rust process.
- **On Client:** Compiles to a `tauri::invoke()` call.
- **On Server:** Not available (throws a compile error if misused).

## 3. Middlewares
Server functions can be protected or extended using Middlewares.
```rust
#[server(middleware = [AuthGuard])]
async fn delete_account(id: u32) -> Result<()> {
    // AuthGuard has already verified the user
    db::remove(id).await
}
```

## 4. Deployment Modes

### Monolith Mode
The frontend assets and server logic are compiled into a single binary. This is the default for simplicity and ease of deployment.

### Distributed Mode
The frontend is decoupled (served via CDN/Edge), while the `#[server]` functions are deployed as independent microservices. The Lithe compiler generates the necessary service discovery and routing logic to connect them.
