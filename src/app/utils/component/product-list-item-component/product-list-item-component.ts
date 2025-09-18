import { CommonModule } from '@angular/common';
import { Component, EventEmitter, Input, OnInit, Output } from '@angular/core';
import { ProductDto } from '../../../generated/clients/regionoix-client';

@Component({
  selector: 'app-product-list-item-component',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './product-list-item-component.html',
  styleUrl: './product-list-item-component.css'
})
export class ProductListItemComponent implements OnInit {

  @Input() product!: ProductDto;
  @Input() quantity!: number;
  final_price: number | null = null;

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

  ngOnInit(): void {
    // Compute final price  
    if (this.product && this.product.discount) {
      this.final_price = this.product.price - (this.product.price * this.product.discount.percentage_off) / 100;
    }
  }
}

