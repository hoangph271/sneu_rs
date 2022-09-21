use super::use_player_state::{MediaContent, MediaFile, PlayerState};
use crate::utils::{no_op, setSrcObject};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlMediaElement;
use yew::prelude::*;

fn use_media_src(file: &MediaFile, media_el: Option<HtmlMediaElement>) {
    use_effect_with_deps(
        {
            let file = file.clone();

            move |_| {
                if let Some(media_el) = media_el {
                    spawn_local(async move {
                        match file.content {
                            MediaContent::Url(url) => media_el.set_src(&url),
                            MediaContent::Blob(blob) => {
                                setSrcObject(&media_el, &blob);
                            }
                        };
                    });
                }

                no_op
            }
        },
        file.clone(),
    );
}

fn use_toggle_playing(is_playing: bool, media_el: Option<HtmlMediaElement>) {
    use_effect_with_deps(
        {
            move |_| {
                if let Some(media_el) = media_el {
                    spawn_local(async move {
                        if is_playing {
                            media_el.play().unwrap().as_bool();
                        } else {
                            media_el.pause().unwrap();
                        }
                    });
                }

                no_op
            }
        },
        is_playing,
    );
}

fn use_toggle_muted(is_muted: bool, media_el: Option<HtmlMediaElement>) {
    use_effect_with_deps(
        {
            move |_| {
                if let Some(media_el) = media_el {
                    spawn_local(async move {
                        media_el.set_muted(is_muted);
                    });
                }

                no_op
            }
        },
        is_muted,
    );
}

fn use_jump_to_next(media_el: Option<HtmlMediaElement>, on_ended: Callback<Event>) {
    use_effect_with_deps(
        {
            let media_el = media_el.clone();
            let on_ended = on_ended.clone();

            move |_| {
                if let Some(media_el) = media_el {
                    let listener: js_sys::Function = Closure::once_into_js(move |e: Event| {
                        on_ended.emit(e);
                    })
                    .into();

                    media_el
                        .add_event_listener_with_callback("ended", &listener)
                        .unwrap_or_else(|e| {
                            panic!("media_el.add_event_listener_with_callback failed: {e:?}")
                        });
                }

                no_op
            }
        },
        (media_el, on_ended),
    );
}

#[derive(PartialEq, Properties)]
pub struct VideoPlayerProps {
    pub file: MediaFile,
    pub player_state: PlayerState,
    pub on_ended: Callback<Event>,
}

#[function_component(VideoPlayer)]
pub fn video_player(props: &VideoPlayerProps) -> Html {
    let VideoPlayerProps {
        file,
        player_state,
        on_ended,
    } = props;
    let video_ref = use_node_ref();

    let PlayerState {
        is_playing,
        is_muted,
        ..
    } = player_state;

    let media_el = video_ref.cast::<HtmlMediaElement>();

    use_media_src(file, media_el.clone());
    use_toggle_playing(*is_playing, media_el.clone());
    use_toggle_muted(*is_muted, media_el.clone());
    use_jump_to_next(media_el.clone(), (*on_ended).clone());

    html! {
        <div>
            <h5>{ file.filename.clone() }</h5>
            <video
                style="max-width: 80vw; max-height: 400px;"
                controls={true}
                autoplay={true}
                ref={video_ref.clone()}
            />
        </div>
    }
}
