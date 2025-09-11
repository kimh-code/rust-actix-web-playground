use crate::auth::Permission;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Role {
    Admin,
    Moderator,
    User,
    Guest,
}

impl Role {
    pub fn permissions(&self) -> Vec<Permission> {
        match self {
            Role::Admin => vec![
                Permission::CreateUser,
                Permission::DeleteUser,
                Permission::UpdateUser,
                Permission::ReadUser,
                Permission::ManageRoles,
            ],
            Role::Moderator => vec![
                Permission::UpdateUser,
                Permission::ReadUser,
                Permission::CreatePost,
                Permission::DeletePost,
            ],
            Role::User => vec![
                Permission::ReadUser,
                Permission::CreatePost,
                Permission::UpdateOwnPost,
            ],
            Role::Guest => vec![
                Permission::ReadUser,
            ],
        }
    }
}