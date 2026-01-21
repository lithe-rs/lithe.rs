use axum::{
    Router,
    routing::get,
    extract::Path,
    response::Html,
};

#[path = "../.lithe/routes.rs"]
mod routes;

async fn handle_root() -> Html<String> {
    Html(routes::dispatch("/"))
}

async fn handle_path(Path(path): Path<String>) -> Html<String> {
    Html(routes::dispatch(&format!("/{}", path)))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handle_root))
        .route("/*path", get(handle_path));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
