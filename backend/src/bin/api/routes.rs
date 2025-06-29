use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use backend::db::domain::Domain;
use backend::db::order::Order;
use backend::db::product::Product;
use backend::db::store::Store;
use backend::utils::types::ResultBodyExt;
use backend::utils::types::ResultPageExt;
use bcrypt::{hash, verify, DEFAULT_COST};
use hyper::StatusCode;
use macros::routes;
use tower_cookies::Cookie;
use tower_cookies::Cookies;

use crate::AppState;
use backend::db::CreatableInDb;
use backend::db::FetchableInDb;
use backend::extractors::UserData;
use backend::models::auth::LoginCredentials;
use backend::models::auth::RegisterCredentials;
use backend::models::domain::*;
use backend::models::order::*;
use backend::models::product::*;
use backend::models::settings::*;
use backend::models::store::*;
use backend::models::user::*;
use backend::models::*;
use backend::routes::generic;
use backend::utils::auth::issue_access_tokens;
use backend::utils::auth::issue_refresh_tokens;
use backend::utils::types::IntoContext;
use backend::utils::types::ResultJsonExt;
use backend::utils::types::WithContext;

pub struct Routes;

#[routes(prefix="/api", state=AppState)]
impl Routes {
    // ---- Auth ----
    #[route(method=post, path="/auth")]
    async fn auth(_: State<AppState>, _: UserData) {}

    #[route(method=post, path="/register")]
    async fn register(
        State(state): State<AppState>,
        #[json] credentrals: RegisterCredentials,
    ) -> impl IntoResponse {
        let RegisterCredentials {
            name,
            email,
            password,
        } = credentrals;
        let hash = hash(password, DEFAULT_COST).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let _ = User::create(
            &state.db,
            UserCreate {
                name,
                email,
                password: hash,
            }
            .into(),
        )
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
        Result::<StatusCode, StatusCode>::Ok(StatusCode::OK)
    }

    #[route(method=post, path="/login")]
    async fn login(
        State(state): State<AppState>,
        cookies: Cookies,
        #[json] credentrals: LoginCredentials,
    ) -> impl IntoResponse {
        let LoginCredentials { email, password } = credentrals;

        let mut user = User::get_one(&state.db, UserFetch { email })
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::UNAUTHORIZED)?;

        if !verify(&password, &user.password).unwrap() {
            return Err(StatusCode::UNAUTHORIZED);
        }

        let data = UserData {
            business_id: user.business_id,
            email: user.email,
        };

        let access_token = issue_access_tokens(&data).ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
        let refresh_token = issue_refresh_tokens(&data).ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

        cookies.add(Cookie::build(("access_token", access_token)).build());
        cookies.add(Cookie::build(("refresh_token", refresh_token)).build());

        user.business_id = data.business_id;
        user.email = data.email;
        Ok(Json(UserPublic::from(user.into())))
    }

    #[route(method=get, path="/logout")]
    async fn logout(cookies: Cookies) {
        cookies.remove(Cookie::build("access_token").build());
        cookies.remove(Cookie::build("refresh_token").build());
    }

    // ---- Products ----
    #[route(method=get, path="/products", type=json, res=Page<ProductPublic>)]
    async fn get_all_products(
        State(state): State<AppState>,
        user: UserData,
        #[query] pagination: Pagination,
    ) -> impl IntoResponse {
        generic::get_all::<Product>(&state, pagination, user.business_id.into_context())
            .await
            .into_page()
            .into_json()
    }

    #[route(method=post, path="/products/", type=json, res=ProductPublic)]
    async fn get_one_product(
        State(state): State<AppState>,
        user: UserData,
        #[json] body: ProductFetch,
    ) -> impl IntoResponse {
        generic::get_one::<Product>(&state, body.with_context(user.business_id))
            .await
            .into_body()
            .into_json()
    }

    #[route(method=post, path="/products", type=json, res=ProductPublic)]
    async fn create_product(
        State(state): State<AppState>,
        user: UserData,
        #[json] body: ProductCreate,
    ) -> impl IntoResponse {
        generic::create::<Product>(&state, body.with_context(user.business_id))
            .await
            .into_body()
            .into_json()
    }

    #[route(method=patch, path="/products/", type=json, res=ProductPublic)]
    async fn update_product(
        State(state): State<AppState>,
        user: UserData,
        #[json] body: ProductUpdate,
    ) -> impl IntoResponse {
        generic::update::<Product>(&state, body.with_context(user.business_id))
            .await
            .into_body()
            .into_json()
    }

    #[route(method=delete, path="/products/", type=json, res=ProductPublic)]
    async fn delete_product(
        State(state): State<AppState>,
        user: UserData,
        #[json] body: ProductDelete,
    ) -> impl IntoResponse {
        generic::delete::<Product>(&state, body.with_context(user.business_id))
            .await
            .into_body()
            .into_json()
    }

    // ---- Orders ----
    #[route(method=get, path="/orders", type=json, res=Page<OrderPublic>)]
    async fn get_all_orders(
        State(state): State<AppState>,
        user: UserData,
        #[query] pagination: Pagination,
    ) -> impl IntoResponse {
        generic::get_all::<Order>(&state, pagination, user.business_id.into_context())
            .await
            .into_page()
            .into_json()
    }

    #[route(method=post, path="/orders/", type=json, res=OrderPublic)]
    async fn get_one_order(
        State(state): State<AppState>,
        user: UserData,
        #[json] body: OrderFetch,
    ) -> impl IntoResponse {
        generic::get_one::<Order>(&state, body.with_context(user.business_id))
            .await
            .into_body()
            .into_json()
    }

    #[route(method=post, path="/orders", type=json, res=OrderPublic)]
    async fn create_order(
        State(state): State<AppState>,
        user: UserData,
        #[json] body: OrderCreate,
    ) -> impl IntoResponse {
        generic::create::<Order>(&state, body.with_context(user.business_id))
            .await
            .into_body()
            .into_json()
    }

    #[route(method=patch, path="/orders/", type=json, res=OrderPublic)]
    async fn update_order(
        State(state): State<AppState>,
        user: UserData,
        #[json] body: OrderUpdate,
    ) -> impl IntoResponse {
        generic::update::<Order>(&state, body.with_context(user.business_id))
            .await
            .into_body()
            .into_json()
    }

    #[route(method=delete, path="/orders/", type=json, res=OrderPublic)]
    async fn delete_order(
        State(state): State<AppState>,
        user: UserData,
        #[json] body: OrderDelete,
    ) -> impl IntoResponse {
        generic::delete::<Order>(&state, body.with_context(user.business_id))
            .await
            .into_body()
            .into_json()
    }

    // ---- Stores ----
    #[route(method=get, path="/stores", type=json, res=Page<StorePublic>)]
    async fn get_all_stores(
        State(state): State<AppState>,
        user: UserData,
        #[query] pagination: Pagination,
    ) -> impl IntoResponse {
        generic::get_all::<Store>(&state, pagination, user.business_id.into_context())
            .await
            .into_page()
            .into_json()
    }

    #[route(method=post, path="/stores/", type=json, res=StorePublic)]
    async fn get_one_store(
        State(state): State<AppState>,
        user: UserData,
        #[json] body: StoreFetch,
    ) -> impl IntoResponse {
        let res = Store::get_one(&state.db, body)
            .await
            .map_err(|_| StatusCode::BAD_REQUEST)?
            .ok_or(StatusCode::NOT_FOUND)?;

        if res.business_id != user.business_id {
            Err(StatusCode::NOT_FOUND)
        } else {
            Ok(res.into()).into_json()
        }
    }

    #[route(method=post, path="/stores", type=json, res=StorePublic)]
    async fn create_store(
        State(state): State<AppState>,
        user: UserData,
        #[json] body: StoreCreate,
    ) -> impl IntoResponse {
        generic::create::<Store>(&state, body.with_context(user.business_id))
            .await
            .into_body()
            .into_json()
    }

    #[route(method=patch, path="/stores/", type=json, res=StorePublic)]
    async fn update_store(
        State(state): State<AppState>,
        user: UserData,
        #[json] body: StoreUpdate,
    ) -> impl IntoResponse {
        generic::update::<Store>(&state, body.with_context(user.business_id))
            .await
            .into_body()
            .into_json()
    }

    #[route(method=delete, path="/stores/", type=json, res=StorePublic)]
    async fn delete_store(
        State(state): State<AppState>,
        user: UserData,
        #[json] body: StoreDelete,
    ) -> impl IntoResponse {
        generic::delete::<Store>(&state, body.with_context(user.business_id))
            .await
            .into_body()
            .into_json()
    }
    // ---- Domains ----
    #[route(method=post, path="/stores/domains", type=json, res=Page<DomainPublic>)]
    async fn get_store_domains(
        State(state): State<AppState>,
        user: UserData,
        #[query] pagination: Pagination,
        #[json] body: DomainList,
    ) -> impl IntoResponse {
        generic::get_all::<Domain>(&state, pagination, body.with_context(user.business_id))
            .await
            .into_page()
            .into_json()
    }

    // ---- Settings ----
    #[route(method=get, path="/settings", type=json, res=SettingsPublic)]
    async fn get_settings(State(state): State<AppState>, user: UserData) -> impl IntoResponse {
        generic::get_one::<Settings>(&state, user.business_id.into_context())
            .await
            .into_body()
            .into_json()
    }
}
