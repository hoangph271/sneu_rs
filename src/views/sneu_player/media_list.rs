use super::use_player_state::{MediaFile, PlayList};
use std::rc::Rc;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct MediaListProps {
    pub play_list: Rc<PlayList>,
    pub on_clicked: Callback<MediaFile>,
}

#[function_component(MediaList)]
pub fn media_list(props: &MediaListProps) -> Html {
    let MediaListProps {
        play_list,
        on_clicked,
    } = props;

    html! {
        <ol>
            {for play_list.media_files.iter().map(|file| {
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
                        key={file.filename.clone()}
                    >
                        { file.filename.clone() }
                    </li>
                }
            })}
        </ol>
    }
}
