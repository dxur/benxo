use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_location};

use crate::routes::{MENU, MenuItem};

fn sidebar_item(item: &'static MenuItem) -> impl IntoView {
    view! {
        <A href=item.path>
            <button
                disabled=move || use_location().pathname.get() == item.path
            >
                {item.name}
            </button>
        </A>
        {
            item.subitems.iter().map(|subitem| {
                sidebar_item(subitem)
            }).collect::<Vec<_>>()
        }
    }
    .into_any()
}

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <aside>
            <h1> Dashboard </h1>
            <nav>
            {
                MENU.iter().map(|item| {
                    sidebar_item(item)
                }).collect::<Vec<_>>()
            }
            </nav>
        </aside>
    }
}
