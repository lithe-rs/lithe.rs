#[path = "../.lithe/routes.rs"]
mod routes;

fn main() {
    println!("Hello from Lithe Rust backend!");

    let html = routes::dispatch("/");
    println!("{}", html);
}
