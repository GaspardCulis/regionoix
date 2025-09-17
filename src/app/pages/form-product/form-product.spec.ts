import { ComponentFixture, TestBed } from '@angular/core/testing';
import { FormProduct } from './form-product';
import { of } from 'rxjs';
import { Router } from '@angular/router';
import { CategoriesService, BrandsService, TagsService, RegionsService, AdminService } from '../../generated/clients/regionoix-client';
import { SnackbarService } from '../../services/snackbar-service';

class MockCategoriesService { get() { return of([]); } }
class MockBrandsService { get() { return of([]); } }
class MockTagsService { get() { return of([]); } }
class MockRegionsService { get() { return of([]); } }
class MockAdminService { upload(file: File, meta: any) { return of(null); } }
class MockSnackbarService { show(message: string, type: string) { } }

describe('FormProduct', () => {
  let component: FormProduct;
  let fixture: ComponentFixture<FormProduct>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [FormProduct],
      providers: [
        { provide: CategoriesService, useClass: MockCategoriesService },
        { provide: BrandsService, useClass: MockBrandsService },
        { provide: TagsService, useClass: MockTagsService },
        { provide: RegionsService, useClass: MockRegionsService },
        { provide: AdminService, useClass: MockAdminService },
        { provide: SnackbarService, useClass: MockSnackbarService },
        { provide: Router, useValue: { navigate: jasmine.createSpy('navigate') } }
      ]
    }).compileComponents();

    fixture = TestBed.createComponent(FormProduct);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
