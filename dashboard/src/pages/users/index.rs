use leptos::prelude::*;

use crate::components::*;
use crate::pages::Page;
use super::state::IndexState as State;

#[allow(non_upper_case_globals)]
pub const UsersIndex : Page = Page {
    title: "Users",
    view: View,
};

#[component]
fn View() -> AnyView {
    let state = State::new();
    view! {
        <Header title=UsersIndex.title>
            <button>
                New
            </button>
        </Header>
        <LazyShow
            when=move || state.status.get()
        >
            <UsersTable state=state />
        </LazyShow>
    }.into_any()
}


#[component]
fn UsersTable(state: State) -> impl IntoView {
    view! {
        <Table head=move || {
            view! {
                <tr>
                    <th>Name</th>
                    <th>Email</th>
                    <th>Options</th>
                </tr>
            }
        }>
            <For
                each=move || state.users.get().data
                key=|_| ()
                let(user)
            >
                <tr>
                    <td>{user.name}</td>
                    <td>{user.email}</td>
                    <td>
                        <button on:click=move |_| {
                            // State::edit(user.id);
                        }> Edit </button>
                        <button type="reset" on:click=move |_| {
                            // state.delete(user.id);
                        }> Delete </button>
                    </td>
                </tr>
            </For>
        </Table>
        <TablePagination page=state.page total=state.total.read_only() />
    }
}
