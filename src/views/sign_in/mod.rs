use crate::components::ButtonType;
use crate::components::{BulmaButton, FormInput, InputType};

mod use_redirect_on_auth;
mod use_sign_in_handler;

use use_redirect_on_auth::use_redirect_on_auth;
use use_sign_in_handler::use_sign_in_handler;

use yew::prelude::*;

#[function_component(SignIn)]
pub fn sign_in() -> Html {
    let username = use_state_eq(|| "".to_owned());
    let password = use_state_eq(|| "".to_owned());

    let (is_loading, onsubmit) = use_sign_in_handler((*username).clone(), (*password).clone());

    use_redirect_on_auth();

    html! {
        <form
            method="post"
            action="http://localhost:8000/api/v1/users/signin"
            style="height: 100vh;"
            class="container is-fluid is-flex is-flex-direction-column is-justify-content-center"
            {onsubmit}
        >
            <FormInput
                fa_icon="fa-user"
                label="Username:"
                placeholder="Your username...?"
                value={(*username).clone()}
                on_value_changed={{
                    let username = username.clone();
                    Callback::from(move |value| username.set(value))
                }}
            />
            <FormInput
                fa_icon="fa-key"
                label="Password:"
                placeholder="Your password...?"
                input_type={InputType::Password}
                value={(*password).clone()}
                on_value_changed={{
                    let password = password.clone();
                    Callback::from(move |value| password.set(value))
                }}
            />
            <BulmaButton
                button_type={ButtonType::Submit}
                disabled={is_loading}
            >
                { "Sign in" }
            </BulmaButton>
        </form>
    }
}
