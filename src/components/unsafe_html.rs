use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct UnsafeHtmlProps {
    #[prop_or_default]
    pub class: Classes,
    pub html: String,
    pub tag: String,
}

#[function_component(UnsafeHtml)]
pub fn unsafe_html(props: &UnsafeHtmlProps) -> Html {
    let UnsafeHtmlProps { tag, html, class } = props;
    let code = gloo_utils::document()
        .create_element(tag)
        .unwrap_or_else(|e| panic!("create_element() [{:?}] failed: {e:?}", &tag));

    code.set_class_name(&class.to_string());
    code.set_inner_html(html);

    Html::VRef(code.into())
}
