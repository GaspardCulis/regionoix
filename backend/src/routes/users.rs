use crate::{
    AppState,
    entities::{cart, cart_line, prelude::CartLine, prelude::Product, product},
};
use actix_web::{HttpRequest, HttpResponse, delete, get, patch, post, web};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityName as _, EntityTrait, ModelTrait,
    QueryFilter,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_cart_by_user)
        .service(add_product_to_cart)
        .service(update_quantity_product_cart)
        .service(remove_product_from_cart)
        .service(empty_cart);
}

#[get("/{id}/cart")]
async fn get_cart_by_user(
    data: web::Data<AppState>,
    req: HttpRequest,
) -> crate::Result<HttpResponse> {
    let db = &data.db;
    let id: i32 = req.match_info().query("id").parse()?;

    // Get cart
    let cart = cart::Entity::find()
        .filter(cart::Column::UserId.eq(id))
        .one(db)
        .await?
        .ok_or(crate::Error::EntityNotFound {
            table_name: cart::Entity.table_name(),
        })?;

    // Get all linked lines
    let lines: Vec<cart_line::Model> = cart.find_related(cart_line::Entity).all(db).await?;

    // Add each line with product
    #[derive(serde::Serialize)]
    struct LineWithProduct {
        product: product::Model,
        quantity: i32,
    }

    let mut enriched_lines = Vec::new();

    for line in lines {
        if let Some(prod) = line.find_related(product::Entity).one(db).await? {
            enriched_lines.push(LineWithProduct {
                product: prod,
                quantity: line.quantity,
            });
        }
    }
    // Final result
    #[derive(serde::Serialize)]
    struct CartWithLines {
        cart: cart::Model,
        lines: Vec<LineWithProduct>,
    }

    Ok(HttpResponse::Ok().json(CartWithLines {
        cart,
        lines: enriched_lines,
    }))
}

#[derive(serde::Serialize, serde::Deserialize)]
struct FormDataAddProductToCart {
    product_id: i32,
    quantity: Option<i32>,
}

#[post("/{id}/cart/products")]
async fn add_product_to_cart(
    data: web::Data<AppState>,
    form_data: web::Json<FormDataAddProductToCart>,
    req: HttpRequest,
) -> crate::Result<HttpResponse> {
    let db = &data.db;
    let id: i32 = req.match_info().query("id").parse()?;
    let quantity = form_data.quantity.unwrap_or(1);

    let product = Product::find_by_id(form_data.product_id)
        .one(db)
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
        .filter(cart::Column::UserId.eq(id))
        .one(db)
        .await?
        .ok_or(crate::Error::EntityNotFound {
            table_name: cart::Entity.table_name(),
        })?;

    let form_data = form_data.into_inner();

    // Check if product already added in cart
    let cart_line = CartLine::find()
        .filter(cart_line::Column::ProductId.eq(form_data.product_id))
        .filter(cart_line::Column::CartId.eq(cart.id))
        .one(db)
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

    let res = cart_line.insert(db).await?;

    Ok(HttpResponse::Created().json(res))
}

#[derive(serde::Serialize, serde::Deserialize)]
struct FormDataUpdateQuantityProductCart {
    quantity: i32,
}

#[patch("/{id}/cart/products/{product_id}")]
async fn update_quantity_product_cart(
    data: web::Data<AppState>,
    form_data: web::Json<FormDataUpdateQuantityProductCart>,
    req: HttpRequest,
) -> crate::Result<HttpResponse> {
    let db = &data.db;
    let id: i32 = req.match_info().query("id").parse()?;
    let product_id: i32 = req.match_info().query("product_id").parse()?;

    let form_data = form_data.into_inner();

    let product =
        Product::find_by_id(product_id)
            .one(db)
            .await?
            .ok_or(crate::Error::EntityNotFound {
                table_name: cart::Entity.table_name(),
            })?;

    // Check if there is enough stock for quantity desired
    if product.stock < form_data.quantity {
        return Err(crate::Error::BadRequestError(
            "Not enough stock for updating product to cart in desired quantity".into(),
        ));
    }
    let cart = cart::Entity::find()
        .filter(cart::Column::UserId.eq(id))
        .one(db)
        .await?
        .ok_or(crate::Error::EntityNotFound {
            table_name: cart::Entity.table_name(),
        })?;

    let line_cart = CartLine::find()
        .filter(cart_line::Column::ProductId.eq(product_id))
        .filter(cart_line::Column::CartId.eq(cart.id))
        .one(db)
        .await?
        .ok_or(crate::Error::EntityNotFound {
            table_name: cart::Entity.table_name(),
        })?;

    let mut cart_line: cart_line::ActiveModel = line_cart.into();

    // Update cart line quantity
    cart_line.quantity = Set(form_data.quantity.to_owned());
    // Update db
    let res = cart_line.update(db).await?;
    Ok(HttpResponse::Ok().json(res))
}

#[delete("/{id}/cart/products/{product_id}")]
async fn remove_product_from_cart(
    data: web::Data<AppState>,
    req: HttpRequest,
) -> crate::Result<HttpResponse> {
    let db = &data.db;
    let id: i32 = req.match_info().query("id").parse()?;
    let product_id: i32 = req.match_info().query("product_id").parse()?;

    let cart = cart::Entity::find()
        .filter(cart::Column::UserId.eq(id))
        .one(db)
        .await?
        .ok_or(crate::Error::EntityNotFound {
            table_name: cart::Entity.table_name(),
        })?;

    let cart_line = CartLine::find()
        .filter(cart_line::Column::ProductId.eq(product_id))
        .filter(cart_line::Column::CartId.eq(cart.id))
        .one(db)
        .await?
        .ok_or(crate::Error::EntityNotFound {
            table_name: cart::Entity.table_name(),
        })?;

    cart_line.delete(db).await?;
    Ok(HttpResponse::Ok().body("Product successfully removed from cart"))
}

#[delete("/{id}/cart")]
async fn empty_cart(data: web::Data<AppState>, req: HttpRequest) -> crate::Result<HttpResponse> {
    let db = &data.db;
    let id: i32 = req.match_info().query("id").parse()?;

    let cart = cart::Entity::find()
        .filter(cart::Column::UserId.eq(id))
        .one(db)
        .await?
        .ok_or(crate::Error::EntityNotFound {
            table_name: cart::Entity.table_name(),
        })?;

    cart_line::Entity::delete_many()
        .filter(cart_line::Column::CartId.eq(cart.id))
        .exec(db)
        .await?;
    Ok(HttpResponse::Ok().body("Cart emptied successfully"))
}
