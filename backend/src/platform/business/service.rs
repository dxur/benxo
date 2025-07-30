use bson::DateTime;
use uuid::Uuid;

use super::api::*;
use super::domain::*;
use super::repo::BusinessRepo;
use crate::platform::user::api::UserSession;
use crate::utils::error::{ApiError, ApiResult};

pub struct BusinessService<B: BusinessRepo> {
    business_repo: B,
}

impl<B: BusinessRepo> BusinessService<B> {
    pub fn new(business_repo: B) -> Self {
        Self { business_repo }
    }

    pub async fn create(
        &self,
        user: UserSession,
        create_req: BusinessCreate,
    ) -> ApiResult<BusinessView> {
        // Check if user already owns a business with this name
        if let Some(_existing) = self
            .business_repo
            .find_by_name(create_req.name.as_str())
            .await?
        {
            return Err(ApiError::conflict(
                "business",
                "Business name already exists",
            ));
        }

        let business = BusinessRecord::new(
            create_req.name,
            user.user_id,
            user.email,
            create_req.description,
        );

        let created = self.business_repo.create(business).await?;
        Ok(BusinessView::from(created))
    }

    pub async fn current(&self, business_session: BusinessSession) -> ApiResult<BusinessView> {
        let business = self
            .business_repo
            .find_by_id(business_session.business_id.into_inner())
            .await?
            .ok_or_else(|| ApiError::not_found("business", "Business not found"))?;

        Ok(BusinessView::from(business))
    }

    pub async fn list_user_businesses(
        &self,
        user: UserSession,
        query: BusinessListQuery,
    ) -> ApiResult<(Vec<BusinessView>, u64)> {
        let filter = BusinessFilter {
            member_email: Some(user.email.to_string()),
            status: query.status,
            ..Default::default()
        };

        let (businesses, total) = self
            .business_repo
            .list(filter, query.page.unwrap_or(1), query.limit.unwrap_or(10))
            .await?;

        let views: Vec<BusinessView> = businesses.into_iter().map(BusinessView::from).collect();
        Ok((views, total))
    }

    pub async fn invite_member(
        &self,
        business_session: BusinessSession,
        inviter: UserSession,
        invitation: InvitationCreate,
    ) -> ApiResult<InvitationView> {
        let mut business = self
            .business_repo
            .find_by_id(business_session.business_id.into_inner())
            .await?
            .ok_or_else(|| ApiError::not_found("business", "Business not found"))?;

        let inviter_member = business
            .find_member_by_user_id(&inviter.user_id)
            .ok_or_else(|| ApiError::forbidden("invitation", "Not a member of this business"))?;

        if !inviter_member.can_invite_members() {
            return Err(ApiError::forbidden(
                "invitation",
                "No permission to invite members",
            ));
        }

        let token = Uuid::new_v4().to_string();

        let mut new_member = BusinessMember::new_invitation(
            invitation.email.clone(),
            invitation.role,
            inviter.user_id,
            token.clone(),
            72,
        );

        if let Some(permissions) = invitation.permissions {
            new_member.permissions = permissions;
        } else {
            new_member.permissions = business.settings.default_member_permissions.clone();
        }

        business
            .add_invitation(new_member)
            .map_err(|e| ApiError::conflict("invitation", e.to_string()))?;

        let updated_business = self
            .business_repo
            .update(business_session.business_id.into_inner(), business)
            .await?;

        let invitation_member = updated_business
            .find_member_by_email(invitation.email.as_str())
            .ok_or_else(|| ApiError::internal("Failed to create invitation"))?;

        Ok(InvitationView {
            email: invitation_member.email.clone(),
            role: invitation_member.role.clone(),
            token,
            expires_at: invitation_member.invitation_expires_at.unwrap().to_chrono(),
        })
    }

    pub async fn accept_invitation(
        &self,
        user: UserSession,
        accept_req: InvitationAccept,
    ) -> ApiResult<BusinessView> {
        let businesses = self
            .business_repo
            .list(
                BusinessFilter::default(),
                1,
                1000, // Get all businesses to search for token
            )
            .await?
            .0;

        let mut target_business = None;
        for business in businesses {
            if business.find_member_by_token(&accept_req.token).is_some() {
                target_business = Some(business);
                break;
            }
        }

        let mut business = target_business
            .ok_or_else(|| ApiError::not_found("invitation", "Invalid or expired invitation"))?;

        business
            .accept_invitation(&accept_req.token, user.user_id)
            .map_err(|_| ApiError::invalid_token())?;

        let updated_business = self.business_repo.update(business._id, business).await?;
        Ok(BusinessView::from(updated_business))
    }

    pub async fn update_member(
        &self,
        business_session: BusinessSession,
        user: UserSession,
        update_req: MemberUpdate,
    ) -> ApiResult<BusinessMemberView> {
        let mut business = self
            .business_repo
            .find_by_id(business_session.business_id.into_inner())
            .await?
            .ok_or_else(|| ApiError::not_found("business", "Business not found"))?;

        // Check if user can manage members
        let user_member = business
            .find_member_by_user_id(&user.user_id)
            .ok_or_else(|| ApiError::forbidden("business", "Not a member of this business"))?;

        if !user_member.can_manage_members() {
            return Err(ApiError::forbidden(
                "business",
                "No permission to manage members",
            ));
        }

        // Update role if provided
        if let Some(new_role) = update_req.role {
            business
                .update_member_role(update_req.email.as_str(), new_role)
                .map_err(|e| ApiError::unauthorized(e.to_string()))?;
        }

        // Update permissions if provided
        if let Some(new_permissions) = update_req.permissions {
            let member_index = business
                .members
                .iter()
                .position(|m| m.email == update_req.email)
                .ok_or_else(|| ApiError::not_found("business", "Member not found"))?;

            business.members[member_index].permissions = new_permissions;
            business.members[member_index].updated_at = DateTime::now();
        }

        let updated_business = self
            .business_repo
            .update(business_session.business_id.into_inner(), business)
            .await?;

        let updated_member = updated_business
            .find_member_by_email(update_req.email.as_str())
            .ok_or_else(|| ApiError::internal("Failed to update member"))?;

        Ok(BusinessMemberView::from(updated_member.clone()))
    }

    pub async fn remove_member(
        &self,
        business_session: BusinessSession,
        user: UserSession,
        member_email: String,
    ) -> ApiResult<()> {
        let mut business = self
            .business_repo
            .find_by_id(business_session.business_id.into_inner())
            .await?
            .ok_or_else(|| ApiError::not_found("business", "Business not found"))?;

        // Check if user can manage members
        let user_member = business
            .find_member_by_user_id(&user.user_id)
            .ok_or_else(|| ApiError::forbidden("business", "Not a member of this business"))?;

        if !user_member.can_manage_members() {
            return Err(ApiError::forbidden(
                "business",
                "No permission to manage members",
            ));
        }

        business
            .remove_member(&member_email)
            .map_err(|e| ApiError::unauthorized(e))?;

        self.business_repo
            .update(business_session.business_id.into_inner(), business)
            .await?;
        Ok(())
    }

    pub async fn check_permission(
        &self,
        business_session: BusinessSession,
        user: UserSession,
        resource: String,
        action: String,
        target: Option<String>,
    ) -> ApiResult<bool> {
        let business = self
            .business_repo
            .find_by_id(business_session.business_id.into_inner())
            .await?
            .ok_or_else(|| ApiError::not_found("business", "Business not found"))?;

        let user_member = business
            .find_member_by_user_id(&user.user_id)
            .ok_or_else(|| ApiError::forbidden("business", "Not a member of this business"))?;

        let has_permission = user_member.has_permission(&resource, &action, target.as_deref());

        Ok(has_permission)
    }

    pub async fn get_pending_invitations_by_email(
        &self,
        email: String,
    ) -> ApiResult<Vec<(BusinessView, InvitationView)>> {
        let businesses = self
            .business_repo
            .list(
                BusinessFilter {
                    member_email: Some(email.clone()),
                    ..Default::default()
                },
                1,
                1000,
            )
            .await?
            .0;

        let mut results = Vec::new();

        for business in businesses {
            if let Some(member) = business.find_member_by_email(&email) {
                if matches!(member.status, MembershipStatus::Pending)
                    && !member.is_invitation_expired()
                {
                    let invitation = InvitationView {
                        email: member.email.clone(),
                        role: member.role.clone(),
                        token: member.invitation_token.clone().unwrap(),
                        expires_at: member.invitation_expires_at.unwrap().to_chrono(),
                    };
                    results.push((BusinessView::from(business), invitation));
                }
            }
        }

        Ok(results)
    }

    pub async fn switch_business(
        &self,
        user: UserSession,
        switch_req: BusinessSwitch,
    ) -> ApiResult<BusinessToken> {
        let business = self
            .business_repo
            .find_by_id(switch_req.business_id.into_inner())
            .await?
            .ok_or_else(|| ApiError::not_found("business", "Business not found"))?;

        // Verify user is a member of this business
        business
            .find_member_by_user_id(&user.user_id)
            .ok_or_else(|| ApiError::forbidden("business", "Not a member of this business"))?;

        let member = business
            .members
            .into_iter()
            .find(|m| {
                if let Some(id) = m.user_id {
                    id == user.user_id
                } else {
                    false
                }
            })
            .ok_or(ApiError::forbidden(
                "business",
                "Not a member of this business",
            ))?;

        Ok(BusinessToken::BusinessSession(BusinessSession {
            user_id: user.user_id.into(),
            business_id: business._id.into(),
            role: member.role,
            permissions: member.permissions,
        }))
    }

    pub async fn get_business_statistics(
        &self,
        business_session: BusinessSession,
        _user: UserSession,
    ) -> ApiResult<BusinessStatistics> {
        let business = self
            .business_repo
            .find_by_id(business_session.business_id.into_inner())
            .await?
            .ok_or_else(|| ApiError::not_found("business", "Business not found"))?;

        let active_members = business.get_active_members().len() as u32;
        let pending_invitations = business.get_pending_invitations().len() as u32;

        let is_plan_expired = business.is_plan_expired();

        Ok(BusinessStatistics {
            total_members: business.members.len() as u32,
            active_members,
            pending_invitations,
            plan_type: business.plan_type,
            plan_expires_at: business.plan_expires_at.to_chrono(),
            is_plan_expired,
        })
    }

    pub async fn update_business_settings(
        &self,
        business_session: BusinessSession,
        user: UserSession,
        settings: BusinessSettings,
    ) -> ApiResult<BusinessView> {
        let mut business = self
            .business_repo
            .find_by_id(business_session.business_id.into_inner())
            .await?
            .ok_or_else(|| ApiError::not_found("business", "Business not found"))?;

        // Check if user is owner or has admin privileges
        let user_member = business
            .find_member_by_user_id(&user.user_id)
            .ok_or_else(|| ApiError::forbidden("business", "Not a member of this business"))?;

        if !matches!(user_member.role, MemberRole::Owner | MemberRole::Admin) {
            return Err(ApiError::forbidden(
                "business",
                "Only owners and admins can update business settings",
            ));
        }

        business.settings = settings;
        business.updated_at = DateTime::now();

        let updated_business = self
            .business_repo
            .update(business_session.business_id.into_inner(), business)
            .await?;

        Ok(BusinessView::from(updated_business))
    }

    pub async fn resend_invitation(
        &self,
        business_session: BusinessSession,
        user: UserSession,
        member_email: String,
    ) -> ApiResult<InvitationView> {
        let mut business = self
            .business_repo
            .find_by_id(business_session.business_id.into_inner())
            .await?
            .ok_or_else(|| ApiError::not_found("business", "Business not found"))?;

        // Check if user can invite members
        let user_member = business
            .find_member_by_user_id(&user.user_id)
            .ok_or_else(|| ApiError::forbidden("business", "Not a member of this business"))?;

        if !user_member.can_invite_members() {
            return Err(ApiError::forbidden(
                "business",
                "No permission to resend invitations",
            ));
        }

        // Find the member to resend invitation to
        let member_index = business
            .members
            .iter()
            .position(|m| {
                m.email.as_str() == member_email && matches!(m.status, MembershipStatus::Pending)
            })
            .ok_or_else(|| {
                ApiError::not_found("invitation", "Pending invitation not found for this email")
            })?;

        // Generate new token and update expiration
        let new_token = Uuid::new_v4().to_string();
        let new_expires_at =
            DateTime::from_chrono(chrono::Utc::now() + chrono::Duration::hours(72));

        business.members[member_index].invitation_token = Some(new_token.clone());
        business.members[member_index].invitation_expires_at = Some(new_expires_at);
        business.members[member_index].updated_at = DateTime::now();
        business.updated_at = DateTime::now();

        let updated_business = self
            .business_repo
            .update(business_session.business_id.into_inner(), business)
            .await?;

        let updated_member = updated_business
            .find_member_by_email(&member_email)
            .ok_or_else(|| ApiError::internal("Failed to update invitation"))?;

        Ok(InvitationView {
            email: updated_member.email.clone(),
            role: updated_member.role.clone(),
            token: new_token,
            expires_at: new_expires_at.to_chrono(),
        })
    }
}
