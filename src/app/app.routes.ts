import { Routes } from '@angular/router';
import { TestComponent } from './component/test-component/test-component';

export const routes: Routes = [
    {path: '', component: TestComponent },
    {path: '**', redirectTo: '' }
];
