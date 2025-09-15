import { CommonModule } from '@angular/common';
import { Component, inject, OnInit } from '@angular/core';
import { Router, RouterModule } from '@angular/router';
import { ProductListItemComponent } from '../../utils/component/product-list-item-component/product-list-item-component';
import { BasketService, CartLineDto } from '../../generated/clients/regionoix-client';
import { BasketStateService } from '../../services/basket-state-service';

@Component({
  selector: 'app-basket',
  standalone: true,
  imports: [CommonModule, ProductListItemComponent, RouterModule,],
  templateUrl: './basket-page.html',
  styleUrls: ['./basket-page.css']
})
export class BasketPage implements OnInit {
  private service = inject(BasketService);
  private basketService = inject(BasketService);
  private readonly basketState = inject(BasketStateService);

  private router = inject(Router);

  lines: CartLineDto[] = [];

  ngOnInit(): void {
    this.loadBasket();
  }

  loadBasket() {
    this.service.getBasket().subscribe({
      next: (data) => {
        this.lines = data.lines ?? [];
        this.basketState.refreshCount();
      },
      error: (err) => console.error('Error during basket recuperation', err)
    });
  }

  getTotalPrice(): number {
    let total = 0;
    // Compute total price with discount
    this.lines.forEach((l) => {
      let final_price = l.product.price;
      if (l.product.discount) {
        final_price = l.product.price - (l.product.price * l.product.discount.percentage_off) / 100;
      }
      total += final_price * l.quantity;
    })
    return total;
  }

  goToPayment() {
    this.router.navigate(['/payment']);
  }

  removeItem(productId: number) {
    this.basketService.removeItem(productId).subscribe();
    this.loadBasket()
  }

  changeQuantity(productId: number, quantity: number) {
    this.basketService.updateItemQuantity(productId, { quantity }).subscribe(() => this.loadBasket());
  }

  emptyBasket() {
    this.basketService.empty().subscribe();
    this.loadBasket();
  }
}
