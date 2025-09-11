import { Product } from "./product-model";

export interface BasketLine {
    product: Product;
    quantity: number;
}

export interface BasketResponse {
    cart: { id: number; user_id: number };
    lines: BasketLine[];
}