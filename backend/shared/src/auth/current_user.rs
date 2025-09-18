use uuid::Uuid;
use crate::{
    auth::{Role, Permission},
    error::Error,
    database::{
        repositories::user_repository::UserRepository,
    }
};
use std::collections::HashSet;
use async_graphql::ID;

#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub id: Uuid,
    pub email: String,
    pub roles: Vec<Role>,
    pub permissions: Vec<Permission>,
}

impl CurrentUser {
    pub fn new(id: Uuid, email: String, roles: Vec<Role>) -> Self {
        let permissions = Self::calculate_permissions(&roles);

        CurrentUser {
            id,
            email,
            roles,
            permissions,
        }
    }

    fn calculate_permissions(roles: &[Role]) -> Vec<Permission> {
        let mut permissions = HashSet::new();

        for role in roles {
            for permission in role.permissions() {
                permissions.insert(permission);
            }
        }

        permissions.into_iter().collect()
    }

    pub fn has_role(&self, role: &Role) -> bool {
        self.roles.contains(&role)
    }

    pub fn has_permission(&self, permission: &Permission) -> bool {
        self.permissions.contains(&permission)
    }

    pub fn require_role(&self, role: &Role) -> Result<(), Error> {
        if self.has_role(role) {
            Ok(())
        } else {
            Err(Error::Forbidden(format!("Required role: {:?}", role)))
        }
    }

    pub fn require_permission(&self, permission: &Permission) -> Result<(), Error> {
        if self.has_permission(permission) {
            Ok(())
        } else {
            Err(Error::Forbidden(format!("Required permission: {:?}", permission)))
        }
    }

    pub fn is_admin(&self) -> bool {
        self.has_role(&Role::Admin)
    }

    pub async fn from_user_id(
        user_id: &str,
        user_repo: &UserRepository,
    ) -> Result<Self, Error> {
        let user_uuid = Uuid::parse_str(user_id)
            .map_err(|_| Error::InvalidInput("Invalid user ID format".to_string()))?;

        let (db_user, role_names) = user_repo.find_user_with_roles(&user_uuid).await
            .map_err(|e| Error::Database(e))?    
            .ok_or(Error::NotFound("User not found".to_string()))?;

        let roles = role_names.into_iter()
            .filter_map(|role_str| match role_str.as_str() {
                "Admain" => Some(Role::Admin),
                "User" => Some(Role::User),
                "Guest" => Some(Role::Guest),
                _ => None,
            })
            .collect::<Vec<Role>>();

        let permissions = Self::calculate_permissions(&roles);

        Ok(CurrentUser {
            id: db_user.id,
            email: db_user.email,
            roles,
            permissions,
        })
    }
}