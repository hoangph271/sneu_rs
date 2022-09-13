use std::rc::Rc;

use yew::prelude::*;

#[derive(PartialEq, Eq, Clone)]
pub struct MediaFile {
    pub filename: String,
    pub url: String,
}

#[derive(PartialEq, Eq, Clone, Default)]
pub struct PlayList {
    pub media_files: Vec<MediaFile>,
}

#[derive(PartialEq, Eq, Clone, Properties)]
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
        }
        .into()
    }
}

pub fn use_player_state() -> UseReducerHandle<PlayerState> {
    use_reducer_eq(PlayerState::default)
}
