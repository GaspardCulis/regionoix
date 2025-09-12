import { Component, Input, inject, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { ActivatedRoute, Router } from '@angular/router';
import { AuthentificationService, LoggedUser } from '../../../generated/clients/regionoix-client';

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
export class TopbarComponent implements OnInit {
  private router = inject(Router);
  private userService = inject(AuthentificationService);

  @Input() pathLogo!: string;
  @Input() title!: string;
  @Input() basketCount = 0;
  @Input() user!: LoggedUser | null;;
  searchText = '';

  ngOnInit(): void {
    this.userService.status().subscribe({
      next: (user) => this.user = user,
      error: () => this.user = null
    });
  }

  onProfileClick() {
    if (this.user) {
      this.router.navigate(['/profile']);
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

  search(): void {
    console.log("Searching for:", this.searchText);
    const query = this.searchText.trim();
    if (!query) {
      this.router.navigate(['/showcase']);
    } else {
      this.router.navigate(['/showcase'],
        {
          queryParams: { search: query },
          queryParamsHandling: 'merge'
        });
    }
  }
}
