use argon2::{Argon2, PasswordHash, PasswordVerifier as _};
use regionoix::prelude::user;

pub fn check_password(password: &String, user: &user::Model) -> crate::Result<()> {
    let parsed_hash = PasswordHash::new(&user.password)
        .map_err(|err| crate::Error::InternalError(anyhow::Error::msg(err)))?;

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| crate::Error::AuthenticationFailure)
}
