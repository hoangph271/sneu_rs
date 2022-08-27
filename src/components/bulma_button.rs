use yew::prelude::*;

#[derive(PartialEq, Eq, Default)]
pub enum ButtonType {
    #[default]
    Button,
    Submit,
}
impl ButtonType {
    fn to_type_attr(&self) -> String {
        match self {
            ButtonType::Button => "button",
            ButtonType::Submit => "submit",
        }
        .to_owned()
    }
}

#[derive(PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Warning,
}
impl ButtonVariant {
    fn to_bulma_classname(&self) -> String {
        let suffix = match self {
            ButtonVariant::Primary => "primary",
            ButtonVariant::Warning => "warning",
        };

        format!("button is-{suffix}")
    }
}

#[derive(PartialEq, Properties)]
pub struct BulmaButtonProps {
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or(Callback::from(|_| {}))]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub button_type: ButtonType,
    #[prop_or_default]
    pub variant: ButtonVariant,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(BulmaButton)]
pub fn bulma_button(props: &BulmaButtonProps) -> Html {
    let BulmaButtonProps {
        onclick,
        button_type,
        disabled,
        variant,
        children,
    } = props;

    html! {
        <button
            onclick={{
                let onclick = onclick.clone();

                move |e| onclick.emit(e)
            }}
            type={ button_type.to_type_attr() }
            disabled={ *disabled }
            class={ variant.to_bulma_classname() }
        >
            { children.clone() }
        </button>
    }
}
