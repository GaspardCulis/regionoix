import { CommonModule } from '@angular/common';
import { Component, EventEmitter, Input, Output, OnInit } from '@angular/core';
import { RouterModule } from '@angular/router';

@Component({
  selector: 'app-product-card-component',
  standalone: true,
  imports: [CommonModule, RouterModule],
  templateUrl: './product-card-component.html',
  styleUrl: './product-card-component.css'
})
export class ProductCardComponent implements OnInit {
  @Input() name!: string;
  @Input() description!: string;
  @Input() image: string | null = null;
  @Input() price!: number;
  @Input() id!: number;
  @Input() discount: number | null = null;
  final_price: number | null = null;

  @Output() addToBasket = new EventEmitter<number>();

  onAddToBasket() {
    this.addToBasket.emit(this.id);
  }

  ngOnInit() {
    if (this.discount) {
      this.final_price = this.price - (this.price * this.discount) / 100;
    }
  }
}
