import { CommonModule } from '@angular/common';
import { Component, inject, Input, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { Router } from '@angular/router';
import { UserModel } from '../../models/user-model';
import { AuthService } from '../../services/auth-service';
import { SnackbarService } from '../../services/snackbar-service';
import { AddressModel } from '../../models/address-model';

@Component({
  selector: 'app-payment-page',
  standalone: true,
  imports: [CommonModule, FormsModule],
  templateUrl: './payment-page.html',
  styleUrl: './payment-page.css'
})
export class PaymentPage implements OnInit {
  @Input() totalPrice!: number;
  client!: UserModel;
  address: AddressModel = {
    address: '',
    city: '',
    postalCode: '',
    country: ''
  };

  private readonly router = inject(Router);
  private readonly authService = inject(AuthService);
  private readonly snackBarService = inject(SnackbarService)

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
      this.snackBarService.show('Paiement validé avec succès ✅', "success");
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

  onCardInput(event: any) {
    let value: string = event.target.value.replace(/\D/g, '');
    value = value.substring(0, 16);

    const parts = [];
    for (let i = 0; i < value.length; i += 4) {
      parts.push(value.substring(i, i + 4));
    }

    event.target.value = parts.join(' ');
  }

  onCVVInput(event: any) {
    let value: string = event.target.value.replace(/\D/g, '');
    event.target.value = value.substring(0, 3);
  }

  onExpiryInput(event: any) {
    let value = event.target.value.replace(/\D/g, ''); // remove non-digits

    if (value.length === 0) {
      event.target.value = '';
      return;
    }

    if (value.length === 1) {
      const firstDigit = parseInt(value[0], 10);
      if (firstDigit === 0 || firstDigit === 1) {
        event.target.value = value;
      } else {
        event.target.value = '0' + value[0];
      }
      return;
    }

    let month = parseInt(value.substring(0, 2), 10);
    if (month === 0) month = 1;
    if (month > 12) month = 12; 

    value = month.toString().padStart(2, '0') + value.substring(2);

    if (value.length > 2) {
      value = value.slice(0, 2) + '/' + value.slice(2, 4);
    }

    event.target.value = value;
  }

}
