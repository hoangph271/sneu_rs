use gloo_file::{File, FileList};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::{
    components::FilePicker,
    utils::{init_wasm::setSrcObject, no_op},
};

#[derive(PartialEq, Properties, Eq)]
pub struct MusicProps {}

#[function_component(Music)]
pub fn music(_: &MusicProps) -> Html {
    let file: UseStateHandle<Option<File>> = use_state_eq(|| None);

    use_effect_with_deps(
        {
            let file = (*file).clone();

            move |_| {
                if let Some(file) = file {
                    spawn_local(async move { play_audio(file).await });
                }

                no_op
            }
        },
        file.clone(),
    );

    let on_files_picked = Callback::from(move |files: FileList| {
        if let Some(first_file) = files.first() {
            file.set(Some(first_file.clone()))
        }
    });

    html! {
        <FilePicker {on_files_picked} />
    }
}

async fn play_audio(file: File) {
    use web_sys::HtmlAudioElement;

    spawn_local(async move {
        let audio_el = HtmlAudioElement::new().unwrap();

        setSrcObject(&audio_el, file.as_ref());

        audio_el.play().unwrap().as_bool();
    });
}
