use axum::{
    routing::{get, post},
    Router,
};
use example_webserver_rs::handlers::{append_to_string, get_json, get_random_number};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/append", post(append_to_string))
        .route("/json", get(get_json))
        .route("/rnd", get(get_random_number));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
