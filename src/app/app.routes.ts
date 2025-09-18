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
import { CreateAccount } from './pages/create-account/create-account';
import { PaymentError } from './pages/payment-error/payment-error';
import { PaymentSuccessful } from './pages/payment-successful/payment-successful';
import { BasketNotEmptyGuard } from './services/basket-not-empty-guard';
import { Cgv } from './pages/cgv/cgv';

export const routes: Routes = [
  { path: '', redirectTo: 'home', pathMatch: 'full' },
  { path: 'showcase', component: ShowcasePage },
  { path: 'basket', component: BasketPage, canActivate: [AuthGuard] },
  { path: 'backoffice', component: BackofficeDashboard, canActivate: [AdminGuard] },
  { path: 'backoffice/products', component: BackofficeProducts, canActivate: [AdminGuard] },
  { path: 'backoffice/create-product', component: FormProduct, canActivate: [AdminGuard] },
  { path: 'backoffice/products/:id', component: BackofficeProduct, canActivate: [AdminGuard] },
  { path: 'payment', component: PaymentPage, canActivate: [AuthGuard, BasketNotEmptyGuard] },
  { path: 'connection', component: ConnectionPage, canActivate: [NoAuthGuard] },
  { path: 'profile', component: ProfilePage, canActivate: [AuthGuard] },
  { path: 'products/:id', component: ProductPage },
  { path: 'home', component: HomePage },
  { path: 'create-account', component: CreateAccount, canActivate: [NoAuthGuard] },
  { path: 'error-payment', component: PaymentError },
  { path: 'payment-successful', component: PaymentSuccessful },
  { path: 'cgv', component: Cgv },
];
