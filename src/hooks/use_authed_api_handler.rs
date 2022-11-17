use crate::{providers::use_auth_context, utils::sneu_api::ApiHandler};
use yew::prelude::*;

pub fn use_authed_api_hander() -> Option<ApiHandler> {
    let auth_context = use_auth_context();
    let api_hander = use_state_eq(|| None);

    use_effect_with_deps(
        {
            let auth_context = auth_context.clone();
            let api_hander = api_hander.clone();

            move |_| {
                if let Some(auth) = auth_context {
                    api_hander.set(Some(ApiHandler::with_jwt((*auth).jwt())));
                }

                || {}
            }
        },
        auth_context,
    );

    (*api_hander).clone()
}

pub fn use_expected_authed_api_hander() -> ApiHandler {
    use_authed_api_hander().expect("use_expected_authed_api_hander() failed")
}
