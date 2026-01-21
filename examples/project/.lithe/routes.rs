use lithe::render_to_string;

#[path = "../src/pages/about.rs"]
mod about_page;
#[path = "../src/pages/index.rs"]
mod index_page;

pub fn dispatch(path: &str) -> String {
    match path {
        "/" => render_to_string(&index_page::page()),
        "/about" => render_to_string(&about_page::page()),
        _ => "404 Not Found".to_string(),
    }
}

pub fn routes() -> Vec<&'static str> {
    vec!["/", "/about"]
}
