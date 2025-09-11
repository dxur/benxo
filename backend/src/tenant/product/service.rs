use super::api::*;
use super::domain::*;
use super::repo::ProductRepo;
use crate::platform::business::api::BusinessSession;
use crate::tenant::product::domain::ProductVariant;
use crate::types::id::Id;
use crate::utils::error::{ApiError, ApiResult};

pub struct ProductService<R: ProductRepo> {
    repo: R,
}

impl<R: ProductRepo> ProductService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn create(
        &self,
        business: BusinessSession,
        create_req: ProductCreateDto,
    ) -> ApiResult<ProductDto> {
        self.repo
            .create(
                business.business_id.into_inner(),
                ProductRecord::new(
                    create_req.title,
                    create_req.description,
                    create_req.status.into(),
                    create_req.featured,
                    create_req.category,
                    create_req.images,
                    create_req.variants,
                    create_req.slug,
                ),
            )
            .await
            .map(Into::into)
    }

    pub async fn update_product(
        &self,
        business: BusinessSession,
        product_id: Id,
        update_req: ProductUpdate,
    ) -> ApiResult<ProductDto> {
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
        update_req.variants.map(|v| record.variants = v);
        update_req.slug.map(|v| record.slug = v);
        update_req.images.map(|v| record.images = v);
        update_req.featured.map(|v| record.featured = v);
        update_req.status.map(|v| record.status = v.into());

        self.repo
            .update(business_id, id, record)
            .await
            .map(Into::into)
    }

    pub async fn delete_product(&self, business: BusinessSession, product_id: Id) -> ApiResult<()> {
        self.repo
            .delete(business.business_id.into_inner(), product_id.into_inner())
            .await
    }

    pub async fn get_product(
        &self,
        business: BusinessSession,
        product_id: Id,
    ) -> ApiResult<ProductDto> {
        let id = product_id.into_inner();
        self.repo
            .find_by_id(business.business_id.into_inner(), id)
            .await?
            .ok_or(ApiError::not_found("product", id.to_hex()))
            .map(Into::into)
    }

    pub async fn pub_get_product(
        &self,
        business_id: Id,
        product_slug: &str,
    ) -> ApiResult<ProductDto> {
        self.repo
            .find_active_by_slug(business_id.into_inner(), product_slug)
            .await?
            .ok_or(ApiError::not_found("product", product_slug.to_string()))
            .map(Into::into)
    }

    pub async fn get_variant_by_sku(
        &self,
        business: BusinessSession,
        variant_sku: String,
    ) -> ApiResult<ProductVariant> {
        todo!()
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
            status: status.map(Into::into),
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

    pub async fn pub_list_products(
        &self,
        business_id: Id,
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
            status: ProductStatus::Active.into(),
            category,
            featured,
            search,
        };
        let page = page.unwrap_or(1);
        let limit = limit.unwrap_or(10);

        let (products, total) = self
            .repo
            .list(business_id.into_inner(), filter, page, limit)
            .await?;

        let views: Vec<_> = products.into_iter().map(Into::into).collect();
        Ok(ProductListResponse {
            products: views,
            total,
            page,
            limit,
        })
    }

    pub async fn pub_list_related_products(
        &self,
        business_id: Id,
        slug: &str,
    ) -> ApiResult<Vec<ProductDto>> {
        let product = self
            .repo
            .find_active_by_slug(business_id.into_inner(), slug)
            .await?
            .ok_or(ApiError::not_found("product", slug.to_string()))?;

        let category = match product.category.len() {
            0 => return Ok(vec![]),
            _ => product.category,
        };

        let filter = ProductFilter {
            status: ProductStatus::Active.into(),
            category: Some(category),
            featured: None,
            search: None,
        };

        let (products, _) = self
            .repo
            .list(business_id.into_inner(), filter, 1, 8)
            .await?;

        let views: Vec<_> = products
            .into_iter()
            .filter(|p| p._id != product._id)
            .map(Into::into)
            .collect();
        Ok(views)
    }
}
