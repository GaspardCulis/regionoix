import { Component, inject } from '@angular/core';
import { ProductCardComponent } from '../../utils/component/product-card-component/product-card-component';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { OnInit } from '@angular/core';
import { Product } from '../../utils/model/product-model';
import { ProductService } from '../../utils/services/product-service';
import { HttpClientModule } from '@angular/common/http';


@Component({
  selector: 'app-showcase',
  standalone: true,
  imports: [CommonModule, ProductCardComponent, FormsModule, HttpClientModule],
  templateUrl: './showcase-page.html',
  styleUrl: './showcase-page.css'
})

export class ShowcasePage implements OnInit {
  private productService = inject(ProductService);

  products: Product[] = [];
  categories = ['Boissons', 'Fromages', 'Charcuterie', 'Épicerie'];
  regions = ['Sud-Ouest', 'Centre', 'Provence', 'Alsace'];

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


  get filteredProducts() {
    return this.products.filter(p => {
      return (
        (!this.selectedCategory || p.category_id === +this.selectedCategory) &&
        (!this.selectedRegion || p.region_id === +this.selectedRegion) &&
        (!this.maxPrice || p.price <= this.maxPrice)
      );
    });
  }
}
