import { Component, Input, inject, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { Router } from '@angular/router';
import { CategoriesService, CategoryDto, LoggedUser, RegionDto, RegionsService } from '../../../generated/clients/regionoix-client';
import { AuthStateService } from '../../../services/auth-state-service';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';
import { faBasketShopping } from '@fortawesome/free-solid-svg-icons';
import { SnackbarService } from '../../../services/snackbar-service';

@Component({
  selector: 'app-topbar',
  standalone: true,
  imports: [CommonModule, FormsModule, FontAwesomeModule, FontAwesomeModule],
  templateUrl: './topbar-component.html',
  styleUrl: './topbar-component.css',
})
export class TopbarComponent implements OnInit {
  private router = inject(Router);
  private authService = inject(AuthStateService);
  private snackbar = inject(SnackbarService);
  // icon for basket shopping
  faBasketShopping = faBasketShopping;

  @Input() pathLogo!: string;
  @Input() title!: string;
  @Input() basketCount = 0;
  searchText = '';

  user: LoggedUser | null = null;

  ngOnInit() {
    this.authService.user$.subscribe((u) => (this.user = u));
    this.regionService.get().subscribe({
      next: (regions) => this.regions = regions,
      error: (err) => {
        console.error("Something went wrong on regions load :", err);
        this.regions = [];
      }
    });
    this.categoryService.getParents().subscribe({
      next: (categories) => {
        this.categories = categories.filter(c => c.category_parent === null);
      },
      error: (err) => {
        console.error("Something went wrong on categories load :", err);
        this.categories = [];
      }
    })
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
    if (this.user) {
      this.router.navigate(['/basket']);
    } else {
      this.snackbar.show('Vous devez être connecté pour accéder au panier.','info')
    }
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

  selectCategory(cat: CategoryDto) {
    this.selectedCategory = cat.name;
    this.router.navigate(['/showcase'], {
      queryParams: { c: cat.name },
      queryParamsHandling: 'merge'
    });
  }

  selectSubCategory(child: CategoryDto) {
    this.router.navigate(['/showcase'], {
      queryParams: { c: child.name },
      queryParamsHandling: 'merge'
    });
  }

  selectRegion(region: RegionDto) {
    this.selectedRegion = region.name;

    this.router.navigate(['/showcase'], {
      queryParams: { region: region.name },
      queryParamsHandling: 'merge'
    });
  }
}