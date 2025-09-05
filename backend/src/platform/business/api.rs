use chrono::{DateTime, Duration, Utc};
use o2o::o2o;
use serde::{Deserialize, Serialize};
use tower_cookies::{Cookie, Cookies};
use ts_rs::TS;

use super::domain::*;
use crate::{
    types::{email::Email, id::Id, name::Name},
    utils::{
        error::ApiError,
        jwt::{decode_jwt, encode_jwt},
    },
};

#[derive(Debug, Deserialize, TS)]
#[ts(export, bound = "")]
pub struct BusinessCreate {
    pub name: Name,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, o2o, TS)]
#[ts(export, bound = "")]
#[from_owned(BusinessRecord)]
pub struct BusinessDto {
    #[from(@._id.into())]
    pub id: Id,
    pub name: Name,
    pub description: Option<String>,
    #[from(~.into())]
    pub owner_id: Id,
    pub plan_type: PlanType,
    #[from(~.to_chrono())]
    pub plan_expires_at: DateTime<Utc>,
    pub status: BusinessStatus,
    pub settings: BusinessSettings,
    #[from(@.members.len())]
    pub total_members: usize,
    #[from(~.to_chrono())]
    pub created_at: DateTime<Utc>,
    #[from(~.to_chrono())]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, o2o, TS)]
#[ts(export, bound = "")]
#[from_owned(BusinessMember)]
pub struct BusinessMemberDto {
    pub email: Email,
    #[from(~.map(|id| id.into()))]
    pub user_id: Option<Id>,
    pub role: MemberRole,
    pub status: MembershipStatus,
    pub permissions: Vec<Permission>,
    #[from(~.into())]
    pub invited_by: Id,
    #[from(~.map(|dt| dt.to_chrono()))]
    pub joined_at: Option<DateTime<Utc>>,
    #[from(~.to_chrono())]
    pub created_at: DateTime<Utc>,
    #[from(~.to_chrono())]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, TS)]
#[ts(export, optional_fields)]
pub struct BusinessListQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub status: Option<BusinessStatus>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BusinessSession {
    pub business_id: Id,
    pub user_id: Id,
    pub role: MemberRole,
    pub permissions: Vec<Permission>,
}

impl BusinessSession {
    pub fn has_permission(&self, resource: &str, action: &str, target: Option<&str>) -> bool {
        if matches!(self.role, MemberRole::Owner) {
            return true;
        }

        self.permissions
            .iter()
            .any(|p| p.matches(resource, action, target))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum BusinessToken {
    None,
    BusinessSession(BusinessSession),
}

impl TryFrom<&Cookies> for BusinessSession {
    type Error = ApiError;

    fn try_from(cookies: &Cookies) -> Result<Self, Self::Error> {
        let value = BusinessToken::try_from(cookies)?;
        match value {
            BusinessToken::BusinessSession(session) => Ok(session),
            _ => Err(ApiError::invalid_token()),
        }
    }
}

impl TryFrom<&Cookies> for BusinessToken {
    type Error = ApiError;

    fn try_from(value: &Cookies) -> Result<Self, Self::Error> {
        Ok(value
            .get("business_token")
            .map(|val| decode_jwt(val.value()).unwrap_or(BusinessToken::None))
            .unwrap_or(BusinessToken::None))
    }
}

impl<'c> TryInto<Cookie<'c>> for BusinessToken {
    type Error = ApiError;
    fn try_into(self) -> Result<Cookie<'c>, ApiError> {
        let token = encode_jwt(self, Duration::days(7))?;
        Ok(Cookie::build(("business_token", token)).path("/").build())
    }
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, bound = "")]
pub struct BusinessSwitch {
    pub business_id: Id,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, bound = "")]
pub struct InvitationCreate {
    pub email: Email,
    pub role: MemberRole,
    pub permissions: Option<Vec<Permission>>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, bound = "")]
pub struct InvitationAccept {
    pub token: String,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, bound = "")]
pub struct MemberUpdate {
    pub email: Email,
    pub role: Option<MemberRole>,
    pub permissions: Option<Vec<Permission>>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, bound = "")]
pub struct MemberRemove {
    pub email: Email,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, bound = "")]
pub struct PermissionCheck {
    pub resource: String,
    pub action: String,
    pub target: Option<String>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, bound = "")]
pub struct InvitationResend {
    pub email: Email,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, bound = "")]
pub struct InvitationDto {
    pub email: Email,
    pub role: MemberRole,
    pub token: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, bound = "")]
pub struct BusinessStatistics {
    pub total_members: u32,
    pub active_members: u32,
    pub pending_invitations: u32,
    pub plan_type: PlanType,
    pub plan_expires_at: DateTime<Utc>,
    pub is_plan_expired: bool,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, bound = "")]
pub struct PermissionCheckResponse {
    pub has_permission: bool,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, bound = "")]
pub struct BusinessListResponse {
    pub businesses: Vec<BusinessDto>,
    pub total: u64,
    pub page: u32,
    pub limit: u32,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, bound = "")]
pub struct PendingInvitationsResponse {
    pub invitations: Vec<(BusinessDto, InvitationDto)>,
}
