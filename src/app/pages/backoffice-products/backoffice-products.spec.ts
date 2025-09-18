import { ComponentFixture, TestBed } from '@angular/core/testing';
import { BackofficeProducts } from './backoffice-products';
import { of } from 'rxjs';
import { Router } from '@angular/router';
import { ProductsService, ProductDto } from '../../generated/clients/regionoix-client';
import { SnackbarService } from '../../services/snackbar-service';

class MockProductsService {
  get() {
    return of([] as ProductDto[]);
  }
  deleteById(id: number) {
    return of(null);
  }
}

class MockSnackbarService {
  show(message: string, type: string) { }
}

describe('BackofficeProducts', () => {
  let component: BackofficeProducts;
  let fixture: ComponentFixture<BackofficeProducts>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [BackofficeProducts],
      providers: [
        { provide: ProductsService, useClass: MockProductsService },
        { provide: SnackbarService, useClass: MockSnackbarService },
        { provide: Router, useValue: { navigate: jasmine.createSpy('navigate') } }
      ]
    }).compileComponents();

    fixture = TestBed.createComponent(BackofficeProducts);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
