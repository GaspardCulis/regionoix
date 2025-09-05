import { Component } from '@angular/core';
import { ProductCardComponent } from '../../utils/component/product-card-component/product-card-component';
import { CommonModule } from '@angular/common';

@Component({
  selector: 'app-showcase',
  standalone: true,
  imports: [CommonModule, ProductCardComponent],
  templateUrl: './showcase-page.html',
  styleUrl: './showcase-page.css'
})
export class ShowcasePage {

  products = [
    {
      title: 'Produit A',
      description: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur condimentum',
      image: 'https://picsum.photos/400/250?random=1',
      price: 9.99
    },
    {
      title: 'Produit B',
      description: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur condimentum  ',
      image: 'https://picsum.photos/400/250?random=1',
      price: 19.99
    },
    {
      title: 'Produit C',
      description: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur  ',
      image: 'https://picsum.photos/400/250?random=1',
      price: 29.99
    },
    {
      title: 'Produit D',
      description: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur ',
      image: 'https://picsum.photos/400/250?random=1',
      price: 39.99
    },
    {
      title: 'Produit E',
      description: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit.',
      image: 'https://picsum.photos/400/250?random=1',
      price: 49.99
    },
  ]
}
