use std::rc::Rc;

use web_sys::File;
use yew::prelude::*;

#[allow(dead_code)]
#[derive(Clone)]
pub enum MediaContent {
    Url(String),
    Blob(File),
}

impl PartialEq for MediaContent {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (MediaContent::Url(self_url), MediaContent::Url(other_url)) => self_url.eq(other_url),
            (MediaContent::Url(_), MediaContent::Blob(_)) => false,
            (MediaContent::Blob(_), MediaContent::Url(_)) => false,
            (MediaContent::Blob(_), MediaContent::Blob(_)) => todo!(),
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct MediaFile {
    pub filename: String,
    pub content: MediaContent,
    pub mime_type: String,
}

#[derive(PartialEq, Clone, Default)]
pub struct PlayList {
    pub media_files: Vec<MediaFile>,
}

#[derive(PartialEq, Clone, Properties)]
pub struct PlayerState {
    pub is_playing: bool,
    pub is_muted: bool,
    pub play_list: Rc<PlayList>,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            is_playing: true,
            is_muted: true,
            play_list: Rc::new(PlayList::default()),
        }
    }
}

pub enum PlayerAction {
    TogglePlaying,
    ToggleMuted,
    ReplacePlaylist(Rc<PlayList>),
}

impl Reducible for PlayerState {
    type Action = PlayerAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            PlayerAction::TogglePlaying => Self {
                is_playing: !self.is_playing,
                play_list: self.play_list.clone(),
                ..*self
            },
            PlayerAction::ToggleMuted => Self {
                is_muted: !self.is_muted,
                play_list: self.play_list.clone(),
                ..*self
            },
            PlayerAction::ReplacePlaylist(play_list) => Self { play_list, ..*self },
        }
        .into()
    }
}

pub fn use_player_state() -> UseReducerHandle<PlayerState> {
    use_reducer_eq(PlayerState::default)
}
