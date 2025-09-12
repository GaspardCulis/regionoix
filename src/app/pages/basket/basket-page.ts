import { CommonModule } from '@angular/common';
import { Component, inject, OnInit } from '@angular/core';
import { Router, RouterModule } from '@angular/router';
import { ProductListItemComponent } from '../../utils/component/product-list-item-component/product-list-item-component';
import { BasketStateService } from '../../services/basket-state-service';
import { BasketService, CartLineDto } from '../../generated/clients/regionoix-client';

@Component({
  selector: 'app-basket',
  standalone: true,
  imports: [CommonModule, ProductListItemComponent, RouterModule,],
  templateUrl: './basket-page.html',
  styleUrls: ['./basket-page.css']
})
export class BasketPage implements OnInit {
  private basketService = inject(BasketService);
  private readonly basketState = inject(BasketStateService);
  private router = inject(Router);

  lines: CartLineDto[] = [];

  ngOnInit(): void {
    this.loadBasket();
  }

  loadBasket() {
    this.basketService.getBasket().subscribe({
      next: (data) => {
        this.lines = data.lines!;
        this.basketState.refreshCount();
      },
      error: (err) => console.error('Error during basket recuperation', err)
    });
  }

  getTotalPrice(): number {
    return this.lines.reduce((total, l) => total + l.product.price * l.quantity, 0);
  }

  goToPayment() {
    this.router.navigate(['/payment']);
  }

  removeItem(productId: number) {
    this.basketService.removeItem(productId).subscribe();
    this.loadBasket()
  }

  changeQuantity(productId: number, quantity: number) {
    this.basketService.updateItemQuantity(productId, { quantity: quantity }).subscribe(() => this.loadBasket());
  }

  emptyBasket() {
    this.basketService.empty().subscribe();
    this.loadBasket();
  }
}
