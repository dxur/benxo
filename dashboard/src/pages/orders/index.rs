use leptos::prelude::*;

use super::state::IndexState as State;
use crate::components::*;
use crate::pages::Page;

#[allow(non_upper_case_globals)]
pub const OrdersIndex: Page = Page {
    title: "Orders",
    view: View,
};

#[component]
fn View() -> AnyView {
    let state = State::new();
    view! {
        <Header title=OrdersIndex.title>
            <button on:click=move |_| { state.dialog.get().map(|d| d.show()); }>
                New
            </button>
        </Header>
        <LazyShow
            when=move || state.status.get()
        >
            <OrdersTable state=state />
        </LazyShow>
        <OrderCreate state=state />

    }
    .into_any()
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
                            State::edit(user.id);
                        }> Details </button>
                    </td>
                </tr>
            </For>
        </Table>
        <TablePagination page=state.page total=state.total.read_only() />
    }
}

#[component]
fn OrderCreate(state: State) -> impl IntoView {
    view! {
        <Dialog
            node_ref=state.dialog
            on_cancel=move || {
                state.dialog.get().map(|d| d.close());
            }
        >
            <header>
                <h2>Create Product</h2>
            </header>
            <form on:submit=move |ev| {
                ev.prevent_default();
                state.create();
            }>
                <fieldset>
                    <label> Full Name
                        <input type="text" bind:value=state.fields.full_name required />
                    </label>
                </fieldset>
                <fieldset>
                    <label> Phone
                        <input type="tel" bind:value=state.fields.phone required />
                    </label>
                </fieldset>
                <fieldset>
                    <label> Email
                        <input type="email" bind:value=state.fields.email required />
                    </label>
                </fieldset>
                <fieldset>
                    <label> Province
                        <input type="text" bind:value=state.fields.province required />
                    </label>
                </fieldset>
                <fieldset>
                    <label> Address
                        <input type="text" bind:value=state.fields.address required />
                    </label>
                </fieldset>
                <fieldset>
                    <label> Note
                        <input type="text" bind:value=state.fields.note required />
                    </label>
                </fieldset>
                <button type="button" 
                    on:click=move |_| { state.dialog.get().map(|d| d.close()); }
                >Close</button>
                <button type="submit">Submit</button>
            </form>
        </Dialog>
    }
}
