import { Routes } from '@angular/router';
import { Showcase } from './pages/showcase/showcase.page';
import { ConnexionPage } from './pages/connexion-page/connexion-page';

export const routes: Routes = [
    { path: '', redirectTo: 'showcase', pathMatch: 'full' },
    { path: 'showcase', component: Showcase },
    { path: 'connexion', component: ConnexionPage }
];
