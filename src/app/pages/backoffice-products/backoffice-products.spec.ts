import { ComponentFixture, TestBed } from '@angular/core/testing';

import { BackofficeProducts } from './backoffice-products';

describe('BackofficeProducts', () => {
  let component: BackofficeProducts;
  let fixture: ComponentFixture<BackofficeProducts>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [BackofficeProducts]
    })
    .compileComponents();

    fixture = TestBed.createComponent(BackofficeProducts);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
