use backend::models::{ObjectId, order::*};
use leptos::prelude::*;

use super::{Accessor, IntoForm};

#[derive(Debug, Copy, Clone, Default)]
pub struct OrderCreateAccessor {
    pub full_name: RwSignal<String>,
    pub phone: RwSignal<String>,
    pub email: RwSignal<String>,
    pub province: RwSignal<String>,
    pub address: RwSignal<String>,
    pub note: RwSignal<String>,
    pub items: RwSignal<Vec<CartItem>>,
}

impl TryFrom<OrderCreateAccessor> for OrderCreate {
    type Error = ();

    fn try_from(value: OrderCreateAccessor) -> Result<Self, Self::Error> {
        Ok(OrderCreate {
            full_name: value.full_name.get(),
            phone: value.phone.get(),
            email: value.email.get(),
            province: value.province.get(),
            address: value.address.get(),
            note: value.note.get(),
            items: value.items.get(),
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct OrderUpdateAccessor {
    pub id: ObjectId,
    pub status_original: OrderStatus,
    pub status: RwSignal<OrderStatus>,
    pub full_name: RwSignal<String>,
    pub phone: RwSignal<String>,
    pub email: RwSignal<String>,
    pub province: RwSignal<String>,
    pub address: RwSignal<String>,
    pub note: RwSignal<String>,
    pub items: RwSignal<Vec<CartItem>>,
}

impl TryFrom<OrderUpdateAccessor> for OrderUpdate {
    type Error = ();

    fn try_from(value: OrderUpdateAccessor) -> Result<Self, Self::Error> {
        Ok(OrderUpdate {
            id: value.id,
            body: OrderUpdateBody {
                full_name: Some(value.full_name.get()).filter(|s| !s.is_empty()),
                status: Some(value.status.get()).filter(|s| *s == value.status_original),
                items: Some(value.items.get()),
                phone: Some(value.phone.get()).filter(|s| !s.is_empty()),
                email: Some(value.email.get()).filter(|s| !s.is_empty()),
                province: Some(value.province.get()).filter(|s| !s.is_empty()),
                address: Some(value.address.get()).filter(|s| !s.is_empty()),
                note: Some(value.note.get()).filter(|s| !s.is_empty()),
            },
        })
    }
}

impl From<&OrderPublic> for OrderUpdateAccessor {
    fn from(value: &OrderPublic) -> Self {
        Self {
            id: value.id,
            status_original: value.status,
            status: RwSignal::new(value.status),
            full_name: RwSignal::default(),
            phone: RwSignal::default(),
            email: RwSignal::default(),
            province: RwSignal::default(),
            address: RwSignal::default(),
            note: RwSignal::default(),
            items: RwSignal::default(),
        }
    }
}

impl Accessor for Order {
    type CreateAccessor = OrderCreateAccessor;
    type UpdateAccessor = OrderUpdateAccessor;
}

impl IntoForm<OrderPublic> for Order {
    fn build_create_form(acc: Self::CreateAccessor, outlet: AnyView) -> AnyView {
        view! {
            <fieldset>
                <label> Full Name
                    <input type="text" bind:value=acc.full_name required />
                </label>
            </fieldset>
            <fieldset>
                <label> Phone
                    <input type="tel" bind:value=acc.phone required />
                </label>
            </fieldset>
            <fieldset>
                <label> Email
                    <input type="text" bind:value=acc.email required />
                </label>
            </fieldset>
            <fieldset>
                <label> Province
                    <input type="text" bind:value=acc.province required />
                </label>
            </fieldset>
            <fieldset>
                <label> Address
                    <input type="text" bind:value=acc.address required />
                </label>
            </fieldset>
            <fieldset>
                <label> Note
                    <input type="text" bind:value=acc.note required />
                </label>
            </fieldset>
            {outlet}
        }
        .into_any()
    }

    fn build_update_form(val: OrderPublic, acc: Self::UpdateAccessor, outlet: AnyView) -> AnyView {
        view! {
            <fieldset>
                <label> Full Name
                    <input type="text" placeholder=val.full_name bind:value=acc.full_name />
                </label>
            </fieldset>
            <fieldset>
                <label> Phone
                    <input type="tel" placeholder=val.phone bind:value=acc.phone />
                </label>
            </fieldset>
            <fieldset>
                <label> Email
                    <input type="text" placeholder=val.email bind:value=acc.email />
                </label>
            </fieldset>
            <fieldset>
                <label> Province
                    <input type="text" placeholder=val.province bind:value=acc.province />
                </label>
            </fieldset>
            <fieldset>
                <label> Address
                    <input type="text" placeholder=val.address bind:value=acc.address />
                </label>
            </fieldset>
            <fieldset>
                <label> Note
                    <input type="text" placeholder=val.note bind:value=acc.note />
                </label>
            </fieldset>
            {outlet}
        }
        .into_any()
    }
}
