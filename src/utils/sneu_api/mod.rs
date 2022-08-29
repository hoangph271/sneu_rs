use super::api_url::with_api_root;
mod api_item;

pub use api_item::*;

use gloo_net::{
    http::{Method, Request},
    Error,
};
use serde::de::DeserializeOwned;
use wasm_bindgen::JsValue;

#[derive(Debug)]
pub enum ApiError {
    JsonError(String),
}
pub type ApiResult<T> = Result<T, ApiError>;

impl From<Error> for ApiError {
    fn from(e: Error) -> Self {
        match e {
            Error::JsError(_) => todo!(),
            Error::SerdeError(e) => ApiError::JsonError(e.to_string()),
        }
    }
}

pub async fn post<T: DeserializeOwned>(url: &str, payload: JsValue) -> ApiResult<T> {
    let res: T = Request::post(&with_api_root(url))
        .method(Method::POST)
        .body(payload)
        .send()
        .await?
        .json()
        .await?;

    Ok(res)
}

pub async fn get<T: DeserializeOwned>(url: &str) -> ApiResult<T> {
    let res: T = Request::post(&with_api_root(url))
        .method(Method::GET)
        .send()
        .await?
        .json()
        .await?;

    Ok(res)
}
