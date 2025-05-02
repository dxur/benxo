use leptos::{ev::Event, html::*, prelude::*};

#[component]
pub fn Dialog<F>(node_ref: NodeRef<Dialog>, on_cancel: F, children: Children) -> impl IntoView
where
    F: Fn() + 'static,
{

    view! {
        <dialog node_ref=node_ref on:cancel=move |_: Event| { on_cancel() }>
            <section aria-modal="true" data-dialog>
                {children()}
            </section>
        </dialog>
    }
}
