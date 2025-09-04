import { Routes } from '@angular/router';
import { Showcase } from './pages/showcase/showcase.page';

export const routes: Routes = [
    { path: '', component: TestComponent },
    { path: '**', redirectTo: '' },

    { path: 'showcase', component: Showcase }
];
