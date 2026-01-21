# UI Patterns and Animations

Lithe.rs provides two ergonomic ways to build UI, both of which are static-by-default and support first-class animations.

## 1. Pattern A: The Append Builder
Best for dynamic generation or logic-heavy UI.
```rust
let mut list = ul().class("list-none");
for item in items {
    list.push(li().text(item));
}
list
```

## 2. Pattern B: The Macro DSL
Best for declarative, structured layouts.
```rust
div!(
    .class("container")
    => [
        h1!("Welcome"),
        p!("Static content"),
        my_island() // Hydrates as WASM
    ]
)
```

## 3. First-Class Animations
Animations are integrated directly into the builder pattern.
```rust
div()
    .transition(Fade::in(Duration::from_millis(300)))
    .animate(Move::up(10))
    .child(text("Hello"))
```

## 4. Suspense and Error Boundaries
Handle loading and error states declaratively.
```rust
ui.add(
    Suspense::new(move || fetch_user())
        .fallback(spinner())
        .child(|user| user_profile(user))
)

ui.add(
    ErrorBoundary::new(my_component)
        .fallback(|err| error_card(err))
)
```
