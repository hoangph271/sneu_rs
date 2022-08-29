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
pub enum BulmaVariant {
    #[default]
    Primary,
    Warning,
    Danger,
}

impl BulmaVariant {
    pub fn to_classname(&self) -> String {
        let suffix = match self {
            BulmaVariant::Primary => "primary",
            BulmaVariant::Warning => "warning",
            BulmaVariant::Danger => "danger",
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
    pub variant: BulmaVariant,
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
            class={ variant.to_classname() }
        >
            { children.clone() }
        </button>
    }
}
