use lithe::{a, div, h1, p, Component};

pub fn page() -> impl Component {
    div()
        .class("container")
        .with_child(h1().with_child("Welcome to Lithe.rs"))
        .with_child(p().with_child("Edit src/pages/index.rs to get started."))
        .with_child(a().set_attribute("href", "/about").with_child("About"))
}
