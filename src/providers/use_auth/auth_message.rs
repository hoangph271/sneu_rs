use serde::{Deserialize, Serialize};
use std::rc::Rc;
use yew::prelude::*;

#[derive(Debug)]
pub enum AuthAction {
    SignIn(AuthPayload),
    SignOut,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct AuthPayload {
    pub username: String,
    pub jwt: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AuthMessage {
    Authed(AuthPayload),
    NotAuthed,
}

mod auth_persist {
    // FIXME: Now using SessionStorage to store JWT
    // NOT safe, consider HttpOnly Cookies...?
    // Or only in-memory...?
    use super::{AuthMessage, AuthPayload};
    use crate::utils::storage_key::AUTH_STORAGE_KEY;
    use gloo_storage::{errors::StorageError, LocalStorage, SessionStorage, Storage};
    use serde::Serialize;

    fn handle_storage_error(e: StorageError) {
        match e {
            StorageError::SerdeError(e) => {
                log::error!("AuthMessage::default() failed [SerdeError]: {e}")
            }
            StorageError::KeyNotFound(_) => {}
            StorageError::JsError(_) => {
                log::error!("AuthMessage::default() failed [JsError]: {e}")
            }
        }
    }

    fn is_pwa() -> bool {
        cfg!(feature = "sneu_tauri")
    }

    pub fn read() -> AuthMessage {
        if is_pwa() {
            LocalStorage::get::<AuthPayload>(AUTH_STORAGE_KEY)
        } else {
            SessionStorage::get::<AuthPayload>(AUTH_STORAGE_KEY)
        }
        .map(AuthMessage::Authed)
        .unwrap_or_else(|e| {
            handle_storage_error(e);
            AuthMessage::NotAuthed
        })
    }

    pub fn remove() {
        if is_pwa() {
            LocalStorage::delete(AUTH_STORAGE_KEY);
        } else {
            SessionStorage::delete(AUTH_STORAGE_KEY);
        }
    }

    pub fn persist(payload: &impl Serialize) {
        if is_pwa() {
            LocalStorage::set(AUTH_STORAGE_KEY, payload)
        } else {
            SessionStorage::set(AUTH_STORAGE_KEY, payload)
        }
        .unwrap_or_else(|e| panic!("Storage::set failed: {e:?}"));
    }
}

impl AuthMessage {
    pub fn persist_locally(&self) {
        match self {
            AuthMessage::Authed(auth) => auth_persist::persist(auth),
            AuthMessage::NotAuthed => auth_persist::remove(),
        }
    }

    pub fn remove_locally() {
        auth_persist::remove();
    }

    pub fn is_authed(&self) -> bool {
        match self {
            AuthMessage::Authed(_) => true,
            AuthMessage::NotAuthed => false,
        }
    }

    pub fn jwt(&self) -> Option<String> {
        match self {
            AuthMessage::Authed(auth) => Some(auth.jwt.to_owned()),
            AuthMessage::NotAuthed => None,
        }
    }
}

impl Default for AuthMessage {
    fn default() -> Self {
        auth_persist::read()
    }
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
