use crate::{
    AppState,
    dtos::{cart::CartDto, cart_line::CartLineDto},
    prelude::{sea_orm_active_enums::OrderStatus, *},
    routes::auth::LoggedUser,
};
use actix_web::web;
use chrono::{Duration, Utc};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, EntityName as _, EntityTrait, ModelTrait, PaginatorTrait as _, QueryFilter,
    QueryOrder, QuerySelect, TransactionTrait,
};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_basket)
        .service(add_item)
        .service(update_item_quantity)
        .service(remove_item)
        .service(empty)
        .service(make_order)
        .service(get_count);
}

#[utoipa::path(
    summary = "Returns basket details of current user",
    tag="Basket",
    responses(
      (
          status = 200,
          description="Basket details successfully returned",
          content_type = "application/json",
          body=CartDto,
      ),
))]
#[get("")]
async fn get_basket(data: Data<AppState>, logged_user: LoggedUser) -> crate::Result<HttpResponse> {
    let db = &data.db;
    let basket = cart::Entity::find()
        .filter(cart::Column::UserId.eq(logged_user.id))
        .into_dto::<CartDto>()
        .one(&db.conn)
        .await?
        .ok_or(crate::Error::EntityNotFound {
            table_name: cart::Entity.table_name(),
        })?
        .finalize(&db.conn)
        .await?;

    Ok(HttpResponse::Ok().json(basket))
}

#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
struct FormAddToBasket {
    product_id: i32,
    quantity: Option<i32>,
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
async fn add_item(
    data: Data<AppState>,
    form_data: Json<FormAddToBasket>,
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

#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
struct FormUpdateQuantityBasket {
    quantity: i32,
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
async fn update_item_quantity(
    data: Data<AppState>,
    form_data: Json<FormUpdateQuantityBasket>,
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
async fn remove_item(
    data: Data<AppState>,
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

#[utoipa::path(
    summary="Empty current user's basket",
    tag="Basket",
    responses(
        (
            status = 200,
            description="Basket successfully emptied",
        ),
    ),
)]
#[delete("")]
async fn empty(data: Data<AppState>, logged_user: LoggedUser) -> crate::Result<HttpResponse> {
    let db = &data.db;

    let cart = cart::Entity::find()
        .filter(cart::Column::UserId.eq(logged_user.id))
        .one(&db.conn)
        .await?
        .ok_or(crate::Error::EntityNotFound {
            table_name: cart::Entity.table_name(),
        })?;

    cart_line::Entity::delete_many()
        .filter(cart_line::Column::CartId.eq(cart.id))
        .exec(&db.conn)
        .await?;
    Ok(HttpResponse::Ok().body("Cart emptied successfully"))
}

#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
struct FormDataMakeOrder {
    firstname: String,
    lastname: String,
    city: String,
    country: String,
    postal_code: String,
    street: String,
}

#[utoipa::path(
    summary = "Make order from current user's basket",
    tag="Basket",
    request_body(content_type = "Application/Json",
    content = FormDataMakeOrder),
    responses(
        (
            status = 200,
            description="Order successfully created",
            body=String,
        ),
    ),
)]
#[post("/order")]
async fn make_order(
    data: web::Data<AppState>,
    form_data: web::Json<FormDataMakeOrder>,
    logged_user: LoggedUser,
) -> crate::Result<HttpResponse> {
    const DELIVERY_DAYS: i64 = 4;
    let db = &data.db;
    let txn = db.begin().await?;

    // Get cart
    let cart = cart::Entity::find()
        .filter(cart::Column::UserId.eq(logged_user.id))
        .one(&txn)
        .await?
        .ok_or(crate::Error::EntityNotFound {
            table_name: cart::Entity.table_name(),
        })?;

    // Get cart lines ordered to avoid deadlock
    let cart_lines: Vec<cart_line::Model> = cart
        .find_related(cart_line::Entity)
        .order_by_asc(cart_line::Column::ProductId)
        .all(&txn)
        .await?;

    if cart_lines.is_empty() {
        return Ok(HttpResponse::BadRequest().body("Cart is empty"));
    }

    let mut total_price = 0.0;
    let mut order_lines = Vec::new();

    // Create Order lines
    for cl in cart_lines {
        // Lock on the read of product to avoid dirty read
        let product = product::Entity::find_by_id(cl.product_id)
            .lock_exclusive()
            .one(&txn)
            .await?
            .ok_or(crate::Error::EntityNotFound {
                table_name: product::Entity.table_name(),
            })?;

        // Order line
        let ol = order_line::ActiveModel {
            id: NotSet,
            quantity: Set(cl.quantity),
            unit_price: Set(product.price),
            product_id: Set(product.id.into()),
            order_id: NotSet,
        };
        order_lines.push(ol);

        // Total
        total_price += product.price * cl.quantity as f32;

        // if product stock is not enough rollback
        if product.stock < cl.quantity {
            txn.rollback().await?;
            return Ok(HttpResponse::InternalServerError().body("Not enough stock"));
        }

        // Decrement stock
        let mut product_am: product::ActiveModel = product.into();
        product_am.stock = Set(product_am.stock.unwrap() - cl.quantity);
        product_am.update(&txn).await?;
    }

    // Create address
    let addr = address::ActiveModel {
        id: NotSet,
        firstname: Set(form_data.firstname.to_owned()),
        lastname: Set(form_data.lastname.to_owned()),
        city: Set(form_data.city.to_owned()),
        country: Set(form_data.country.to_owned()),
        street: Set(form_data.street.to_owned()),
        postal_code: Set(form_data.postal_code.to_owned()),
    };
    let addr = addr.insert(&txn).await?;

    // Order dates
    let today = Utc::now().date_naive();
    let arrival = (Utc::now() + Duration::days(DELIVERY_DAYS)).date_naive();

    // Create command commande
    let order = order::ActiveModel {
        id: NotSet,
        total_price: Set(total_price.to_owned()),
        status: Set(OrderStatus::Payed),
        creation_date: Set(today.into()),
        arrival_date: Set(arrival.into()),
        user_id: Set(logged_user.id.into()),
        adress_id: Set(addr.id.into()),
        ..Default::default()
    };
    let order = order.insert(&txn).await?;

    // For all order_lines insert order ids
    for mut ol in order_lines {
        ol.order_id = Set(order.id.into());
        ol.insert(&txn).await?;
    }

    // Empty cart
    cart_line::Entity::delete_many()
        .filter(cart_line::Column::CartId.eq(cart.id))
        .exec(&txn)
        .await?;

    txn.commit().await?;

    Ok(HttpResponse::Ok().body("Order successfully created"))
}

#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
struct CountBasket {
    count: i32,
}

#[utoipa::path(
    summary = "Returns basket count of current user",
    description = "Returns count of differents products in cart of current user.",
    tag="Basket",
    responses(
      (
          status = 200,
          description="Basket count successfully returned",
          content_type = "application/json",
          body=CountBasket,
      ),
))]
#[get("/count")]
async fn get_count(data: Data<AppState>, logged_user: LoggedUser) -> crate::Result<HttpResponse> {
    let db = &data.db;

    let cart = cart::Entity::find()
        .filter(cart::Column::UserId.eq(logged_user.id))
        .one(&db.conn)
        .await?
        .ok_or(crate::Error::EntityNotFound {
            table_name: cart::Entity.table_name(),
        })?;

    // Get count of all cart lines with at least quantity of 1
    let count = CartLine::find()
        .filter(cart_line::Column::CartId.eq(cart.id))
        .filter(cart_line::Column::Quantity.gte(1))
        .count(&db.conn)
        .await? as i32;

    Ok(HttpResponse::Ok().json(CountBasket { count }))
}
