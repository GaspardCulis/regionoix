use crate::prelude::*;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityName as _, EntityTrait, ModelTrait,
    QueryFilter,
};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_basket)
        .service(add_item)
        .service(update_item_quantity)
        .service(remove_item)
        .service(empty);
}

#[utoipa::path()]
#[get("")]
async fn get_basket(data: Data<AppState>, logged_user: LoggedUser) -> crate::Result<HttpResponse> {
    let db = &data.db;

    // Get cart
    let cart = cart::Entity::find()
        .filter(cart::Column::UserId.eq(logged_user.id))
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

#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
struct FormAddToBasket {
    product_id: i32,
    quantity: Option<i32>,
}

#[utoipa::path()]
#[post("/items")]
async fn add_item(
    data: Data<AppState>,
    form_data: Json<FormAddToBasket>,
    logged_user: LoggedUser,
) -> crate::Result<HttpResponse> {
    let db = &data.db;
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
        .filter(cart::Column::UserId.eq(logged_user.id))
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

#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
struct FormUpdateQuantityBasket {
    quantity: i32,
}

#[utoipa::path()]
#[patch("/items/{product_id}")]
async fn update_item_quantity(
    data: Data<AppState>,
    form_data: Json<FormUpdateQuantityBasket>,
    req: HttpRequest,
    logged_user: LoggedUser,
) -> crate::Result<HttpResponse> {
    let db = &data.db;
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
        .filter(cart::Column::UserId.eq(logged_user.id))
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

#[utoipa::path()]
#[delete("/items/{product_id}")]
async fn remove_item(
    data: Data<AppState>,
    req: HttpRequest,
    logged_user: LoggedUser,
) -> crate::Result<HttpResponse> {
    let db = &data.db;
    let product_id: i32 = req.match_info().query("product_id").parse()?;

    let cart = cart::Entity::find()
        .filter(cart::Column::UserId.eq(logged_user.id))
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

#[utoipa::path()]
#[delete("")]
async fn empty(data: Data<AppState>, logged_user: LoggedUser) -> crate::Result<HttpResponse> {
    let db = &data.db;

    let cart = cart::Entity::find()
        .filter(cart::Column::UserId.eq(logged_user.id))
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
