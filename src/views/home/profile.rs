use super::use_profile::UserProfile;
use crate::components::*;
use crate::theme::*;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ProfileProps {
    pub profile: UserProfile,
    pub on_sign_out: Callback<()>,
}

#[function_component(Profile)]
pub fn profile(props: &ProfileProps) -> Html {
    let ProfileProps {
        profile,
        on_sign_out,
    } = props;

    let UserProfile {
        avatar_url,
        username,
        description,
        title: _title, // TODO: Use this...?
    } = profile;
    let onclick = Callback::from({
        let on_sign_out = on_sign_out.clone();

        move |_| on_sign_out.emit(())
    });

    html! {
        <div>
            <div>
                <h4>{ username }</h4>
                if let Some(avatar_url) = avatar_url {
                    <Logo src={avatar_url.clone()} />
                }
                if let Some(description) = description {
                    <code> { description } </code>
                }
            </div>
            <PillButton
                variant={ColorVariant::Warning}
                {onclick}
            >
                { "Sign out" }
            </PillButton>
        </div>
    }
}
