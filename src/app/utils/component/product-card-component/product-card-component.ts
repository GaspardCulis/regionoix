import { CommonModule } from '@angular/common';
import { Component, inject, Input } from '@angular/core';
import { RouterModule } from '@angular/router';

@Component({
  selector: 'app-product-card-component',
  standalone: true,
  imports: [CommonModule, RouterModule],
  templateUrl: './product-card-component.html',
  styleUrl: './product-card-component.css'
})
export class ProductCardComponent {
  @Input() name!: string;
  @Input() description!: string;
  @Input() image: string | null = null;
  @Input() price!: number;
  @Input() id!: number;
}
