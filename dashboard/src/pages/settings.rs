use leptos::prelude::*;

use super::Page;

#[allow(non_upper_case_globals)]
pub const Settings: Page = Page {
    title: "Settings",
    view: SettingsView,
};

#[component]
pub fn SettingsView() -> AnyView {
    view! {
        <div class="flex flex-1 flex-col w-full p-3 space-y-6 overflow-y-auto">
            <div class="flex items-center justify-between h-16 border-b-2 border-surface-border pb-2">
                <title>Settings</title>
                <h1 class="text-3xl font-semibold text-gray-800">Settings</h1>
            </div>
        </div>
    }
    .into_any()
}
