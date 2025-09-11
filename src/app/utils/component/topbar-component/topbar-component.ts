import { Component, inject, Input, Output } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { Router } from '@angular/router';
import { AuthCookieService } from '../../../services/auth-cookie';

@Component({
  selector: 'app-topbar',
  standalone: true,
  imports: [
    CommonModule,
    FormsModule
  ],
  templateUrl: './topbar-component.html',
  styleUrl: './topbar-component.css'
})
export class TopbarComponent {
  @Input() pathLogo!: string;
  @Input() title!: string;
  @Input() basketCount = 0;
  @Input() user!: string | null;
  @Output() searchText = '';

  constructor(
    private router: Router,
    private authCookieService: AuthCookieService
  ) {
    this.authCookieService.user$.subscribe(user => {
      this.user = user;
    });
  }

  onProfileClick() {
    if (this.user) {
      this.router.navigate(['/profile']); //TODO: replace with profile page
    }
    else {
      this.router.navigate(['/connection']);
    }
  }

  goToBasket() {
    this.router.navigate(['/basket'])
  }

  goHome() {
    this.router.navigate(['/showcase']);
  }
}
