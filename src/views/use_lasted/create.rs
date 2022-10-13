use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct CreateUseLastedProps {}

#[function_component(CreateUseLasted)]
pub fn create_use_lasted(props: &CreateUseLastedProps) -> Html {
    let CreateUseLastedProps {} = props;
    html! {
        <form>
        </form>
    }
}
