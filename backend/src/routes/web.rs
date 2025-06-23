use axum::extract::{Query, State};
use axum::response::{Html, IntoResponse};
use hyper::{HeaderMap, StatusCode};
use macros::routes;
use serde::Deserialize;
use ts_rs::TS;

use crate::extractors::{StoreMeta, UserData};
use crate::models::domain::{Domain, DomainFetch};
use crate::models::store::{Store, StoreCreate, StoreFetch};
use crate::models::Pagination;
use crate::routes::generic;
use crate::utils::types::{IntoContext, WithContext};
use crate::AppState;

pub struct WebRoutes;

#[routes(prefix="/", state=AppState)]
impl WebRoutes {
    #[route(method=get, path="/")]
    async fn home(State(state): State<AppState>, StoreMeta(store): StoreMeta) -> impl IntoResponse {
        let res = generic::get_all::<crate::models::product::Product>(
            &state,
            Pagination {
                page: None,
                per_page: None,
            },
            store.business_id.into_context(),
        )
        .await?;

        let template = liquid::ParserBuilder::with_stdlib()
            .build()
            .unwrap()
            .parse_file(".trash/template.liquid")
            .unwrap();

        let globals = liquid::object!({
            "store": store,
            "products": res
        });
        let output = template
            .render(&globals)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Result::<_, StatusCode>::Ok(Html(output))
    }

    #[route(method=get, path="/should-issue")]
    async fn cert(
        State(state): State<AppState>,
        Query(domain): Query<DomainFetch>,
    ) -> impl IntoResponse {
        generic::get_one::<Domain>(&state, domain)
            .await
            .map(|_| StatusCode::OK)
    }
}
