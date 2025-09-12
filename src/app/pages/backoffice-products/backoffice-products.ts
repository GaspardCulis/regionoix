import { Component, inject, OnInit } from '@angular/core';
import { Product } from '../../models/product-model';
import { ProductService } from '../../services/product-service';
import { AdminMenu } from "../../utils/admin-menu/admin-menu";
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';
import { faPlus } from '@fortawesome/free-solid-svg-icons';
import { Router } from '@angular/router';

@Component({
  selector: 'app-backoffice-products',
  imports: [AdminMenu, FontAwesomeModule],
  templateUrl: './backoffice-products.html',
  styleUrl: './backoffice-products.css'
})
export class BackofficeProducts implements OnInit {
  faPlus = faPlus;

  products: Product[] = [];
  private readonly productService = inject(ProductService);
  private readonly router = inject(Router);

  ngOnInit(): void {
    this.productService.getProducts().subscribe({
      next: (data) => this.products = data,
      error: (err) => {
        console.error('Somethings went wrong during products recuperation', err);
      }
    });
  }

  onCreateProduct(): void {
    this.router.navigate(["/backoffice/create-product"]);
  }

}
