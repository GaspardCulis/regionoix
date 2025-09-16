import { CommonModule } from '@angular/common';
import { Component, inject, Input, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { Router } from '@angular/router';
import { SnackbarService } from '../../services/snackbar-service';
import { AuthentificationService, BasketService, CartDto, FormDataMakeOrder, LoggedUser, ProductDto } from '../../generated/clients/regionoix-client';
import { PaymentModel } from '../../models/payment-model';

@Component({
  selector: 'app-payment-page',
  standalone: true,
  imports: [CommonModule, FormsModule],
  templateUrl: './payment-page.html',
  styleUrls: ['./payment-page.css']
})
export class PaymentPage implements OnInit {
  @Input() totalPrice!: number;
  client!: LoggedUser;
  basket!: CartDto;

  address: FormDataMakeOrder = {
    city: '',
    country: '',
    firstname: '',
    lastname: '',
    postal_code: '',
    street: ''
  };

  payment: PaymentModel = {
    cardNumber: '',
    cardExpiryMonth: 0,
    cardExpiryYear: 0,
    cardCvv: ''
  };

  private readonly router = inject(Router);
  private readonly authService = inject(AuthentificationService);
  private readonly basketService = inject(BasketService);
  private readonly snackBarService = inject(SnackbarService);

  openSection: string = 'info'; // default open accordion section

  ngOnInit(): void {
    this.authService.status().subscribe({
      next: (user) => {
        this.client = {
          id: user.id,
          email: user.email,
          firstname: user.firstname,
          lastname: user.lastname,
          role: user.role
        };
      },
      error: () => this.router.navigate(['/connection'])
    });

    this.basketService.get().subscribe({
      next: (basket) => {
        this.basket = basket;
      },
      error: () => this.snackBarService.show('Erreur lors du chargement du panier.', 'error')
    });
  }

  toggleSection(section: string) {
    this.openSection = this.openSection === section ? '' : section;
  }

  submitAll() {
    //TODO

    this.basketService.make(this.address).subscribe({
      next: () => this.snackBarService.show('Paiement validé avec succès ✅', 'success'),
      error: () => this.snackBarService.show('Erreur lors du paiement.', 'error')
    });
  }

  onCardInput(event: Event) {
    const input = event.target as HTMLInputElement;
    let value = input.value.replace(/\D/g, '').substring(0, 16);
    const parts: string[] = [];
    for (let i = 0; i < value.length; i += 4) parts.push(value.substring(i, i + 4));
    input.value = parts.join(' ');
    this.payment.cardNumber = value; // update model
  }

  onCVVInput(event: Event) {
    const input = event.target as HTMLInputElement;
    const value = input.value.replace(/\D/g, '').substring(0, 3);
    input.value = value;
    this.payment.cardCvv = value;
  }

  onExpiryInput(event: Event) {
    const input = event.target as HTMLInputElement;
    let value = input.value.replace(/\D/g, '').substring(0, 4);
    if (value.length >= 2) value = value.substring(0, 2) + '/' + value.substring(2);
    input.value = value;

    if (value.includes('/')) {
      const [month, year] = value.split('/').map(Number);
      this.payment.cardExpiryMonth = month;
      this.payment.cardExpiryYear = 2000 + (year || 0);
    }
  }

  getProductPrice(product: ProductDto): number {
    if(product.discount) return (product.price * (100 - product.discount.percentage_off))/100;
    else return product.price;
  }
}
