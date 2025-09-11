use uuid::Uuid;
use crate::{
    auth::{Role, Permission},
    error::Error,
};
use std::collections::HashSet;

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
}