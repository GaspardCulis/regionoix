import { Component } from '@angular/core';
import { ProductCardComponent } from '../../utils/component/product-card-component/product-card-component';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';

@Component({
  selector: 'app-showcase',
  standalone: true,
  imports: [CommonModule, ProductCardComponent, FormsModule],
  templateUrl: './showcase-page.html',
  styleUrl: './showcase-page.css'
})
export class ShowcasePage {

  products = [
    {
      title: 'Produit A',
      description: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur condimentum',
      image: 'https://picsum.photos/400/250?random=1',
      price: 9.99,
      category: 'vin',
      region: 'grenoble'
    },
    {
      title: 'Produit B',
      description: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur condimentum  ',
      image: 'https://picsum.photos/400/250?random=1',
      price: 19.99,
      category: 'vin',
      region: 'grenoble'
    },
    {
      title: 'Produit C',
      description: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur  ',
      image: 'https://picsum.photos/400/250?random=1',
      price: 29.99,
      category: 'vin',
      region: 'grenoble'
    },
    {
      title: 'Produit D',
      description: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur ',
      image: 'https://picsum.photos/400/250?random=1',
      price: 39.99,
      category: 'vin',
      region: 'grenoble'
    },
    {
      title: 'Produit E',
      description: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit.',
      image: 'https://picsum.photos/400/250?random=1',
      price: 49.99,
      category: 'vin',
      region: 'grenoble'
    },
    {
      title: 'Produit A',
      description: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur condimentum',
      image: 'https://picsum.photos/400/250?random=1',
      price: 9.99,
      category: 'vin',
      region: 'grenoble'
    },
    {
      title: 'Produit B',
      description: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur condimentum  ',
      image: 'https://picsum.photos/400/250?random=1',
      price: 19.99,
      category: 'vin',
      region: 'grenoble'
    },
    {
      title: 'Produit C',
      description: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur  ',
      image: 'https://picsum.photos/400/250?random=1',
      price: 29.99,
      category: 'vin',
      region: 'grenoble'
    },
    {
      title: 'Produit D',
      description: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur ',
      image: 'https://picsum.photos/400/250?random=1',
      price: 39.99,
      category: 'vin',
      region: 'grenoble'
    },
    {
      title: 'Produit E',
      description: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit.',
      image: 'https://picsum.photos/400/250?random=1',
      price: 49.99,
      category: 'vin',
      region: 'grenoble'
    },
  ]

  categories = ['Boissons', 'Fromages', 'Charcuterie', 'Épicerie'];
  regions = ['Sud-Ouest', 'Centre', 'Provence', 'Alsace'];

  selectedCategory = '';
  selectedRegion = '';
  maxPrice: number | null = null;

  get filteredProducts() {
    return this.products.filter(p => {
      return (
        (!this.selectedCategory || p.category === this.selectedCategory) &&
        (!this.selectedRegion || p.region === this.selectedRegion) &&
        (!this.maxPrice || p.price <= this.maxPrice)
      );
    });
  }
}
