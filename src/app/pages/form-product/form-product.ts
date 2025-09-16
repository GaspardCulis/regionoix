import { Component, inject, OnInit } from '@angular/core';
import { Router } from '@angular/router';
import { faArrowLeft, faCircleXmark } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';
import { FormArray, FormControl, FormGroup, ReactiveFormsModule, Validators } from '@angular/forms';
import {
  AdminService,
  BrandDto,
  BrandsService,
  CategoriesService,
  CategoryDto,
  RegionDto,
  RegionsService,
  TagDto,
  TagsService,
  UploadFormMeta,
} from '../../generated/clients/regionoix-client';
import { SnackbarService } from '../../services/snackbar-service';

@Component({
  selector: 'app-form-product',
  imports: [FontAwesomeModule, ReactiveFormsModule],
  templateUrl: './form-product.html',
  styleUrl: './form-product.css',
})
export class FormProduct implements OnInit {
  // font awesome icons plus
  faCircleXmark = faCircleXmark;
  faArrowLeft = faArrowLeft;

  hasTriedSubmit = false;

  private readonly categoriesService = inject(CategoriesService);
  private readonly adminService = inject(AdminService);
  private readonly regionService = inject(RegionsService);
  private readonly tagsService = inject(TagsService);
  private readonly brandsService = inject(BrandsService);
  private readonly snackbarService = inject(SnackbarService);

  private router = inject(Router);

  productForm = new FormGroup({
    name: new FormControl('', [Validators.required, Validators.minLength(3)]),
    stock: new FormControl(null, [Validators.required, Validators.min(0)]),
    brand: new FormControl('', Validators.required),
    category: new FormControl('', Validators.required),
    region: new FormControl('', Validators.required),
    image: new FormControl<null | File>(null, Validators.required),
    weight: new FormControl(null),
    price: new FormControl(null, [Validators.required, Validators.min(0.1)]),
    description: new FormControl(null),
    tags: new FormArray([]),
  });
  tagsArray = new FormArray([] as FormControl<boolean>[]);
  categories: CategoryDto[] = [];
  tags: TagDto[] = [];
  brands: BrandDto[] = [];
  regions: RegionDto[] = [];

  ngOnInit(): void {
    this.categoriesService.get().subscribe({
      next: (data) => {
        this.categories = data;
      },
    });

    this.tagsService.get().subscribe((tags) => {
      this.tags = tags;
    });

    this.regionService.get().subscribe((regions) => {
      this.regions = regions;
    });

    this.brandsService.get().subscribe((brands) => {
      this.brands = brands;
    });
  }

  onBack(): void {
    this.router.navigate(['/backoffice/products']);
  }

  onUploadImage(event: Event): void {
    const input = event.target as HTMLInputElement;

    const file = input?.files?.[0];

    if (file) {
      this.productForm.get('image')?.setValue(file);
    }
  }

  onSubmit(): void {
    this.hasTriedSubmit = true;

    if (this.productForm.valid) {
      const imageFile = this.productForm.get('image')?.value as File;

      if (!imageFile) {
        console.error('No image selected');
        return;
      }

      const metadata: UploadFormMeta = {
        name: this.productForm.get('name')?.value as string,
        price: Number(this.productForm.get('price')?.value),
        stock: Number(this.productForm.get('stock')?.value),
        description: this.productForm.get('description')?.value || null,
        weight: this.productForm.get('weight')?.value || null,
        brand_id: Number(this.productForm.get('brand')?.value) || null,
        category_id: Number(this.productForm.get('category')?.value) || null,
        region_id: Number(this.productForm.get('region')?.value) || null,
        tags: (this.productForm.get('tags')?.value as number[]) || [],
      };

      console.log(metadata)

      // Call the API
      this.adminService.upload(imageFile, metadata).subscribe({
        next: (response) => {
          this.snackbarService.show('Produit ajouté ✅', 'success');
          console.log('Upload successful', response);
          this.router.navigate(['/backoffice/products']);
        },
        error: (err) => {
          this.snackbarService.show('Echec de la création du produit ', 'error');
          console.error('Upload failed', err);
        },
      });
    }
  }

  onResetTags(): void {
    const formArray = this.productForm.get('tags') as FormArray;
    formArray.clear();
  }

  onCheckChangeTag(event: Event) {
    const input = event.target as HTMLInputElement;
    const formArray = this.productForm.get('tags') as FormArray;
    const value = Number(input.value);

    if (input.checked) {
      if (!formArray.value.includes(value)) {
        formArray.push(new FormControl(value));
      }
    } else {
      const index = formArray.controls.findIndex((ctrl) => ctrl.value === value);
      if (index !== -1) {
        formArray.removeAt(index);
      }
    }
  }
  isTagSelected(tagId: number): boolean {
    const formArray = this.productForm.get('tags') as FormArray;
    return formArray.value.includes(tagId.toString());
  }

}
