use crate::Component;

pub struct Element {
    tag: String,
    attributes: Vec<(String, String)>,
    children: Vec<Box<dyn Component>>,
}

impl Element {
    fn new(tag: &str) -> Self {
        Element {
            tag: tag.to_string(),
            attributes: Vec::new(),
            children: Vec::new(),
        }
    }

    fn set_attribute(mut self, key: &str, value: &str) -> Self {
        self.attributes.push((key.to_string(), value.to_string()));
        self
    }

    fn child<C: Component + 'static>(mut self, child: C) -> Self {
        self.children.push(Box::new(child));
        self
    }
}

impl Component for Element {
    fn render(&self, buf: &mut String) {
        buf.push('<');
        buf.push_str(&self.tag);
        for (key, value) in &self.attributes {
            buf.push(' ');
            buf.push_str(key);
            buf.push_str("=\"");
            buf.push_str(value);
            buf.push('"');
        }
        buf.push('>');
        for child in &self.children {
            child.render(buf);
        }
        buf.push_str("</");
        buf.push_str(&self.tag);
        buf.push('>');
    }
}

pub struct VoidElement {
    tag: String,
    attributes: Vec<(String, String)>,
}

impl VoidElement {
    fn new(tag: &str) -> Self {
        VoidElement {
            tag: tag.to_string(),
            attributes: Vec::new(),
        }
    }
}

impl Component for VoidElement {
    fn render(&self, buf: &mut String) {
        buf.push('<');
        buf.push_str(&self.tag);
        for (key, value) in &self.attributes {
            buf.push(' ');
            buf.push_str(key);
            buf.push_str("=\"");
            buf.push_str(value);
            buf.push('"');
        }
        buf.push_str(" />");
    }
}

pub fn div() -> Element {
    Element::new("div")
}
pub fn span() -> Element {
    Element::new("span")
}
pub fn p() -> Element {
    Element::new("p")
}
pub fn a() -> Element {
    Element::new("a")
}

pub fn br() -> VoidElement {
    VoidElement::new("br")
}
pub fn img() -> VoidElement {
    VoidElement::new("img")
}
