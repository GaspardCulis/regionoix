import { Component, Input, inject, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { Router } from '@angular/router';
import { CategoriesService, CategoryDto, LoggedUser, RegionDto, RegionsService } from '../../../generated/clients/regionoix-client';
import { AuthStateService } from '../../../services/auth-state-service';
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
  private authService = inject(AuthStateService);
  // icon for basket shopping
  faBasketShopping = faBasketShopping;

  @Input() pathLogo!: string;
  @Input() title!: string;
  @Input() basketCount = 0;
  searchText = '';

  user: LoggedUser | null = null;

  ngOnInit() {
    this.authService.user$.subscribe((u) => (this.user = u));
  }
  private categoryService = inject(CategoriesService);
  private regionService = inject(RegionsService);

  categories: CategoryDto[] = [];
  regions: RegionDto[] = [];
  selectedCategory = '';
  selectedSubCategory = '';
  selectedRegion = '';

  onProfileClick() {
    if (this.user) {
      this.router.navigate(['/profile']);
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

  search(): void {
    const query = this.searchText.trim();
    if (!query) {
      this.router.navigate(['/showcase']);
    } else {
      this.router.navigate(['/showcase'], {
        queryParams: { search: query },
        queryParamsHandling: 'merge',
      });
    }
  }
}
