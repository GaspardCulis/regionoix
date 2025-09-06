use crate::{
    AppState,
    entities::{cart, cart_line, prelude::CartLine, product},
};
use actix_web::{HttpRequest, HttpResponse, Responder, get, patch, post, web};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, ModelTrait, QueryFilter,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_cart_by_user)
        .service(add_product_to_cart)
        .service(update_quantity_product_cart);
}

#[get("/{id}/cart")]
async fn get_cart_by_user(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let db = &data.db;
    let id: i32 = req.match_info().query("id").parse().unwrap();

    //  Récupérer le panier
    let cart: Option<cart::Model> = cart::Entity::find()
        .filter(cart::Column::UserId.eq(id))
        .one(db)
        .await
        .expect(&format!("Failed to get cart of user id {}", id));

    if let Some(cart) = cart {
        //  Récupérer toutes les lignes liées
        let lines: Vec<cart_line::Model> = cart
            .find_related(cart_line::Entity)
            .all(db)
            .await
            .expect("Failed to load cart lines");

        // 3 Enrichir chaque ligne avec son produit
        #[derive(serde::Serialize)]
        struct LineWithProduct {
            product: product::Model,
            quantity: i32,
        }

        let mut enriched_lines = Vec::new();

        for line in lines {
            if let Some(prod) = line
                .find_related(product::Entity)
                .one(db)
                .await
                .expect("Failed to load product")
            {
                enriched_lines.push(LineWithProduct {
                    product: prod,
                    quantity: line.quantity,
                });
            }
        }

        // Résultat final

        #[derive(serde::Serialize)]
        struct CartWithLines {
            cart: cart::Model,
            lines: Vec<LineWithProduct>,
        }

        HttpResponse::Ok().json(CartWithLines {
            cart,
            lines: enriched_lines,
        })
    } else {
        HttpResponse::NotFound().body("Cart not found")
    }
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
) -> impl Responder {
    let db = &data.db;
    let id: i32 = req.match_info().query("id").parse().unwrap();

    let cart: Option<cart::Model> = cart::Entity::find()
        .filter(cart::Column::UserId.eq(id))
        .one(db)
        .await
        .expect(&format!("Failed to get cart of user id {}", id));

    if let Some(cart) = cart {
        let form_data = form_data.into_inner();

        // Check if product already added in cart
        let cart_line: Option<cart_line::Model> = CartLine::find()
            .filter(cart_line::Column::ProductId.eq(form_data.product_id))
            .filter(cart_line::Column::CartId.eq(cart.id))
            .one(db)
            .await
            .expect("Failed to get cart line");

        if cart_line.is_some() {
            return HttpResponse::BadRequest()
                .body("Product already added to cart, try updating quantity");
        }

        let quantity = if form_data.quantity.is_some() {
            form_data.quantity.unwrap()
        } else {
            1
        };

        let cart_line = cart_line::ActiveModel {
            cart_id: Set(Some(cart.id)),
            product_id: Set(form_data.product_id),
            quantity: Set(quantity),
            ..Default::default()
        };

        cart_line
            .insert(db)
            .await
            .expect("Failed to add product to cart");

        HttpResponse::Ok().body("Product successfully added to cart")
    } else {
        HttpResponse::NotFound().body("Cart not found")
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Quantity {
    quantity: i32,
}

#[patch("/{id}/cart/products/{product_id}")]
async fn update_quantity_product_cart(
    data: web::Data<AppState>,
    form_data: web::Json<Quantity>,
    req: HttpRequest,
) -> impl Responder {
    let db = &data.db;
    let id: i32 = req.match_info().query("id").parse().unwrap();
    let product_id: i32 = req.match_info().query("product_id").parse().unwrap();

    let form_data = form_data.into_inner();

    let cart: Option<cart::Model> = cart::Entity::find()
        .filter(cart::Column::UserId.eq(id))
        .one(db)
        .await
        .expect(&format!("Failed to get cart of user id {}", id));

    if let Some(cart) = cart {
        let line_cart: Option<cart_line::Model> = CartLine::find()
            .filter(cart_line::Column::ProductId.eq(product_id))
            .filter(cart_line::Column::CartId.eq(cart.id))
            .one(db)
            .await
            .expect("Failed");

        let mut cart_line: cart_line::ActiveModel = line_cart.unwrap().into();

        // Update cart line quantity
        cart_line.quantity = Set(form_data.quantity.to_owned());
        // Update db
        cart_line
            .update(db)
            .await
            .expect("Failed to update cart line");
        HttpResponse::Ok().body("Updated quantity of product in cart")
    } else {
        HttpResponse::NotFound().body("Cart not found")
    }
}
