import { CommonModule } from '@angular/common';
import { Component, inject, Input } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { Router } from '@angular/router';

@Component({
  selector: 'app-payment-page',
  standalone: true,
  imports: [CommonModule, FormsModule],
  templateUrl: './payment-page.html',
  styleUrl: './payment-page.css'
})
export class PaymentPage {
  @Input() totalPrice!: number;

  private router = inject(Router);
  currentStep = 1;
  showSnackbar = false;

  nextStep() {
    if (this.currentStep < 4) {
      this.currentStep++;
    }
    if (this.currentStep === 4) {
      this.showSnackbar = true;
      setTimeout(() => {
        this.showSnackbar = false;
      }, 3000);
    }
  }

  prevStep() {
    if (this.currentStep > 1) {
      this.currentStep--;
    }
  }

  finish() {
    console.log('Commande validée ✅');
    this.router.navigate(['/showcase']);
  }
}
