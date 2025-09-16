import { Component, inject } from '@angular/core';
import { FormsModule, NgForm } from '@angular/forms';
import { Router } from '@angular/router';
import { SnackbarService } from '../../services/snackbar-service';

@Component({
  selector: 'app-create-account',
  imports: [FormsModule],
  templateUrl: './create-account.html',
  styleUrl: './create-account.css'
})
export class CreateAccount {
  fullname = '';
  email = '';
  password = '';
  confirmPassword = '';
  private readonly router = inject(Router);
  private readonly snackBar = inject(SnackbarService);

  onRegister(form: NgForm) {
    if (form.invalid) {
      this.snackBar.show('Veuillez remplir correctement tous les champs.', 'error');
      return;
    }

    if (this.password !== this.confirmPassword) {
      this.snackBar.show('Les mots de passe ne correspondent pas.', 'error');
      return;
    }
    const emailPattern = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    if (!emailPattern.test(this.email)) {
      this.snackBar.show('Veuillez saisir une adresse email correct.', 'error');
      return;
    }

    this.snackBar.show('Compte créé avec succès !', 'success');

    form.resetForm();
  }

  goToLogin() {
    this.router.navigate(['/connection']);
  }
}
