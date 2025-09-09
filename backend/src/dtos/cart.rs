use crate::dtos::cart_line::CartLineDto;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct CartDto {
    pub id: i32,
    pub user_id: i32,
    pub cart_lines: Vec<CartLineDto>,
}
