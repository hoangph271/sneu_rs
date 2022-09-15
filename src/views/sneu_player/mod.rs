mod local_library;
mod media_list;
mod use_player_state;
mod video_player;

use self::use_player_state::*;
use crate::components::*;
use crate::utils::expect_input_target;
use core::panic;
use gloo_file::{File, FileList};
use local_library::*;
use media_list::*;
use std::rc::Rc;
use video_player::*;
use yew::prelude::*;
use yew_hooks::use_state_ptr_eq;

#[derive(Properties, PartialEq, Eq)]
pub struct SneuPlayerProps {}

#[function_component(SneuPlayer)]
pub fn sneu_player(props: &SneuPlayerProps) -> Html {
    let SneuPlayerProps {} = props;
    let opening_file = use_state_ptr_eq(|| Option::<File>::None);
    let selected_files = use_state_ptr_eq(|| Option::<FileList>::None);
    let file_count = (selected_files.as_ref())
        .map(|files| files.len())
        .unwrap_or_default();
    let player_state = use_player_state();

    let handle_file_change = Callback::from({
        let selected_files = selected_files.clone();
        let player_state = player_state.clone();

        move |e: InputEvent| {
            let files = expect_input_target(e.target())
                .and_then(|el| el.files())
                .map(|files| files.into());

            selected_files.set(files.clone());

            if let Some(files) = files {
                let media_files = files
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

                let play_list = PlayList { media_files };

                player_state.dispatch(PlayerAction::ReplacePlaylist(Rc::new(play_list)));
            }
        }
    });

    html! {
        <div>
            <LocalLibrary />
            if file_count > 0 {
                <MediaList
                    on_clicked={Callback::from({
                        let opening_file = opening_file.clone();

                        move |file| {
                            opening_file.set(Some(file));
                        }
                    })}
                    files={(*selected_files).clone().unwrap_or_else(|| {
                        panic!("Expect file list to NOT be empty")
                    })}
                />
            }
            <input
                type="file"
                multiple={true}
                accept=".mkv,video/*,audio/*"
                oninput={handle_file_change}
            />
            <div>
                <PillButton
                    onclick={Callback::from({
                        let player_state = player_state.clone();
                        move |_| player_state.clone().dispatch(PlayerAction::TogglePlaying)
                    })}
                >
                    { if player_state.is_playing { "Pause" } else  { "Play" } }
                </PillButton>
                <PillButton
                    onclick={Callback::from({
                        let player_state = player_state.clone();
                        move |_| player_state.clone().dispatch(PlayerAction::ToggleMuted)
                    })}
                >
                    { if player_state.is_muted { "Unmute" } else  { "Mute" } }
                </PillButton>
            </div>
            if let Some(opening_file) = (*opening_file).clone() {
                <VideoPlayer
                    file={opening_file}
                    player_state={(*player_state).clone()}
                />
            }
        </div>
    }
}
