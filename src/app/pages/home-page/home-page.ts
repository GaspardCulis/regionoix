import { Component, inject, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ProductsService, ProductDto, CategoryDto } from '../../generated/clients/regionoix-client';
import { ProductCardComponent } from '../../utils/component/product-card-component/product-card-component';
import { Router } from '@angular/router';

@Component({
  selector: 'app-home-page',
  standalone: true,
  imports: [CommonModule, ProductCardComponent],
  templateUrl: './home-page.html',
  styleUrl: './home-page.css',
})
export class HomePage implements OnInit {
  private readonly productService = inject(ProductsService);
  private readonly router = inject(Router);
  promotionalProducts: ProductDto[] = [];
  newProducts: ProductDto[] = [];
  bestProducts: ProductDto[] = [];
  topCategories: CategoryDto[] = [];

  currentIndex = 0;

  activeTab: 'promotion' | 'nouveaute' | 'bestSellers' = 'promotion';

  ngOnInit(): void {
    this.loadPromotionalProducts();
    this.loadNewProducts();
    this.loadBestProducts();
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
