use super::api::*;
use super::domain::*;
use super::repo::{StoreRegRepo, StoreRepo};
use crate::platform::business::api::BusinessSession;
use crate::types::id::Id;
use crate::utils::error::{ApiError, ApiResult};
use crate::utils::serde_helpers::JsonOption;

pub struct StoreService<R: StoreRepo, Reg: StoreRegRepo> {
    repo: R,
    reg: Reg,
}

impl<R: StoreRepo, Reg: StoreRegRepo> StoreService<R, Reg> {
    pub fn new(repo: R, reg: Reg) -> Self {
        Self { repo, reg }
    }

    pub async fn create(&self, business: BusinessSession) -> ApiResult<StoreDto> {
        self.repo
            .create(business.business_id.into_inner(), Default::default())
            .await
            .map(Into::into)
    }

    pub async fn update_store(
        &self,
        business: BusinessSession,
        store_id: Id,
        update_req: StoreUpdate,
    ) -> ApiResult<StoreDto> {
        let id = store_id.into_inner();
        let business_id = business.business_id.into_inner();
        let mut record = self
            .repo
            .find_by_id(business_id, id)
            .await?
            .ok_or(ApiError::not_found("store", id.to_hex()))?;

        update_req.name.map(|v| record.name = v);
        update_req.description.map(|v| record.description = v);
        update_req.status.map(|v| record.status = v.into());

        self.repo
            .update(business_id, id, record)
            .await
            .map(Into::into)
    }

    pub async fn delete_store(&self, business: BusinessSession, store_id: Id) -> ApiResult<()> {
        self.repo
            .delete(business.business_id.into_inner(), store_id.into_inner())
            .await
    }

    pub async fn get_store(&self, business: BusinessSession, store_id: Id) -> ApiResult<StoreDto> {
        let id = store_id.into_inner();
        self.repo
            .find_by_id(business.business_id.into_inner(), id)
            .await?
            .ok_or(ApiError::not_found("store", id.to_hex()))
            .map(Into::into)
    }

    pub async fn get_slug(&self, slug: &str) -> ApiResult<StoreRegDto> {
        self.reg
            .find_by_slug(slug)
            .await?
            .ok_or(ApiError::not_found("store", slug.to_owned()))
            .map(Into::into)
    }

    pub async fn get_domain(&self, domain: &str) -> ApiResult<StoreRegDto> {
        self.reg
            .find_by_domain(domain)
            .await?
            .ok_or(ApiError::not_found("store", domain.to_owned()))
            .map(Into::into)
    }

    pub async fn set_reg(
        &self,
        business: BusinessSession,
        store_id: Id,
        update_req: StoreRegUpdate,
    ) -> ApiResult<StoreRegDto> {
        let business_id = business.business_id.into_inner();
        let store_id = store_id.into_inner();

        if let Some(this) = self.repo.find_by_id(business_id, store_id).await? {
            if let JsonOption::Value(ref domain) = update_req.domain {
                if let Some(mut other) = self.reg.find_by_domain(&domain).await? {
                    if other.store_id != store_id {
                        // TODO: confirms that the domain is owned by this user otherwise
                        // throw an error
                        other.domain = None;
                        self.reg
                            .update(other.business_id, other.store_id, other)
                            .await?;
                    }
                }
            }

            if let Some(mut store_reg) = self.reg.find_by_store(business_id, this._id).await? {
                store_reg.slug = update_req.slug;
                update_req
                    .domain
                    .ok_then(|domain| store_reg.domain = domain);
                self.reg
                    .update(store_reg.business_id, store_reg.store_id, store_reg)
                    .await
                    .map(Into::into)
            } else {
                self.reg
                    .create(StoreRegRecord::new(
                        business_id,
                        this._id,
                        update_req.slug,
                        update_req.domain.to_option(),
                    ))
                    .await
                    .map(Into::into)
            }
        } else {
            Err(ApiError::not_found("store", store_id.to_hex()))
        }
    }

    pub async fn list_stores(
        &self,
        business: BusinessSession,
        query: StoreListQuery,
    ) -> ApiResult<StoreListResponse> {
        let StoreListQuery {
            page,
            limit,
            status,
            search,
        } = query;

        let filter = StoreFilter {
            status: status.map(Into::into),
            search,
        };
        let page = page.unwrap_or(1);
        let limit = limit.unwrap_or(10);

        let (stores, total) = self
            .repo
            .list(business.business_id.into_inner(), filter, page, limit)
            .await?;

        let views: Vec<_> = stores.into_iter().map(Into::into).collect();
        Ok(StoreListResponse {
            stores: views,
            total,
            page,
            limit,
        })
    }
}
