import { ComponentFixture, TestBed } from '@angular/core/testing';

import { BackofficeProduct } from './backoffice-product';

describe('BackofficeProduct', () => {
  let component: BackofficeProduct;
  let fixture: ComponentFixture<BackofficeProduct>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [BackofficeProduct]
    })
    .compileComponents();

    fixture = TestBed.createComponent(BackofficeProduct);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
