import { CommonModule } from '@angular/common';
import { Component, inject, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { Product } from '../../utils/model/product-model';
import { ProductService } from '../../utils/services/product-service';
import { BasketService } from '../../utils/services/basket-service';

@Component({
  selector: 'app-product-page',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './product-page.html',
  styleUrls: ['./product-page.css']
})
export class ProductPage implements OnInit {
  product!: Product;
  quantity = 1;

  private basketService = inject(BasketService);
  private productService = inject(ProductService);
  private route = inject(ActivatedRoute);

  ngOnInit() {
    const id = this.route.snapshot.paramMap.get('id');
    if (id) {
      this.productService.getProductById(id).subscribe({
        next: (data) => {
          this.product = {
            ...data,
            image: data.image ?? 'https://picsum.photos/400/250?random=1'
          };
        },
        error: (err) => {
          console.error('Something went wrong during product recuperation', err);
        }
      });
    }
  }

  addItem(productId: number) {
    this.basketService.addItem(productId).subscribe({
      next: () => console.log('Product add to basket'),
      error: (err) => console.error(err)
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
