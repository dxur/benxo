use super::api::{ProductListQuery, ProductListResponse, ProductUpdate, ProductView};
use super::domain::ProductFilter;
use super::repo::ProductRepo;
use crate::platform::business::api::BusinessSession;
use crate::types::id::Id;
use crate::utils::error::{ApiError, ApiResult};

pub struct ProductService<R: ProductRepo> {
    repo: R,
}

impl<R: ProductRepo> ProductService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn create(&self, business: BusinessSession) -> ApiResult<ProductView> {
        self.repo
            .create(business.business_id.into_inner(), Default::default())
            .await
            .map(Into::into)
    }

    pub async fn update_product(
        &self,
        business: BusinessSession,
        product_id: Id,
        update_req: ProductUpdate,
    ) -> ApiResult<ProductView> {
        let id = product_id.into_inner();
        let business_id = business.business_id.into_inner();
        let mut record = self
            .repo
            .find_by_id(business_id, id)
            .await?
            .ok_or(ApiError::not_found("product", id.to_hex()))?;

        update_req.title.map(|v| record.title = v);
        update_req.description.map(|v| record.description = v);
        update_req.category.map(|v| record.category = v);
        update_req.options.map(|v| record.options = v);
        update_req.variants.map(|v| record.variants = v);
        update_req.slug.map(|v| record.slug = v);
        update_req.images.map(|v| record.images = v);
        update_req.featured.map(|v| record.featured = v);
        update_req.status.map(|v| record.status = v);

        self.repo
            .update(business_id, id, record)
            .await
            .map(Into::into)
    }

    pub async fn get_product(
        &self,
        business: BusinessSession,
        product_id: Id,
    ) -> ApiResult<ProductView> {
        let id = product_id.into_inner();
        self.repo
            .find_by_id(business.business_id.into_inner(), id)
            .await?
            .ok_or(ApiError::not_found("product", id.to_hex()))
            .map(Into::into)
    }

    pub async fn list_products(
        &self,
        business: BusinessSession,
        query: ProductListQuery,
    ) -> ApiResult<ProductListResponse> {
        let ProductListQuery {
            page,
            limit,
            status,
            category,
            featured,
            search,
        } = query;

        let filter = ProductFilter {
            status,
            category,
            featured,
            search,
        };
        let page = page.unwrap_or(1);
        let limit = limit.unwrap_or(10);

        let (products, total) = self
            .repo
            .list(business.business_id.into_inner(), filter, page, limit)
            .await?;

        let views: Vec<_> = products.into_iter().map(Into::into).collect();
        Ok(ProductListResponse {
            products: views,
            total,
            page,
            limit,
        })
    }
}
