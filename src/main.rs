use axum::{
    routing::{get, post},
    Router,
};
use example_webserver_rs::handlers::{
    append_to_string, get_json, get_json_counter, get_random_number,
};
use example_webserver_rs::structs::AppState;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(AppState::new()));
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/append", post(append_to_string))
        .route("/json", get(get_json))
        .route("/json-counter", get(get_json_counter))
        .route("/rnd", get(get_random_number))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
