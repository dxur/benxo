use axum::extract::{Path, Query, State};
use axum::response::{Html, IntoResponse};
use axum::http::StatusCode;
use chrono::Utc;
use hyper::{header, HeaderMap};
use macros::routes;
use std::str::FromStr;

use crate::extractors::cookies::FromCookies;
use crate::extractors::json::Json;
use crate::platform::business::api::BusinessSession;
use crate::tenant::product::api::ProductListQuery;
use crate::types::id::Id;
use crate::utils::error::ApiResult;
use crate::AppState;
use tracing::{trace, error, debug};

use super::api::*;

pub struct StoreRoutes;

#[routes(prefix = "/api/v1/stores", state = AppState)]
impl StoreRoutes {}


pub struct PubStoreRoutes;

#[routes(prefix = "", state = AppState)]
impl PubStoreRoutes {
    #[route(method=get, path="/{id}")]
    async fn home(
        State(state): State<AppState>,
        Path(business_id): Path<Id>,
    ) -> impl IntoResponse {
        trace!("request for the home page");
        let products = state
            .product_service
            .pub_list_products(business_id, ProductListQuery {
                page: None,
                limit: None,
                status: None,
                category: None,
                featured: None,
                search: None,
            })
            .await.map_err(Into::<StatusCode>::into)?;

        let template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse_file("./templates/index.liquid")
        .unwrap();

        let store = "Store123";

        let globals = liquid::object!({
            "store": store,
            "products": products,
        });

        debug!("{:?}", globals);

        let output = template.render(&globals).map_err(|e| {
            error!("{:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
        Result::<_, StatusCode>::Ok(Html(output))
    }
}
