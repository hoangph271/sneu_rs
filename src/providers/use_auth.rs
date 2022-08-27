use std::rc::Rc;

use yew::Reducible;

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
