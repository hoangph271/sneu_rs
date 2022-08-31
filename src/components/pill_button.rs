use crate::theme::*;
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

#[derive(PartialEq, Properties)]
pub struct BulmaButtonProps {
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or(Callback::from(|_| {}))]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub button_type: ButtonType,
    #[prop_or_default]
    pub variant: ColorVariant,
    #[prop_or_default]
    pub children: Children,
}

pub struct PillButtonTheme {
    variant: ColorVariant,
}
impl Themable for PillButtonTheme {
    fn sub_theme(&self) -> Option<Box<dyn Themable>> {
        None
    }

    fn classnames(&self) -> String {
        format!("rounded-full px-6 py-1 text-white {}", self.variant.bg())
    }
}

#[function_component(PillButton)]
pub fn pill_button(props: &BulmaButtonProps) -> Html {
    let BulmaButtonProps {
        onclick,
        button_type,
        disabled,
        variant,
        children,
    } = props;

    let theme = PillButtonTheme {
        variant: (*variant).clone(),
    };

    html! {
        <button
            onclick={{
                let onclick = onclick.clone();

                move |e| onclick.emit(e)
            }}
            type={ button_type.to_type_attr() }
            disabled={ *disabled }
            class={theme.classnames()}
        >
            { children.clone() }
        </button>
    }
}
