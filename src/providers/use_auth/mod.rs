use yew::prelude::*;

mod auth_message;
mod auth_provider;

pub use auth_message::*;
pub use auth_provider::*;

pub type AuthContext = UseReducerHandle<AuthMessage>;

pub fn use_auth_context() -> UseReducerHandle<AuthMessage> {
    use_context::<AuthContext>().expect("use_auth_context() got None")
}
