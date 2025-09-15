use regionoix::{dtos::cart_line::CartLineDto, prelude::*};
use sea_orm::{ActiveValue::Set, prelude::*};

use crate::{AppState, routes::auth::LoggedUser};

#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
struct FormAddToBasket {
    product_id: i32,
    quantity: Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
struct FormUpdateQuantityBasket {
    quantity: i32,
}

#[utoipa::path(
    summary = "Add product to basket of current user",
    tag="Basket",
    request_body(content= FormAddToBasket, content_type= "Application/Json"),
    responses(
    (
        status = 200,
        description="Product successfully added to basket",
        content_type = "application/json",
        body=CartLineDto,
    ),
))]
#[post("/items")]
async fn add(
    data: web::Data<AppState>,
    form_data: web::Json<FormAddToBasket>,
    logged_user: LoggedUser,
) -> crate::Result<HttpResponse> {
    let db = &data.db;
    let quantity = form_data.quantity.unwrap_or(1);

    let product = Product::find_by_id(form_data.product_id)
        .one(&db.conn)
        .await?
        .ok_or(crate::Error::EntityNotFound {
            table_name: cart::Entity.table_name(),
        })?;

    // Check if there is enough stock for quantity desired
    if product.stock < quantity {
        return Err(crate::Error::BadRequestError(
            "Not enough stock for adding product to cart in desired quantity".into(),
        ));
    }

    let cart = cart::Entity::find()
        .filter(cart::Column::UserId.eq(logged_user.id))
        .one(&db.conn)
        .await?
        .ok_or(crate::Error::EntityNotFound {
            table_name: cart::Entity.table_name(),
        })?;

    let form_data = form_data.into_inner();

    // Check if product already added in cart
    let cart_line = CartLine::find()
        .filter(cart_line::Column::ProductId.eq(form_data.product_id))
        .filter(cart_line::Column::CartId.eq(cart.id))
        .one(&db.conn)
        .await?;

    if cart_line.is_some() {
        return Err(crate::Error::BadRequestError(
            "Product already added to cart, try updating quantity".into(),
        ));
    }

    let cart_line = cart_line::ActiveModel {
        cart_id: Set(Some(cart.id)),
        product_id: Set(form_data.product_id),
        quantity: Set(quantity),
        ..Default::default()
    };

    let res = cart_line.insert(&db.conn).await?;

    Ok(HttpResponse::Created().json(res))
}

#[utoipa::path(
    summary = "Update quantity of product in current user's basket",
    tag="Basket",
    request_body(content_type = "Application/Json",
    content = FormUpdateQuantityBasket),
    params (("product_id" = i32, Path, description = "Product id")),
    responses(
        (
            status = 200,
            description="Basket details successfully returned",
            content_type = "application/json",
            body=CartLineDto,
        ),
))]
#[patch("/items/{product_id}")]
async fn update_quantity(
    data: web::Data<AppState>,
    form_data: web::Json<FormUpdateQuantityBasket>,
    req: HttpRequest,
    logged_user: LoggedUser,
) -> crate::Result<HttpResponse> {
    let db = &data.db;
    let product_id: i32 = req.match_info().query("product_id").parse()?;

    let form_data = form_data.into_inner();

    let product = Product::find_by_id(product_id).one(&db.conn).await?.ok_or(
        crate::Error::EntityNotFound {
            table_name: cart::Entity.table_name(),
        },
    )?;

    // Check if there is enough stock for quantity desired
    if product.stock < form_data.quantity {
        return Err(crate::Error::BadRequestError(
            "Not enough stock for updating product to cart in desired quantity".into(),
        ));
    }
    let cart = cart::Entity::find()
        .filter(cart::Column::UserId.eq(logged_user.id))
        .one(&db.conn)
        .await?
        .ok_or(crate::Error::EntityNotFound {
            table_name: cart::Entity.table_name(),
        })?;

    let line_cart = CartLine::find()
        .filter(cart_line::Column::ProductId.eq(product_id))
        .filter(cart_line::Column::CartId.eq(cart.id))
        .one(&db.conn)
        .await?
        .ok_or(crate::Error::EntityNotFound {
            table_name: cart::Entity.table_name(),
        })?;

    let mut cart_line: cart_line::ActiveModel = line_cart.into();

    // Update cart line quantity
    cart_line.quantity = Set(form_data.quantity.to_owned());
    // Update db
    let res = cart_line.update(&db.conn).await?;
    Ok(HttpResponse::Ok().json(res))
}

#[utoipa::path(
    summary="Remove product from current user's basket",
    tag="Basket",
    params(("product_id" = i32, Path, description = "Product id")),
    responses(
        (
            status = 200,
            description="Product successfully removed from basket",
            body=String,
        ),
    ),
)]
#[delete("/items/{product_id}")]
async fn remove(
    data: web::Data<AppState>,
    req: HttpRequest,
    logged_user: LoggedUser,
) -> crate::Result<HttpResponse> {
    let db = &data.db;
    let product_id: i32 = req.match_info().query("product_id").parse()?;

    let cart = cart::Entity::find()
        .filter(cart::Column::UserId.eq(logged_user.id))
        .one(&db.conn)
        .await?
        .ok_or(crate::Error::EntityNotFound {
            table_name: cart::Entity.table_name(),
        })?;

    let cart_line = CartLine::find()
        .filter(cart_line::Column::ProductId.eq(product_id))
        .filter(cart_line::Column::CartId.eq(cart.id))
        .one(&db.conn)
        .await?
        .ok_or(crate::Error::EntityNotFound {
            table_name: cart::Entity.table_name(),
        })?;

    cart_line.delete(&db.conn).await?;
    Ok(HttpResponse::Ok().body("Product successfully removed from cart"))
}
