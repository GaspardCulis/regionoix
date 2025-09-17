import { CommonModule } from '@angular/common';
import { Component, inject, Input, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { Router } from '@angular/router';
import { SnackbarService } from '../../services/snackbar-service';
import { AuthentificationService, BasketService, CartDto, FormDataCreateCheckoutSession, LoggedUser, PaymentService, PostalInfo, ProductDto } from '../../generated/clients/regionoix-client';

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

  address: PostalInfo = {
    city: '',
    country: '',
    firstname: '',
    lastname: '',
    postal_code: '',
    street: ''
  };

  private readonly router = inject(Router);
  private readonly authService = inject(AuthentificationService);
  private readonly basketService = inject(BasketService);
  private readonly paymentService = inject(PaymentService);
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
    const checkoutInterface: FormDataCreateCheckoutSession = {
      cancel_url: window.location.origin + "/error-payment",
      postal_info: this.address,
      success_url: window.location.origin + "/payment-successful",
    };

    this.paymentService.createCheckoutSession(checkoutInterface).subscribe({
      next: (redirectUrl: string) => {
        window.location.href = redirectUrl;
      },
      error: () => {
        this.snackBarService.show('Echec de redirection vers la page de paiement', "error");
      }
    });
  }

  getProductPrice(product: ProductDto): number {
    if (product.discount) return (product.price * (100 - product.discount.percentage_off)) / 100;
    else return product.price;
  }
}
