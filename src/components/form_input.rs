use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(PartialEq, Eq)]
pub enum InputType {
    Text,
    Password,
}
impl InputType {
    fn to_type_attr(&self) -> String {
        match self {
            InputType::Text => "text".to_owned(),
            InputType::Password => "password".to_owned(),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct FormInputProps {
    pub input_type: InputType,
    pub label: String,
    pub value: String,
    pub on_value_changed: Callback<String>,
}

#[function_component(FormInput)]
pub fn form_input(props: &FormInputProps) -> Html {
    let on_value_changed = props.on_value_changed.clone();

    html! {
        <div class="field">
            <label class="label">{ "Username:" }</label>
            <div class="control has-icons-left has-icons-right">
                <input
                    type={props.input_type.to_type_attr()}
                    class="input"
                    placeholder="Your username"
                    value={props.value.clone()}
                    oninput={move |e: InputEvent| {
                        let value = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok()).unwrap().value();

                        on_value_changed.emit(value);
                    }}
                />
                <span class="icon is-small is-left">
                    <i class="fas fa-user"></i>
                </span>
            </div>
        </div>
    }
}
