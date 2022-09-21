use super::use_player_state::PlayList;
use std::rc::Rc;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct MediaListProps {
    pub play_list: Rc<PlayList>,
    pub on_clicked: Callback<usize>,
}

#[function_component(MediaList)]
pub fn media_list(props: &MediaListProps) -> Html {
    let MediaListProps {
        play_list,
        on_clicked,
    } = props;

    html! {
        <ol>
            {for play_list.media_files.iter().enumerate().map(|(index, file)| {
                html! {
                    <li
                        class="cursor-pointer"
                        onclick={Callback::from({
                            let on_clicked = on_clicked.clone();

                            move |_| {
                                on_clicked.emit(index);
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
