import { Component, inject } from '@angular/core';
import { FormsModule, NgForm } from '@angular/forms';
import { Router } from '@angular/router';
import { SnackbarService } from '../../services/snackbar-service';
import { ClientService } from '../../generated/clients/regionoix-client';

@Component({
  selector: 'app-create-account',
  imports: [FormsModule],
  templateUrl: './create-account.html',
  styleUrl: './create-account.css'
})
export class CreateAccount {
  lastname = '';
  fisrtname = '';
  email = '';
  password = '';
  confirmPassword = '';
  private readonly router = inject(Router);
  private readonly snackBar = inject(SnackbarService);
  private readonly clientService = inject(ClientService);

  onRegister(form: NgForm) {
    if (form.invalid) {
      this.snackBar.show('Veuillez remplir correctement tous les champs.', 'error');
      return;
    }

    if (this.password !== this.confirmPassword) {
      this.snackBar.show('Les mots de passe ne correspondent pas.', 'error');
      return;
    }
    if (this.password.length < 8) {
      this.snackBar.show('Le mot de passe doit contenir au minimum 8 cractères', 'error');
      return;
    }
    const emailPattern = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    if (!emailPattern.test(this.email)) {
      this.snackBar.show('Veuillez saisir une adresse email correct.', 'error');
      return;
    }
    this.clientService.register({ email: this.email, firstname: this.fisrtname, lastname: this.lastname, password: this.password }).subscribe({
      next: () => {
        this.snackBar.show('Compte créé avec succès !', 'success');
        form.resetForm();
        this.goToLogin();
      },
      error: () => this.snackBar.show('Une erreur est survenue, veuillez contacter le support', 'error')
    })
  }

  goToLogin() {
    this.router.navigate(['/connection']);
  }
}
