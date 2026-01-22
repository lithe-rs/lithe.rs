#[lithe::page]
pub fn page() -> impl lithe::Component {
    lithe::div()
        .class("about-container")
        .with_child(lithe::h1().with_child("About Lithe.rs"))
        .with_child(lithe::p().with_child(
            "Lithe.rs is a lightweight framework for building web applications in Rust.",
        ))
}
