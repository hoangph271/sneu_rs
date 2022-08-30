use yew::prelude::*;

const LOGO_URL: &str = "https://alpha-sneu.xyz/static/svg/sneu.svg";

#[function_component(Logo)]
pub fn logo() -> Html {
    html! {
        <div
            class="w-32 h-32 rounded-full bg-[length:80%] bg-center bg-no-repeat shadow-xl shadow-slate-300"
            style={format!("background-image: url({LOGO_URL})")}
        />
    }
}
