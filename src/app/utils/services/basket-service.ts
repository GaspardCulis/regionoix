import { HttpClient } from '@angular/common/http';
import { inject, Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { BasketResponse } from '../../models/basket-model';

@Injectable({
  providedIn: 'root'
})
export class BasketService {
  private readonly http = inject(HttpClient);
  private readonly apiUrl = '/api/basket';

  getBasket(): Observable<BasketResponse> {
    return this.http.get<BasketResponse>(this.apiUrl);
  }

  addItem(product_id: number, quantity = 1) {
    return this.http.post(this.apiUrl + '/items', { product_id, quantity });
  }

  updateItem(product_id: number, quantity: number) {
    return this.http.patch(this.apiUrl + `/items/${product_id}`, { quantity });
  }

  removeItem(product_id: number) {
    return this.http.delete(this.apiUrl + `/items/${product_id}`);
  }

  empty() {
    return this.http.delete(this.apiUrl);
  }

}
