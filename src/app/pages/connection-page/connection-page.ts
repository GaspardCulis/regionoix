import { Component, inject } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { Router } from '@angular/router';
import { SnackbarService } from '../../services/snackbar-service';
import { AuthentificationService, Roles } from '../../generated/clients/regionoix-client';
import { AuthStateService } from '../../services/auth-state-service';
import { BasketStateService } from '../../services/basket-state-service';

@Component({
  selector: 'app-connection-page',
  imports: [FormsModule],
  templateUrl: './connection-page.html',
  styleUrl: './connection-page.css',
})
export class ConnectionPage {
  email = '';
  password = '';

  private readonly router = inject(Router);
  private readonly authService = inject(AuthentificationService);
  private readonly authStateService = inject(AuthStateService);
  private readonly snackBar = inject(SnackbarService);
  private readonly basketState = inject(BasketStateService);

  onSubmit() {
    if (this.checkCredentials()) {
      const user = { email: this.email, password: this.password };
      this.authService.login({ email: user.email, password: user.password }).subscribe({
        next: () => {
          this.authStateService.notifyAuthChanged();
          this.basketState.refreshCount();

          this.snackBar.show(`Connexion réussie. Bienvenue, ${user.email}!`, 'success');

          // Subscribe to global user already updated
          this.authStateService.user$.subscribe((data) => {
            if (!data) return;

            if (data.role === Roles.Admin) {
              this.router.navigate(['/backoffice']);
            } else {
              this.router.navigate(['/showcase']);
            }
          });
        },
        error: () => {
          this.snackBar.show(
            'Échec de la connexion. Veuillez vérifier vos identifiants et réessayer.',
            'error'
          );
        },
      });
    } else {
      this.snackBar.show('L’adresse e-mail saisie est invalide.', 'error');
    }
  }

  checkCredentials(): boolean {
    const emailPattern = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    const emailVerification = emailPattern.test(this.email);

    // Implement later password strength verification
    const passwordVerification = true;
    return emailVerification && passwordVerification;
  }

  goToCreate() {
    this.router.navigate(['/create-account']);
  }
}
