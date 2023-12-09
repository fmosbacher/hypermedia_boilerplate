use askama::Template;
use axum::{routing::get, Router};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexPage {
    title: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .nest_service("/public", ServeDir::new("public"));
    let listener = TcpListener::bind("127.0.0.1:5000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}

async fn index() -> IndexPage {
    IndexPage {
        title: String::from("Hypermedia Server"),
    }
}
