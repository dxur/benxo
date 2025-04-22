use leptos::prelude::*;

#[component]
pub fn Users() -> AnyView {
    view! {
        <div class="flex flex-1 flex-col w-full p-3 space-y-6 overflow-y-auto">
            <div class="flex items-center justify-between h-16 border-b-2 border-surface-border pb-2">
                <title>Users</title>
                <h1 class="text-3xl font-semibold text-gray-800">Users</h1>
            </div>
        </div>
    }
    .into_any()
}
