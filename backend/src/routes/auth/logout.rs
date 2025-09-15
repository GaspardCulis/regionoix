use actix_identity::Identity;
use regionoix::prelude::*;

#[utoipa::path(
    summary="Logs user out",
    tag="Authentification",
    responses(
        (
            status=200,
            description="Logged out successfully",
        ),
    ),
)]
#[post("/logout")]
async fn logout(user: Option<Identity>) -> impl Responder {
    if let Some(user) = user {
        user.logout();
    }
    HttpResponse::Ok()
}
