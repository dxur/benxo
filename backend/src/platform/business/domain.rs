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
    Premium,
    Enterprise,
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
    Active,    // User is active member
    Pending,   // Invitation sent but not accepted yet
    Suspended, // Temporarily suspended
    Removed,   // Permanently removed
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct Permission {
    pub resource: String,      // e.g., "workers", "departments", "reports"
    pub action: String,        // e.g., "read", "write", "delete", "manage"
    pub scope: Option<String>, // e.g., "department:*", "worker:123", "*"
}

impl Permission {
    pub fn new(resource: &str, action: &str, scope: Option<&str>) -> Self {
        Self {
            resource: resource.to_string(),
            action: action.to_string(),
            scope: scope.map(|s| s.to_string()),
        }
    }

    pub fn matches(&self, resource: &str, action: &str, target: Option<&str>) -> bool {
        if self.resource != resource || self.action != action {
            return false;
        }

        match (&self.scope, target) {
            (None, _) => true, // No scope restriction
            (Some(scope), None) => scope == "*",
            (Some(scope), Some(target)) => {
                if scope == "*" {
                    return true;
                }

                if scope.ends_with(":*") {
                    let prefix = &scope[..scope.len() - 2];
                    return target.starts_with(prefix);
                }

                scope == target
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessMember {
    pub email: Email,
    pub user_id: Option<ObjectId>, // None if user hasn't registered yet
    pub role: MemberRole,
    pub status: MembershipStatus,
    pub permissions: Vec<Permission>,
    pub invited_by: ObjectId,
    pub invitation_token: Option<String>, // Present when status is Pending
    pub invitation_expires_at: Option<DateTime>, // Present when status is Pending
    pub joined_at: Option<DateTime>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl BusinessMember {
    pub fn new_invitation(
        email: Email,
        role: MemberRole,
        invited_by: ObjectId,
        token: String,
        expires_in_hours: i64,
    ) -> Self {
        let now = DateTime::now();
        let expires_at =
            DateTime::from_chrono(chrono::Utc::now() + chrono::Duration::hours(expires_in_hours));

        Self {
            email,
            user_id: None,
            role,
            status: MembershipStatus::Pending,
            permissions: Vec::new(),
            invited_by,
            invitation_token: Some(token),
            invitation_expires_at: Some(expires_at),
            joined_at: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn new_active_member(
        email: Email,
        user_id: ObjectId,
        role: MemberRole,
        invited_by: ObjectId,
    ) -> Self {
        let now = DateTime::now();

        Self {
            email,
            user_id: Some(user_id),
            role,
            status: MembershipStatus::Active,
            permissions: Vec::new(),
            invited_by,
            invitation_token: None,
            invitation_expires_at: None,
            joined_at: Some(now),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn is_invitation_expired(&self) -> bool {
        match self.invitation_expires_at {
            Some(expires_at) => DateTime::now() > expires_at,
            None => false,
        }
    }

    pub fn can_accept_invitation(&self) -> bool {
        matches!(self.status, MembershipStatus::Pending) && !self.is_invitation_expired()
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

    pub fn has_permission(&self, resource: &str, action: &str, target: Option<&str>) -> bool {
        // Owner has all permissions
        if matches!(self.role, MemberRole::Owner) {
            return true;
        }

        // Check explicit permissions
        self.permissions
            .iter()
            .any(|p| p.matches(resource, action, target))
    }

    pub fn accept_invitation(&mut self, user_id: ObjectId) {
        self.user_id = Some(user_id);
        self.status = MembershipStatus::Active;
        self.invitation_token = None;
        self.invitation_expires_at = None;
        self.joined_at = Some(DateTime::now());
        self.updated_at = DateTime::now();
    }

    pub fn add_permission(&mut self, permission: Permission) {
        self.permissions.push(permission);
        self.updated_at = DateTime::now();
    }

    pub fn remove_permission(&mut self, resource: &str, action: &str, scope: Option<&str>) {
        self.permissions.retain(|p| {
            !(p.resource == resource && p.action == action && p.scope.as_deref() == scope)
        });
        self.updated_at = DateTime::now();
    }
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
    pub members: Vec<BusinessMember>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct BusinessSettings {
    pub allow_member_invitations: bool,
    pub require_invitation_approval: bool,
    pub default_member_permissions: Vec<Permission>,
}

impl Default for BusinessSettings {
    fn default() -> Self {
        Self {
            allow_member_invitations: true,
            require_invitation_approval: false,
            default_member_permissions: vec![Permission::new("*", "read", Some("*"))],
        }
    }
}

impl BusinessRecord {
    pub fn new(
        name: Name,
        owner_id: ObjectId,
        owner_email: Email,
        description: Option<String>,
    ) -> Self {
        let now = DateTime::now();
        let plan_expires_at =
            DateTime::from_chrono(chrono::Utc::now() + chrono::Duration::days(30));

        // Owner is automatically an active member
        let owner_member = BusinessMember::new_active_member(
            owner_email,
            owner_id,
            MemberRole::Owner,
            owner_id, // Owner invited themselves
        );

        Self {
            _id: ObjectId::new(),
            name,
            description,
            owner_id,
            plan_type: PlanType::Free,
            plan_expires_at,
            status: BusinessStatus::Active,
            settings: BusinessSettings::default(),
            members: vec![owner_member],
            created_at: now,
            updated_at: now,
        }
    }

    pub fn is_plan_expired(&self) -> bool {
        DateTime::now() > self.plan_expires_at
    }

    pub fn find_member_by_email(&self, email: &str) -> Option<&BusinessMember> {
        self.members.iter().find(|m| m.email.as_str() == email)
    }

    pub fn find_member_by_user_id(&self, user_id: &ObjectId) -> Option<&BusinessMember> {
        self.members
            .iter()
            .find(|m| m.user_id.as_ref() == Some(user_id))
    }

    pub fn find_member_by_token(&self, token: &str) -> Option<&BusinessMember> {
        self.members.iter().find(|m| {
            m.invitation_token.as_ref() == Some(&token.to_string()) && m.can_accept_invitation()
        })
    }

    pub fn add_invitation(&mut self, member: BusinessMember) -> Result<(), String> {
        if self.find_member_by_email(member.email.as_str()).is_some() {
            return Err("Member with this email already exists".to_string());
        }

        self.members.push(member);
        self.updated_at = DateTime::now();
        Ok(())
    }

    pub fn accept_invitation(&mut self, token: &str, user_id: ObjectId) -> Result<(), String> {
        let member_index = self
            .members
            .iter()
            .position(|m| {
                m.invitation_token.as_ref() == Some(&token.to_string()) && m.can_accept_invitation()
            })
            .ok_or("Invalid or expired invitation token")?;

        self.members[member_index].accept_invitation(user_id);
        self.updated_at = DateTime::now();
        Ok(())
    }

    pub fn remove_member(&mut self, email: &str) -> Result<(), String> {
        let member_index = self
            .members
            .iter()
            .position(|m| m.email.as_str() == email)
            .ok_or("Member not found")?;

        // Can't remove the owner
        if matches!(self.members[member_index].role, MemberRole::Owner) {
            return Err("Cannot remove business owner".to_string());
        }

        self.members[member_index].status = MembershipStatus::Removed;
        self.members[member_index].updated_at = DateTime::now();
        self.updated_at = DateTime::now();
        Ok(())
    }

    pub fn update_member_role(&mut self, email: &str, new_role: MemberRole) -> Result<(), String> {
        let member_index = self
            .members
            .iter()
            .position(|m| m.email.as_str() == email)
            .ok_or("Member not found")?;

        // Can't change owner role
        if matches!(self.members[member_index].role, MemberRole::Owner) {
            return Err("Cannot change owner role".to_string());
        }

        self.members[member_index].role = new_role;
        self.members[member_index].updated_at = DateTime::now();
        self.updated_at = DateTime::now();
        Ok(())
    }

    pub fn add_member_permission(
        &mut self,
        email: &str,
        permission: Permission,
    ) -> Result<(), String> {
        let member_index = self
            .members
            .iter()
            .position(|m| m.email.as_str() == email)
            .ok_or("Member not found")?;

        self.members[member_index].add_permission(permission);
        self.updated_at = DateTime::now();
        Ok(())
    }

    pub fn get_active_members(&self) -> Vec<&BusinessMember> {
        self.members
            .iter()
            .filter(|m| matches!(m.status, MembershipStatus::Active))
            .collect()
    }

    pub fn get_pending_invitations(&self) -> Vec<&BusinessMember> {
        self.members
            .iter()
            .filter(|m| matches!(m.status, MembershipStatus::Pending))
            .collect()
    }
}

#[derive(Debug, Clone, Default)]
pub struct BusinessFilter {
    pub owner_id: Option<ObjectId>,
    pub plan_type: Option<PlanType>,
    pub status: Option<BusinessStatus>,
    pub name: Option<String>,
    pub member_email: Option<String>, // Filter by member email
    pub created_after: Option<DateTime>,
    pub created_before: Option<DateTime>,
}
