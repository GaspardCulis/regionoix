import { Component, inject, OnDestroy, OnInit } from '@angular/core';
import { ProductCardComponent } from '../../utils/component/product-card-component/product-card-component';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { SnackbarService } from '../../services/snackbar-service';
import { BasketStateService } from '../../services/basket-state-service';
import { ActivatedRoute, Router } from '@angular/router';
import {
  BasketService,
  BrandDto,
  BrandsService,
  CategoriesService,
  CategoryDto,
  LoggedUser,
  ProductDto,
  ProductsService,
  RegionDto,
  RegionsService,
  TagDto,
  TagsService
} from '../../generated/clients/regionoix-client';
import { Subscription } from 'rxjs';
import { AuthStateService } from '../../services/auth-state-service';
import { FilterDropdownComponent } from '../../utils/component/filter-dropdown-component/filter-dropdown-component';

@Component({
  selector: 'app-showcase',
  standalone: true,
  imports: [CommonModule, ProductCardComponent, FormsModule, FilterDropdownComponent],
  templateUrl: './showcase-page.html',
  styleUrl: './showcase-page.css'
})
export class ShowcasePage implements OnInit, OnDestroy {
  private readonly snackbar = inject(SnackbarService);
  private readonly route = inject(ActivatedRoute);
  private readonly router = inject(Router);

  private readonly basketState = inject(BasketStateService);
  private readonly basketService = inject(BasketService);
  private readonly productService = inject(ProductsService);
  private readonly categoriesService = inject(CategoriesService);
  private readonly regionsService = inject(RegionsService);
  private readonly tagsService = inject(TagsService);
  private readonly authStateService = inject(AuthStateService);
  private readonly brandsService = inject(BrandsService);

  private queryParamSub!: Subscription;

  products: ProductDto[] = [];
  categories: CategoryDto[] = [];
  regions: RegionDto[] = [];
  tags: TagDto[] = [];
  brands: BrandDto[] = [];
  user: null | LoggedUser = null;

  // For dropdown components
  categoriesState: [string, boolean][] = [];
  regionsState: [string, boolean][] = [];
  propertiesState: [string, boolean][] = [];
  brandsState: [string, boolean][] = [];
  sortsState: [string, boolean][] = [];
  // Filters
  maxPrice: number | null = null;
  minPrice: number | null = null;

  // Pagination
  currentPage = 1;
  pageSize = 20;

  ngOnInit(): void {
    this.queryParamSub = this.route.queryParamMap.subscribe(() => this.loadProducts());
    this.loadCategories();
    this.loadRegions();
    this.loadTags();
    this.loadBrands();
    this.loadSorts();
    this.basketState.refreshCount();
    this.user = this.authStateService.currentUser;
    this.loadProducts();
  }

  ngOnDestroy(): void {
    this.queryParamSub?.unsubscribe();
  }

  // ---- Loading methods ----
  loadProducts(): void {
    const filters = this.buildFilters();
    const sorts = this.buildSorts();

    const queryParams = this.route.snapshot.queryParamMap;
    const search = queryParams.get('search') || '';

    this.productService.search(search, filters || undefined, sorts || undefined, this.pageSize, this.currentPage)
      .subscribe({
        next: (products) => this.products = products,
        error: () => this.snackbar.show('Erreur lors de la récupération des produits', 'error')
      });
  }


  loadCategories(): void {
    this.categoriesService.get().subscribe({
      next: (data) => {
        this.categories = data;
        this.categoriesState = data.map(c => [c.name, false]);
      },
      error: () => {
        this.snackbar.show('Erreur lors de la récupération des catégories', 'error');
        this.categoriesState = [];
      }
    });
  }

  loadRegions(): void {
    this.regionsService.get().subscribe({
      next: (data) => {
        this.regions = data;
        this.regionsState = data.map(r => [r.name, false]);
      },
      error: () => {
        this.snackbar.show('Erreur lors de la récupération des régions', 'error');
        this.regionsState = [];
      }
    });
  }

  loadTags(): void {
    this.tagsService.get().subscribe({
      next: (data) => {
        this.tags = data;
        this.propertiesState = data.map(t => [t.name, false]);
      },
      error: () => {
        this.snackbar.show('Erreur lors de la récupération des tags', 'error');
        this.propertiesState = [];
      }
    });
  }

  loadBrands(): void {
    this.brandsService.get().subscribe({
      next: (data) => {
        this.brands = data;
        this.brandsState = data.map(b => [b.name, false]);
      },
      error: () => {
        this.snackbar.show('Erreur lors de la récupération des marques', 'error');
        this.brandsState = [];
      }
    });
  }

  loadSorts(): void {
    this.sortsState = [
      ["Nom A-Z", false],
      ["Nom Z-A", false],
      ["Prix croissant", false],
      ["Prix décroissant", false],
      ["Moins lourd", false],
      ["Plus lourd", false],
    ]
  }

  // ---- Handlers for dropdown selections ----
  onCategoriesChange({ name, checked }: { name: string; checked: boolean }) {
    this.categoriesState = this.categoriesState.map(([opt, state]) =>
      opt === name ? [opt, checked] : [opt, state]
    );
    this.loadProducts();
  }

  onRegionsChange({ name, checked }: { name: string; checked: boolean }) {
    this.regionsState = this.regionsState.map(([opt, state]) =>
      opt === name ? [opt, checked] : [opt, state]
    );
    this.loadProducts();
  }

  onTagChange({ name, checked }: { name: string; checked: boolean }) {
    this.propertiesState = this.propertiesState.map(([opt, state]) =>
      opt === name ? [opt, checked] : [opt, state]
    );
    this.loadProducts();
  }

  onBrandsChange({ name, checked }: { name: string; checked: boolean }) {
    this.brandsState = this.brandsState.map(([opt, state]) =>
      opt === name ? [opt, checked] : [opt, state]
    );
    this.loadProducts();
  }

  onSortsChange({ name, checked }: { name: string; checked: boolean }) {
    this.sortsState = this.sortsState.map(([opt]) => [opt, opt === name && checked]);

    this.loadProducts();
  }



  addItem(productId: number) {
    const user = this.authStateService.currentUser;
    if (!user) {
      this.snackbar.show('Veuillez vous connecter pour ajouter au panier !', 'error');
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

  // ---- Filters ----
  private buildFilters(): string {
    const filters: string[] = [];

    if (this.minPrice !== null) filters.push(`price >= ${this.minPrice}`);
    if (this.maxPrice !== null) filters.push(`price <= ${this.maxPrice}`);

    const selectedCategories = this.categoriesState.filter(([_, checked]) => checked).map(([name]) => name);
    if (selectedCategories.length > 0) {
      filters.push(`(${selectedCategories.map(c => `categories = "${c}"`).join(' OR ')})`);
    }

    const selectedRegions = this.regionsState.filter(([_, checked]) => checked).map(([name]) => name);
    if (selectedRegions.length > 0) {
      filters.push(`(${selectedRegions.map(r => `region_name = "${r}"`).join(' OR ')})`);
    }

    const selectedTags = this.propertiesState.filter(([_, checked]) => checked).map(([name]) => name);
    if (selectedTags.length > 0) {
      filters.push(`(${selectedTags.map(t => `tags = "${t}"`).join(' OR ')})`);
    }

    const selectedBrands = this.brandsState.filter(([_, checked]) => checked).map(([name]) => name);
    if (selectedBrands.length > 0) {
      filters.push(`(${selectedBrands.map(b => `brand_name = "${b}"`).join(' OR ')})`);
    }

    return filters.join(' AND ');
  }

  private buildSorts(): string {
    if (!this.sortsState || this.sortsState.length === 0) return '';

    const sortMap: { [key: string]: string } = {
      'Nom A-Z': 'name:asc',
      'Nom Z-A': 'name:desc',
      'Prix croissant': 'price:asc',
      'Prix décroissant': 'price:desc',
      'Moins lourd': 'weight:asc',
      'Plus lourd': 'weight:desc',
    };

    const selected = this.sortsState.filter(([_, checked]) => checked).map(([label]) => label);

    if (selected.length === 0) return '';

    const fieldMap: { [key: string]: string } = {};

    for (const label of selected) {
      const [field, direction] = sortMap[label].split(':');

      if (fieldMap[field]) {
        fieldMap[field] = direction;
      } else {
        fieldMap[field] = direction;
      }
    }

    return Object.entries(fieldMap).map(([field, dir]) => `${field}:${dir}`).join(',');
  }

  resetFilters(): void {
    this.categoriesState = this.categoriesState.map(([opt]) => [opt, false]);
    this.regionsState = this.regionsState.map(([opt]) => [opt, false]);
    this.propertiesState = this.propertiesState.map(([opt]) => [opt, false]);
    this.brandsState = this.brandsState.map(([opt]) => [opt, false]);
    this.minPrice = null;
    this.maxPrice = null;
    this.currentPage = 1;
    this.loadProducts();
  }

  onPageSizeChange(): void {
    this.currentPage = 1;
    this.loadProducts();
  }

  nextPage(): void {
    if (this.products.length >= this.pageSize) {
      this.currentPage++;
      this.loadProducts();
    }
  }

  prevPage(): void {
    if (this.currentPage > 1) {
      this.currentPage--;
      this.loadProducts();
    }
  }
}
