use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::types::{email::Email, name::Name};

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum BusinessStatus {
    Active,
    Suspended,
    Deleted,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum PlanType {
    Free,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum MemberRole {
    Owner,
    Admin,
    Manager,
    Member,
    ReadOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum MembershipStatus {
    Active,
    Pending,
    Suspended,
    Removed,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum InvitationStatus {
    Pending,
    Accepted,
    Declined,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessRecord {
    pub _id: ObjectId,
    pub name: Name,
    pub description: Option<String>,
    pub owner_id: ObjectId,
    pub plan_type: PlanType,
    pub plan_expires_at: DateTime,
    pub status: BusinessStatus,
    pub settings: BusinessSettings,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessSettings {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipRecord {
    pub _id: ObjectId,
    pub business_id: ObjectId,
    pub user_id: ObjectId,
    pub role: MemberRole,
    pub status: MembershipStatus,
    pub permissions: Vec<String>, // For future field-based access control
    pub invited_by: Option<ObjectId>,
    pub joined_at: Option<DateTime>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvitationRecord {
    pub _id: ObjectId,
    pub business_id: ObjectId,
    pub invited_by: ObjectId,
    pub email: Email,
    pub role: MemberRole,
    pub status: InvitationStatus,
    pub token: String,
    pub expires_at: DateTime,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl BusinessRecord {
    pub fn new(name: Name, owner_id: ObjectId, description: Option<String>) -> Self {
        let now = DateTime::now();
        let plan_expires_at =
            DateTime::from_chrono(chrono::Utc::now() + chrono::Duration::days(30));

        Self {
            _id: ObjectId::new(),
            name,
            description,
            owner_id,
            plan_type: PlanType::Free,
            plan_expires_at,
            status: BusinessStatus::Active,
            settings: BusinessSettings {},
            created_at: now,
            updated_at: now,
        }
    }

    pub fn is_plan_expired(&self) -> bool {
        DateTime::now() > self.plan_expires_at
    }
}

impl MembershipRecord {
    pub fn new(
        business_id: ObjectId,
        user_id: ObjectId,
        role: MemberRole,
        invited_by: Option<ObjectId>,
    ) -> Self {
        let now = DateTime::now();
        Self {
            _id: ObjectId::new(),
            business_id,
            user_id,
            role,
            status: MembershipStatus::Active,
            permissions: Vec::new(),
            invited_by,
            joined_at: Some(now),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn can_manage_members(&self) -> bool {
        matches!(self.role, MemberRole::Owner | MemberRole::Admin)
    }

    pub fn can_invite_members(&self) -> bool {
        matches!(
            self.role,
            MemberRole::Owner | MemberRole::Admin | MemberRole::Manager
        )
    }
}

impl InvitationRecord {
    pub fn new(
        business_id: ObjectId,
        invited_by: ObjectId,
        email: Email,
        role: MemberRole,
        token: String,
        expires_in_hours: i64,
    ) -> Self {
        let now = DateTime::now();
        let expires_at =
            DateTime::from_chrono(chrono::Utc::now() + chrono::Duration::hours(expires_in_hours));

        Self {
            _id: ObjectId::new(),
            business_id,
            invited_by,
            email,
            role,
            status: InvitationStatus::Pending,
            token,
            expires_at,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn is_expired(&self) -> bool {
        DateTime::now() > self.expires_at
    }

    pub fn can_accept(&self) -> bool {
        matches!(self.status, InvitationStatus::Pending) && !self.is_expired()
    }
}

#[derive(Debug, Clone, Default)]
pub struct BusinessFilter {
    pub owner_id: Option<ObjectId>,
    pub plan_type: Option<PlanType>,
    pub status: Option<BusinessStatus>,
    pub name: Option<String>,
    pub created_after: Option<DateTime>,
    pub created_before: Option<DateTime>,
}

#[derive(Debug, Clone, Default)]
pub struct MembershipFilter {
    pub business_id: Option<ObjectId>,
    pub user_id: Option<ObjectId>,
    pub role: Option<MemberRole>,
    pub status: Option<MembershipStatus>,
}

#[derive(Debug, Clone, Default)]
pub struct InvitationFilter {
    pub business_id: Option<ObjectId>,
    pub email: Option<String>,
    pub status: Option<InvitationStatus>,
    pub invited_by: Option<ObjectId>,
}
