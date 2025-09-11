import { CommonModule } from '@angular/common';
import { Component, EventEmitter, Input, Output } from '@angular/core';
import { Product } from '../../model/product-model';

@Component({
  selector: 'app-product-list-item-component',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './product-list-item-component.html',
  styleUrl: './product-list-item-component.css'
})
export class ProductListItemComponent {
  @Input() product!: Product;
  @Input() quantity!: number;

  @Output() removeFromBasket = new EventEmitter<number>();
  @Output() quantityChange = new EventEmitter<{ productId: number, quantity: number }>();


  onRemoveFromBasket() {
    this.removeFromBasket.emit(this.product.id);
  }

  increaseQuantity() {
    if (this.quantity && this.quantity < this.product.stock) {
      this.quantity++;
    }
    this.quantityChange.emit({ productId: this.product.id, quantity: this.quantity });
  }

  decreaseQuantity() {
    if (this.quantity > 1) {
      this.quantity--;
    }
    this.quantityChange.emit({ productId: this.product.id, quantity: this.quantity });
  }
}

