use crate::utils::expect_input_target;
use gloo_file::FileList;
use web_sys::HtmlInputElement;
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
            let file_list =
                expect_input_target(e.target()).and_then(|el: HtmlInputElement| el.files());

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
            </label>
        </div>
    }
}
