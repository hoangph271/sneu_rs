mod use_sign_in_handler;

use crate::components::*;
use use_sign_in_handler::use_sign_in_handler;

use yew::prelude::*;

#[function_component(SignInForm)]
pub fn sign_in_form() -> Html {
    let username = use_state_eq(|| "".to_owned());
    let password = use_state_eq(|| "".to_owned());

    let (is_loading, onsubmit, sign_in_error, clear_error) =
        use_sign_in_handler((*username).clone(), (*password).clone());

    html! {
        <div
            class="inline-flex flex-col items-center p-6 gap-4 border rounded border-zinc-200 shadow-lg shadow-zinc-200"
        >
            <Logo />
            <form
                method="post"
                action="http://localhost:8000/api/v1/users/signin"
                {onsubmit}
                class="inline-flex flex-col items-center gap-4"
            >
                if !sign_in_error.is_empty() {
                    <Toast
                        variant={ColorVariant::Danger}
                        header="Sign in error...!"
                        ondismiss={Callback::from(move |_| clear_error.emit(()))}
                    >
                        { sign_in_error }
                    </Toast>
                }
                <FormInput
                    id="username"
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
                    id="password"
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
                <PillButton
                    button_type={ButtonType::Submit}
                    disabled={is_loading}
                >
                    { "Sign in" }
                </PillButton>
            </form>
        </div>
    }
}
