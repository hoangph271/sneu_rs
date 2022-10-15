use crate::{providers::use_auth_context, utils::sneu_api::ApiHandler};
use yew::prelude::*;

#[allow(dead_code)]
pub fn use_authed_api_hander() -> ApiHandler {
    let auth_context = use_auth_context();
    let api_hander = use_state_eq(|| ApiHandler::with_jwt((*auth_context).jwt()));

    (*api_hander).clone()
}
