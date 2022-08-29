use std::fmt::Display;

use super::api_url::with_api_root;
mod api_item;

pub use api_item::*;
use gloo_net::{
    http::{Method, Request, Response},
    Error,
};
use serde::de::DeserializeOwned;
use wasm_bindgen::JsValue;

#[derive(Debug)]
pub enum ApiError {
    JsError(String),
    JsonError(String),
}
pub type ApiResult<T> = Result<T, ApiError>;

impl From<Error> for ApiError {
    fn from(e: Error) -> Self {
        match e {
            Error::JsError(e) => ApiError::JsError(e.to_string()),
            Error::SerdeError(e) => ApiError::JsonError(e.to_string()),
        }
    }
}
impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ApiError::JsonError(e) => e.to_owned(),
                ApiError::JsError(e) => e.to_owned(),
            }
        )
    }
}

pub async fn json_post<T: DeserializeOwned>(url: &str, payload: JsValue) -> ApiResult<T> {
    let res: T = Request::post(&with_api_root(url))
        .method(Method::POST)
        .body(payload)
        .send()
        .await?
        .json()
        .await?;

    Ok(res)
}

#[allow(dead_code)]
pub async fn json_get<T: DeserializeOwned>(url: &str) -> ApiResult<T> {
    let res: T = Request::post(&with_api_root(url))
        .method(Method::GET)
        .send()
        .await?
        .json()
        .await?;

    Ok(res)
}

pub async fn raw_get(url: &str) -> ApiResult<Response> {
    let res = Request::post(&with_api_root(url))
        .method(Method::GET)
        .send()
        .await?;

    Ok(res)
}
