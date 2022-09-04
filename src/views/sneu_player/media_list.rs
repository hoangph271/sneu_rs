use gloo_file::{File, FileList};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct MediaListProps {
    pub files: FileList,
    pub on_clicked: Callback<File>,
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
