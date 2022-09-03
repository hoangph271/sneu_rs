use core::panic;

use gloo_file::{File, FileList};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlVideoElement;
use yew::prelude::*;
use yew_hooks::use_state_ptr_eq;

use crate::utils::{expect_input_target, no_op, setSrcObject};

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

    let handle_file_change = Callback::from({
        let selected_files = selected_files.clone();

        move |e: InputEvent| {
            let files = expect_input_target(e.target())
                .and_then(|el| el.files())
                .map(|files| files.into());

            selected_files.set(files);
        }
    });

    html! {
        <div>
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
                accept=".mkv,video/*"
                oninput={handle_file_change}
            />
            if let Some(opening_file) = (*opening_file).clone() {
                <VideoPlayer file={opening_file} />
            }
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct MediaListProps {
    files: FileList,
    on_clicked: Callback<File>,
}

#[function_component(MediaList)]
pub fn media_list(props: &MediaListProps) -> Html {
    let MediaListProps { files, on_clicked } = props;

    html! {
        <ol>
            {for files.iter().map(|file| {
                html! {
                    <li
                        class="cursor-pointer"
                        onclick={Callback::from({
                            let on_clicked = on_clicked.clone();
                            let file = (*file).clone();

                            move |_| {
                                on_clicked.emit(file.clone());
                            }
                        })}
                        key={file.name()}
                    >
                        { file.name() }
                    </li>
                }
            })}
        </ol>
    }
}

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
