use bson::oid::ObjectId;
use hickory_resolver::name_server::TokioConnectionProvider;
use hickory_resolver::proto::rr::RData;
use hickory_resolver::Resolver;
use std::time::Instant;

use super::api::*;
use super::domain::*;
use super::repo::DomainRepo;
use crate::platform::business::api::BusinessSession;
use crate::platform::user::api::UserSession;
use crate::utils::error::{ApiError, ApiResult};

pub struct DnsService<D: DomainRepo> {
    domain_repo: D,
    resolver: Resolver<TokioConnectionProvider>,
}

impl<D: DomainRepo> DnsService<D> {
    pub fn new(domain_repo: D, resolver: Resolver<TokioConnectionProvider>) -> Self {
        Self {
            domain_repo,
            resolver,
        }
    }

    pub async fn resolve_domain(&self, domain: &str) -> ApiResult<Option<ObjectId>> {
        let domain = domain.trim().to_lowercase();
        if domain.is_empty() {
            return Ok(None);
        }

        let cached = self.domain_repo.find_by_domain(&domain).await?;

        if let Some(record) = &cached {
            if !record.is_expired() {
                return Ok(Some(record.business_id));
            }
        }

        let Some((business_id, ttl)) = self.verify_domain(&domain).await? else {
            return Ok(None);
        };

        match cached {
            Some(mut record) => {
                record.business_id = business_id;
                record.refresh(ttl);
                self.domain_repo.update(record).await?;
            }
            None => {
                let record = DomainRecord::new(domain, business_id, ttl);
                self.domain_repo.create(record).await?;
            }
        }

        Ok(Some(business_id))
    }

    pub async fn get_business_domains(
        &self,
        business_session: BusinessSession,
        _user: UserSession,
    ) -> ApiResult<DomainListResponse> {
        let domains = self
            .domain_repo
            .find_by_business_id(business_session.business_id.into_inner())
            .await?;

        let domain_dtos: Vec<DomainDto> = domains.into_iter().map(DomainDto::from).collect();

        Ok(DomainListResponse {
            domains: domain_dtos,
        })
    }

    pub async fn cleanup_expired(&self) -> ApiResult<u64> {
        self.domain_repo.delete_expired().await
    }

    async fn verify_domain(&self, domain: &str) -> ApiResult<Option<(ObjectId, u32)>> {
        let res = self.resolver.txt_lookup(domain).await.map_err(|e| {
            ApiError::internal(format!("Failed to resolve domain {}: {}", domain, e))
        })?;

        let ttl = res
            .valid_until()
            .checked_duration_since(Instant::now())
            .map(|d| d.as_secs() as u32)
            .unwrap_or(0);

        for record in res {
            let txt_value: String = record.to_string();

            if let Some(rest) = txt_value.strip_prefix("business_id=") {
                match ObjectId::parse_str(rest.trim()) {
                    Ok(oid) => return Ok(Some((oid, ttl))),
                    Err(_) => {
                        return Err(ApiError::malformed(format!(
                            "Invalid ObjectId in TXT record: {}",
                            rest
                        )))
                    }
                }
            }
        }

        Ok(None)
    }
}
