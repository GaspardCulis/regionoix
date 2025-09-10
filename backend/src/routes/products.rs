use crate::dtos::product::ProductDto;
use crate::prelude::*;
use crate::{
    AppState,
    entities::{
        brand, category,
        prelude::{Brand, Category, Product, Region, Tag},
        product, region, tag,
    },
};
use actix_web::{HttpRequest, HttpResponse, get, web::Data};
use sea_orm::{EntityName, EntityTrait as _, ModelTrait};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get)
        .service(get_by_id)
        .service(get_by_id_expand);
}

#[utoipa::path()]
#[get("")]
pub async fn get(data: Data<AppState>) -> crate::Result<HttpResponse> {
    let db = &data.db;
    let products: Vec<ProductDto> = Product::find().into_dto().all(db).await?;

    Ok(HttpResponse::Ok().json(products))
}

#[utoipa::path()]
#[get("/{id}")]
pub async fn get_by_id(data: Data<AppState>, req: HttpRequest) -> crate::Result<HttpResponse> {
    let db = &data.db;
    let id: i32 = req.match_info().query("id").parse()?;

    let product = Product::find_by_id(id)
        .into_dto::<ProductDto>()
        .one(db)
        .await?
        .ok_or(crate::Error::EntityNotFound {
            table_name: product::Entity.table_name(),
        })?
        .finalize(db)
        .await?;

    Ok(HttpResponse::Ok().json(product))
}

#[utoipa::path()]
#[get("/{id}/expand")]
pub async fn get_by_id_expand(
    data: Data<AppState>,
    req: HttpRequest,
) -> crate::Result<HttpResponse> {
    let db = &data.db;
    let id: u8 = req.match_info().query("id").parse()?;

    let product = Product::find_by_id(id)
        .one(db)
        .await?
        .ok_or(crate::Error::EntityNotFound {
            table_name: product::Entity.table_name(),
        })?;

    let region = product.find_related(Region).one(db).await?;
    let brand = product.find_related(Brand).one(db).await?;
    let category = product.find_related(Category).one(db).await?;
    let tags = product.find_related(Tag).all(db).await?;

    #[derive(serde::Serialize)]
    struct ProductExpanded {
        product: product::Model,
        region: Option<region::Model>,
        brand: Option<brand::Model>,
        category: Option<category::Model>,
        tags: Vec<tag::Model>,
    }

    let response = ProductExpanded {
        product,
        region,
        brand,
        category,
        tags,
    };

    Ok(HttpResponse::Ok().json(response))
}
