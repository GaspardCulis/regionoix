import { ComponentFixture, TestBed } from '@angular/core/testing';
import { BackofficeProduct } from './backoffice-product';
import { of } from 'rxjs';
import { Router } from '@angular/router';
import { ActivatedRoute } from '@angular/router';
import { ProductsService, ProductDto } from '../../generated/clients/regionoix-client';

class MockProductsService {
  getById(id: number) {
    return of({ id, name: 'Test', price: 10, stock: 5 } as ProductDto);
  }
}

describe('BackofficeProduct', () => {
  let component: BackofficeProduct;
  let fixture: ComponentFixture<BackofficeProduct>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [BackofficeProduct],
      providers: [
        { provide: ProductsService, useClass: MockProductsService },
        { provide: Router, useValue: { navigate: jasmine.createSpy('navigate') } },
        { provide: ActivatedRoute, useValue: { snapshot: { paramMap: new Map([['id', '1']]) } } }
      ]
    }).compileComponents();

    fixture = TestBed.createComponent(BackofficeProduct);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
