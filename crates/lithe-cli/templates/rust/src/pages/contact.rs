use lithe::{div, h1, p, Component};

#[lithe::page]
pub fn page() -> impl Component {
    div()
        .class("container")
        .with_child(h1().with_child("Contact Lithe.rs"))
        .with_child(p().with_child("This is the contact page generated via FS routing."))
}
