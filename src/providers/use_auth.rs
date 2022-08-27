use std::rc::Rc;

use yew::{use_context, use_reducer_eq, Reducible, UseReducerHandle};

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AuthContext {
    Authed(Auth),
    NotAuthed,
}

impl Default for AuthContext {
    fn default() -> Self {
        Self::NotAuthed
    }
}

impl Reducible for AuthContext {
    type Action = AuthAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            AuthAction::SignIn(auth) => Self::Authed(auth),
            AuthAction::SignOut => Self::NotAuthed,
        }
        .into()
    }
}

pub fn use_auth_reducer() -> UseReducerHandle<AuthContext> {
    let auth_context = use_context::<AuthContext>().unwrap();

    use_reducer_eq(|| auth_context)
}
