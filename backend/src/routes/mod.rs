pub mod auth;
pub mod generic;

use axum::extract::Query;
use axum::extract::State;
use axum::Json;
use bcrypt::{hash, verify, DEFAULT_COST};
use hyper::StatusCode;
use macros::{route, routes};
use tower_cookies::Cookie;
use tower_cookies::Cookies;

use crate::db::CreatableInDb;
use crate::db::FetchableInDb;
use crate::extractors::StoreId;
use crate::extractors::UserData;
use crate::models::auth::LoginCredentials;
use crate::models::auth::RegisterCredentials;
use crate::models::order::*;
use crate::models::product::*;
use crate::models::settings::*;
use crate::models::user::*;
use crate::models::*;
use crate::utils::auth::issue_access_tokens;
use crate::utils::auth::issue_refresh_tokens;
use crate::utils::types::IntoContext;
use crate::utils::types::WithContext;
use crate::AppState;

pub struct ApiRoutes;

#[routes(prefix = "/api", state = "AppState")]
impl ApiRoutes {
    // ---- Auth ----
    #[route(method=post, path = "/register")]
    async fn register(state: State<AppState>, #[json] credentrals: RegisterCredentials) {
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
                store_id: "some".to_string(),
                email,
                password: hash,
            }
            .into(),
        )
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
        Ok(Json(()))
    }

    #[route(method=post, path = "/login")]
    async fn login(
        state: State<AppState>,
        cookies: Cookies,
        #[json] credentrals: LoginCredentials,
    ) -> UserPublic {
        let LoginCredentials { email, password } = credentrals;

        let mut user = User::get_one(&state.db, UserFetch { email })
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::UNAUTHORIZED)?;

        if !verify(&password, &user.password).unwrap() {
            return Err(StatusCode::UNAUTHORIZED);
        }

        let data = UserData {
            store_id: user.store_id,
            email: user.email,
        };

        let access_token = issue_access_tokens(&data).ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
        let refresh_token = issue_refresh_tokens(&data).ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

        cookies.add(Cookie::build(("access_token", access_token)).build());
        cookies.add(Cookie::build(("refresh_token", refresh_token)).build());

        user.store_id = data.store_id;
        user.email = data.email;
        Ok(Json(UserPublic::from(user.into())))
    }

    #[route(method=get, path = "/logout")]
    async fn logout(cookies: Cookies) {
        cookies.remove(Cookie::build("access_token").build());
        cookies.remove(Cookie::build("refresh_token").build());
        Ok(Default::default())
    }

    // ---- Products ----
    #[route(method=get, path = "/products")]
    async fn get_all_products(
        state: State<AppState>,
        StoreId(store_id): StoreId,
        #[query] pagination: Pagination,
    ) -> Page<ProductPublic> {
        generic::get_all::<Product>(state, Query(pagination), Json(store_id.into_context())).await
    }

    #[route(method=post, path = "/products/")]
    async fn get_one_product(
        state: State<AppState>,
        StoreId(store_id): StoreId,
        #[json] body: ProductFetch,
    ) -> ProductPublic {
        generic::get_one::<Product>(state, Json(body.with_context(store_id))).await
    }

    #[route(method=post, path = "/products")]
    async fn create_product(
        state: State<AppState>,
        user: UserData,
        #[json] body: ProductCreate,
    ) -> ProductPublic {
        generic::create::<Product>(state, Json(body.with_context(user.store_id))).await
    }

    #[route(method=patch, path = "/products/")]
    async fn update_product(
        state: State<AppState>,
        user: UserData,
        #[json] body: ProductUpdate,
    ) -> ProductPublic {
        generic::update::<Product>(state, Json(body.with_context(user.store_id))).await
    }

    #[route(method=delete, path = "/products/")]
    async fn delete_product(
        state: State<AppState>,
        user: UserData,
        #[json] body: ProductDelete,
    ) -> ProductPublic {
        generic::delete::<Product>(state, Json(body.with_context(user.store_id))).await
    }

    // ---- Orders ----
    #[route(method=get, path = "/orders")]
    async fn get_all_orders(
        state: State<AppState>,
        user: UserData,
        #[query] pagination: Pagination,
    ) -> Page<OrderPublic> {
        generic::get_all::<Order>(state, Query(pagination), Json(user.store_id.into_context()))
            .await
    }

    #[route(method=post, path = "/orders/")]
    async fn get_one_order(
        state: State<AppState>,
        user: UserData,
        #[json] body: OrderFetch,
    ) -> OrderPublic {
        generic::get_one::<Order>(state, Json(body.with_context(user.store_id))).await
    }

    #[route(method=post, path = "/orders")]
    async fn create_order(
        state: State<AppState>,
        user: UserData,
        #[json] body: OrderCreate,
    ) -> OrderPublic {
        generic::create::<Order>(state, Json(body.with_context(user.store_id))).await
    }

    #[route(method=patch, path = "/orders/")]
    async fn update_order(
        state: State<AppState>,
        user: UserData,
        #[json] body: OrderUpdate,
    ) -> OrderPublic {
        generic::update::<Order>(state, Json(body.with_context(user.store_id))).await
    }

    #[route(method=delete, path = "/orders/")]
    async fn delete_order(
        state: State<AppState>,
        user: UserData,
        #[json] body: OrderDelete,
    ) -> OrderPublic {
        generic::delete::<Order>(state, Json(body.with_context(user.store_id))).await
    }

    // ---- Settings ----
    #[route(method=get, path = "/settings")]
    async fn get_settings(state: State<AppState>, user: UserData) -> SettingsPublic {
        generic::get_one::<Settings>(state, Json(user.store_id.into_context())).await
    }
}
