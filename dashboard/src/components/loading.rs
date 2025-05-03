use leptos::{either::Either, prelude::*};

use crate::utils::LoadingStatus;

#[component]
pub fn LazyShow<W, C>(
    children: TypedChildrenFn<C>,
    when: W,
) -> impl IntoView
where
    W: Fn() -> LoadingStatus + Send + Sync + 'static,
    C: IntoView + 'static,
{
    let memoized_when = ArcMemo::new(move |_| when());
    let children = children.into_inner();

    move || match memoized_when.get() {
        LoadingStatus::Ok => Either::Left(children()),
        LoadingStatus::Err(e) => Either::Right(view! {
            <div data-error>
                <span>Error: {e}</span>
            </div>
        }.into_any()),
        LoadingStatus::Loading => Either::Right(view! {
            <div data-loading>
                <span>Loading</span>
            </div>
        }.into_any()),
    }
}
