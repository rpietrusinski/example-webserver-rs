//! In axum a “handler” is an async function that accepts zero or more “extractors”
//! as arguments and returns something that can be converted into a response.
use crate::structs::AppState;
use crate::structs::{RequestJson, ResponseJson};
use axum::extract::State;
use axum::Json;
use rand::Rng;
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};

///Return arbitrary json object. `serde_json::Value` can be a valid json object of any kind.
///Wrapping output around `axum::Json` as it implements `IntoResponse` trait.
pub async fn get_json(State(s): State<Arc<Mutex<AppState>>>) -> Json<Value> {
    let mut binding = s.lock().unwrap();
    binding.increment();

    Json(json!({
        "data": 42,
        "fields": ["a", "b", "c"],
        "X": {"Y": 1, "z": 2},
        "float_val": "1.34",
    }))
}

///Return how many times `get_json` has been called. This introduces shared state implemented with
/// Arc and Mutex.
pub async fn get_json_counter<'a>(State(s): State<Arc<Mutex<AppState>>>) -> String {
    let binding = s.lock().unwrap();
    binding.get_counter().to_string()
}

///Provide request json in form of `RequestJson` and outputs `ResponseJson`. Inside function
/// body the input json will be parsed to `state` object.
pub async fn append_to_string(Json(mut payload): Json<RequestJson>) -> Json<ResponseJson> {
    payload.data.push_str(" world!");
    println!("Success!!");
    Json(ResponseJson {
        output: payload.data,
    })
}

///Function generates single random number and returns it as `String`. `String` is viable output
/// because `IntoResponse` trait is by default implemented for it.
pub async fn get_random_number() -> String {
    let mut rng = rand::thread_rng();
    let y: f64 = rng.random();
    y.to_string()
}
