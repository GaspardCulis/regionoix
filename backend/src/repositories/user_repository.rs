use crate::{AppState, Error, entities::user};
use actix_web::web::Data;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityName, EntityTrait, QueryFilter};

#[derive(Clone)]
pub struct UserRepository {
    db: DatabaseConnection,
}

impl UserRepository {
    pub fn new(state: Data<AppState>) -> Self {
        Self {
            db: state.db.clone(),
        }
    }

    /// Get all users
    pub async fn get_all_users(&self) -> Result<Vec<user::Model>, Error> {
        let users = user::Entity::find().all(&self.db).await?;
        Ok(users)
    }

    /// Get user by ID
    pub async fn get_user_by_id(&self, id: i32) -> Result<user::Model, Error> {
        let user = user::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| Error::EntityNotFound {
                table_name: user::Entity.table_name(),
            })?;
        Ok(user)
    }

    /// Get user by email
    pub async fn get_user_by_email(&self, email: &str) -> Result<user::Model, Error> {
        let user = user::Entity::find()
            .filter(user::Column::Email.eq(email))
            .one(&self.db)
            .await?
            .ok_or_else(|| Error::EntityNotFound {
                table_name: user::Entity.table_name(),
            })?;
        Ok(user)
    }

    /// Check authentication: find by email and verify password
    pub async fn check_auth(&self, email: &str, password: &str) -> Result<user::Model, Error> {
        let user = self.get_user_by_email(email).await?;

        // Use AuthenticationFailure from your Error enum
        if user.password != password {
            return Err(Error::AuthenticationFailure);
        }

        Ok(user)
    }
}
