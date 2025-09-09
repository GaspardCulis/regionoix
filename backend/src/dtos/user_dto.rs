use serde::{Deserialize, Serialize};

use crate::entities::user::Model as UserModel;
use crate::entities::sea_orm_active_enums::Roles as Roles;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDto {
    pub id: i32,
    pub email: String,
    pub role: Roles,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
}

impl From<UserModel> for UserDto {
    fn from(user: UserModel) -> Self {
        Self {
            id: user.id,
            email: user.email,
            role: user.role,
            firstname: user.fistname,
            lastname: user.lastname,
        }
    }
}
