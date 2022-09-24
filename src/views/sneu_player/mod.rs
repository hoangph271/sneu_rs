mod media_list;
mod use_player_state;
mod video_player;
use self::use_player_state::*;
use crate::components::*;
use crate::router::{SneuLink, SneuRoutes};
use crate::utils::expect_input_target;
use gloo_file::FileList;
use media_list::*;
use std::rc::Rc;
use video_player::*;
use yew::prelude::*;

#[derive(Properties, PartialEq, Eq)]
pub struct SneuPlayerProps {}

#[function_component(SneuPlayer)]
pub fn sneu_player(props: &SneuPlayerProps) -> Html {
    let SneuPlayerProps {} = props;
    let player_state = use_player_state();

    let handle_file_change = Callback::from({
        let player_state = player_state.clone();

        move |e: InputEvent| {
            let files: Option<FileList> = expect_input_target(e.target())
                .and_then(|el| el.files())
                .map(|files| files.into());

            if let Some(files) = files {
                let play_list = Rc::new(files.into());

                player_state.dispatch(PlayerAction::ReplacePlaylist(play_list));
            }
        }
    });

    html! {
        <div>
            <SneuLink to={SneuRoutes::LocalLibrary} />
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
                if player_state.has_next() {
                    <PillButton
                        onclick={Callback::from({
                            let player_state = player_state.clone();
                            move |_| player_state.clone().dispatch(PlayerAction::JumpToNext)
                        })}
                    >
                        { "Next" }
                    </PillButton>
                }
            </div>
            if let Some(opening_file) = player_state.opening_file() {
                <VideoPlayer
                    file={opening_file}
                    on_ended={{
                        let player_state = player_state.clone();

                        Callback::from(move |_| {
                            player_state.dispatch(PlayerAction::JumpToNext)
                        })
                    }}
                    player_state={(*player_state).clone()}
                />
            }
            <MediaList
                on_clicked={Callback::from({
                    let player_state = player_state.clone();

                    move |index| {
                        player_state.dispatch(PlayerAction::StartAtIndex(index));
                    }
                })}
                play_list={(*player_state).play_list.clone()}
            />
        </div>
    }
}
