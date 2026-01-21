# Navigation and Rendering Modes

Lithe.rs supports both traditional website behavior and modern application behavior under a single routing system.

## 1. Rendering Modes

### Website Mode (MPA)
Standard Multi-Page Application behavior. Each navigation triggers a full page load from the server. This is optimal for:
- SEO-heavy content (Blogs, Marketing).
- Ultra-low JS payloads.
- Maximum initial load performance.

### App Mode (SPA)
Single-Page Application behavior. The Lithe Router intercepts link clicks and performs a client-side transition without refreshing the page. This is optimal for:
- Interactive Dashboards.
- Native-like Desktop/Mobile apps.
- Maintaining state across views.

## 2. The Unified Router
The router is type-safe and handles both browser URLs and native window management.

```rust
router.navigate("/settings"); 
// Web: Updates URL and renders component.
// Desktop: Can be configured to open a new Tauri window.
```

## 3. View Transitions
Lithe.rs integrates with the browser's View Transition API and provides OS-level transitions for Native targets. This allows elements to "morph" between pages seamlessly.

```rust
a().href("/profile")
   .transition(Morph::new("avatar"))
   .child(img().id("avatar").src(url))
```

## 4. Resumability & Navigation
When navigating in SPA mode, the WASM client only fetches the necessary data or component fragments. Because the state is **Resumable**, the new view "wakes up" instantly by merging its signal arena with the existing one.
