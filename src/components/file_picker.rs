use gloo_file::FileList;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::utils::expect_target;

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
            let file_list = expect_target(e.target()).and_then(|el: HtmlInputElement| el.files());

            if let Some(files) = file_list {
                on_files_picked.emit(files.into());
            }
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
