use leptos::{ev::Event, html, prelude::*};

#[component]
pub fn Dialog<F>(show_on_mount: bool, on_close: F, children: Children) -> impl IntoView
where
    F: Fn() + 'static,
{
    let dialog_element: NodeRef<html::Dialog> = NodeRef::new();
    let show = move || {
        dialog_element.get().map(|dialog| dialog.show_modal());
    };
    let close = move || {
        dialog_element.get().map(|dialog| dialog.close());
        on_close();
    };

    view! {
        <dialog node_ref=dialog_element on:cancel=move |_: Event| { close() }>
            <iframe on:load=move |_| { if show_on_mount { show() } }></iframe>
            <section aria-modal="true" data-dialog>
                {children()}
            </section>
        </dialog>
    }
}
