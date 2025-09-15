import { Routes } from '@angular/router';
import { ShowcasePage } from './pages/showcase/showcase-page';
import { BasketPage } from './pages/basket/basket-page';
import { PaymentPage } from './pages/payment/payment-page';
import { ConnectionPage } from './pages/connection-page/connection-page';
import { ProductPage } from './pages/product-page/product-page';
import { NoAuthGuard } from './services/no-auth-guard';
import { AuthGuard } from './services/auth-guard';
import { AdminGuard } from './services/auth-admin';
import { BackofficeDashboard } from './pages/backoffice-dashboard/backoffice-dashboard';
import { BackofficeProducts } from './pages/backoffice-products/backoffice-products';
import { FormProduct } from './pages/form-product/form-product';

export const routes: Routes = [
    { path: '', redirectTo: 'showcase', pathMatch: 'full' },
    { path: 'showcase', component: ShowcasePage },
    { path: 'basket', component: BasketPage, canActivate: [AuthGuard] },
    { path: 'backoffice', component: BackofficeDashboard, canActivate: [AdminGuard] },
    { path: 'backoffice/products', component: BackofficeProducts, canActivate: [AdminGuard] },
    { path: 'backoffice/create-product', component: FormProduct, canActivate: [AdminGuard] },
    { path: 'payment', component: PaymentPage, canActivate: [AuthGuard] },
    { path: 'connection', component: ConnectionPage, canActivate: [NoAuthGuard] },
    { path: 'products/:id', component: ProductPage }
];
