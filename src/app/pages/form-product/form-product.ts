import { Component, inject } from '@angular/core';
import { AdminMenu } from '../../utils/admin-menu/admin-menu';
import { Router } from '@angular/router';
import { faArrowLeft } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';

@Component({
  selector: 'app-form-product',
  imports: [AdminMenu, FontAwesomeModule],
  templateUrl: './form-product.html',
  styleUrl: './form-product.css'
})
export class FormProduct {

  faArrowLeft = faArrowLeft;

  private router = inject(Router);


  onBack(): void {
    console.log("cc")
    this.router.navigate(['/backoffice/products']);
  }
}
