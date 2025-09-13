import { Component, inject, OnInit } from '@angular/core';
import { AdminMenu } from '../../utils/admin-menu/admin-menu';
import { Router } from '@angular/router';
import { faArrowLeft, faCircleXmark } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';
import { FormArray, FormControl, FormGroup, ReactiveFormsModule, Validators } from '@angular/forms';
import {
  BrandDto,
  CategoriesService,
  CategoryDto,
  TagDto,
  TagsService,
} from '../../generated/clients/regionoix-client';

@Component({
  selector: 'app-form-product',
  imports: [AdminMenu, FontAwesomeModule, ReactiveFormsModule],
  templateUrl: './form-product.html',
  styleUrl: './form-product.css',
})
export class FormProduct implements OnInit {
  faCircleXmark = faCircleXmark;
  faArrowLeft = faArrowLeft;
  hasTriedSubmit = false;

  private readonly categoriesService = inject(CategoriesService);
  private readonly tagsService = inject(TagsService);

  private router = inject(Router);

  productForm = new FormGroup({
    name: new FormControl('', [Validators.required, Validators.minLength(3)]),
    stock: new FormControl(null, [Validators.required, Validators.min(0)]),
    brand: new FormControl(''),
    category: new FormControl('', Validators.required),
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

  ngOnInit(): void {
    this.categoriesService.get().subscribe({
      next: (data) => {
        this.categories = data;
      },
    });

    this.tagsService.get().subscribe((tags) => {
      this.tags = tags;
    });

    //TODO get brands
  }

  onBack(): void {
    this.router.navigate(['/backoffice/products']);
  }

  onUploadImage(event: Event): void {
    const input = event.target as HTMLInputElement;

    const file = input?.files?.[0];

    if (file) {
      console.log(file);
      this.productForm.get('image')?.setValue(file);
    }
  }

  onSubmit(): void {
    this.hasTriedSubmit = true;
    if (this.productForm.valid) {
      // valid
      // TODO call api
    }
    console.log(this.productForm.controls);
  }

  onResetTags(): void {
    const formArray = this.productForm.get('tags') as FormArray;
    formArray.clear();
  }

  onCheckChangeTag(event: Event) {
    const input = event.target as HTMLInputElement;
    const formArray = this.productForm.get('tags') as FormArray;

    if (input.checked) {
      formArray.push(new FormControl(input.value));
    } else {
      const index = formArray.controls.findIndex((ctrl) => ctrl.value === input.value);
      if (index !== -1) {
        formArray.removeAt(index);
      }
    }
  }
}
