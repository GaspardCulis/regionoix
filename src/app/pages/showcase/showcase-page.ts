import { Component, inject, OnDestroy } from '@angular/core';
import { ProductCardComponent } from '../../utils/component/product-card-component/product-card-component';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { OnInit } from '@angular/core';
import { SnackbarService } from '../../services/snackbar-service';
import { BasketStateService } from '../../services/basket-state-service';
import { ActivatedRoute, Router } from '@angular/router';
import { BasketService, BrandDto, BrandsService, CategoriesService, CategoryDto, LoggedUser, ProductDto, ProductsService, RegionDto, RegionsService, TagDto, TagsService } from '../../generated/clients/regionoix-client';
import { Subscription } from 'rxjs';
import { AuthStateService } from '../../services/auth-state-service';

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


  products!: ProductDto[];
  categories!: CategoryDto[];
  regions!: RegionDto[];
  tags!: TagDto[];
  user: null | LoggedUser = null;
  brands!: BrandDto[];

  productAvailable = false;
  productUnavailable = false;
  selectedCategorys: string[] = [];
  selectedRegions: string[] = [];
  selectedTags: string[] = [];
  selectedBrands: string[] = [];
  maxPrice: number | null = null;
  minPrice: number | null = null;

  //pagination variables
  currentPage = 1;
  pageSize = 20;


  ngOnInit(): void {
    this.queryParamSub = this.route.queryParamMap.subscribe(() => this.loadProducts());
    this.loadCategories();
    this.loadRegions();
    this.loadTags();
    this.loadBrands();
    this.basketState.refreshCount();
    this.user = this.authStateService.currentUser;
    this.loadProducts();
  }

  ngOnDestroy(): void {
    this.queryParamSub?.unsubscribe();
  }

  // Load methods
  loadProducts(): void {
    const queryParams = this.route.snapshot.queryParamMap;
    // Category
    const categoryFilter = queryParams.get('c');
    if (categoryFilter) {
      this.selectedCategorys = [categoryFilter];
    }

    // Region
    const regionFilter = queryParams.get('region');
    if (regionFilter) {
      this.selectedRegions = [regionFilter];
    }
    const filters = this.buildFilters();

    if (queryParams.has('search')) {
      const search = queryParams.get('search') || '';
      this.productService.search(search, filters, undefined, this.pageSize, this.currentPage).subscribe({
        next: (products) => {
          this.products = products;
        },
        error: () => this.snackbar.show("Erreur lors de la récupération des produits", "error")
      });
    } else {
      this.productService.search("", filters, undefined, this.pageSize, this.currentPage).subscribe({
        next: (data) => {
          this.products = data;
        },
        error: () => this.snackbar.show("Erreur lors de la récupération des produits", "error")
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

  loadBrands(): void {
    this.brandsService.get().subscribe({
      next: (data) => {
        console.log("brands: " + data);
        this.brands = data
      },
      error: () => {
        this.snackbar.show('Erreur lors de la récupération des marques', 'error')
        this.brands = [];
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

  toggleBrand(brandName: string, checked: boolean): void {
    if (checked) {
      this.selectedBrands.push(brandName);
    } else {
      this.selectedBrands = this.selectedBrands.filter(t => t !== brandName);
    }
    console.log('Selected brands:', this.selectedBrands);
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
        `(${this.selectedRegions.map(r => `region_name = "${r}"`).join(' OR ')})`
      );
    }

    // Tags
    if (this.selectedTags.length > 0) {
      filters.push(
        `(${this.selectedTags.map(t => `tags = "${t}"`).join(' OR ')})`
      );
    }
    // Brands
    if (this.selectedBrands.length > 0) {
      filters.push(
        `(${this.selectedBrands.map(b => `brand_name = "${b}"`).join(' OR ')})`
      );

    }

    // Availability  TODO
    /*if (this.productAvailable && !this.productUnavailable) filters.push(`stock > 0`);
    if (this.productUnavailable && !this.productAvailable) filters.push(`stock = 0`);*/

    const filterString = filters.join(' AND ');
    console.log('API filters:', filterString);
    return filterString;
  }

  resetFilters(): void {
    this.router.navigate(["/"]);
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