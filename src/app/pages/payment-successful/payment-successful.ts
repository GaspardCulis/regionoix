import { Component, inject } from '@angular/core';
import { Router } from '@angular/router';
import { CommonModule } from '@angular/common';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';
import { faCheck } from '@fortawesome/free-solid-svg-icons';

@Component({
  selector: 'app-payment-successful',
  imports: [CommonModule, FontAwesomeModule],
  templateUrl: './payment-successful.html',
  styleUrl: './payment-successful.css'
})
export class PaymentSuccessful {
  private readonly router = inject(Router);
  
  faCheck = faCheck;

  goHome() {
    this.router.navigate(['/home']);
  }
}
