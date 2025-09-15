import { Component, Input, inject, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { Router } from '@angular/router';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';
import { faBasketShopping } from '@fortawesome/free-solid-svg-icons';
import { AuthentificationService, CategoriesService, CategoryDto, LoggedUser, RegionDto, RegionsService } from '../../../generated/clients/regionoix-client';

@Component({
  selector: 'app-topbar',
  standalone: true,
  imports: [
    CommonModule,
    FormsModule, FontAwesomeModule
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
  @Input() user!: LoggedUser | null;
  faBasketShopping = faBasketShopping;
  searchText = '';

  private categoryService = inject(CategoriesService);
  private regionService = inject(RegionsService)

  categories: CategoryDto[] = [];
  regions: RegionDto[] = [];
  selectedCategory = '';
  selectedSubCategory = '';
  selectedRegion = '';


  ngOnInit(): void {
    this.userService.status().subscribe({
      next: (user) => this.user = user,
      error: () => this.user = null
    });
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