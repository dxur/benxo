use macros::{route, routes};

use crate::models::order::*;
use crate::models::product::*;
use crate::models::user::*;
use crate::models::{Page, Pagination};

pub struct ApiRoutes;

#[allow(async_fn_in_trait)]
#[routes(prefix = "/api", for = ApiRoutes)]
pub trait Routes {
    // ---- Products ----
    #[route(method=get, path = "/products")]
    async fn get_all_products(#[query] pagination: Pagination) -> Page<ProductPublic>;

    #[route(method=post, path = "/products/")]
    async fn get_one_product(#[body] body: ProductFetch) -> ProductPublic;

    #[route(method=post, path = "/products")]
    async fn create_product(#[body] body: ProductCreate) -> ProductPublic;

    #[route(method=patch, path = "/products/")]
    async fn update_product(#[body] body: ProductUpdate) -> ProductPublic;

    #[route(method=delete, path = "/products/")]
    async fn delete_product(#[body] body: ProductDelete) -> ProductPublic;

    // ---- Product Variants ----
    #[route(method=get, path = "/products/variants")]
    async fn get_some_variants(
        #[query] pagination: Pagination,
        #[body] body: ProductVarFilter,
    ) -> Page<ProductVarPublic>;

    #[route(method=post, path = "/products/variants/")]
    async fn get_one_variant(#[body] body: ProductVarFetch) -> ProductVarPublic;

    #[route(method=post, path = "/products/variants")]
    async fn create_variant(#[body] body: ProductVarCreate) -> ProductVarPublic;

    #[route(method=patch, path = "/products/variants/")]
    async fn update_variant(#[body] body: ProductVarUpdate) -> ProductVarPublic;

    #[route(method=delete, path = "/products/variants/")]
    async fn delete_variant(#[body] body: ProductVarDelete) -> ProductVarPublic;

    // ---- Orders ----
    #[route(method=get, path = "/orders")]
    async fn get_all_orders(#[query] pagination: Pagination) -> Page<OrderPublic>;

    #[route(method=post, path = "/orders/")]
    async fn get_one_order(#[body] body: OrderFetch) -> OrderPublic;

    #[route(method=post, path = "/orders")]
    async fn create_order(#[body] body: OrderCreate) -> OrderPublic;

    #[route(method=patch, path = "/orders/")]
    async fn update_order(#[body] body: OrderUpdate) -> OrderPublic;

    #[route(method=delete, path = "/orders/")]
    async fn delete_order(#[body] body: OrderDelete) -> OrderPublic;

    // ---- Users ----
    #[route(method=get, path = "/users")]
    async fn get_all_users(#[query] pagination: Pagination) -> Page<UserPublic>;

    #[route(method=post, path = "/users/")]
    async fn get_one_user(#[body] body: UserFetch) -> UserPublic;

    #[route(method=post, path = "/users")]
    async fn create_user(#[body] body: UserCreate) -> UserPublic;

    #[route(method=patch, path = "/users/")]
    async fn update_user(#[body] body: UserUpdate) -> UserPublic;

    #[route(method=delete, path = "/users/")]
    async fn delete_user(#[body] body: UserDelete) -> UserPublic;
}
