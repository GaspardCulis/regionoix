import { CommonModule } from '@angular/common';
import { Component } from '@angular/core';
import { ProductListItemComponent } from '../../utils/component/product-list-item-component/product-list-item-component';
import { Router } from '@angular/router';

@Component({
  selector: 'app-basket',
  standalone: true,
  imports: [CommonModule, ProductListItemComponent],
  templateUrl: './basket-page.html',
  styleUrl: './basket-page.css'
})
export class BasketPage {

  products = [
    {
      title: 'Produit A',
      quantity: 1,
      image: 'https://picsum.photos/400/250?random=1',
      price: 9.99
    },
    {
      title: 'Produit B',
      quantity: 2,
      image: 'https://picsum.photos/400/250?random=1',
      price: 19.99
    },
    {
      title: 'Produit C',
      quantity: 3,
      image: 'https://picsum.photos/400/250?random=1',
      price: 29.99
    },
    {
      title: 'Produit D',
      quantity: 4,
      image: 'https://picsum.photos/400/250?random=1',
      price: 39.99
    },
    {
      title: 'Produit E',
      quantity: 5,
      image: 'https://picsum.photos/400/250?random=1',
      price: 49.99
    },
  ]

  constructor(private router: Router) { }

  goToPayment() {
    this.router.navigate(['/payment']);
  }

}
