import { Component, inject, OnInit } from '@angular/core';
import { Product } from '../../models/product-model';
import { ProductService } from '../../services/product-service';
import { AdminMenu } from "../../utils/admin-menu/admin-menu";

@Component({
  selector: 'app-backoffice-products',
  imports: [AdminMenu],
  templateUrl: './backoffice-products.html',
  styleUrl: './backoffice-products.css'
})
export class BackofficeProducts implements OnInit {
  products: Product[] = [];
  private readonly productService = inject(ProductService);


  ngOnInit(): void {
    this.productService.getProducts().subscribe({
      next: (data) => this.products = data,
      error: (err) => {
        console.error('Somethings went wrong during products recuperation', err);
      }
    });
  }

}
