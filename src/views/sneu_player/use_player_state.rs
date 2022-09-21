use std::rc::Rc;

use gloo_file::FileList;
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
            (MediaContent::Blob(self_blob), MediaContent::Blob(other_blob)) => {
                self_blob.eq(other_blob)
            }
        }
    }
}

#[derive(PartialEq, Clone, Properties)]
pub struct MediaFile {
    pub filename: String,
    pub content: MediaContent,
    pub mime_type: String,
}

#[derive(PartialEq, Clone, Default, Properties)]
pub struct PlayList {
    pub media_files: Vec<MediaFile>,
}

impl From<FileList> for PlayList {
    fn from(file_list: FileList) -> Self {
        let media_files = file_list
            .iter()
            .map(|media_file| {
                let mime_type = media_file.raw_mime_type();
                let media_file: &web_sys::File = media_file.as_ref();

                MediaFile {
                    filename: media_file.name(),
                    content: MediaContent::Blob(media_file.clone()),
                    mime_type,
                }
            })
            .collect();

        Self { media_files }
    }
}

#[derive(PartialEq, Properties)]
pub struct PlayerState {
    pub is_playing: bool,
    pub is_muted: bool,
    pub play_list: Rc<PlayList>,
    pub playing_index: Option<usize>,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            is_playing: true,
            is_muted: true,
            play_list: Rc::new(PlayList::default()),
            playing_index: None,
        }
    }
}

impl Clone for PlayerState {
    fn clone(&self) -> Self {
        Self {
            play_list: self.play_list.clone(),
            ..*self
        }
    }
}

impl PlayerState {
    pub fn opening_file(&self) -> Option<MediaFile> {
        match self.playing_index {
            Some(playing_index) => self.play_list.media_files.get(playing_index).cloned(),
            None => None,
        }
    }

    pub fn has_media(&self) -> bool {
        !self.play_list.media_files.is_empty()
    }

    pub fn has_next(&self) -> bool {
        if !self.has_media() {
            return false;
        }

        match self.playing_index {
            Some(playing_index) => playing_index + 1 < self.play_list.media_files.len(),
            None => true,
        }
    }
}

pub enum PlayerAction {
    TogglePlaying,
    ToggleMuted,
    JumpToNext,
    StartAtIndex(usize),
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
                ..(*self).clone()
            },
            PlayerAction::ReplacePlaylist(play_list) => Self { play_list, ..*self },
            PlayerAction::StartAtIndex(playing_index) => Self {
                playing_index: Some(playing_index),
                ..(*self).clone()
            },
            PlayerAction::JumpToNext => Self {
                playing_index: match self.playing_index {
                    Some(playing_index) => {
                        if playing_index + 1 < self.play_list.media_files.len() {
                            Some(playing_index + 1)
                        } else {
                            None
                        }
                    }
                    None => {
                        if self.has_media() {
                            Some(0)
                        } else {
                            None
                        }
                    }
                },
                ..(*self).clone()
            },
        }
        .into()
    }
}

pub fn use_player_state() -> UseReducerHandle<PlayerState> {
    use_reducer_eq(PlayerState::default)
}
