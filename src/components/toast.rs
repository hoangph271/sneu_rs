use super::BulmaVariant;

use gloo_timers::callback::Timeout;
use gloo_utils::document;
use std::time::Duration;
use web_sys::Element;
use yew::prelude::*;

#[derive(PartialEq, Properties, Default)]
pub struct ToastProps {
    #[prop_or_default]
    pub variant: BulmaVariant,
    pub header: String,
    #[prop_or_default]
    pub children: Children,
    #[prop_or(Callback::from(|_| {}))]
    pub ondismiss: Callback<()>,
    #[prop_or_else(|| Some(Duration::from_secs(5)))]
    pub duration: Option<Duration>,
}

#[function_component(Toast)]
pub fn toast(props: &ToastProps) -> Html {
    let ToastProps {
        variant,
        header,
        children,
        ondismiss,
        duration,
    } = props;

    use_cleanup_toast(*duration, ondismiss.clone());

    create_portal(
        html! {
            <dialog
                open={true}
                style="position: absolute; border: none; bottom: 0; right: 0; left: unset; z-index: 5; margin: 1rem; padding: 0;"
            >
                <article class={ format!("message is-{}", variant.to_classname()) }>
                    <div class="message-header">
                        <p>{ header }</p>
                        <button
                            class="delete"
                            aria-label="delete"
                            onclick={{
                                let ondismiss = ondismiss.clone();
                                move |_| ondismiss.emit(())
                            }}
                        />
                    </div>
                    <div class="message-body">
                        { children.clone() }
                    </div>
                </article>
            </dialog>
        },
        get_toast_host(),
    )
}

const TOAST_HOST: &str = "toast-host";
fn get_toast_host() -> Element {
    document()
        .get_element_by_id(TOAST_HOST)
        .or_else(|| {
            let element = document()
                .create_element("div")
                .unwrap_or_else(|e| panic!("create_element(\"div\") failed: {e:?}"));

            element.set_id(TOAST_HOST);

            document()
                .body()
                .expect("Expectec to find the body element")
                .append_child(&*element)
                .unwrap_or_else(|e| panic!("append_child() failed: {e:?}"));

            Some(element)
        })
        .unwrap_or_else(|| panic!("Expected to find the #{TOAST_HOST} element"))
}

fn use_cleanup_toast(duration: Option<Duration>, ondismiss: Callback<()>) {
    use_effect_with_deps(
        {
            move |_| {
                let mut timeout = None;

                if let Some(duration) = duration {
                    let duration = duration.as_millis() as u32;

                    timeout = Some(Timeout::new(duration, move || {
                        ondismiss.emit(());
                    }));
                };

                || {
                    if let Some(timeout) = timeout {
                        timeout.cancel();
                    }
                }
            }
        },
        duration,
    );
}
