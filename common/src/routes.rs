use macros::{route, routes};

use crate::models::product::*;
use crate::models::user::*;
use crate::models::{Page, Pagination};

pub struct ApiRoutes;

#[allow(async_fn_in_trait)]
#[routes(prefix = "/api", for = ApiRoutes)]
pub trait Routes {
    // ---- Products ----
    #[route(method=get, path = "/products")]
    async fn get_all_products(#[query] pagination: Pagination) -> Page<ProductModelPublic>;

    #[route(method=get, path = "/products/")]
    async fn get_one_product(#[body] body: ProductModelFetch) -> ProductModelPublic;

    #[route(method=post, path = "/products/")]
    async fn create_product(#[body] body: ProductModelCreate) -> ProductModelPublic;

    #[route(method=patch, path = "/products/")]
    async fn update_product(#[body] body: ProductModelUpdate) -> ProductModelPublic;

    #[route(method=delete, path = "/products/")]
    async fn delete_product(#[body] body: ProductModelDelete) -> ProductModelPublic;

    // ---- Product Variants ----
    #[route(method=get, path = "/products/variants")]
    async fn get_some_variants(
        #[query] pagination: Pagination,
        #[body] body: ProductVarModelFilter,
    ) -> Page<ProductVarModelPublic>;

    #[route(method=get, path = "/products/variants/")]
    async fn get_one_variant(#[body] body: ProductVarModelFetch) -> ProductVarModelPublic;

    #[route(method=post, path = "/products/variants/")]
    async fn create_variant(#[body] body: ProductVarModelCreate) -> ProductVarModelPublic;

    #[route(method=patch, path = "/products/variants/")]
    async fn update_variant(#[body] body: ProductVarModelUpdate) -> ProductVarModelPublic;

    #[route(method=delete, path = "/products/variants/")]
    async fn delete_variant(#[body] body: ProductVarModelDelete) -> ProductVarModelPublic;

    // ---- Users ----
    #[route(method=get, path = "/users")]
    async fn get_all_users(#[query] pagination: Pagination) -> Page<UserModelPublic>;

    #[route(method=get, path = "/users/")]
    async fn get_one_user(#[body] body: UserModelFetch) -> UserModelPublic;

    #[route(method=post, path = "/users/")]
    async fn create_user(#[body] body: UserModelCreate) -> UserModelPublic;

    #[route(method=patch, path = "/users/")]
    async fn update_user(#[body] body: UserModelUpdate) -> UserModelPublic;

    #[route(method=delete, path = "/users/")]
    async fn delete_user(#[body] body: UserModelDelete) -> UserModelPublic;
}
