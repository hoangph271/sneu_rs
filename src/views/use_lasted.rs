use crate::components::*;
use yew::prelude::*;

#[derive(PartialEq, Eq, Properties)]
pub struct UseLastedProps {}

#[function_component(UseLasted)]
pub fn use_lasted(props: &UseLastedProps) -> Html {
    let UseLastedProps {} = props;

    html! {
        <div
            class="w-screen h-screen bg-cover bg-no-repeat bg-center flex flex-col justify-center text-white px-2"
            style="font-family: Monocraft; background-image: url(https://uselasted.netlify.app/static/media/751400.530ccd0e600c0697d435.png)"
        >
            <Lasted />
            <Whys />
            <Wasted />
        </div>
    }
}
