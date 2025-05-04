use leptos::prelude::*;

use crate::components::*;
use crate::pages::Page;
use super::state::IndexState as State;

#[allow(non_upper_case_globals)]
pub const OrdersIndex : Page = Page {
    title: "Orders",
    view: View,
};

#[component]
fn View() -> AnyView {
    let state = State::new();
    view! {
        <Header title=OrdersIndex.title>
            <button>
                New
            </button>
        </Header>
        <LazyShow
            when=move || state.status.get()
        >
            <OrdersTable state=state />
        </LazyShow>
    }.into_any()
}


#[component]
fn OrdersTable(state: State) -> impl IntoView {
    view! {
        <Table head=view! {
            <tr>
                <th>Full Name</th>
                <th>Phone</th>
                <th>Email</th>
                <th>Province</th>
                <th>Delivery</th>
                <th>Status</th>
                <th>Options</th>
            </tr>
        }>
            <For
                each=move || state.orders.get().data
                key=|_| ()
                let(user)
            >
                <tr>
                    <td>{user.full_name}</td>
                    <td>{user.phone}</td>
                    <td>{user.email}</td>
                    <td>{user.province}</td>
                    <td>{user.delivery.to_string()}</td>
                    <td>{user.status.to_string()}</td>
                    <td>
                        <button on:click=move |_| {
                            State::view(user.id);
                        }> Details </button>
                    </td>
                </tr>
            </For>
        </Table>
        <TablePagination page=state.page total=state.total.read_only() />
    }
}
