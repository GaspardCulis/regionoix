import { Routes } from '@angular/router';
import { ShowcasePage } from './pages/showcase/showcase-page';
import { BasketPage } from './pages/basket/basket-page';
import { PaymentPage } from './pages/payment/payment-page';
import { ConnectionPage } from './pages/connection-page/connection-page';
import { ProductPage } from './pages/product-page/product-page';
import { NoAuthGuard } from './services/no-auth-guard';
import { AuthGuard } from './services/auth-guard';
import { ProfilePage } from './pages/profile-page/profile-page';
import { AdminGuard } from './services/auth-admin';
import { BackofficeDashboard } from './pages/backoffice-dashboard/backoffice-dashboard';
import { BackofficeProducts } from './pages/backoffice-products/backoffice-products';
import { FormProduct } from './pages/form-product/form-product';
import { HomePage } from './pages/home-page/home-page';
import { BackofficeProduct } from './pages/backoffice-product/backoffice-product';

export const routes: Routes = [
    { path: '', redirectTo: 'home', pathMatch: 'full' },
    { path: 'showcase', component: ShowcasePage },
    { path: 'basket', component: BasketPage, canActivate: [AuthGuard] },
    { path: 'backoffice', component: BackofficeDashboard, canActivate: [AdminGuard] },
    { path: 'backoffice/products', component: BackofficeProducts, canActivate: [AdminGuard] },
    { path: 'backoffice/create-product', component: FormProduct, canActivate: [AdminGuard] },
    { path: 'backoffice/products/:id', component: BackofficeProduct, canActivate: [AdminGuard] },
    { path: 'payment', component: PaymentPage, canActivate: [AuthGuard] },
    { path: 'connection', component: ConnectionPage, canActivate: [NoAuthGuard] },
    { path: 'profile', component: ProfilePage, canActivate: [AuthGuard] },
    { path: 'products/:id', component: ProductPage },
    { path: 'home', component: HomePage }
];
