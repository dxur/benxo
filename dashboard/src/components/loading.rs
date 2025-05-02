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
        LoadingStatus::Err(_) => Either::Right(view! {
            <div data-error>
                <span>Error</span>
            </div>
        }),
        LoadingStatus::Loading => Either::Right(view! {
            <div data-loading>
                <span>Loading</span>
            </div>
        }),
    }
}
