use yew::prelude::*;

const WHYS: [&str; 3] = [
    "Stay brown for 30 days, and THEN back to green...! :\"}",
    "Really, it makes me feel so tired sometimes...!",
    "Cuz you\'ve been well behaved for the last 9 days, keep it going...! 🍀",
];

#[function_component(Whys)]
pub fn whys() -> Html {
    html! {
        <ul>
            { WHYS.iter().map(|why| {
                html! {
                    <li key={*why}>
                        { format!("• {why}") }
                    </li>
                }
            }).collect::<Html>() }
        </ul>
    }
}
