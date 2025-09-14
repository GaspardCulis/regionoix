import { Component, Input, Output, inject, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { Router } from '@angular/router';
import { AuthCookieService } from '../../../services/auth-cookie';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';
import { faBasketShopping } from '@fortawesome/free-solid-svg-icons';

@Component({
  selector: 'app-topbar',
  standalone: true,
  imports: [CommonModule, FormsModule, FontAwesomeModule],
  templateUrl: './topbar-component.html',
  styleUrl: './topbar-component.css',
})
export class TopbarComponent implements OnInit {
  private router = inject(Router);
  private authCookieService = inject(AuthCookieService);
  faBasketShopping = faBasketShopping;

  @Input() pathLogo!: string;
  @Input() title!: string;
  @Input() basketCount = 0;
  @Input() user!: string | null;
  @Output() searchText = '';

  ngOnInit(): void {
    this.authCookieService.user$.subscribe((user) => {
      this.user = user;
    });
  }

  onProfileClick() {
    if (this.user) {
      this.router.navigate(['/profile']); //TODO: replace with profile page
    } else {
      this.router.navigate(['/connection']);
    }
  }

  goToBasket() {
    this.router.navigate(['/basket']);
  }

  goHome() {
    this.router.navigate(['/showcase']);
  }
}
