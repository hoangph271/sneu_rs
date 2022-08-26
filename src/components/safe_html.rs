use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SafeHtmlProps {
    pub html: String,
    pub tag: String,
}

#[function_component(SafeHtml)]
pub fn safe_html(props: &SafeHtmlProps) -> Html {
    let code = gloo_utils::document()
        .create_element(&props.tag)
        .unwrap_or_else(|e| panic!("create_element() [{:?}] failed: {e:?}", &props.tag));

    code.set_inner_html(&props.html.clone());

    Html::VRef(code.into())
}
