import { Routes } from '@angular/router';
import { TestComponent } from './component/test-component/test-component';
import { Showcase } from './pages/showcase/showcase-page';

export const routes: Routes = [
    { path: '', component: TestComponent },
    { path: '**', redirectTo: '' },

    { path: 'showcase', component: Showcase }
];
