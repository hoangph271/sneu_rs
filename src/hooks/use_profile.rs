use crate::utils::sneu_api::ApiHandler;
use crate::{providers::use_auth_context, utils::no_op};
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Serialize, Clone, Deserialize, Debug, PartialEq, Eq)]
pub struct Profile {
    pub username: String,
    pub title: String,
    #[serde(rename(deserialize = "avatarUrl"))]
    pub avatar_url: Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiItem<T: Serialize> {
    status_code: u16,
    item: T,
}

pub fn use_profile() -> Option<Profile> {
    let profile = use_state_eq(|| Option::<Profile>::None);
    let auth_context = use_auth_context();
    let api_hander = use_state_eq(|| ApiHandler::with_jwt((*auth_context).jwt()));

    use_effect_with_deps(
        {
            let profile = profile.clone();
            let api_hander = (*api_hander).clone();

            move |_| {
                spawn_local(async move {
                    profile.set(Some(
                        api_hander
                            .json_get::<ApiItem<Profile>>("/profiles")
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
