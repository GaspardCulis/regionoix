use regionoix::prelude::*;

use super::LoggedUser;

#[utoipa::path(
    summary="Get user authentication status",
    tag="Authentification",
    responses(
        (
            status=200,
            content_type="Application/Json",
            body=LoggedUser,
        ),
    ),
)]
#[get("/status")]
async fn status(logged_user: LoggedUser) -> impl Responder {
    HttpResponse::Ok().json(logged_user)
}
