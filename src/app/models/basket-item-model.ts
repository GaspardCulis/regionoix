import { Product } from "./product-model";


export interface BasketItem extends Product {
    quantity: number;
}
