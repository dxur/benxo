use std::str::FromStr;
use std::mem;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use chrono::Utc;
use chrono::Datelike;
use hyper::{header, HeaderMap};
use indexmap::IndexMap;
use liquid::partials::{EagerCompiler, InMemorySource, PartialSource};
use liquid::{Parser, ParserBuilder};
use macros::routes;
use tracing::{debug, error, trace};

use super::api::*;
use super::extractors::*;
use crate::extractors::cookies::FromCookies;
use crate::extractors::json::Json;
use crate::platform::business::api::BusinessSession;
use crate::platform::user::api::MessageResponse;
use crate::tenant::product::api::ProductListQuery;
use crate::types::id::Id;
use crate::utils::error::ApiResult;
use crate::utils::error::ApiError;
use crate::utils::types::CowStr;
use crate::AppState;

pub struct StoreRoutes;

#[routes(prefix = "/api/v1/stores", state = AppState)]
impl StoreRoutes {
    #[route(method=post, path="/create", res=StoreDto)]
    async fn create_store(
        State(state): State<AppState>,
        FromCookies(business): FromCookies<BusinessSession>,
        #[json] create_req: StoreCreateDto,
    ) -> ApiResult<Json<StoreDto>> {
        state
            .store_service
            .create(business, create_req)
            .await
            .map(Json)
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
        #[query] query: StoreListQuery,
    ) -> ApiResult<Json<StoreListResponse>> {
        state
            .store_service
            .list_stores(business, query)
            .await
            .map(Json)
    }

    #[route(method=patch, path="/{store_id}", res=StoreDto)]
    async fn update_store(
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

    #[route(method=post, path="/store-reg/{store_id}", res=StoreRegDto)]
    async fn get_reg(
        State(state): State<AppState>,
        FromCookies(business): FromCookies<BusinessSession>,
        #[path] store_id: Id,
    ) -> ApiResult<Json<StoreRegDto>> {
        state
            .store_service
            .get_reg(business, store_id)
            .await
            .map(Json)
    }

    #[route(method=patch, path="/store-reg/{store_id}", res=StoreRegDto)]
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
    // ------ Helpers ------
    fn build_partials(snippets: IndexMap<String, CowStr>) -> EagerCompiler<InMemorySource> {
        let mut partials: EagerCompiler<InMemorySource> = Default::default();
        for (name, tpl) in snippets {
            partials.add(name.as_str(), tpl.as_ref());
        }
        partials
    }

    fn base_store_globals(store: StoreDto) -> liquid::model::Object {
        liquid::object!({
            "store": {
                "id": store.id,
                "name": store.name,
                "description": store.description,
                "status": store.status,

                // Contact
                "category": store.category,
                "contact_email": store.contact_email,
                "contact_phone": store.contact_phone,
                "address": store.address,
                "city": store.city,
                "zip_code": store.zip_code,

                "logo": store.logo,
                "logo_alt": store.logo_alt,
                "favicon": store.favicon,

                "menu_items": store.menu_items,
                "featured_collections": store.featured_collections,
                "social_links": store.social_links,
                "footer_lists": store.footer_lists,

                // SEO
                "meta": {
                    "title": store.meta_title,
                    "description": store.meta_description,
                    "keywords": store.meta_keywords,
                },

                // Analytics / tracking
                "analytics": {
                    "google_analytics_id": store.google_analytics_id,
                    "gtm_container_id": store.gtm_container_id,
                },

                // Custom key/values
                "custom": store.custom_key_values,
            },
            "year": Utc::now().year()
        })
    }

    fn merge_globals(mut base: liquid::Object, extras: liquid::Object) -> liquid::Object {
        for (k, v) in extras {
            base.insert(k, v);
        }
        base
    }

    fn parser_for(snippets: IndexMap<String, CowStr>) -> ApiResult<Parser> {
        ParserBuilder::with_stdlib()
            .partials(Self::build_partials(snippets))
            .build().map_err(|e| ApiError::internal(format!("Failed to build liquid parser: {}", e)))
    }

    fn render_page(
        globals: liquid::Object,
        snippets: IndexMap<String, CowStr>,
        template_src: &str,
    ) -> ApiResult<String> {
        let parser = Self::parser_for(snippets)?;
        let template = parser.parse(template_src).map_err(|e| ApiError::internal(format!("Failed to parse template: {}", e)))?;
        template.render(&globals).map_err(|e| ApiError::internal(format!("Failed to render template: {}", e)))
    }

    fn store_not_found_page(mut store: StoreDto, slug: Option<String>) -> (StatusCode, Html<CowStr>) {
        let template = mem::take(&mut store.not_found_page_template);
        let snippets = mem::take(&mut store.snippets);

        let extras = liquid::object!({
            "page": {
                "slug": slug,
            }
        });

        let globals = Self::merge_globals(Self::base_store_globals(store), extras);

        match Self::render_page(globals, snippets, &template) {
            Ok(out) => (StatusCode::NOT_FOUND, Html(CowStr::from(out))),
            Err(e) => {
                error!("404 render error: {:?}", e);
                    Into::<(StatusCode, Html<CowStr>)>::into(ApiError::internal("Failed to render page"))
            }
        }
    }

    // ------ Public routes ------

    #[route(method=get, path="/")]
    pub async fn home(State(state): State<AppState>, Store(store_key): Store) -> impl IntoResponse {
        let mut store = state
            .store_service
            .get_active_store(store_key.business_id, store_key.store_id)
            .await
            .map_err(Into::<(StatusCode, Html<CowStr>)>::into)?;

        let featured = state
            .product_service
            .pub_list_products(
                store_key.business_id,
                ProductListQuery {
                    page: None,
                    limit: None,
                    status: None,
                    category: None,
                    featured: Some(true),
                    search: None,
                },
            )
            .await
            .map_err(Into::<(StatusCode, Html<CowStr>)>::into)?.products;

        let extras = liquid::object!({
            "featured_products": featured,
        });

        let template = mem::take(&mut store.homepage_template);
        let snippets = mem::take(&mut store.snippets);

        let globals = Self::merge_globals(Self::base_store_globals(store), extras);

        let out = Self::render_page(globals, snippets, &template).map_err(|e| {
            error!("liquid render error: {:?}", e);
            ApiError::internal("Failed to render page")
        }).map_err(Into::<(StatusCode, Html<CowStr>)>::into)?;

        Ok::<_, (StatusCode, Html<CowStr>)>(Html(out))
    }

    #[route(method=get, path="/products/{slug}")]
    pub async fn product_page(
        State(state): State<AppState>,
        Store(store_key): Store,
        Path(slug): Path<String>,
    ) -> impl IntoResponse {
        let mut store = state
            .store_service
            .get_active_store(store_key.business_id, store_key.store_id)
            .await
            .map_err(Into::<(StatusCode, Html<CowStr>)>::into)?;

        let product = match state
            .product_service
            .pub_get_product(store_key.business_id, &slug)
            .await {
                Err(ApiError::NotFound { .. }) => {
                    return Err(Self::store_not_found_page(store, Some(slug)));
                }
                Err(e) => {
                    return Err(Into::<(StatusCode, Html<CowStr>)>::into(e));
                }
                Ok(p) => p,
            };

        let related = state
            .product_service
            .pub_list_related_products(
                store_key.business_id,
                &slug,
            )
            .await
            .unwrap_or_default();

        let extras = liquid::object!({
            "product": product,
            "related_products": related,
        });

        let template = mem::take(&mut store.product_page_template);
        let snippets = mem::take(&mut store.snippets);

        let globals = Self::merge_globals(Self::base_store_globals(store), extras);
        std::fs::write("/tmp/debug.liquid", format!("{:#?}", globals)).ok();
        let out =
            Self::render_page(globals, snippets, &template).map_err(|e| {
                error!("liquid render error: {:?}", e);
                Into::<(StatusCode, Html<CowStr>)>::into(e)
            })?;

        Ok::<_, (StatusCode, Html<CowStr>)>(Html(out))
    }

    #[route(method=get, path="/cart")]
    pub async fn cart_page(
        State(state): State<AppState>,
        Store(store_key): Store,
    ) -> impl IntoResponse {
        let mut store = state
            .store_service
            .get_active_store(store_key.business_id, store_key.store_id)
            .await
            .map_err(Into::<(StatusCode, Html<CowStr>)>::into)?;

        let template = mem::take(&mut store.cart_page_template);
        let snippets = mem::take(&mut store.snippets);

        let globals = Self::base_store_globals(store);

        let out = Self::render_page(globals, snippets, &template).map_err(|e| {
            error!("liquid render error: {:?}", e);
            Into::<(StatusCode, Html<CowStr>)>::into(e)
        })?;

        Ok::<_, (StatusCode, Html<CowStr>)>(Html(out))
    }

    #[route(method=get, path="/shop")]
    pub async fn shop_page(
        State(state): State<AppState>,
        Store(store_key): Store,
    ) -> impl IntoResponse {
        let mut store = state
            .store_service
            .get_active_store(store_key.business_id, store_key.store_id)
            .await
            .map_err(Into::<(StatusCode, Html<CowStr>)>::into)?;

        let products = state
            .product_service
            .pub_list_products(
                store_key.business_id,
                // TODO: accept query params
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
            .map_err(Into::<(StatusCode, Html<CowStr>)>::into)?.products;

        let extras = liquid::object!({
            "query": None::<String>,
            "products": products
        });

        let template = mem::take(&mut store.shop_page_template);
        let snippets = mem::take(&mut store.snippets);

        let globals = Self::merge_globals(Self::base_store_globals(store), extras);

        let out = Self::render_page(globals, snippets, &template).map_err(|e| {
            error!("liquid render error: {:?}", e);
            Into::<(StatusCode, Html<CowStr>)>::into(e)
        })?;

        Ok::<_, (StatusCode, Html<CowStr>)>(Html(out))
    }

    #[route(method=get, path="/pages/{slug}")]
    pub async fn custom_page(
        State(state): State<AppState>,
        Store(store_key): Store,
        Path(slug): Path<String>,
    ) -> impl IntoResponse {
        let mut store = state
            .store_service
            .get_active_store(store_key.business_id, store_key.store_id)
            .await
            .map_err(Into::<(StatusCode, Html<CowStr>)>::into)?;

        match mem::take(&mut store.custom_pages).get(&slug) {
            Some(tpl) => {
                let extras = liquid::object!({
                    "page": {
                        "slug": slug,
                    }
                });

                let snippets = mem::take(&mut store.snippets);
                let globals = Self::merge_globals(Self::base_store_globals(store), extras);

                let out = Self::render_page(globals, snippets, tpl.as_ref()).map_err(|e| {
                    error!("liquid render error: {:?}", e);
                    Into::<(StatusCode, Html<CowStr>)>::into(e)
                })?;

                Ok::<_, (StatusCode, Html<CowStr>)>(Html(out))
            }
            None => Err(Self::store_not_found_page(store, Some(slug))),
        }
    }

    #[fallback]
    pub async fn fallback(
        State(state): State<AppState>,
        Store(store_key): Store,
    ) -> impl IntoResponse {
        let mut store = state
            .store_service
            .get_active_store(store_key.business_id, store_key.store_id)
            .await
            .map_err(Into::<(StatusCode, Html<CowStr>)>::into)?;

        let template = mem::take(&mut store.not_found_page_template);
        let snippets = mem::take(&mut store.snippets);

        let globals = Self::base_store_globals(store);

        let out = Self::render_page(globals, snippets, &template).map_err(|e| {
            error!("liquid render error: {:?}", e);
            Into::<(StatusCode, Html<CowStr>)>::into(e)
        })?;

        Ok::<_, (StatusCode, Html<CowStr>)>(Html(out))
    }
}
