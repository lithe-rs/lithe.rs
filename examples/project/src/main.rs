use axum::{Router, extract::Path, response::Html, routing::get};

#[path = "../.lithe/routes.rs"]
mod routes;

async fn handle_root() -> Html<String> {
    Html(routes::dispatch("/"))
}

async fn handle_path(Path(path): Path<String>) -> Html<String> {
    Html(routes::dispatch(&format!("/{}", path)))
}

async fn dev() {
    let app = Router::new()
        .route("/", get(handle_root))
        .route("/*path", get(handle_path));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn build() {
    let _ = std::fs::remove_dir_all("dist");
    let routes = routes::routes();
    for route in routes {
        let content = routes::dispatch(route);
        let path = if route == "/" {
            "dist/index.html".to_string()
        } else {
            format!("dist{}/index.html", route)
        };
        let dir = std::path::Path::new(&path).parent().unwrap();
        std::fs::create_dir_all(dir).unwrap();
        std::fs::write(&path, content).unwrap();
        println!("Wrote {}", path);
    }
}

#[tokio::main]
async fn main() {
    // build().await;
    dev().await;
}
