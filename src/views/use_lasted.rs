use crate::components::*;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct UseLastedProps {}

#[function_component(UseLasted)]
pub fn use_lasted(props: &UseLastedProps) -> Html {
    let UseLastedProps {} = props;

    html! {
        <div class="">
            <Lasted />
        </div>
    }
}
