use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct GalleryProps {}

#[function_component(Gallery)]
pub fn gallery(props: &GalleryProps) -> Html {
    let GalleryProps {} = props;

    html! {
        <img src="https://alpha-sneu.xyz/api/v1/files/random/raw?mime=image/&preview=true" />
    }
}
