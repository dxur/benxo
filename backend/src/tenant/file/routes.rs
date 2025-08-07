use axum::extract::{Path, Query, State};
use macros::routes;

use crate::extractors::cookies::FromCookies;
use crate::extractors::json::Json;
use crate::platform::business::api::BusinessSession;
use crate::platform::user::api::MessageResponse;
use crate::types::id::Id;
use crate::utils::error::ApiResult;
use crate::AppState;

use super::api::*;

pub struct FileRoutes;

#[routes(prefix = "/api/v1/files", state = AppState)]
impl FileRoutes {
    #[route(method=post, path="/create", res=FileDto)]
    async fn create_file(
        State(state): State<AppState>,
        FromCookies(business): FromCookies<BusinessSession>,
        #[json] create_req: FileCreate,
    ) -> ApiResult<Json<FileDto>> {
        state
            .file_service
            .create_file(business, create_req)
            .await
            .map(Json)
    }

    #[route(method=post, path="/{file_id}", res=FileDto)]
    async fn get_file(
        State(state): State<AppState>,
        FromCookies(business): FromCookies<BusinessSession>,
        #[path] file_id: Id,
    ) -> ApiResult<Json<FileDto>> {
        state
            .file_service
            .get_file(business, file_id)
            .await
            .map(Json)
    }

    #[route(method=post, path="/key/{key}", res=FileDto)]
    async fn get_file_by_key(
        State(state): State<AppState>,
        FromCookies(business): FromCookies<BusinessSession>,
        #[path] key: String,
    ) -> ApiResult<Json<FileDto>> {
        state
            .file_service
            .get_file_by_key(business, key)
            .await
            .map(Json)
    }

    #[route(method=post, path="/list", res=FileListResponse)]
    async fn list_files(
        State(state): State<AppState>,
        FromCookies(business): FromCookies<BusinessSession>,
        Query(query): Query<FileListQuery>,
    ) -> ApiResult<Json<FileListResponse>> {
        state
            .file_service
            .list_files(business, query)
            .await
            .map(Json)
    }

    #[route(method=delete, path="/{file_id}", res=MessageResponse)]
    async fn delete_file(
        State(state): State<AppState>,
        FromCookies(business): FromCookies<BusinessSession>,
        #[path] file_id: Id,
    ) -> ApiResult<Json<MessageResponse>> {
        state
            .file_service
            .delete_file(business, file_id)
            .await
            .map(|_| MessageResponse {
                message: "File deleted successfully".to_string(),
            })
            .map(Json)
    }

    #[route(method=post, path="/upload/{file_id}", res=PresignedUrlResponse)]
    async fn generate_upload_url(
        State(state): State<AppState>,
        FromCookies(business): FromCookies<BusinessSession>,
        #[path] file_id: Id,
    ) -> ApiResult<Json<PresignedUrlResponse>> {
        state
            .file_service
            .generate_file_upload_url(business, file_id)
            .await
            .map(Json)
    }
}
