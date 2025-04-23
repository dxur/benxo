use leptos::{prelude::*, view};

pub mod product;

pub trait Accessor {
    type CreateAccessor: Copy + Clone;
    type UpdateAccessor: Copy + Clone;
}

pub trait IntoForm: Accessor {
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

    fn build_update_form(_: Self::UpdateAccessor, outlet: AnyView) -> AnyView {
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
