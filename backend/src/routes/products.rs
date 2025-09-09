use crate::{
    AppState,
    repositories::product_repository::ProductRepository,
    services::product_service::ProductService,
};
use actix_web::{HttpRequest, HttpResponse, get, web};

use utoipa_actix_web::service_config::ServiceConfig;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_all)
       .service(get_by_id)
       .service(get_by_id_expand);
}

#[utoipa::path()]
#[get("")]
pub async fn get_all(data: web::Data<AppState>) -> crate::Result<HttpResponse> {
    let repo = ProductRepository::new(data.clone());
    let service = ProductService::new(repo);

    let products = service.get_all().await?;
    Ok(HttpResponse::Ok().json(products))
}

#[utoipa::path()]
#[get("/{id}")]
pub async fn get_by_id(data: web::Data<AppState>, req: HttpRequest) -> crate::Result<HttpResponse> {
    let id: i32 = req.match_info().query("id").parse()?;

    let repo = ProductRepository::new(data.clone());
    let service = ProductService::new(repo);

    let product = service.get_product_by_id(id).await?;
    Ok(HttpResponse::Ok().json(product))
}

#[utoipa::path()]
#[get("/{id}/expand")]
pub async fn get_by_id_expand(data: web::Data<AppState>, req: HttpRequest) -> crate::Result<HttpResponse> {
    let id: i32 = req.match_info().query("id").parse()?;

    let repo = ProductRepository::new(data.clone());
    let service = ProductService::new(repo);

    let dto = service.get_product_expanded(id).await?;
    Ok(HttpResponse::Ok().json(dto))
}
