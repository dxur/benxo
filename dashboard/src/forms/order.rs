use backend::models::{order::*, ObjectId};
use leptos::prelude::*;

use super::{Accessor, IntoForm};

#[derive(Debug, Copy, Clone, Default)]
pub struct OrderCreateAccessor {
    pub full_name: RwSignal<String>,
    pub items: RwSignal<Vec<CartItem>>,
}

impl TryFrom<OrderCreateAccessor> for OrderCreate {
    type Error = ();

    fn try_from(value: OrderCreateAccessor) -> Result<Self, Self::Error> {
        Ok(OrderCreate {
            full_name: value.full_name.get(),
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
    pub items: RwSignal<Vec<CartItem>>,
}

impl TryFrom<OrderUpdateAccessor> for OrderUpdate {
    type Error = ();

    fn try_from(value: OrderUpdateAccessor) -> Result<Self, Self::Error> {
        Ok(OrderUpdate {
            id: value.id,
            body: OrderUpdateBody {
                full_name: Some(value.full_name.get()).filter(|s| !s.is_empty()),
                status:  Some(value.status.get()).filter(|s| *s == value.status_original),
                items: Some(value.items.get())
            }
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
            {outlet}
        }
        .into_any()
    }

    fn build_update_form(
        val: OrderPublic,
        acc: Self::UpdateAccessor,
        outlet: AnyView,
    ) -> AnyView {
        view! {
            <fieldset>
                <label> Full Name
                    <input type="text" placeholder=val.full_name bind:value=acc.full_name />
                </label>
            </fieldset>
            {outlet}
        }
        .into_any()
    }
}


