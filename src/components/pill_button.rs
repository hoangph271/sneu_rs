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
pub enum ColorVariant {
    #[default]
    Primary,
    Warning,
    Danger,
}

impl ColorVariant {
    pub fn bg(&self) -> String {
        match self {
            ColorVariant::Primary => "bg-sky-500 hover:bg-sky-700",
            ColorVariant::Warning => "bg-amber-500 hover:bg-amber-700",
            ColorVariant::Danger => "bg-rose-500 hover:bg-rose-700",
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

#[function_component(PillButton)]
pub fn pill_button(props: &BulmaButtonProps) -> Html {
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
            class={ format!("rounded-full px-6 py-1 text-white {}", variant.bg()) }
        >
            { children.clone() }
        </button>
    }
}