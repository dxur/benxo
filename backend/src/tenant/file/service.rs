use chrono::{DateTime, Duration, Utc};
use s3::bucket::Bucket;
use s3::post_policy::*;
use std::borrow::Cow;

use super::api::*;
use super::domain::*;
use super::repo::FileRepo;
use crate::platform::business::api::BusinessSession;
use crate::types::id::Id;
use crate::utils::error::{ApiError, ApiResult};

pub struct FileService<R: FileRepo> {
    repo: R,
    bucket: Bucket,
}

impl<R: FileRepo> FileService<R> {
    pub fn new(repo: R, bucket: Bucket) -> Self {
        Self { repo, bucket }
    }

    pub fn get_prefix(business_id: Id) -> String {
        format!("biz/{}/files/", business_id.to_string())
    }

    pub fn get_full_key(business_id: Id, file_id: Id, file_name: &str) -> String {
        format!(
            "biz/{}/files/{}/{}",
            business_id.to_string(),
            file_id.to_string(),
            file_name
        )
    }

    pub async fn create_file(
        &self,
        business: BusinessSession,
        create_req: FileCreate,
    ) -> ApiResult<FileDto> {
        let mut record = FileRecord {
            key: sanitize_filename::sanitize(create_req.key),
            name: create_req.name,
            size: create_req.size,
            ..Default::default()
        };

        if let Some(mime_type) = create_req.mime_type {
            record.mime_type = mime_type.into();
        }

        self.repo
            .create(business.business_id.into_inner(), record)
            .await
            .map(Into::into)
    }

    pub async fn get_file(&self, business: BusinessSession, file_id: Id) -> ApiResult<FileDto> {
        let id = file_id.into_inner();
        self.repo
            .find_by_id(business.business_id.into_inner(), id)
            .await?
            .ok_or(ApiError::not_found("file", id.to_hex()))
            .map(Into::into)
    }

    pub async fn get_file_by_key(
        &self,
        business: BusinessSession,
        key: String,
    ) -> ApiResult<FileDto> {
        self.repo
            .find_by_key(business.business_id.into_inner(), &key)
            .await?
            .ok_or(ApiError::not_found("file", key))
            .map(Into::into)
    }

    pub async fn delete_file(&self, business: BusinessSession, file_id: Id) -> ApiResult<()> {
        let id = file_id.into_inner();
        let business_id = business.business_id.into_inner();

        let file_record = self
            .repo
            .find_by_id(business_id, id)
            .await?
            .ok_or(ApiError::not_found("file", id.to_hex()))?;

        self.bucket
            .delete_object(Self::get_full_key(
                business.business_id,
                file_id,
                &file_record.key,
            ))
            .await
            .map_err(|_| ApiError::internal("Failed to delete file from storage"))?;

        self.repo.delete(business_id, id).await
    }

    pub async fn list_files(
        &self,
        business: BusinessSession,
        query: FileListQuery,
    ) -> ApiResult<FileListResponse> {
        let FileListQuery {
            page,
            limit,
            mime_type,
            search,
        } = query;

        let filter = FileFilter { mime_type, search };
        let page = page.unwrap_or(1);
        let limit = limit.unwrap_or(10);

        let (files, total) = self
            .repo
            .list(business.business_id.into_inner(), filter, page, limit)
            .await?;

        let views: Vec<_> = files.into_iter().map(Into::into).collect();
        Ok(FileListResponse {
            files: views,
            total,
            page,
            limit,
        })
    }

    pub async fn generate_file_upload_url(
        &self,
        business: BusinessSession,
        file_id: Id,
    ) -> ApiResult<PresignedUrlResponse> {
        let id = file_id.into_inner();
        let file_record = self
            .repo
            .find_by_id(business.business_id.into_inner(), id)
            .await?
            .ok_or(ApiError::not_found("file", id.to_hex()))?;

        let key = Self::get_full_key(business.business_id, file_id, &file_record.key);

        let policy = PostPolicy::new(600)
            .condition(PostPolicyField::Key, PostPolicyValue::Exact(Cow::from(key)))
            .map_err(|_| ApiError::internal("Failed to apply key condition to policy"))?
            .condition(
                PostPolicyField::ContentLengthRange,
                PostPolicyValue::Range(4, 10 * 1024 * 1024),
            )
            .map_err(|_| {
                ApiError::internal("Failed to apply content length condition to policy")
            })?;

        let res = self
            .bucket
            .presign_post(policy)
            .await
            .map_err(|_| ApiError::internal("Failed to generate presigned URL"))?;

        Ok(PresignedUrlResponse {
            url: res.url,
            fields: res.fields,
            dynamic_fields: res.dynamic_fields,
            expiration: DateTime::from_timestamp(res.expiration.unix_timestamp(), 0).ok_or(
                ApiError::internal("Failed to convert expiration to DateTime"),
            )?,
        })
    }

    pub async fn generate_file_download_url(
        &self,
        business: BusinessSession,
        file_id: Id,
    ) -> ApiResult<PresignedUrlResponse> {
        let id = file_id.into_inner();
        let file_record = self
            .repo
            .find_by_id(business.business_id.into_inner(), id)
            .await?
            .ok_or(ApiError::not_found("file", id.to_hex()))?;

        let ttl = 600; // TODO: use env variable

        let res = self
            .bucket
            .presign_get(
                Self::get_full_key(business.business_id, file_id, &file_record.key),
                ttl,
                None,
            )
            .await
            .map_err(|_| ApiError::internal("Failed to generate presigned URL"))?;

        Ok(PresignedUrlResponse {
            url: res,
            fields: Default::default(),
            dynamic_fields: Default::default(),
            expiration: Utc::now() + Duration::seconds(ttl.into()),
        })
    }
}
