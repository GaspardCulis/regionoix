use actix_identity::Identity;
use regionoix::prelude::*;
use sea_orm::prelude::*;

use super::{LoggedUser, utils::check_password};

#[derive(Serialize, Deserialize, ToSchema, Debug)]
struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[utoipa::path(
    summary="Logs user in",
    tag="Authentification",
    request_body(content_type="application/json", content=LoginRequest),
    responses(
        (
            status=200,
            description="Logged in successfully",
        ),
        (
            status=404,
            description="Email not found",
        ),
    ),
)]
#[post("/login")]
async fn login(
    request: HttpRequest,
    login_request: web::Json<LoginRequest>,
    db: web::Data<DatabaseService>,
) -> crate::Result<HttpResponse> {
    let user = user::Entity::find()
        .filter(user::Column::Email.eq(&login_request.email))
        .one(&db.conn)
        .await?
        .ok_or(crate::Error::EntityNotFound {
            table_name: user::Entity.table_name(),
        })?;

    check_password(&login_request.password, &user)?;

    let logged_user = LoggedUser {
        id: user.id,
        email: user.email,
        role: user.role,
        lastname: user.lastname,
        firstname: user.fistname,
    };
    let user_string = serde_json::to_string(&logged_user).unwrap();
    Identity::login(&request.extensions(), user_string).map_err(|err| anyhow::Error::new(err))?;

    Ok(HttpResponse::Ok().finish())
}

#[cfg(test)]
mod tests {
    use actix_identity::IdentityMiddleware;
    use actix_session::{SessionMiddleware, storage::RedisSessionStore};
    use actix_web::{
        App,
        cookie::Key,
        dev::{ServiceFactory, ServiceRequest, ServiceResponse},
        http::{StatusCode, header::ContentType},
        test,
    };
    use regionoix::utils::get_env_var;

    use super::*;

    async fn app_setup() -> App<
        impl ServiceFactory<
            ServiceRequest,
            Config = (),
            Error = actix_web::Error,
            InitError = (),
            Response = ServiceResponse,
        >,
    > {
        dotenv::dotenv().unwrap();

        let database_service = DatabaseService::build().await.unwrap();

        let redis_url: String = get_env_var("REDIS_URL").unwrap();
        info!("Connecting to Redis session store");
        let redis_store = RedisSessionStore::new(redis_url)
            .await
            .expect("Failed to connect to Redis session store");

        App::new()
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(redis_store, Key::generate()))
            .app_data(web::Data::new(database_service))
    }

    #[actix_web::test]
    async fn login_success() {
        let app = test::init_service(app_setup().await.service(login)).await;
        let req = test::TestRequest::post()
            .uri("/login")
            .insert_header(ContentType::json())
            .set_json(LoginRequest {
                email: "testuser@regionoix.fr".into(),
                password: "testpassword".into(),
            })
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn login_email_failure() {
        let app = test::init_service(app_setup().await.service(login)).await;
        let req = test::TestRequest::post()
            .uri("/login")
            .insert_header(ContentType::json())
            .set_json(LoginRequest {
                email: "testuser_absent@regionoix.fr".into(),
                password: "doesnotmatter".into(),
            })
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status() == StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn login_password_failure() {
        let app = test::init_service(app_setup().await.service(login)).await;
        let req = test::TestRequest::post()
            .uri("/login")
            .insert_header(ContentType::json())
            .set_json(LoginRequest {
                email: "testuser@regionoix.fr".into(),
                password: "wrongpassword".into(),
            })
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status() == StatusCode::UNAUTHORIZED);
    }
}
