import { CommonModule } from '@angular/common';
import { Component, Input } from '@angular/core';
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

  constructor(private router: Router) { }

  currentStep = 1;

  nextStep() {
    if (this.currentStep < 4) {
      this.currentStep++;
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
