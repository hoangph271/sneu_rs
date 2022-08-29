use super::BulmaVariant;

use yew::prelude::*;

// <Toast
//     header="Danger...!"
//     variant={BulmaVariant::Danger}
// >
//     { "Lorem ipsum dolor sit amet" }
// </Toast>

#[derive(PartialEq, Properties, Default)]
pub struct ToastProps {
    #[prop_or_default]
    variant: BulmaVariant,
    header: String,
    children: Children,
    #[prop_or(Callback::from(|_| {}))]
    ondismiss: Callback<MouseEvent>,
}

#[function_component(Toast)]
pub fn toast(props: &ToastProps) -> Html {
    let ToastProps {
        variant,
        header,
        children,
        ondismiss,
    } = props;

    html! {
        <article class={ format!("message is-{}", variant.to_classname()) }>
            <div class="message-header">
                <p>{ header }</p>
                <button
                    class="delete"
                    aria-label="delete"
                    onclick={ondismiss}
                />
            </div>
            <div class="message-body">
                { children.clone() }
            </div>
        </article>
    }
}
