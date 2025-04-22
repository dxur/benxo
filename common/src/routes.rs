use macros::{route, routes};

use crate::models::product::*;
use crate::models::{Page, Pagination};

pub struct ApiRoutes;

#[allow(async_fn_in_trait)]
#[routes(prefix = "/api", for = ApiRoutes)]
pub trait Routes {
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
}
