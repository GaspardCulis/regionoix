import { Component, inject, OnDestroy } from '@angular/core';
import { ProductCardComponent } from '../../utils/component/product-card-component/product-card-component';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { OnInit } from '@angular/core';
import { SnackbarService } from '../../services/snackbar-service';
import { BasketStateService } from '../../services/basket-state-service';
import { ActivatedRoute } from '@angular/router';
import { BasketService, CategoriesService, CategoryDto, ProductDto, ProductsService, RegionDto, RegionsService, TagDto, TagsService } from '../../generated/clients/regionoix-client';
import { Subscription } from 'rxjs';

@Component({
  selector: 'app-showcase',
  standalone: true,
  imports: [CommonModule, ProductCardComponent, FormsModule],
  templateUrl: './showcase-page.html',
  styleUrl: './showcase-page.css'
})

export class ShowcasePage implements OnInit, OnDestroy {
  private readonly snackbar = inject(SnackbarService);
  private readonly route = inject(ActivatedRoute);

  private readonly basketState = inject(BasketStateService);
  private readonly basketService = inject(BasketService);
  private readonly productService = inject(ProductsService);
  private readonly categoriesService = inject(CategoriesService);
  private readonly regionsService = inject(RegionsService);
  private readonly tagsService = inject(TagsService);

  private queryParamSub!: Subscription;


  products!: ProductDto[];
  categories!: CategoryDto[];
  regions!: RegionDto[];
  tags!: TagDto[];

  productAvailable = false;
  productUnavailable = false;
  selectedCategorys: string[] = [];
  selectedRegions: string[] = [];
  selectedTags: string[] = [];
  maxPrice: number | null = null;
  minPrice: number | null = null;

  ngOnInit(): void {
    this.queryParamSub = this.route.queryParamMap.subscribe(() => this.loadProducts());
    this.loadCategories();
    this.loadRegions();
    this.loadTags();
    this.basketState.refreshCount();
  }

  ngOnDestroy(): void {
    this.queryParamSub?.unsubscribe();
  }

  // Load methods
  loadProducts(): void {
    const filters = this.buildFilters();

    if (this.route.snapshot.queryParamMap.has('search')) {
      const search = this.route.snapshot.queryParamMap.get('search') || '';
      this.productService.search(search, filters).subscribe({
        next: (products) => this.products = products,
        error: () => {
          this.snackbar.show("Erreur lors de la récupération des produits", "error");
        }
      });
      return;
    } else {
      this.productService.search("", filters).subscribe({
        next: (data) => this.products = data,
        error: () => {
          this.snackbar.show("Erreur lors de la récupération des produits", "error");
        }
      });
    }
  }

  loadCategories(): void {
    this.categoriesService.get().subscribe({
      next: (data) => {
        console.log("catégories: " + data);
        this.categories = data;
      },
      error: () => {
        this.snackbar.show('Erreur lors de la récupération des catégories', 'error')
        this.categories = [];
      }
    });
  }

  loadRegions(): void {
    this.regionsService.get().subscribe({
      next: (data) => {
        console.log("régions: " + data);
        this.regions = data
      },
      error: () => {
        this.snackbar.show('Erreur lors de la récupération des régions', 'error')
        this.regions = [];
      }
    });
  }

  loadTags(): void {
    this.tagsService.get().subscribe({
      next: (data) => {
        console.log("tags: " + data);
        this.tags = data
      },
      error: () => {
        this.snackbar.show('Erreur lors de la récupération des tags', 'error')
        this.tags = [];
      }
    });
  }

  // Toggle methods
  toggleCategory(categoryName: string, checked: boolean): void {
    if (checked) {
      this.selectedCategorys.push(categoryName);
    } else {
      this.selectedCategorys = this.selectedCategorys.filter(c => c !== categoryName);
    }
    console.log('Selected categories:', this.selectedCategorys);
    this.loadProducts();
  }

  toggleRegion(regionName: string, checked: boolean): void {
    if (checked) {
      this.selectedRegions.push(regionName);
    } else {
      this.selectedRegions = this.selectedRegions.filter(r => r !== regionName);
    }
    console.log('Selected regions:', this.selectedRegions);
    this.loadProducts();
  }

  toggleTag(tagName: string, checked: boolean): void {
    if (checked) {
      this.selectedTags.push(tagName);
    } else {
      this.selectedTags = this.selectedTags.filter(t => t !== tagName);
    }
    console.log('Selected tags:', this.selectedTags);
    this.loadProducts();
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

  private buildFilters(): string {
    const filters: string[] = [];

    // Price filters
    if (this.minPrice !== null) filters.push(`price >= ${this.minPrice}`);
    if (this.maxPrice !== null) filters.push(`price <= ${this.maxPrice}`);

    // Categories
    if (this.selectedCategorys.length > 0) {
      filters.push(
        `(${this.selectedCategorys.map(c => `categories = "${c}"`).join(' OR ')})`
      );
    }

    // Regions
    if (this.selectedRegions.length > 0) {
      filters.push(
        `(${this.selectedRegions.map(r => `region = "${r}"`).join(' OR ')})`
      );
    }

    // Tags
    if (this.selectedTags.length > 0) {
      filters.push(
        `(${this.selectedTags.map(t => `tags = "${t}"`).join(' OR ')})`
      );
    }

    // Availability
    if (this.productAvailable && !this.productUnavailable) filters.push(`stock > 0`);
    if (this.productUnavailable && !this.productAvailable) filters.push(`stock = 0`);

    const filterString = filters.join(' AND ');
    console.log('API filters:', filterString);
    return filterString;
  }
}
