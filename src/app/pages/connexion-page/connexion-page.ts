import { Component, EventEmitter, inject, Output } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { UserModel } from '../../models/user-model';
import { Router } from '@angular/router';

@Component({
  selector: 'app-connexion-page',
  imports: [FormsModule],
  templateUrl: './connexion-page.html',
  styleUrl: './connexion-page.css'
})
export class ConnexionPage {
  email = '';
  password = '';

  @Output() login = new EventEmitter<UserModel>();

  private router = inject(Router);

  //TODO: add constructor with service injection for actual login
  
  onSubmit() {
    if (this.checkCredentials()) {
      const user: UserModel = { email: this.email, password: this.password };
      this.login.emit(user);
      this.router.navigate(['/showcase']);
    }
  }

  checkCredentials(): boolean {
    //TODO: implement actual credential checking logic with backend
    return true;
  }
}
