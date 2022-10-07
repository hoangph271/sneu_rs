use crate::router::*;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct HeaderProps {}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let HeaderProps {} = props;

    html! {
        <header class="p-4 shadow-sm shadow-slate-200 hover:shadow-md">
            <nav>
                <SneuLink to={SneuRoutes::Home} classes="hover:underline">
                    { "@sneu_rs" }
                </SneuLink>
            </nav>
        </header>
    }
}
