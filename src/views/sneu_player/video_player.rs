use crate::utils::{no_op, setSrcObject};
use gloo_file::File;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlVideoElement;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct VideoPlayerProps {
    pub file: File,
}

#[function_component(VideoPlayer)]
pub fn video_player(props: &VideoPlayerProps) -> Html {
    let VideoPlayerProps { file } = props;
    let video_ref = use_node_ref();

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
