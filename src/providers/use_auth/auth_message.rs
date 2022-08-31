use std::rc::Rc;

use gloo_storage::{SessionStorage, Storage};
use serde::{Deserialize, Serialize};
use yew::prelude::*;

use crate::utils::storage_key::AUTH_STORAGE_KEY;

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

impl AuthMessage {
    // FIXME: Now using SessionStorage to store JWT
    // NOT safe, consider HttpOnly Cookies...?
    // Or only in-memory...?
    pub fn persist_locally(&self) {
        match self {
            AuthMessage::Authed(auth) => SessionStorage::set(AUTH_STORAGE_KEY, auth),
            AuthMessage::NotAuthed => {
                SessionStorage::delete(AUTH_STORAGE_KEY);
                Ok(())
            }
        }
        .unwrap_or_else(|e| panic!("persist_locally() failed: {e}"));
    }

    pub fn remove_locally() {
        SessionStorage::delete(AUTH_STORAGE_KEY);
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
        match SessionStorage::get::<AuthPayload>(AUTH_STORAGE_KEY) {
            Ok(auth) => Self::Authed(auth),
            Err(e) => {
                match e {
                    gloo_storage::errors::StorageError::SerdeError(e) => {
                        log::error!("AuthMessage::default() failed [SerdeError]: {e}")
                    }
                    gloo_storage::errors::StorageError::KeyNotFound(_) => {}
                    gloo_storage::errors::StorageError::JsError(_) => {
                        log::error!("AuthMessage::default() failed [JsError]: {e}")
                    }
                };

                Self::NotAuthed
            }
        }
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
