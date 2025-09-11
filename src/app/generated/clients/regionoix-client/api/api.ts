export * from './authentification.service';
import { AuthentificationService } from './authentification.service';
export * from './basket.service';
import { BasketService } from './basket.service';
export * from './products.service';
import { ProductsService } from './products.service';
export const APIS = [AuthentificationService, BasketService, ProductsService];
