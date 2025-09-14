import { Component, inject, OnDestroy } from '@angular/core';
import { ProductCardComponent } from '../../utils/component/product-card-component/product-card-component';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { OnInit } from '@angular/core';
import { SnackbarService } from '../../services/snackbar-service';
import { BasketStateService } from '../../services/basket-state-service';
import { ActivatedRoute } from '@angular/router';
import { BasketService, ProductDto, ProductsService } from '../../generated/clients/regionoix-client';
import { Subscription } from 'rxjs';

@Component({
  selector: 'app-showcase',
  standalone: true,
  imports: [CommonModule, ProductCardComponent, FormsModule],
  templateUrl: './showcase-page.html',
  styleUrl: './showcase-page.css'
})

export class ShowcasePage implements OnInit, OnDestroy {
  private readonly basketService = inject(BasketService);
  private readonly productService = inject(ProductsService);
  private readonly snackbar = inject(SnackbarService);
  private readonly basketState = inject(BasketStateService);
  private readonly route = inject(ActivatedRoute);
  private queryParamSub!: Subscription;


  products: ProductDto[] = [];
  categories = ['Boissons', 'Fromages', 'Charcuterie', 'Épicerie'];
  regions = ['Sud-Ouest', 'Centre', 'Provence', 'Alsace', 'Normandie'];

  selectedCategory = '';
  selectedRegion = '';
  maxPrice: number | null = null;

  ngOnInit(): void {
    this.queryParamSub = this.route.queryParamMap.subscribe(() => this.loadProducts());
  }

  ngOnDestroy(): void {
    this.queryParamSub?.unsubscribe();
  }

  loadProducts(): void {
    if (this.route.snapshot.queryParamMap.has('search')) {
      const search = this.route.snapshot.queryParamMap.get('search') || '';
      this.productService.search(search).subscribe({
        next: (products) => this.products = products,
        error: () => {
          this.snackbar.show("Erreur lors de la récupération des produits", "error");
        }
      });
      return;
    } else {
      this.productService.get().subscribe({
        next: (data) => this.products = data,
        error: (err) => {
          console.error('Somethings went wrong during products recuperation', err);
        }
      });
    }
  }

  addItem(productId: number) {
    this.basketService.addItem({ product_id: productId, quantity: 1 }).subscribe({
      next: () => {
        this.snackbar.show('Produit ajouté au panier ✅', 'success');
        this.basketState.refreshCount();
      },
      error: () => {
        this.snackbar.show('Stock insuffisant !', 'error');
      }
    });
  }
}
