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
