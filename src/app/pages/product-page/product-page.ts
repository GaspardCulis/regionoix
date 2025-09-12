import { CommonModule } from '@angular/common';
import { Component, inject, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { Product } from '../../models/product-model';
import { BasketService } from '../../services/basket-service';
import { ProductService } from '../../services/product-service';
import { SnackbarService } from '../../services/snackbar-service';

@Component({
  selector: 'app-product-page',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './product-page.html',
  styleUrls: ['./product-page.css']
})
export class ProductPage implements OnInit {
  product!: Product;
  quantity = 1;

  private basketService = inject(BasketService);
  private productService = inject(ProductService);
  private route = inject(ActivatedRoute);
  private snackbarService = inject(SnackbarService);

  ngOnInit() {
    const id = this.route.snapshot.paramMap.get('id');
    if (id) {
      this.productService.getProductById(id).subscribe({
        next: (data) => {
          this.product = {
            ...data,
            image: data.image ?? 'assets/default.png'
          };
        },
        error: (err) => console.error('Erreur lors de la récupération du produit', err)
      });
    }
  }

  addItem(productId: number) {
    this.basketService.addItem(productId, this.quantity).subscribe({
      next: () => {
        this.snackbarService.show('Produit ajouté au panier ✅', 'success');
      },
      error: (err) => {
        console.error(err);
        this.snackbarService.show('Stock insuffisant !', 'error');
      }
    });

  }

  increaseQuantity() {
    if (this.product && this.quantity < this.product.stock) {
      this.quantity++;
    }
  }

  decreaseQuantity() {
    if (this.quantity > 1) {
      this.quantity--;
    }
  }
}