import { CommonModule } from '@angular/common';
import { Component } from '@angular/core';

@Component({
  selector: 'app-product-page',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './product-page.html',
  styleUrl: './product-page.css'
})
export class ProductPage {
  product = {
    title: 'Cœur de Celtes Chocolat Noir et Salidou',
    description:
      "Découvrez les Biscuits Cœur de Celtes au Chocolat Noir et Salidou Le Cœur de Celtes chocolat noir et Salidou est la parfaite alliance entre tradition bretonne et gourmandise raffinée. Ces biscuits bretons artisanaux sont conçus pour offrir une expérience gustative inégalée. En croquant dans ces délicieuses douceurs, vous serez enveloppé par la richesse du chocolat noir et le fondant du caramel au beurre salé Salidou, une spécialité bretonne emblématique. Leur texture croquante, combinée à un cœur fondant, fait de ces biscuits le choix idéal pour une pause gourmande ou un cadeau exquis à offrir.",
    image: 'https://picsum.photos/400/300',
    price: 7.5
  };

}
