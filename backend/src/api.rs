use macros::{route, routes};

use crate::models::order::*;
use crate::models::product::*;
use crate::models::settings::SettingsPublic;
use crate::models::user::*;
use crate::models::{Page, Pagination};

pub struct ApiRoutes;

#[allow(async_fn_in_trait)]
#[routes(prefix = "/api", for = ApiRoutes)]
pub trait Routes {
    // ---- Products ----
    #[route(method=get, path = "/products")]
    async fn get_all_products(
        #[ignore] store: crate::extractors::StoreId,
        #[query] pagination: Pagination,
    ) -> Page<ProductPublic>;

    #[route(method=post, path = "/products/")]
    async fn get_one_product(
        #[ignore] store: crate::extractors::StoreId,
        #[body] body: ProductFetch,
    ) -> ProductPublic;

    #[route(method=post, path = "/products")]
    async fn create_product(
        #[ignore] store: crate::extractors::StoreId,
        #[body] body: ProductCreate,
    ) -> ProductPublic;

    #[route(method=patch, path = "/products/")]
    async fn update_product(
        #[ignore] store: crate::extractors::StoreId,
        #[body] body: ProductUpdate,
    ) -> ProductPublic;

    #[route(method=delete, path = "/products/")]
    async fn delete_product(
        #[ignore] store: crate::extractors::StoreId,
        #[body] body: ProductDelete,
    ) -> ProductPublic;

    // ---- Orders ----
    #[route(method=get, path = "/orders")]
    async fn get_all_orders(
        #[ignore] store: crate::extractors::StoreId,
        #[query] pagination: Pagination,
    ) -> Page<OrderPublic>;

    #[route(method=post, path = "/orders/")]
    async fn get_one_order(
        #[ignore] store: crate::extractors::StoreId,
        #[body] body: OrderFetch,
    ) -> OrderPublic;

    #[route(method=post, path = "/orders")]
    async fn create_order(
        #[ignore] store: crate::extractors::StoreId,
        #[body] body: OrderCreate,
    ) -> OrderPublic;

    #[route(method=patch, path = "/orders/")]
    async fn update_order(
        #[ignore] store: crate::extractors::StoreId,
        #[body] body: OrderUpdate,
    ) -> OrderPublic;

    #[route(method=delete, path = "/orders/")]
    async fn delete_order(
        #[ignore] store: crate::extractors::StoreId,
        #[body] body: OrderDelete,
    ) -> OrderPublic;

    // settings
    #[route(method=get, path = "/settings")]
    async fn get_settings(#[ignore] store: crate::extractors::StoreId) -> SettingsPublic;

    // // ---- Users ----
    // #[route(method=get, path = "/users")]
    // async fn get_all_users(
    //     #[ignore] store: crate::extractors::StoreId,
    //     #[query] pagination: Pagination,
    // ) -> Page<UserPublic>;

    // #[route(method=post, path = "/users/")]
    // async fn get_one_user(
    //     #[ignore] store: crate::extractors::StoreId,
    //     #[body] body: UserFetch,
    // ) -> UserPublic;

    // #[route(method=post, path = "/users")]
    // async fn create_user(
    //     #[ignore] store: crate::extractors::StoreId,
    //     #[body] body: UserCreate,
    // ) -> UserPublic;

    // #[route(method=patch, path = "/users/")]
    // async fn update_user(
    //     #[ignore] store: crate::extractors::StoreId,
    //     #[body] body: UserUpdate,
    // ) -> UserPublic;

    // #[route(method=delete, path = "/users/")]
    // async fn delete_user(
    //     #[ignore] store: crate::extractors::StoreId,
    //     #[body] body: UserDelete,
    // ) -> UserPublic;
}
