use crate::{Error, dtos::user_dto::UserDto, repositories::user_repository::UserRepository};

pub struct AuthService {
    repo: UserRepository,
}

impl AuthService {
    pub fn new(repo: UserRepository) -> Self {
        Self { repo }
    }

    pub async fn authenticate(&self, email: &str, password: &str) -> Result<UserDto, Error> {
        let user = self.repo.check_auth(email, password).await?;
        Ok(UserDto::from(user))
    }
}
