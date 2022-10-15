use super::api_url::with_api_root;
use gloo_net::{
    http::{Request, Response},
    Error,
};
use serde::de::DeserializeOwned;
use std::fmt::Display;
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

pub async fn raw_get(url: &str) -> ApiResult<Response> {
    let res = Request::get(&with_api_root(url)).send().await?;

    Ok(res)
}

#[derive(Default, PartialEq, Eq, Clone)]
pub struct ApiHandler {
    pub jwt: Option<String>,
}

impl ApiHandler {
    pub fn with_jwt(jwt: Option<String>) -> Self {
        Self { jwt }
    }

    fn append_jwt_query_param(&self, url: &str) -> String {
        if let Some(jwt) = &self.jwt {
            if url.contains('?') {
                format!("{url}&jwt={jwt}")
            } else {
                format!("{url}?jwt={jwt}")
            }
        } else {
            url.to_owned()
        }
    }

    pub async fn json_get<T: DeserializeOwned>(&self, url: &str) -> ApiResult<T> {
        let url = with_api_root(&self.append_jwt_query_param(url));

        let res: T = Request::get(&url).send().await?.json().await?;

        Ok(res)
    }

    pub async fn json_post<T: DeserializeOwned>(
        &self,
        url: &str,
        payload: JsValue,
    ) -> ApiResult<T> {
        let url = with_api_root(&self.append_jwt_query_param(url));

        let res: T = Request::post(&url)
            .body(payload)
            .send()
            .await?
            .json()
            .await?;

        Ok(res)
    }

    pub async fn json_put<T: DeserializeOwned>(&self, url: &str, payload: JsValue) -> ApiResult<T> {
        let url = with_api_root(&self.append_jwt_query_param(url));

        let res: T = Request::put(&url)
            .body(payload)
            .send()
            .await?
            .json()
            .await?;

        Ok(res)
    }

    pub async fn json_delete<T: DeserializeOwned>(
        &self,
        url: &str,
        payload: JsValue,
    ) -> ApiResult<T> {
        let url = with_api_root(&self.append_jwt_query_param(url));

        let res: T = Request::delete(&url)
            .body(payload)
            .send()
            .await?
            .json()
            .await?;

        Ok(res)
    }
}
