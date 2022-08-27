use std::rc::Rc;
use yew::prelude::*;

#[derive(Debug)]
pub enum AuthAction {
    SignIn(Auth),
    SignOut,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Auth {
    pub username: String,
    pub jwt: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum AuthMessage {
    Authed(Auth),
    #[default]
    NotAuthed,
}

impl Reducible for AuthMessage {
    type Action = AuthAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            AuthAction::SignIn(auth) => Self::Authed(auth),
            AuthAction::SignOut => Self::NotAuthed,
        }
        .into()
    }
}

pub type AuthContext = UseReducerHandle<AuthMessage>;

pub fn use_auth_context() -> UseReducerHandle<AuthMessage> {
    use_context::<AuthContext>().expect("use_auth_context() got None")
}

#[derive(PartialEq, Properties)]
pub struct AuthProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(AuthProvider)]
pub fn auth_provider(props: &AuthProviderProps) -> Html {
    let auth = use_reducer(AuthMessage::default);

    html! {
        <ContextProvider<AuthContext> context={auth}>
            {props.children.clone()}
        </ContextProvider<AuthContext>>
    }
}
