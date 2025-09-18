use std::{io::Read, iter, path::Path, time::Duration};

use actix_multipart::form::{MultipartForm, json::Json as MpJson, tempfile::TempFile};
use regionoix::prelude::sea_orm_active_enums::Roles;
use reqwest::{Client, header::ETAG};
use rusty_s3::{
    S3Action as _,
    actions::{CompleteMultipartUpload, CreateMultipartUpload, UploadPart},
};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    prelude::Uuid,
};
use utoipa::PartialSchema;

use crate::{prelude::*, routes::auth::LoggedUser};

const SIGN_DURATION: Duration = Duration::from_secs(300);

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(upload);
}

#[derive(Debug, Deserialize, ToSchema)]
struct ProductMetadata {
    name: String,
    description: Option<String>,
    price: f32,
    weight: Option<f64>,
    stock: i32,
    region_id: Option<i32>,
    brand_id: Option<i32>,
    category_id: Option<i32>,
    tags: Vec<i32>,
}

#[derive(Debug, MultipartForm, ToSchema)]
struct UploadForm {
    #[multipart(limit = "10MB")]
    #[schema(value_type = String, format = Binary, content_media_type = "application/octet-stream")]
    image: TempFile,
    #[schema(value_type = String, schema_with = ProductMetadata::schema, content_media_type = "application/json")]
    meta: MpJson<ProductMetadata>,
}

#[utoipa::path(
    summary="Upload new product",
    tag="Admin",
    request_body(content = UploadForm, content_type = "multipart/form-data"),
    responses(
        (
            status=200,
            description="Uploaded sucessfully",
        ),
        (
            status=400,
            description="Invalid data",
        ),
        (
            status=401,
            description="Unauthorized",
        ),
        (
            status=500,
            description="Upload error",
        ),
    ),
)]
#[post("/upload")]
async fn upload(
    MultipartForm(mut form): MultipartForm<UploadForm>,
    db: web::Data<DatabaseService>,
    s3: web::Data<S3Service>,
    logged_user: LoggedUser,
) -> crate::Result<HttpResponse> {
    if logged_user.role != Roles::Admin {
        return Err(crate::Error::Unauthorized);
    }

    let client = Client::new();

    let image_name = form.image.file_name.ok_or(crate::Error::BadRequestError(
        "missing image file name".into(),
    ))?;
    let image_ext = Path::new(&image_name)
        .extension()
        .ok_or(crate::Error::BadRequestError(
            "missing image file extension".into(),
        ))?
        .to_str()
        .ok_or(crate::Error::BadRequestError(
            "invalid image file name encoding".into(),
        ))?;
    let image_name = format!("{}.{}", Uuid::new_v4(), image_ext);

    info!(
        "Beginning uploading {} for product '{}'",
        image_name, form.meta.name
    );
    let action = CreateMultipartUpload::new(&s3.api_bucket, Some(&s3.credentials), &image_name);
    let url = action.sign(SIGN_DURATION);
    let resp = client.post(url).send().await?.error_for_status()?;
    let body = resp.text().await?;

    let multipart =
        CreateMultipartUpload::parse_response(&body).map_err(|e| anyhow::Error::from(e))?;

    debug!(
        "multipart upload created - upload id: {}",
        multipart.upload_id()
    );

    let part_upload = UploadPart::new(
        &s3.api_bucket,
        Some(&s3.credentials),
        &image_name,
        1,
        multipart.upload_id(),
    );
    let url = part_upload.sign(SIGN_DURATION);

    // Read uploaded file
    let mut body = Vec::new();
    let _ = form
        .image
        .file
        .read_to_end(&mut body)
        .map_err(|e| anyhow::Error::from(e))?;
    let resp = client
        .put(url)
        .body(body)
        .send()
        .await?
        .error_for_status()?;
    let etag = resp
        .headers()
        .get(ETAG)
        .expect("every UploadPart request returns an Etag");

    debug!("etag: {}", etag.to_str().unwrap());

    let action = CompleteMultipartUpload::new(
        &s3.api_bucket,
        Some(&s3.credentials),
        &image_name,
        multipart.upload_id(),
        iter::once(etag.to_str().unwrap()),
    );
    let url = action.sign(SIGN_DURATION);

    let resp = client
        .post(url)
        .body(action.body())
        .send()
        .await?
        .error_for_status()?;
    let _ = resp.text().await?;

    info!("Upload successful!");

    let upload_url = s3
        .web_bucket
        .object_url(&image_name)
        .map_err(|e| anyhow::Error::new(e))?;

    info!("Beginning INSERT new product to database");
    let meta = form.meta;
    let new_product = product::ActiveModel {
        id: NotSet,
        name: Set(meta.name.to_owned()),
        description: Set(meta.description.to_owned()),
        weight: Set(meta.weight.to_owned()),
        price: Set(meta.price.to_owned()),
        brand_id: Set(meta.brand_id.to_owned()),
        stock: Set(meta.stock.to_owned()),
        region_id: Set(meta.region_id.to_owned()),
        category_id: Set(meta.category_id.to_owned()),
        image: Set(Some(upload_url.to_string())),
        ..Default::default()
    };

    let inserted_product = new_product.save(&db.conn).await?;

    info!("Updating product tags");
    for tag in meta.tags.iter() {
        let new_product_tag = product_tag::ActiveModel {
            id: NotSet,
            product_id: Set(Some(inserted_product.id.clone().unwrap())),
            tag_id: Set(Some(*tag)),
        };

        let _ = new_product_tag.save(&db.conn).await?;
    }

    info!("Successfully created new product");
    Ok(HttpResponse::Ok().finish())
}
