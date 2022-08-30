use yew::prelude::*;

const LOGO_URL: &str = "https://alpha-sneu.xyz/static/svg/sneu.svg";

#[function_component(Logo)]
pub fn logo() -> Html {
    html! {
        <div
            class="w-16 h-16"
            style={format!("background-image: url({LOGO_URL})")}
        />
    }
}
