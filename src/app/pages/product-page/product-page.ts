import { CommonModule } from '@angular/common';
import { Component, inject } from '@angular/core';
import { HttpClient, HttpClientModule } from '@angular/common/http';
import { OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router';

interface Product {
  id: number;
  name: string;
  description: string;
  weight: number;
  price: number;
  image: string | null;
  stock: number;
  region_id: number;
  brand_id: number;
  category_id: number;
}

@Component({
  selector: 'app-product-page',
  standalone: true,
  imports: [CommonModule, HttpClientModule],
  templateUrl: './product-page.html',
  styleUrl: './product-page.css'
})
export class ProductPage implements OnInit {
  product!: Product;
  quantity = 1;
  private http = inject(HttpClient);
  private route = inject(ActivatedRoute);

  ngOnInit() {
    const id = this.route.snapshot.paramMap.get('id');
    this.http.get<Product>(`/api/products/${id}`)
      .subscribe({
        next: (data) => {
          this.product = {
            ...data,
            price: Number(data.price),
            image: data.image ?? 'https://picsum.photos/400/250?random=1'
          };
        },
        error: (err) => {
          console.error('Erreur lors de la récupération du produit', err);
        }
      });
  }

  increaseQuantity() {
    if (this.product && this.quantity < this.product.stock) {
      this.quantity++;
    }
  }

  decreaseQuantity() {
    if (this.quantity > 1) {
      this.quantity--;
    }
  }
}
