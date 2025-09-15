import { CommonModule } from '@angular/common';
import { Component, inject, Input, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { Router } from '@angular/router';
import { SnackbarService } from '../../services/snackbar-service';
import { AuthentificationService, BasketService, FormDataMakeOrder, LoggedUser } from '../../generated/clients/regionoix-client';

@Component({
  selector: 'app-payment-page',
  standalone: true,
  imports: [CommonModule, FormsModule],
  templateUrl: './payment-page.html',
  styleUrl: './payment-page.css'
})
export class PaymentPage implements OnInit {
  @Input() totalPrice!: number;
  client!: LoggedUser;
  address: FormDataMakeOrder = {
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
  private readonly snackBarService = inject(SnackbarService);

  currentStep = 1;

  ngOnInit(): void {
    this.authService.status().subscribe({
      next: (user) => {
        this.client = {
          id: user.id,
          email: user.email,
          firstname: user.firstname,
          lastname: user.lastname,
          role: user.role
        }
      },
      error: () => {
        this.router.navigate(['/connection']);
      },
    });
  }

  nextStep() {
    if (this.currentStep < 4) {
      this.currentStep++;
    }
    if (this.currentStep === 4) {
      this.basketService.get().subscribe({
        next: (basket) => {
          console.log(basket);
        }
      }
      );

      this.basketService.make(this.address).subscribe({
        next: () => {
          this.snackBarService.show('Paiement validé avec succès ✅', "success");
        },
        error: () => {
          this.snackBarService.show('Une erreur est survenue lors de la validation du paiement. Veuillez réessayer.', 'error');
          this.currentStep = 3;
        },
      });
    }
  }

  prevStep() {
    if (this.currentStep > 1) {
      this.currentStep--;
    }
  }

  finish() {
    this.router.navigate(['/showcase']);
  }

  wrongEmail() {
    this.snackBarService.show('Veuillez vous déconnecter et vous reconnecter avec le bon compte.', 'info');
  }

  onCardInput(event: Event) {
    const input = event.target as HTMLInputElement;
    let value: string = input.value.replace(/\D/g, '');
    value = value.substring(0, 16);

    const parts: string[] = [];
    for (let i = 0; i < value.length; i += 4) {
      parts.push(value.substring(i, i + 4));
    }

    input.value = parts.join(' ');
  }

  onCVVInput(event: Event) {
    const input = event.target as HTMLInputElement;
    const value: string = input.value.replace(/\D/g, '');
    input.value = value.substring(0, 3);
  }

  onExpiryInput(event: Event) {
    const input = event.target as HTMLInputElement;
    let value = input.value.replace(/\D/g, '');

    if (value.length === 0) {
      input.value = '';
      return;
    }

    if (value.length === 1) {
      const firstDigit = parseInt(value[0], 10);
      input.value = (firstDigit === 0 || firstDigit === 1) ? value : '0' + value[0];
      return;
    }

    let month = parseInt(value.substring(0, 2), 10);
    if (month === 0) month = 1;
    if (month > 12) month = 12;

    value = month.toString().padStart(2, '0') + value.substring(2);

    if (value.length > 2) {
      value = value.slice(0, 2) + '/' + value.slice(2, 4);
    }

    input.value = value;
  }

}
