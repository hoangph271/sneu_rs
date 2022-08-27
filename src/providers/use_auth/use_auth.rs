use yew::prelude::*;

use super::AuthMessage;

pub type AuthContext = UseReducerHandle<AuthMessage>;

pub fn use_auth_context() -> UseReducerHandle<AuthMessage> {
    use_context::<AuthContext>().expect("use_auth_context() got None")
}
