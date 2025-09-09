import { Component, inject } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { UserAuthModel } from '../../models/user-model';
import { Router } from '@angular/router';
import { AuthService } from '../../services/auth-service';

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
  
  onSubmit() {
    if (this.checkCredentials()) {
      const user: UserAuthModel = { email: this.email, password: this.password };
      this.authService.login(user.email, user.password).subscribe({
        next: (response) => {
          console.log('Login successful:', response);
          this.router.navigate(['/showcase']);
        },
        error: (error) => {
          console.error('Login failed:', error);
        }
      });
    } else {
      console.error('Invalid credential format');
    }
  }

  checkCredentials(): boolean {
    const emailPattern = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    let emailVerification = emailPattern.test(this.email);

    // Implement later password strength verification
    let passwordVerification = true;
    return emailVerification && passwordVerification;
  }
}
