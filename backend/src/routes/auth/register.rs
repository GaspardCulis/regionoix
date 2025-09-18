use sea_orm::ColumnTrait;

use argon2::{
    Argon2,
    password_hash::{SaltString, rand_core::OsRng},
};

use argon2::PasswordHasher;
use regionoix::prelude::sea_orm_active_enums::Roles;

use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    EntityTrait, QueryFilter,
};

use crate::{prelude::*, routes::auth::LoggedUser};

#[derive(Debug, Deserialize, ToSchema)]
struct RegisterForm {
    pub email: String,
    pub password: String,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
}

#[utoipa::path(
    summary="Create new client",
    tag="Client",
    request_body(content = RegisterForm, content_type = "application/json"),
    responses(
        (
            status=200,
            description="Registered sucessfully",
            body=LoggedUser
        ),
        (
            status=400,
            description="Email already used for another user",
            body=String

        ),
        (
            status=400,
            description="Password is too short, should be at least 8 caracters long.",
            body=String

        ),
        (
            status=500,
            description="Register error",
            body=String
        ),
    ),
)]
#[post("/register")]
async fn register(
    form_data: web::Json<RegisterForm>,
    db: web::Data<DatabaseService>,
) -> crate::Result<HttpResponse> {
    let form_data = form_data.into_inner();

    // Check if user already exists with email
    let existing_user = user::Entity::find()
        .filter(user::Column::Email.eq(form_data.email.to_owned()))
        .one(&db.conn)
        .await?;

    if existing_user.is_some() {
        return Err(crate::Error::BadRequestError(
            "Email already used for another user.".into(),
        ));
    }

    // Check if password has correct length
    // TODO: Check more properties (maj, nums, special)
    if form_data.password.chars().count() < 8 {
        return Err(crate::Error::BadRequestError(
            "Password is too short, should be at least 8 caracters long.".into(),
        ));
    }

    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password
    let password_hash = argon2
        .hash_password(&form_data.password.as_bytes(), &salt)
        .map_err(|err| crate::Error::InternalError(anyhow::Error::msg(err)))?
        .to_string();

    let client = user::ActiveModel {
        id: NotSet,
        email: Set(form_data.email.to_owned()),
        password: Set(password_hash.to_owned()),
        fistname: Set(form_data.firstname.to_owned()),
        lastname: Set(form_data.lastname.to_owned()),
        role: Set(Roles::Client),
        ..Default::default()
    };

    let client: user::Model = client.insert(&db.conn).await?;

    let logged_user = LoggedUser {
        id: client.id,
        email: client.email,
        role: client.role,
        lastname: client.lastname,
        firstname: client.fistname,
    };

    Ok(HttpResponse::Ok().json(logged_user))
}
