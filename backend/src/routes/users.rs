use crate::{
    AppState,
    entities::{cart, cart_line, product},
};
use actix_web::{HttpRequest, HttpResponse, Responder, get, web};
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_cart_by_user);
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
