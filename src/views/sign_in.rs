use crate::{
    components::{CenteredViewport, SignInForm},
    hooks::use_redirect_on_auth,
};
use yew::prelude::*;

#[function_component(SignIn)]
pub fn sign_in() -> Html {
    use_redirect_on_auth();

    html! {
        <CenteredViewport>
            <SignInForm />
        </CenteredViewport>
    }
}
