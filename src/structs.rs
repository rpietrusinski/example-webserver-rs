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
