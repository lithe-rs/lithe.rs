use lithe::{a, button, client, div, h1, p, Component, HtmlPage};

#[lithe::client]
pub fn handle_local_click() {
    lithe::client::alert("Local function called!");
}

#[lithe::page]
pub fn page() -> impl Component {
    let body = div()
        .class("container")
        .with_child(h1().with_child("Welcome to Lithe.rs"))
        .with_child(p().with_child("1. Inline closure (Svelte-like):"))
        .with_child(
            button()
                .on_click(|| {
                    client::console_log("Inline handler executed!");
                    client::alert("Hello from an inline closure!");
                })
                .with_child("Click Me (Inline)"),
        )
        .with_child(p().with_child("2. Local function in same file:"))
        .with_child(
            button()
                .on_click(handle_local_click)
                .with_child("Click Me (Local)"),
        )
        .with_child(p().with_child("3. Imported function from utils.rs:"))
        .with_child(
            button()
                .on_click(crate::utils::alert_from_utils)
                .with_child("Click Me (Imported)"),
        )
        .with_child(
            div()
                .style("margin-top: 20px;")
                .with_child(a().href("/about").with_child("Go to About")),
        );
    HtmlPage::new("Lithe.rs - Full Test", body)
}
