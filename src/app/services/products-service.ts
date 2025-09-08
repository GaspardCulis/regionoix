import { HttpClient } from '@angular/common/http';
import { Injectable, inject } from '@angular/core';
import { Observable } from 'rxjs';

export interface Product {
  id: number;
  name: string;
  price: number;
  // ajoutez ici les autres propriétés selon votre API
}

@Injectable({
  providedIn: 'root'
})
export class ProductsService {
  private readonly baseUrl = 'https://www.regionoix.gasdev.fr/api/';
  private readonly endpoint = 'products';
  private readonly http = inject(HttpClient);

  /**
   * Récupère la liste des produits
   */
  public getProducts(): Observable<Product[]> {
    return this.http.get<Product[]>(`${this.baseUrl}${this.endpoint}`);
  }
}
