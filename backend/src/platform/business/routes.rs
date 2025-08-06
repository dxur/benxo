use axum::extract::Query;
use axum::extract::State;
use macros::routes;
use tower_cookies::Cookies;

use crate::extractors::cookies::FromCookies;
use crate::extractors::json::Json;
use crate::platform::business::domain::BusinessSettings;
use crate::platform::user::api::{MessageResponse, UserSession};
use crate::utils::error::ApiResult;
use crate::AppState;

use super::api::*;

pub struct BusinessRoutes;

#[routes(prefix = "/api/v1/businesses", state = AppState)]
impl BusinessRoutes {
    /// Create a new business
    #[route(method = post, path = "/create", res = BusinessDto)]
    async fn create(
        State(state): State<AppState>,
        FromCookies(user_token): FromCookies<UserSession>,
        #[json] create_req: BusinessCreate,
    ) -> ApiResult<Json<BusinessDto>> {
        state
            .business_service
            .create(user_token, create_req)
            .await
            .map(Json)
    }

    /// List businesses for the current user
    #[route(method = post, path = "/list", res = BusinessListResponse)]
    async fn list(
        State(state): State<AppState>,
        FromCookies(user_token): FromCookies<UserSession>,
        Query(query): Query<BusinessListQuery>,
    ) -> ApiResult<Json<BusinessListResponse>> {
        let (businesses, total) = state
            .business_service
            .list_user_businesses(user_token, query.clone())
            .await?;

        Ok(Json(BusinessListResponse {
            businesses,
            total,
            page: query.page.unwrap_or(1),
            limit: query.limit.unwrap_or(10),
        }))
    }

    /// Switch to a different business context
    #[route(method = post, path = "/switch", res = MessageResponse)]
    async fn switch_business(
        State(state): State<AppState>,
        FromCookies(user_token): FromCookies<UserSession>,
        cookies: Cookies,
        #[json] switch_req: BusinessSwitch,
    ) -> ApiResult<Json<MessageResponse>> {
        cookies.add(
            state
                .business_service
                .switch_business(user_token, switch_req)
                .await?
                .try_into()?,
        );

        Ok(Json(MessageResponse {
            message: "You have successfully switch business".to_string(),
        }))
    }

    /// Get current business details
    #[route(method = post, path = "/current", res = BusinessDto)]
    async fn current(
        State(state): State<AppState>,
        FromCookies(business_token): FromCookies<BusinessSession>,
    ) -> ApiResult<Json<BusinessDto>> {
        state
            .business_service
            .get(business_token.business_id)
            .await
            .map(Json)
    }

    /// Get business statistics
    #[route(method = post, path = "/statistics", res = BusinessStatistics)]
    async fn statistics(
        State(state): State<AppState>,
        FromCookies(business_token): FromCookies<BusinessSession>,
        FromCookies(user_token): FromCookies<UserSession>,
    ) -> ApiResult<Json<BusinessStatistics>> {
        state
            .business_service
            .get_business_statistics(business_token, user_token)
            .await
            .map(Json)
    }

    /// Update business settings
    #[route(method = put, path = "/settings", res = BusinessDto)]
    async fn update_settings(
        State(state): State<AppState>,
        FromCookies(business_token): FromCookies<BusinessSession>,
        FromCookies(user_token): FromCookies<UserSession>,
        #[json] settings: BusinessSettings,
    ) -> ApiResult<Json<BusinessDto>> {
        state
            .business_service
            .update_business_settings(business_token, user_token, settings)
            .await
            .map(Json)
    }

    /// Create an invitation to join the business
    #[route(method = post, path = "/invitations", res = InvitationDto)]
    async fn create_invitation(
        State(state): State<AppState>,
        FromCookies(business_token): FromCookies<BusinessSession>,
        FromCookies(user_token): FromCookies<UserSession>,
        #[json] invitation: InvitationCreate,
    ) -> ApiResult<Json<InvitationDto>> {
        state
            .business_service
            .invite_member(business_token, user_token, invitation)
            .await
            .map(Json)
    }

    /// Accept an invitation to join a business
    #[route(method = post, path = "/invitations/accept", res = BusinessDto)]
    async fn accept_invitation(
        State(state): State<AppState>,
        FromCookies(user_token): FromCookies<UserSession>,
        #[json] accept_req: InvitationAccept,
    ) -> ApiResult<Json<BusinessDto>> {
        state
            .business_service
            .accept_invitation(user_token, accept_req)
            .await
            .map(Json)
    }

    /// Resend an invitation
    #[route(method = post, path = "/invitations/resend", res = InvitationDto)]
    async fn resend_invitation(
        State(state): State<AppState>,
        FromCookies(business_token): FromCookies<BusinessSession>,
        FromCookies(user_token): FromCookies<UserSession>,
        #[json] resend_req: InvitationResend,
    ) -> ApiResult<Json<InvitationDto>> {
        state
            .business_service
            .resend_invitation(business_token, user_token, resend_req.email.to_string())
            .await
            .map(Json)
    }

    /// Get pending invitations for the current user's email
    #[route(method = post, path = "/invitations/pending", res = PendingInvitationsResponse)]
    async fn get_pending_invitations(
        State(state): State<AppState>,
        FromCookies(user_token): FromCookies<UserSession>,
    ) -> ApiResult<Json<PendingInvitationsResponse>> {
        let invitations = state
            .business_service
            .get_pending_invitations_by_email(user_token.email.to_string())
            .await?;

        Ok(Json(PendingInvitationsResponse { invitations }))
    }

    /// Update a member's role and permissions
    #[route(method = put, path = "/members/edit", res = BusinessMemberDto)]
    async fn update_member(
        State(state): State<AppState>,
        FromCookies(business_token): FromCookies<BusinessSession>,
        FromCookies(user_token): FromCookies<UserSession>,
        #[json] update_req: MemberUpdate,
    ) -> ApiResult<Json<BusinessMemberDto>> {
        state
            .business_service
            .update_member(business_token, user_token, update_req)
            .await
            .map(Json)
    }

    /// Remove a member from the business
    #[route(method = delete, path = "/members", res = MessageResponse)]
    async fn remove_member(
        State(state): State<AppState>,
        FromCookies(business_token): FromCookies<BusinessSession>,
        FromCookies(user_token): FromCookies<UserSession>,
        #[json] remove_req: MemberRemove,
    ) -> ApiResult<Json<MessageResponse>> {
        state
            .business_service
            .remove_member(business_token, user_token, remove_req.email.to_string())
            .await?;

        Ok(Json(MessageResponse {
            message: "Member removed successfully".to_string(),
        }))
    }

    /// Check if current user has specific permission
    #[route(method = post, path = "/permissions/check", res = PermissionCheckResponse)]
    async fn check_permission(
        State(state): State<AppState>,
        FromCookies(business_token): FromCookies<BusinessSession>,
        FromCookies(user_token): FromCookies<UserSession>,
        #[json] permission_check: PermissionCheck,
    ) -> ApiResult<Json<PermissionCheckResponse>> {
        state
            .business_service
            .check_permission(
                business_token,
                user_token,
                permission_check.resource,
                permission_check.action,
                permission_check.target,
            )
            .await
            .map(|v| Json(PermissionCheckResponse { has_permission: v }))
    }
}
