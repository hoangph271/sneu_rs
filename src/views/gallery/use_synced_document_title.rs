use crate::utils::no_op;

use gloo_utils::document;
use yew::use_effect_with_deps;

pub fn use_synced_document_title(title: Option<String>) {
    use_effect_with_deps(
        {
            let title = title.clone();

            move |_| {
                if let Some(title) = title {
                    document().set_title(&title);
                }

                no_op
            }
        },
        title,
    );
}
