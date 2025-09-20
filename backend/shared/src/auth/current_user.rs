use uuid::Uuid;
use crate::{
    auth::{Role, Permission},
    error::Error,
    database::{
        models::db_user::DbUser,
    }
};
use std::collections::HashSet;
use time::OffsetDateTime;

#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub roles: Vec<Role>,
    pub permissions: Vec<Permission>,
}

impl CurrentUser {
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
}

impl From<(DbUser, Vec<String>)> for CurrentUser {
    fn from((db_user, role_names): (DbUser, Vec<String>)) -> Self {
        let roles = role_names.into_iter()
            .filter_map(|role_str| match role_str.as_str() {
                "Admin" => Some(Role::Admin),
                "User" => Some(Role::User),
                "Guest" => Some(Role::Guest),
                _ => None,
            })
            .collect::<Vec<Role>>();

        let permissions = Self::calculate_permissions(&roles);

        CurrentUser {
            id: db_user.id,
            username: db_user.username,
            email: db_user.email,
            created_at: db_user.created_at,
            updated_at: db_user.updated_at,
            roles,
            permissions,
        }
    }
}