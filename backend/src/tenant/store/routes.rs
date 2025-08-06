use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use chrono::Utc;
use hyper::{header, HeaderMap};
use liquid::partials::{EagerCompiler, InMemorySource, PartialSource};
use macros::routes;
use std::str::FromStr;
use tracing::{debug, error, trace};

use crate::extractors::cookies::FromCookies;
use crate::extractors::json::Json;
use crate::platform::business::api::BusinessSession;
use crate::platform::user::api::MessageResponse;
use crate::tenant::product::api::ProductListQuery;
use crate::types::id::Id;
use crate::utils::error::ApiResult;
use crate::AppState;

use super::api::*;
use super::extractors::*;

pub struct StoreRoutes;

#[routes(prefix = "/api/v1/stores", state = AppState)]
impl StoreRoutes {
    #[route(method=post, path="/create", res=StoreDto)]
    async fn create_store(
        State(state): State<AppState>,
        FromCookies(business): FromCookies<BusinessSession>,
    ) -> ApiResult<Json<StoreDto>> {
        state.store_service.create(business).await.map(Json)
    }

    #[route(method=post, path="/{store_id}", res=StoreDto)]
    async fn get_store(
        State(state): State<AppState>,
        FromCookies(business): FromCookies<BusinessSession>,
        #[path] store_id: Id,
    ) -> ApiResult<Json<StoreDto>> {
        state
            .store_service
            .get_store(business, store_id)
            .await
            .map(Json)
    }

    #[route(method=post, path="/list", res=StoreListResponse)]
    async fn list_stores(
        State(state): State<AppState>,
        FromCookies(business): FromCookies<BusinessSession>,
        Query(query): Query<StoreListQuery>,
    ) -> ApiResult<Json<StoreListResponse>> {
        state
            .store_service
            .list_stores(business, query)
            .await
            .map(Json)
    }

    #[route(method=patch, path="/{store_id}", res=StoreDto)]
    async fn edit_store(
        State(state): State<AppState>,
        FromCookies(business): FromCookies<BusinessSession>,
        #[path] store_id: Id,
        #[json] update_req: StoreUpdate,
    ) -> ApiResult<Json<StoreDto>> {
        state
            .store_service
            .update_store(business, store_id, update_req)
            .await
            .map(Json)
    }

    #[route(method=patch, path="/set-reg/{store_id}", res=StoreRegDto)]
    async fn set_reg(
        State(state): State<AppState>,
        FromCookies(business): FromCookies<BusinessSession>,
        #[path] store_id: Id,
        #[json] update_req: StoreRegUpdate,
    ) -> ApiResult<Json<StoreRegDto>> {
        state
            .store_service
            .set_reg(business, store_id, update_req)
            .await
            .map(Json)
    }

    #[route(method=delete, path="/{store_id}", res=MessageResponse)]
    async fn delete_store(
        State(state): State<AppState>,
        FromCookies(business): FromCookies<BusinessSession>,
        #[path] store_id: Id,
    ) -> ApiResult<Json<MessageResponse>> {
        state
            .store_service
            .delete_store(business, store_id)
            .await
            .map(|_| MessageResponse {
                message: "Store deleted successfully".to_string(),
            })
            .map(Json)
    }
}

pub struct PubStoreRoutes;

#[routes(prefix = "", state = AppState)]
impl PubStoreRoutes {
    #[route(method=get, path="/")]
    async fn home(State(state): State<AppState>, Store(store): Store) -> impl IntoResponse {
        trace!("request for the home page: {:#?}", store);

        let business = state
            .business_service
            .get(store.business_id.clone())
            .await
            .map_err(Into::<StatusCode>::into)?;
        let products = state
            .product_service
            .pub_list_products(
                store.business_id,
                ProductListQuery {
                    page: None,
                    limit: None,
                    status: None,
                    category: None,
                    featured: None,
                    search: None,
                },
            )
            .await
            .map_err(Into::<StatusCode>::into)?;

        type Partials = EagerCompiler<InMemorySource>;

        let partials = {
            let mut partials = Partials::empty();
            let filepath = String::from("../templates/style.css.liquid");
            let contents = std::fs::read_to_string(&filepath).unwrap();
            partials.add(filepath, contents);
            partials
        };

        let template = liquid::ParserBuilder::with_stdlib()
            .partials(partials)
            .build()
            .unwrap()
            .parse_file("../templates/index.liquid")
            .unwrap();

        let store = "Store123";

        let globals = liquid::object!({
            "store": business.name,
            "year": "2025",
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
