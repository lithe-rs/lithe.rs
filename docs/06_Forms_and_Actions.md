# Forms, Actions, and Validation

Lithe.rs provides a type-safe way to handle user input across the full stack.

## 1. The `#[action]` Macro
Turning a struct into a full-stack form handler.
```rust
#[derive(Action, Serialize, Deserialize)]
struct SignupForm {
    #[validate(email)]
    email: String,
    #[validate(min = 8)]
    password: String,
}

#[server]
async fn handle_signup(data: SignupForm) -> Result<()> {
    // Server-side logic
}
```

## 2. Isomorphic Validation
Validation logic defined in the struct is compiled into:
1. **WASM:** For instant client-side feedback.
2. **Server:** For secondary security verification.

## 3. Form States
Actions provide built-in signals for their current status.
```rust
let signup = create_action(handle_signup);

if signup.is_pending() {
    ui.add(spinner());
}

if let Some(err) = signup.error() {
    ui.add(error_message(err));
}
```
