use yew::prelude::*;

#[derive(Properties, PartialEq, Eq)]
pub struct UnsafeHtmlProps {
    pub html: String,
    pub tag: String,
}

#[function_component(UnsafeHtml)]
pub fn unsafe_html(props: &UnsafeHtmlProps) -> Html {
    let code = gloo_utils::document()
        .create_element(&props.tag)
        .unwrap_or_else(|e| panic!("create_element() [{:?}] failed: {e:?}", &props.tag));

    code.set_inner_html(&props.html.clone());

    Html::VRef(code.into())
}
