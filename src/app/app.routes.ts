import { Routes } from '@angular/router';
import { Showcase } from './pages/showcase/showcase.page';

export const routes: Routes = [
    { path: '', redirectTo: 'showcase', pathMatch: 'full' },
    { path: 'showcase', component: Showcase }
];
