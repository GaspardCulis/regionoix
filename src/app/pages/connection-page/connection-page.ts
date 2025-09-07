import { Component, EventEmitter, inject, Output } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { UserModel } from '../../models/user-model';
import { Router } from '@angular/router';

@Component({
  selector: 'app-connection-page',
  imports: [FormsModule],
  templateUrl: './connection-page.html',
  styleUrl: './connection-page.css'
})
export class ConnectionPage {
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
