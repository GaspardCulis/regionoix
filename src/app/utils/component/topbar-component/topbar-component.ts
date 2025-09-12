import { Component, Input, Output, inject, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { Router } from '@angular/router';
import { AuthentificationService, CategoriesService, CategoryDto, RegionDto, RegionsService } from '../../../generated/clients/regionoix-client';
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
  private authCookieService = inject(AuthentificationService);

  @Input() pathLogo!: string;
  @Input() title!: string;
  @Input() basketCount = 0;
  @Input() user!: string | null;
  @Output() searchText = '';

  ngOnInit(): void {
    this.authCookieService.user$.subscribe(user => {
      this.user = user;
    });
    this.categoryService.getHierarchy().subscribe({
      next: (data) => {
        this.categories = data;
      },
      error: (err) => {
        console.error('Something went wrong during categories recuperation', err);
      }
    });
    this.regionService.getRegions().subscribe({
      next: (data) => {
        this.regions = data;
      },
      error: (err) => {
        console.error('Something went wrong during regions recuperation', err);
      }
    });
  }
  private categoryService = inject(CategoriesService);
  private regionService = inject(RegionsService)

  categories: CategoryDto[] = [];
  regions: RegionDto[] = [];
  selectedCategory = '';
  selectedSubCategory = '';
  selectedRegion = '';



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
