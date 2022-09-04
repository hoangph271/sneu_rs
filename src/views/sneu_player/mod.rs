mod local_library;
mod media_list;
mod video_player;

use crate::utils::expect_input_target;
use core::panic;
use gloo_file::{File, FileList};
use local_library::*;
use media_list::*;
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
                accept=".mkv,video/*"
                oninput={handle_file_change}
            />
            if let Some(opening_file) = (*opening_file).clone() {
                <VideoPlayer file={opening_file} />
            }
        </div>
    }
}
