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
  email = '';
  password = '';

  @Output() login = new EventEmitter<UserModel>();

  //TODO: add constructor with service injection for actual login
  
  onSubmit() {
    if (this.checkCredentials()) {
      const user: UserModel = { email: this.email, password: this.password };
      this.login.emit(user);
      //TODO: navigate to another page on successful login
    }
  }

  checkCredentials(): boolean {
    //TODO: implement actual credential checking logic with backend
    return true;
  }
}
