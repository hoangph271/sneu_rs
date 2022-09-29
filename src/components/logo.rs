use yew::prelude::*;

pub const LOGO_URL: &str = "https://sneu.date/static/svg/sneu.svg";

#[derive(Default, Properties, PartialEq, Eq)]
pub struct LogoProps {
    #[prop_or_else (|| LOGO_URL.to_owned())]
    pub src: String,
}

#[function_component(Logo)]
pub fn logo(props: &LogoProps) -> Html {
    let LogoProps { src } = props;

    html! {
        <div
            class="w-32 h-32 rounded-full bg-[length:80%] bg-center bg-no-repeat border border-zinc-200 shadow-inner shadow-zinc-200"
            style={format!("background-image: url({src})")}
        />
    }
}
