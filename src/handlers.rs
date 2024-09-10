//! In axum a “handler” is an async function that accepts zero or more “extractors”
//! as arguments and returns something that can be converted into a response.
use crate::structs::AppState;
use crate::structs::{Country, CountryQueryParams, RequestJson, ResponseJson};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use rand::Rng;
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};

///Return arbitrary json object. `serde_json::Value` can be a valid json object of any kind.
///Wrapping output around `axum::Json` as it implements `IntoResponse` trait.
pub async fn get_json(State(s): State<Arc<Mutex<AppState>>>) -> Result<Json<Value>, StatusCode> {
    match s.lock() {
        Ok(mut x) => x.increment(),
        Err(_e) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    }

    Ok(Json(json!({
        "data": 42,
        "fields": ["a", "b", "c"],
        "X": {"Y": 1, "z": 2},
        "float_val": "1.34",
    })))
}

///Return how many times `get_json` has been called. This introduces shared state implemented with
/// Arc and Mutex.
pub async fn get_json_counter(State(s): State<Arc<Mutex<AppState>>>) -> Result<String, StatusCode> {
    match s.lock() {
        Ok(x) => Ok(x.get_counter().to_string()),
        Err(_e) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

///Provide request json in form of `RequestJson` and outputs `ResponseJson`. Inside function
/// body the input json will be parsed to `state` object.
pub async fn append_to_string(Json(mut payload): Json<RequestJson>) -> Json<ResponseJson> {
    payload.data.push_str(" world!");
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

///Function calls external RestCountries API and extracts information about given country. Function
/// expects query parameter in form of `?name=<country_name>`.
pub async fn get_country(
    query_params: Query<CountryQueryParams>,
) -> Result<Json<Country>, StatusCode> {
    let queried_name = query_params.0.name;

    let response = match reqwest::get(format!(
        "https://restcountries.com/v3.1/name/{queried_name}"
    ))
    .await
    {
        Ok(x) => x,
        Err(_e) => return Err(StatusCode::SERVICE_UNAVAILABLE),
    };

    let mut parsed_vec = match response.json::<Vec<Country>>().await {
        Ok(x) => x,
        Err(_e) => return Err(StatusCode::UNPROCESSABLE_ENTITY),
    };

    match parsed_vec.pop() {
        Some(x) => Ok(Json(x)),
        None => Err(StatusCode::BAD_REQUEST),
    }
}
