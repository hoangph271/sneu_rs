use crate::components::*;
use yew::prelude::*;

#[derive(PartialEq, Eq, Properties)]
pub struct UseLastedProps {}

#[function_component(UseLasted)]
pub fn use_lasted(props: &UseLastedProps) -> Html {
    let UseLastedProps {} = props;

    html! {
        <div
            class="w-screen h-screen bg-cover bg-no-repeat bg-center flex flex-col justify-center"
            style="font-family: Monocraft, monospace; background-image: url(https://uselasted.netlify.app/static/media/751400.530ccd0e600c0697d435.png)"
        >
            <div
                class="max-w-fit m-auto flex flex-col text-white px-2"
            >
                <Lasted />
                <Whys />
                <Wasted />
            </div>
        </div>
    }
}
