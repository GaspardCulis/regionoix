import { Component, inject } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { UserAuthModel } from '../../models/user-model';
import { Router } from '@angular/router';
import { AuthService } from '../../services/auth-service';
import { SnackbarService } from '../../services/snackbar-service';

@Component({
  selector: 'app-connection-page',
  imports: [FormsModule],
  templateUrl: './connection-page.html',
  styleUrl: './connection-page.css'
})
export class ConnectionPage {
  email = '';
  password = '';

  private readonly router = inject(Router);
  private readonly authService = inject(AuthService);
  private readonly snackBar = inject(SnackbarService);
  
  onSubmit() {
    if (this.checkCredentials()) {
      const user: UserAuthModel = { email: this.email, password: this.password };
      this.authService.login(user.email, user.password).subscribe({
        next: () => {
          this.snackBar.show(`Connexion réussie. Bienvenue, ${user.email} !`, 'success');

          this.router.navigate(['/showcase']);
        },
        error: () => {
          this.snackBar.show('Échec de la connexion. Veuillez vérifier vos identifiants et réessayer.', 'error');
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
}
