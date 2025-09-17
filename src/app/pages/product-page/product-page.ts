import { CommonModule } from '@angular/common';
import { Component, inject, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { SnackbarService } from '../../services/snackbar-service';
import { BasketService, LoggedUser, ProductDto, ProductsService } from '../../generated/clients/regionoix-client';
import { BasketStateService } from '../../services/basket-state-service';
import { AuthStateService } from '../../services/auth-state-service';

@Component({
  selector: 'app-product-page',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './product-page.html',
  styleUrls: ['./product-page.css']
})
export class ProductPage implements OnInit {
  product!: ProductDto;
  quantity = 1;
  final_price: number | null = null;

  private basketService = inject(BasketService);
  private basketStateService = inject(BasketStateService);
  private authStateService = inject(AuthStateService);
  private productService = inject(ProductsService);
  private route = inject(ActivatedRoute);
  private snackbarService = inject(SnackbarService);
  user: LoggedUser | null = null;

  ngOnInit() {
    const id = Number(this.route.snapshot.paramMap.get('id'));
    if (id) {
      this.productService.getById(id).subscribe({
        next: (data) => {
          this.product = {
            ...data,
            image: data.image ?? 'assets/default.png'
          };

          if (this.product.discount) {
            this.final_price = this.product.price - (this.product.price * this.product.discount.percentage_off) / 100;
          }

        },
        error: (err) => console.error('Erreur lors de la récupération du produit', err)
      });
    }
  }


  addItem(productId: number) {
    const user = this.authStateService.currentUser;
    if (!user) {
      this.snackbarService.show('Vous devez être connecté pour ajouter au panier.', 'info');
      return;
    }

    this.basketService.add({ product_id: productId, quantity: this.quantity }).subscribe({
      next: () => {
        this.snackbarService.show('Produit ajouté au panier ✅', 'success');
        this.basketStateService.refreshCount();
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