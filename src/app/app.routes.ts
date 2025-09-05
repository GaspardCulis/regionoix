import { Routes } from '@angular/router';
import { ShowcasePage } from './pages/showcase/showcase-page';
import { BasketPage } from './pages/basket/basket-page';

export const routes: Routes = [
    { path: '', redirectTo: 'showcase', pathMatch: 'full' },
    { path: 'showcase', component: ShowcasePage },
    { path: 'basket', component: BasketPage }
];
