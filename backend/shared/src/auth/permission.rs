use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Permission {
    // User permissions
    CreateUser,
    ReadUser,
    UpdateUser,
    DeleteUser,
    ManageRoles,

    // Post permissions
    CreatePost,
    ReadPost,
    UpdatePost,
    UpdateOwnPost,
    DeletePost,
    DeleteOwnPost,

    // Admin permissions
    ViewAuditLog,
    ManageSystem,
}

impl Permission {
    pub fn description(&self) -> &'static str {
        match self {
            Permission::CreateUser => "Create new users",
            Permission::ReadUser => "View user information",
            Permission::UpdateUser => "Update user information",
            Permission::DeleteUser => "Delete users",
            Permission::ManageRoles => "Assign and remove roles",
            Permission::CreatePost => "Create new posts",
            Permission::ReadPost => "View posts",
            Permission::UpdatePost => "Edit any post",
            Permission::UpdateOwnPost => "Edit own posts only",
            Permission::DeletePost => "Delete any post",
            Permission::DeleteOwnPost => "Delete own posts only",
            Permission::ViewAuditLog => "View system audit logs",
            Permission::ManageSystem => "Mange system settings",
        }
    }
}