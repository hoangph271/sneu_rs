use crate::utils::sneu_api::ApiHandler;
use crate::{providers::use_expected_auth_context, utils::no_op};
use hbp_types::ApiItem;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Serialize, Clone, Deserialize, Debug, PartialEq, Eq)]
pub struct UserProfile {
    pub username: String,
    pub title: String,
    #[serde(rename(deserialize = "avatarUrl"))]
    pub avatar_url: Option<String>,
    pub description: Option<String>,
}

pub fn use_profile() -> Option<UserProfile> {
    let profile = use_state_eq(|| Option::<UserProfile>::None);
    let auth_context = use_expected_auth_context();
    let api_hander = use_state_eq(|| ApiHandler::with_jwt((*auth_context).jwt()));

    use_effect_with_deps(
        {
            let profile = profile.clone();
            let api_hander = (*api_hander).clone();

            move |_| {
                spawn_local(async move {
                    profile.set(Some(
                        api_hander
                            .json_get::<ApiItem<UserProfile>>("/profiles")
                            .await
                            .unwrap()
                            .item,
                    ))
                });

                no_op
            }
        },
        api_hander.jwt.clone(),
    );

    (*profile).clone()
}
