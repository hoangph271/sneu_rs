use gloo_file::FileList;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[derive(PartialEq, Properties, Default)]
pub struct FilePickerProps {
    #[prop_or_default]
    pub on_files_picked: Callback<FileList>,
}

#[function_component(FilePicker)]
pub fn file_picker(props: &FilePickerProps) -> Html {
    let FilePickerProps { on_files_picked } = props;
    let oninput = Callback::from({
        let on_files_picked = on_files_picked.clone();

        move |e: InputEvent| {
            expect_target(e.target())
                .map(|el: HtmlInputElement| el.files())
                .flatten()
                .map(|files| {
                    on_files_picked.emit(files.into());
                });
        }
    });

    html! {
        <div class="file has-name is-boxed">
            <label class="file-label">
                <input
                    {oninput}
                    type="file"
                    multiple={true}
                    class="file-input"
                    accept="audio/*"
                />
                <span class="file-cta">
                    <span class="file-icon">
                        <i class="fas fa-upload"></i>
                    </span>
                    <span class="file-label">
                        { "Choose a file…!" }
                    </span>
                </span>
                <span class="file-name">
                    { "Screen Shot 2017-07-29 at 15.54.25.png" }
                </span>
            </label>
        </div>
    }
}

fn expect_target<T: JsCast>(target: Option<EventTarget>) -> Option<T> {
    target.and_then(|t| match t.dyn_into::<T>() {
        Ok(value) => Some(value),
        Err(e) => {
            log::error!("expect_target() failed: {e:?}");

            None
        }
    })
}
