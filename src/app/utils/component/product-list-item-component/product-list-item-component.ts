import { CommonModule } from '@angular/common';
import { Component, Input } from '@angular/core';

@Component({
  selector: 'app-product-list-item-component',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './product-list-item-component.html',
  styleUrl: './product-list-item-component.css'
})
export class ProductListItemComponent {
  @Input() name!: string;
  @Input() image!: string;
  @Input() price!: number;
  @Input() quantity!: number;
}

