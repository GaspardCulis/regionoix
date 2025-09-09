use crate::{AppState, Error, entities::user};
use actix_web::web::Data;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, ModelTrait};

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

    pub async fn get_all_users(&self) -> Result<Vec<user::Model>, Error> {
        let users = user::Entity::find().all(&self.db).await?;
        Ok(users)
    }

    pub async fn get_user_by_id(&self, id: i32) -> Result<user::Model, Error> {
        let user = user::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| Error::NotFound(format!("User with id {} not found", id)))?;
        Ok(user)
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<user::Model, Error> {
        let user = user::Entity::find()
            .filter(user::Column::Email.eq(email))
            .one(&self.db)
            .await?
            .ok_or_else(|| Error::NotFound(format!("User with email {} not found", email)))?;
        Ok(user)
    }

    pub async fn check_auth(&self, email: &str, password: &str) -> Result<user::Model, Error> {
        let user = self.get_user_by_email(email).await?;

        if user.password != password {
            return Err(Error::Unauthorized("Invalid credentials".into()));
        }

        Ok(user)
    }
}
