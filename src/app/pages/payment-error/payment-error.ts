import { Component, inject } from '@angular/core';
import { Router } from '@angular/router';
import { CommonModule } from '@angular/common';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';
import { faExclamation } from '@fortawesome/free-solid-svg-icons';

@Component({
  selector: 'app-payment-error',
  imports: [CommonModule, FontAwesomeModule],
  templateUrl: './payment-error.html',
  styleUrl: './payment-error.css'
})
export class PaymentError {
  private readonly router = inject(Router);
  
  faExclamation = faExclamation;

  goHome() {
    this.router.navigate(['/home']);
  }
}
