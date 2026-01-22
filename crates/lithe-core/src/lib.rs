pub mod element;

pub use element::*;
pub trait Component {
    fn render(&self, buf: &mut String);
}

pub fn render_to_string<C: Component>(component: &C) -> String {
    let mut buf = String::new();
    component.render(&mut buf);
    buf
}

pub fn doctype() -> &'static str {
    "<!DOCTYPE html>\n"
}

impl<A: Component, B: Component> Component for (A, B) {
    fn render(&self, buf: &mut String) {
        self.0.render(buf);
        self.1.render(buf);
    }
}

impl<A: Component, B: Component, C: Component> Component for (A, B, C) {
    fn render(&self, buf: &mut String) {
        self.0.render(buf);
        self.1.render(buf);
        self.2.render(buf);
    }
}

impl<A: Component, B: Component, C: Component, D: Component> Component for (A, B, C, D) {
    fn render(&self, buf: &mut String) {
        self.0.render(buf);
        self.1.render(buf);
        self.2.render(buf);
        self.3.render(buf);
    }
}

impl Component for String {
    fn render(&self, buf: &mut String) {
        buf.push_str(self);
    }
}

impl Component for &str {
    fn render(&self, buf: &mut String) {
        buf.push_str(self);
    }
}

impl Component for Box<dyn Component> {
    fn render(&self, buf: &mut String) {
        (**self).render(buf);
    }
}

impl<T: Component> Component for Vec<T> {
    fn render(&self, buf: &mut String) {
        for item in self {
            item.render(buf);
        }
    }
}

impl<T: Component> Component for &Vec<T> {
    fn render(&self, buf: &mut String) {
        for item in *self {
            item.render(buf);
        }
    }
}

impl<T: Component, const N: usize> Component for [T; N] {
    fn render(&self, buf: &mut String) {
        for item in self {
            item.render(buf);
        }
    }
}

impl<T: Component> Component for Option<T> {
    fn render(&self, buf: &mut String) {
        if let Some(inner) = self {
            inner.render(buf);
        }
    }
}

pub struct HtmlPage {
    pub title: String,
    pub body: Box<dyn Component>,
    pub styles: Vec<String>,
    pub scripts: Vec<String>,
    pub wasm_fragment: Option<String>,
}

impl HtmlPage {
    pub fn new(title: &str, body: impl Component + 'static) -> Self {
        Self {
            title: title.to_string(),
            body: Box::new(body),
            styles: Vec::new(),
            scripts: Vec::new(),
            wasm_fragment: None,
        }
    }

    pub fn add_style(mut self, css: &str) -> Self {
        self.styles.push(css.to_string());
        self
    }

    pub fn render_to_string(&self) -> String {
        let mut buf = String::new();
        self.render(&mut buf);
        buf
    }

    pub fn with_wasm_fragment(mut self, fragment: &str) -> Self {
        self.wasm_fragment = Some(fragment.to_string());
        self
    }
    pub fn add_script(mut self, js: &str) -> Self {
        self.scripts.push(js.to_string());
        self
    }
}

impl Component for HtmlPage {
    fn render(&self, buf: &mut String) {
        buf.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
        buf.push_str("    <meta charset=\"UTF-8\">\n");
        buf.push_str(
            "    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n",
        );
        buf.push_str("    <title>");
        buf.push_str(&self.title);
        buf.push_str("</title>\n");

        for style in &self.styles {
            buf.push_str("    <style>");
            buf.push_str(style);
            buf.push_str("</style>\n");
        }
        for script in &self.scripts {
            buf.push_str("    <script>");
            buf.push_str(script);
            buf.push_str("</script>\n");
        }

        buf.push_str("</head>\n<body>\n    ");
        self.body.render(buf);
        buf.push_str("\n</body>\n</html>");
    }
}

#[cfg(test)]
mod tests;
