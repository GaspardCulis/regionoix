import { Component, EventEmitter, Output } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { UserModel } from '../../models/user-model';

@Component({
  selector: 'app-connexion-page',
  imports: [FormsModule],
  templateUrl: './connexion-page.html',
  styleUrl: './connexion-page.css'
})
export class ConnexionPage {
  email: string = '';
  password: string = '';

  @Output() onLogin = new EventEmitter<UserModel>();

  //TODO: add constructor with service injection for actual login
  
  onSubmit() {
    if (this.checkCredentials()) {
      let user: UserModel = { email: this.email, password: this.password };
      this.onLogin.emit(user);
    }
  }

  checkCredentials(): boolean {
    //TODO: implement actual credential checking logic with backend
    return true;
  }
}
