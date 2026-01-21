# Compile-Time Magic: Assets and i18n

Lithe.rs leverages the Rust compiler to catch errors and optimize assets before your app even runs.

## 1. Asset Pipeline
Macros for high-performance asset handling.
- **`img!`:** Resizes and optimizes images (WebP/AVIF) at compile time.
- **`svg!`:** Inlines SVGs as optimized Rust UI code, making them styleable and type-safe.

```rust
ui.add(img!(.src("logo.png").width(400)));
```

## 2. Type-Safe i18n
Internationalization that fails at compile time if a key is missing.
```rust
// If "welcome_message" is missing in any language file, 
// the compiler will throw an error.
ui.add(text(t!("welcome_message")));
```

## 3. Resumability Serialization
The compiler automatically generates the serialization logic for your `#[island]` props, ensuring that the transition from static HTML to interactive WASM is seamless and zero-config.
