use askama::Template;
use axum::{extract::State, routing::get, serve, Router};
use dotenv::{dotenv, var};
use sqlx::postgres::PgPool;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexPage {
    title: String,
}

#[derive(Clone)]
struct AppState {
    pg_pool: PgPool,
}

#[tokio::main]
async fn main() {
    dotenv().unwrap();

    let db_url = var("DATABASE_URL").unwrap();
    let app_port = var("APP_PORT").unwrap();

    let app_state = AppState {
        pg_pool: PgPool::connect(db_url.as_str()).await.unwrap(),
    };

    let router = Router::new()
        .route("/", get(index))
        .nest_service("/public", ServeDir::new("public"))
        .with_state(app_state);

    let addr = format!("127.0.0.1:{app_port}");
    let listener = TcpListener::bind(addr).await.unwrap();

    serve(listener, router).await.unwrap()
}

async fn index(State(app_state): State<AppState>) -> IndexPage {
    let record = sqlx::query!(
        "select * from (select (1) as id, 'Herp Derpinson' as name) accounts where id = $1",
        1i32
    )
    .fetch_one(&app_state.pg_pool)
    .await
    .unwrap();

    IndexPage {
        title: record.name.unwrap(),
    }
}
