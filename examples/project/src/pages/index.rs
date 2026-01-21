use lithe::{Component, Element, a, div, h1, p};

pub fn page() -> impl Component {
    div()
        .class("container")
        .with_child(h1().with_child("Welcome to Lithe.rs"))
        .with_child(p().with_child(
            "This page was generated via FS routing. Go to /about to see another page.",
        ))
        .with_child(a().set_attribute("href", "/about").with_child("About"))
        .with_child(a().set_attribute("href", "/contact").with_child("Contact"))
}
