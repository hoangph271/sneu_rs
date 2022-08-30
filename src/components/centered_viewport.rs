use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct CenteredViewportProps {
    pub children: Children,
}

#[function_component(CenteredViewport)]
pub fn centered_viewport(props: &CenteredViewportProps) -> Html {
    let CenteredViewportProps { children } = props;

    html! {
        <div class="h-screen w-screen flex items-center justify-center">
            { children.clone() }
        </div>
    }
}
