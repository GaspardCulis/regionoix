import { ComponentFixture, TestBed } from '@angular/core/testing';

import { PaymentError } from './payment-error';

describe('PaymentError', () => {
  let component: PaymentError;
  let fixture: ComponentFixture<PaymentError>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [PaymentError]
    })
    .compileComponents();

    fixture = TestBed.createComponent(PaymentError);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
