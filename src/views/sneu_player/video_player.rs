use crate::utils::{no_op, setSrcObject};
use gloo_file::File;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlVideoElement;
use yew::prelude::*;

use super::use_player_state::PlayerState;

#[derive(PartialEq, Properties)]
pub struct VideoPlayerProps {
    pub file: File,
    pub player_state: PlayerState,
}

fn use_video_src(file: &File, video_ref: &NodeRef) {
    use_effect_with_deps(
        {
            let video_ref = video_ref.clone();
            let file = file.clone();

            move |_| {
                if let Some(video_el) = video_ref.cast::<HtmlVideoElement>() {
                    spawn_local(async move {
                        setSrcObject(&video_el, file.as_ref());
                    });
                }

                no_op
            }
        },
        file.clone(),
    );
}

#[function_component(VideoPlayer)]
pub fn video_player(props: &VideoPlayerProps) -> Html {
    let VideoPlayerProps { file, player_state } = props;
    let video_ref = use_node_ref();

    let PlayerState {
        is_playing,
        is_muted,
    } = player_state;

    use_video_src(file, &video_ref);

    use_effect_with_deps(
        {
            let is_playing = *is_playing;
            let video_ref = video_ref.clone();

            move |_| {
                if let Some(video_el) = video_ref.cast::<HtmlVideoElement>() {
                    spawn_local(async move {
                        if is_playing {
                            video_el.play().unwrap().as_bool();
                        } else {
                            video_el.pause().unwrap();
                        }
                    });
                }

                no_op
            }
        },
        *is_playing,
    );

    use_effect_with_deps(
        {
            let is_muted = *is_muted;
            let video_ref = video_ref.clone();

            move |_| {
                if let Some(video_el) = video_ref.cast::<HtmlVideoElement>() {
                    spawn_local(async move {
                        video_el.set_muted(is_muted);
                    });
                }

                no_op
            }
        },
        *is_muted,
    );

    html! {
        <div>
            <h5>{ file.name() }</h5>
            <video
                style="max-width: 80vw; max-height: 400px;"
                controls={true}
                autoplay={true}
                ref={video_ref.clone()}
            />
        </div>
    }
}
