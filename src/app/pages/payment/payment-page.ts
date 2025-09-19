import { CommonModule } from '@angular/common';
import { Component, inject, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { Router } from '@angular/router';
import { SnackbarService } from '../../services/snackbar-service';
import {
  AuthentificationService,
  BasketService,
  CartDto,
  FormDataCreateCheckoutSession,
  LoggedUser,
  PaymentService,
  PostalInfo,
  ProductDto,
} from '../../generated/clients/regionoix-client';
import { HttpErrorResponse } from '@angular/common/http';

@Component({
  selector: 'app-payment-page',
  standalone: true,
  imports: [CommonModule, FormsModule],
  templateUrl: './payment-page.html',
  styleUrls: ['./payment-page.css'],
})
export class PaymentPage implements OnInit {
  totalPrice!: number;
  client!: LoggedUser;
  basket!: CartDto;

  address: PostalInfo = {
    city: '',
    country: '',
    firstname: '',
    lastname: '',
    postal_code: '',
    street: '',
  };

  private readonly router = inject(Router);
  private readonly authService = inject(AuthentificationService);
  private readonly basketService = inject(BasketService);
  private readonly paymentService = inject(PaymentService);
  private readonly snackBarService = inject(SnackbarService);

  openSection = 'info';

  ngOnInit(): void {
    this.authService.status().subscribe({
      next: (user) => {
        this.client = {
          id: user.id,
          email: user.email,
          firstname: user.firstname,
          lastname: user.lastname,
          role: user.role,
        };
      },
      error: () => this.router.navigate(['/connection']),
    });

    this.basketService.get().subscribe({
      next: (basket) => {
        this.basket = basket;
        this.totalPrice = this.totalBasketPrice;
      },
      error: () => this.snackBarService.show('Erreur lors du chargement du panier.', 'error'),
    });
  }

  toggleSection(section: string) {
    this.openSection = this.openSection === section ? '' : section;
  }

  submitAll() {
    if (
      !this.address.lastname ||
      !this.address.firstname ||
      !this.address.street ||
      !this.address.city ||
      !this.address.postal_code ||
      !this.address.country
    ) {
      this.snackBarService.show('Veuillez remplir tous les champs requis.', 'info');
      return;
    }
    const checkoutInterface: FormDataCreateCheckoutSession = {
      cancel_url: window.location.origin + '/error-payment',
      postal_info: this.address,
      success_url: window.location.origin + '/payment-successful',
    };

    this.paymentService.createCheckoutSession(checkoutInterface).subscribe({
      next: (redirectUrl: string) => {
        window.location.href = redirectUrl;
      },

      error: (e: HttpErrorResponse) => {
        if (e.status === 400) {
          this.snackBarService.show(
            "Au moins un des produits dans votre panier n'a plus de stock.",
            'error'
          );
        } else {
          this.snackBarService.show('Echec de redirection vers la page de paiement', 'error');
        }
      },
    });
  }

  getProductPrice(product: ProductDto): number {
    if (product.discount) return (product.price * (100 - product.discount.percentage_off)) / 100;
    else return product.price;
  }

  get totalBasketPrice(): number {
    return (
      this.basket?.lines?.reduce(
        (total, line) => total + this.getProductPrice(line.product) * line.quantity,
        0
      ) ?? 0
    );
  }
}
