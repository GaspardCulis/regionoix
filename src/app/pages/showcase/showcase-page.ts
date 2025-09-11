import { Component, inject } from '@angular/core';
import { ProductCardComponent } from '../../utils/component/product-card-component/product-card-component';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { OnInit } from '@angular/core';
import { Product } from '../../models/product-model';
import { BasketService } from '../../services/basket-service';
import { ProductService } from '../../services/product-service';
import { SnackbarService } from '../../services/snackbar-service';
import { BasketStateService } from '../../services/basket-state-service';

@Component({
  selector: 'app-showcase',
  standalone: true,
  imports: [CommonModule, ProductCardComponent, FormsModule],
  templateUrl: './showcase-page.html',
  styleUrl: './showcase-page.css'
})

export class ShowcasePage implements OnInit {
  private readonly basketService = inject(BasketService);
  private readonly productService = inject(ProductService);
  private readonly snackbarService = inject(SnackbarService);
  private readonly basketState = inject(BasketStateService);

  products: Product[] = [];
  categories = ['Boissons', 'Fromages', 'Charcuterie', 'Épicerie'];
  regions = ['Sud-Ouest', 'Centre', 'Provence', 'Alsace', 'Normandie'];
  tags = ['vegan', 'bio', 'nouveauté', 'végétarien'];

  selectedCategory = '';
  selectedRegion = '';
  selectedTags = '';
  filterAvailable = false;
  filterUnavailable = false;

  minPrice = 0;
  maxPrice = 500;

  ngOnInit(): void {
    this.productService.getProducts().subscribe({
      next: (data) => this.products = data,
      error: (err) => {
        console.error('Somethings went wrong during products recuperation', err);
      }
    });
  }

  addItem(productId: number) {
    this.basketService.addItem(productId, 1).subscribe({
      next: () => {
        this.snackbarService.show('Produit ajouté au panier ✅', 'success');
        this.basketState.refreshCount();
      },
      error: () => {
        this.snackbarService.show('Stock insuffisant !', 'error');
      }
    });
  }

  updatePriceRange() {
    if (this.minPrice > this.maxPrice) {
      const temp = this.minPrice;
      this.minPrice = this.maxPrice;
      this.maxPrice = temp;
    }
  }


  get filteredProducts() {
    return this.products.filter((p) => {
      // price
      if (p.price < this.minPrice || p.price > this.maxPrice) return false;

      // categories
      if (this.selectedCategory && p.category.name !== this.selectedCategory) return false;

      // regions
      if (this.selectedRegion && p.region.name !== this.selectedRegion) return false;

      // tags
      //TODO

      // availabilty
      if (this.filterAvailable && !p.stock) return false;
      if (this.filterUnavailable && p.stock) return false;

      return true;
    });
  }
}
