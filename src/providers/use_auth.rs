use std::rc::Rc;

use yew::{use_context, Reducible, UseReducerHandle};

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
    let auth_context = use_context::<AuthContext>().expect("use_auth_context() got None");

    auth_context
}
