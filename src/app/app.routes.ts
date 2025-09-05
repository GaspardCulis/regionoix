import { Routes } from '@angular/router';
import { ShowcasePage } from './pages/showcase/showcase-page';
import { BasketPage } from './pages/basket/basket-page';
import { PaymentPage } from './pages/payment/payment-page';
import { ConnectionPage } from './pages/connection-page/connection-page';

export const routes: Routes = [
    { path: '', redirectTo: 'showcase', pathMatch: 'full' },
    { path: 'showcase', component: ShowcasePage },
    { path: 'basket', component: BasketPage },
    { path: 'payment', component: PaymentPage },
    { path: 'connection', component: ConnectionPage }
];
