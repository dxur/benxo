use leptos::{prelude::*, view};

pub mod order;

pub trait Accessor {
    type CreateAccessor: Clone;
    type UpdateAccessor: Clone;
}

pub trait IntoForm<T>: Accessor {
    fn build_create_form(_: Self::CreateAccessor, outlet: AnyView) -> AnyView {
        view! {
            <fieldset>
                <label> Test Field
                    <input type="text" />
                </label>
            </fieldset>
            {outlet}
        }
        .into_any()
    }

    fn build_update_form(_: T, _: Self::UpdateAccessor, outlet: AnyView) -> AnyView {
        view! {
            <fieldset>
                <label> Test Field
                    <input type="text" />
                </label>
            </fieldset>
            {outlet}
        }
        .into_any()
    }
}
