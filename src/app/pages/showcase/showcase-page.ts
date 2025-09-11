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
import { Category, CategoryService } from '../../services/category-service';
import { Tag, TagService } from '../../services/tag-service';
import { Region, RegionService } from '../../services/region-service';

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
  private categoryService = inject(CategoryService);
  private tagService = inject(TagService);
  private regionService = inject(RegionService);

  products: Product[] = [];
  categories: Category[] = [];
  regions: Region[] = [];
  tags: Tag[] = [];
  selectedCategories: string[] = [];
  selectedRegions: string[] = [];
  selectedTags: string[] = [];
  filterAvailable = false;
  filterUnavailable = false;

  minPrice = 0;
  maxPrice = 500;

  ngOnInit(): void {
    //products
    this.productService.getProducts().subscribe({
      next: (data) => this.products = data,
      error: (err) => {
        console.error('Something went wrong during products recuperation', err);
      }
    });

    //categories
    this.categoryService.getCategories().subscribe({
      next: (data) => this.categories = data,
      error: (err) => console.error('Something went wrong during categories recuperation', err)
    });

    // regions
    this.regionService.getRegions().subscribe({
      next: (data) => this.regions = data,
      error: (err) => console.error('Something went wrong during regions recuperation', err)
    });

    // tags
    this.tagService.getTags().subscribe({
      next: (data) => this.tags = data,
      error: (err) => console.error('Something went wrong during tags recuperation', err)
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

  toggleCategory(cat: string, checked: boolean) {
    if (checked) {
      this.selectedCategories.push(cat);
    } else {
      this.selectedCategories = this.selectedCategories.filter(c => c !== cat);
    }
  }

  toggleRegion(region: string, checked: boolean) {
    if (checked) {
      this.selectedRegions.push(region);
    } else {
      this.selectedRegions = this.selectedRegions.filter(r => r !== region);
    }
  }

  toggleTag(tag: string, checked: boolean) {
    if (checked) {
      this.selectedTags.push(tag);
    } else {
      this.selectedTags = this.selectedTags.filter(t => t !== tag);
    }
  }

  get filteredProducts() {
    return this.products.filter((p) => {
      // price
      if (p.price < this.minPrice || p.price > this.maxPrice) return false;

      // categories
      if (this.selectedCategories.length && !this.selectedCategories.includes(p.category.name)) return false;

      // regions
      if (this.selectedRegions.length && !this.selectedRegions.includes(p.region.name)) return false;

      // tags
      //TODO

      // availabilty
      if (this.filterAvailable && !p.stock) return false;
      if (this.filterUnavailable && p.stock) return false;

      return true;
    });
  }
}
