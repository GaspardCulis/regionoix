import { HttpClient } from '@angular/common/http';
import { inject, Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { BasketResponse } from '../../models/basket-model';

@Injectable({
  providedIn: 'root'
})
export class BasketService {
  private http = inject(HttpClient);
  getBasket(): Observable<BasketResponse> {
    return this.http.get<BasketResponse>('https://www.regionoix.gasdev.fr/api/basket');
  }

  addItem(product_id: number, quantity = 1) {
    return this.http.post('https://www.regionoix.gasdev.fr/api/basket/items', { product_id, quantity });
  }

  updateItem(product_id: number, quantity: number) {
    return this.http.patch(`https://www.regionoix.gasdev.fr/api/basket/items/${product_id}`, { quantity });
  }

  removeItem(product_id: number) {
    return this.http.delete(`https://www.regionoix.gasdev.fr/api/basket/items/${product_id}`);
  }

  empty() {
    return this.http.delete('https://www.regionoix.gasdev.fr/api/basket');
  }

}
