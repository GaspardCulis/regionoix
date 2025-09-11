import { HttpClient } from '@angular/common/http';
import { inject, Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { BasketResponse } from '../models/basket-model';

@Injectable({
  providedIn: 'root'
})
export class BasketService {
  private readonly endpoint = 'api/basket/';

  private readonly httpClient = inject(HttpClient);

  public getBasketDetails(): Observable<BasketResponse> {
    const url = `${this.endpoint}`;
    return this.httpClient.get<BasketResponse>(url);
  }

  public emptyBasket(): Observable<object> {
    const url = `${this.endpoint}`;
    return this.httpClient.delete<object>(url);
  }

  public getNumberOfItems(): Observable<number> {
    const url = `${this.endpoint}count`;
    return this.httpClient.get<number>(url);
  }

  public addProduct(): Observable<object> {
    const url = `${this.endpoint}items`;
    return this.httpClient.post<object>(url, {});
  }

  public removeProduct(id: number): Observable<object> {
    const url = `${this.endpoint}items/${id}`;
    return this.httpClient.delete<object>(url);
  }

  public updateProductQuantity(id: number): Observable<object> {
    const url = `${this.endpoint}items/${id}`;
    return this.httpClient.patch<object>(url, {});
  }

  public orderBasket(): Observable<object> {
    const url = `${this.endpoint}order`;
    return this.httpClient.post<object>(url, {});
  }
}
