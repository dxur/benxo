use leptos::prelude::*;

#[component]
pub fn Table<F, IV>(
    head: F,
    children: Children,
) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! {
        <div data-table>
            <table>
                <thead>
                    {head()}
                </thead>
                <tbody>
                    {children()}
                </tbody>
            </table>
        </div>
    }
}


#[component]
pub fn TablePagination(
    page: RwSignal<usize>,
    total: ReadSignal<usize>,
) -> impl IntoView {
    view! {
        <nav>
            <button
                disabled=move || { page.get() == 1 }
                on:click=move |_| {
                    page.set(page.get_untracked().saturating_sub(1));
                }
            >
                Previous
            </button>
            <span>{"Page: "}{move || page.get()}</span>
            <button
                disabled=move || { page.get() >= total.get() }
                on:click=move |_| {
                    page.set(page.get_untracked().saturating_add(1).min(total.get_untracked()));
                }
            >
                Next
            </button>
        </nav>
    }
}
