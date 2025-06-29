use axum::extract::{Path, Query, State};
use axum::response::{Html, IntoResponse};
use backend::db::domain::Domain;
use hyper::StatusCode;
use macros::routes;
use tracing::{debug, error};

use crate::AppState;
use backend::db::product::Product;
use backend::extractors::StoreMeta;
use backend::models::domain::DomainFetch;
use backend::models::product::*;
use backend::models::Pagination;
use backend::routes::generic;
use backend::utils::types::{IntoContext, WithContext};

pub struct Routes;

#[routes(prefix="/", state=AppState)]
impl Routes {
    #[route(method=get, path="/")]
    async fn home(State(state): State<AppState>, StoreMeta(store): StoreMeta) -> impl IntoResponse {
        let res = generic::get_all::<Product>(
            &state,
            Pagination {
                page: None,
                per_page: None,
            },
            store.business_id.into_context(),
        )
        .await?;

        // TODO: cache the template
        let template = liquid::ParserBuilder::with_stdlib()
            .build()
            .unwrap()
            .parse_file("../templates/index.liquid")
            .unwrap();

        let globals = liquid::object!({
            "store": store,
            "products": res,
        });

        debug!("{:?}", globals);

        let output = template.render(&globals).map_err(|e| {
            error!("{:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
        Result::<_, StatusCode>::Ok(Html(output))
    }

    #[route(method=get, path="/products/{slug}")]
    async fn product(
        State(state): State<AppState>,
        StoreMeta(store): StoreMeta,
        Path(slug): Path<String>,
    ) -> impl IntoResponse {
        let res = generic::get_one::<Product>(
            &state,
            ProductFetch::Slug(slug).with_context(store.business_id),
        )
        .await;

        format!("product {:?}", res)
    }

    #[route(method=get, path="/should-issue")]
    async fn issue_cert(
        State(state): State<AppState>,
        Query(domain): Query<DomainFetch>,
    ) -> impl IntoResponse {
        generic::get_one::<Domain>(&state, domain)
            .await
            .map(|_| StatusCode::OK)
    }
}
