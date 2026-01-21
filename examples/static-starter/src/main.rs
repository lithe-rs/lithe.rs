use lithe::{
    Component, Element, a, body, div, doctype, h1, head, html, li, main_tag, meta, p,
    render_to_string, style, title, ul,
};
use std::fs;

fn nav_item(name: &str, link: &str) -> Element {
    li().with_child(a().href(link).with_child(name.to_string()))
}

fn card(title_text: &str, body_content: impl Component + 'static) -> Element {
    div()
        .class("card")
        .with_child(
            div()
                .class("card-header")
                .with_child(title_text.to_string()),
        )
        .with_child(div().class("card-body").with_child(body_content))
}

fn layout(title_text: &str, content: impl Component + 'static) -> impl Component {
    (
        doctype(),
        html().lang("en").with_child((
            head()
                .with_child(meta().charset("UTF-8"))
                .with_child(meta().set_attribute("name", "viewport").set_attribute("content", "width=device-width, initial-scale=1.0"))
                .with_child(title().with_child(title_text.to_string()))
                .with_child(style().with_child(r#"
                    body { font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; background: #f0f2f5; padding: 40px; color: #1c1e21; margin: 0; }
                    .container { max-width: 900px; margin: 0 auto; }
                    .nav { list-style: none; padding: 20px; display: flex; gap: 30px; margin-bottom: 40px; background: white; border-radius: 12px; box-shadow: 0 2px 4px rgba(0,0,0,0.05); }
                    .nav a { color: #f74c00; text-decoration: none; font-weight: 600; transition: opacity 0.2s; }
                    .nav a:hover { opacity: 0.7; }
                    .card { background: white; border-radius: 16px; overflow: hidden; box-shadow: 0 4px 12px rgba(0,0,0,0.05); margin-bottom: 30px; border: 1px solid #e4e6eb; }
                    .card-header { background: #ffffff; color: #1c1e21; padding: 20px 25px; font-size: 20px; font-weight: 700; border-bottom: 1px solid #e4e6eb; }
                    .card-body { padding: 25px; line-height: 1.6; font-size: 16px; }
                    h1 { color: #1c1e21; margin-bottom: 10px; font-size: 32px; }
                    .subtitle { color: #65676b; margin-bottom: 30px; font-size: 18px; }
                "#)),
            body().with_child(
                main_tag().class("container").with_child((
                    ul().class("nav").with_child((
                        nav_item("Home", "#"),
                        nav_item("Documentation", "#"),
                        nav_item("GitHub", "https://github.com/lithe-rs"),
                    )),
                    h1().with_child(title_text.to_string()),
                    div().class("subtitle").with_child("A high-performance Tri-tier Rust Framework"),
                    content
                ))
            )
        ))
    )
}

fn main() {
    let app = layout(
        "Lithe.rs Flexible Components",
        (
            card(
                "Single Element",
                p().with_child("This card accepts a single element directly."),
            ),
            card(
                "Multiple Elements (Tuple)",
                (
                    p().with_child("This card accepts a tuple of elements."),
                    p().with_child("Tuples allow mixing different component types."),
                ),
            ),
            card(
                "Multiple Elements (Array)",
                [
                    p().with_child("This card accepts an array."),
                    p().with_child("Arrays are great for homogeneous collections."),
                ],
            ),
        ),
    );

    let full_html = render_to_string(&app);

    fs::write("examples/static-starter/index.html", full_html).expect("Unable to write file");

    println!("Successfully generated index.html with flexible card component!");
}
