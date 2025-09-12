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

  selectedCategory = '';
  selectedRegion = '';
  maxPrice: number | null = null;

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

  get filteredProducts() {
    return this.products.filter(p => {
      return (
        (!this.selectedCategory || p.category.name === this.selectedCategory) &&
        (!this.selectedRegion || p.region.name === this.selectedRegion) &&
        (!this.maxPrice || p.price <= this.maxPrice)
      );
    });
  }
}
