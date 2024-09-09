//!Enable to parse request/response JSONs into the following object structs.
use serde::{Deserialize, Serialize};

///Needs to implement `serde::Deserialize` trait in order to use it as proper handler extractor
#[derive(Deserialize)]
pub struct RequestJson {
    pub data: String,
}

///Needs to implement `serde::Serialize` trait in order to use it as proper json Response object.
#[derive(Serialize)]
pub struct ResponseJson {
    pub output: String,
}

///Implements app state struct.
#[derive(Debug, Clone)]
pub struct AppState {
    counter: u32,
}

///Implement getters, setters and constructor for `AppState`.
impl AppState {
    pub fn new() -> Self {
        Self { counter: 0 }
    }
    pub fn get_counter(&self) -> &u32 {
        &self.counter
    }
    pub fn increment(&mut self) {
        self.counter += 1;
    }
}

///Used in parsing RestCountries API
#[derive(Serialize, Deserialize, Debug)]
pub struct Country {
    name: Name,
}

///Used in parsing RestCountries API
#[derive(Serialize, Deserialize, Debug)]
pub struct Name {
    common: String,
    official: String,
}

///Used in parsing RestCountries API as query parameters
#[derive(Deserialize, Debug)]
pub struct CountryQueryParams {
    pub name: String,
}
