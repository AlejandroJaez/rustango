#![forbid(unsafe_code)]

use askama_axum::{Template, Response};
use axum_macros::debug_handler;

#[derive(Template)]
#[template(path = "index.html")]
struct HelloTemplate<'a> {
    name: &'a str
}
#[debug_handler]
pub async fn hello() -> Response {
    let context = "alejandro";
    askama_axum::into_response(&HelloTemplate{name: context}, "")
}