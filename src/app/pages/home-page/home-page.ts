import { Component, inject, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ProductsService, ProductDto, CategoryDto, LoggedUser, BasketService } from '../../generated/clients/regionoix-client';
import { ProductCardComponent } from '../../utils/component/product-card-component/product-card-component';
import { Router } from '@angular/router';
import { AuthStateService } from '../../services/auth-state-service';
import { SnackbarService } from '../../services/snackbar-service';
import { BasketStateService } from '../../services/basket-state-service';

@Component({
  selector: 'app-home-page',
  standalone: true,
  imports: [CommonModule, ProductCardComponent],
  templateUrl: './home-page.html',
  styleUrl: './home-page.css',
})
export class HomePage implements OnInit {
  private readonly snackbar = inject(SnackbarService);

  private readonly productService = inject(ProductsService);
  private readonly authStateService = inject(AuthStateService);
  private readonly basketService = inject(BasketService);
  private readonly basketState = inject(BasketStateService);

  private readonly router = inject(Router);
  promotionalProducts: ProductDto[] = [];
  newProducts: ProductDto[] = [];
  bestProducts: ProductDto[] = [];
  topCategories: CategoryDto[] = [];
  user: null | LoggedUser = null;


  currentIndex = 0;

  activeTab: 'promotion' | 'nouveaute' | 'bestSellers' = 'promotion';

  ngOnInit(): void {
    this.loadPromotionalProducts();
    this.loadNewProducts();
    this.loadBestProducts();
    this.basketState.refreshCount();
    this.user = this.authStateService.currentUser;
  }

  addItem(productId: number) {
    const user = this.authStateService.currentUser;
    if (!user) {
      this.snackbar.show('Vous devez vous connecter pour ajouter au panier.', 'info');
      return;
    }
    this.basketService.add({ product_id: productId, quantity: 1 }).subscribe({
      next: () => {
        this.snackbar.show('Produit ajouté au panier ✅', 'success');
        this.basketState.refreshCount();
      },
      error: () => {
        this.snackbar.show('Stock insuffisant !', 'error');
      }
    });
  }

  loadPromotionalProducts() {
    this.productService.getDiscounts(4, 0).subscribe({
      next: (products) => (this.promotionalProducts = products),
      error: () => console.error('Erreur chargement promotions'),
    });
  }

  loadBestProducts() {
    this.productService.search('', 'tags = "Best-seller"').subscribe({
      next: (products) => (this.bestProducts = products),
      error: () => console.error('Erreur chargement best-sellers'),
    });
  }

  loadNewProducts() {
    this.productService.search('', 'tags = "nouveauté"').subscribe({
      next: (products) => (this.newProducts = products),
      error: () => console.error('Erreur chargement nouveautés'),
    });
  }

  goToProduct(id: number) {
    this.router.navigate(['/products/', id]);
  }

  goToShowcase() {
    this.router.navigate(['/showcase']);
  }
}
