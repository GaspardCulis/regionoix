import { Component, inject, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ProductsService, ProductDto, CategoriesService, CategoryDto } from '../../generated/clients/regionoix-client';
import { ProductCardComponent } from '../../utils/component/product-card-component/product-card-component';

@Component({
  selector: 'app-home-page',
  standalone: true,
  imports: [CommonModule, ProductCardComponent],
  templateUrl: './home-page.html',
  styleUrl: './home-page.css'
})
export class HomePage implements OnInit {
  private productService = inject(ProductsService);
  private categoriesService = inject(CategoriesService);

  promotionalProducts: ProductDto[] = [];
  newProducts: ProductDto[] = [];
  topCategories: CategoryDto[] = [];

  currentIndex = 0;

  ngOnInit(): void {
    this.loadPromotionalProducts();
    this.loadNewProducts();
    this.loadTopCategories();
  }

  loadPromotionalProducts() {
    this.productService.search('', 'discount').subscribe({
      next: (products) => this.promotionalProducts = products,
      error: () => console.error('Erreur chargement promotions')
    });
  }

  loadNewProducts() {
    this.productService.search('', 'tags = "nouveauté"').subscribe({
      next: (products) => this.newProducts = products,
      error: () => console.error('Erreur chargement nouveautés')
    });
  }

  loadTopCategories() {
    this.categoriesService.get().subscribe({
      next: (cats) => this.topCategories = cats.filter(c => !c.category_parent),
      error: () => console.error('Erreur chargement catégories')
    });
  }

  nextSlide() {
    if (this.currentIndex < this.promotionalProducts.length - 1) this.currentIndex++;
    this.updateTrack();
  }

  prevSlide() {
    if (this.currentIndex > 0) this.currentIndex--;
    this.updateTrack();
  }

  updateTrack() {
    const track = document.querySelector<HTMLElement>('.carousel-track');
    if (track) {
      const offset = this.currentIndex * 260; // largeur produit + gap
      track.style.transform = `translateX(-${offset}px)`;
    }
  }

}
