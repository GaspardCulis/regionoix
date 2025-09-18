import { inject, Injectable } from '@angular/core';
import { CanActivate, Router, UrlTree } from '@angular/router';
import { Observable, of } from 'rxjs';
import { map, catchError } from 'rxjs/operators';
import { BasketService, CountBasket } from '../generated/clients/regionoix-client';

@Injectable({
  providedIn: 'root',
})
export class BasketNotEmptyGuard implements CanActivate {
  private readonly router = inject(Router);
  private readonly basketService = inject(BasketService);

  canActivate(): Observable<boolean | UrlTree> {
    return this.basketService.get_1().pipe(
      map((count: CountBasket) => {
        if (count.count > 0) {
          return true;
        }
        return this.router.createUrlTree(['/basket']);
      }),
      catchError(() => {
        return of(this.router.createUrlTree(['/basket'])); // redirect on error
      })
    );
  }
}
