use crate::components::*;
use crate::hooks::use_redirect_unauthed;
use crate::providers::{use_auth_context, AuthAction, AuthMessage};
use crate::utils::no_op;
use crate::utils::sneu_api::ApiHandler;
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

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiItem<T: Serialize> {
    status_code: u16,
    item: T,
}

fn use_profile() -> Option<UserProfile> {
    let profile = use_state_eq(|| Option::<UserProfile>::None);
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

#[function_component(Home)]
pub fn index() -> Html {
    let auth_context = use_auth_context();
    let profile = use_profile();

    use_redirect_unauthed();

    match profile {
        Some(profile) => html! {
            <div>
                if let Some(avatar_url) = profile.avatar_url {
                    <Logo src={avatar_url} />
                }
                <h4>{ format!("Welcome, {}...!", profile.username) }</h4>
                <PillButton
                    variant={ColorVariant::Warning}
                    onclick={Callback::from(move |_| {
                        AuthMessage::remove_locally();
                        auth_context.dispatch(AuthAction::SignOut);
                    })}
                >
                    { "Sign out" }
                </PillButton>
            </div>
        },
        _ => html! {},
    }
}
