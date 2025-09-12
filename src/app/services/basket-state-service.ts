import { inject, Injectable } from '@angular/core';
import { BehaviorSubject } from 'rxjs';
import { CountBasket } from '../generated/clients/regionoix-client';
import { BasketService } from '../generated/clients/regionoix-client';

@Injectable({
    providedIn: 'root'
})
export class BasketStateService {
    private basketCountSubject = new BehaviorSubject<number>(0);
    basketCount$ = this.basketCountSubject.asObservable();

    private readonly basketService = inject(BasketService);

    refreshCount(): void {
        this.basketService.getCount().subscribe({
            next: (count: CountBasket) => this.basketCountSubject.next(count.count),
            error: () => this.basketCountSubject.next(0)
        });
    }

    setCount(count: number): void {
        this.basketCountSubject.next(count);
    }
}
