import { Product } from '../../utils/model/product-model';

export interface BasketItem extends Product {
    quantity: number;
}
