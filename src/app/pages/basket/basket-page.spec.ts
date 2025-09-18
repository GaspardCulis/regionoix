import { ComponentFixture, TestBed, fakeAsync, tick } from '@angular/core/testing';
import { of, throwError } from 'rxjs';
import { Router } from '@angular/router';
import { BasketPage } from './basket-page';
import { BasketService, CartLineDto, CartDto } from '../../generated/clients/regionoix-client';
import { BasketStateService } from '../../services/basket-state-service';
import { ProductListItemComponent } from '../../utils/component/product-list-item-component/product-list-item-component';
import { HttpResponse } from '@angular/common/http';
import { RouterTestingModule } from '@angular/router/testing';

describe('BasketPage', () => {
  let component: BasketPage;
  let fixture: ComponentFixture<BasketPage>;
  let basketServiceSpy: jasmine.SpyObj<BasketService>;
  let basketStateSpy: jasmine.SpyObj<BasketStateService>;

  const mockLines: CartLineDto[] = [
    {
      product: { id: 1, price: 100, discount: { percentage_off: 10 } } as any,
      quantity: 2,
      cart_id: 0,
      id: 0
    },
    {
      product: { id: 2, price: 50 } as any,
      quantity: 1,
      cart_id: 0,
      id: 0
    }
  ];

  const mockCart: CartDto = {
    id: 1,
    user_id: 123,
    lines: mockLines
  };

  beforeEach(async () => {
    basketServiceSpy = jasmine.createSpyObj('BasketService', ['get', 'remove', 'updateQuantity', 'empty']);
    basketStateSpy = jasmine.createSpyObj('BasketStateService', ['refreshCount']);

    await TestBed.configureTestingModule({
      imports: [BasketPage, ProductListItemComponent, RouterTestingModule],
      providers: [
        { provide: BasketService, useValue: basketServiceSpy },
        { provide: BasketStateService, useValue: basketStateSpy }
      ]
    }).compileComponents();

    fixture = TestBed.createComponent(BasketPage);
    component = fixture.componentInstance;
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
  it('should calculate total price correctly', () => {
    component.lines = mockLines;
    const total = component.getTotalPrice();
    // Ligne 1: 100 - 10% = 90 * 2 = 180
    // Ligne 2: 50 * 1 = 50
    // Total = 230
    expect(total).toBe(230);
  });

  it('should navigate to payment', () => {
    const router = TestBed.inject(Router);
    spyOn(router, 'navigate');
    component.goToPayment();
    expect(router.navigate).toHaveBeenCalledWith(['/payment']);
  });

  it('should remove item and reload basket', fakeAsync(() => {
    basketServiceSpy.remove.and.returnValue(of(new HttpResponse({ body: 'ok' })));
    basketServiceSpy.get.and.returnValue(of(new HttpResponse({ body: mockCart })));
    component.removeItem(1);
    tick();
    expect(basketServiceSpy.remove).toHaveBeenCalledWith(1);
    expect(basketServiceSpy.get).toHaveBeenCalled();
  }));

  const mockLine: CartLineDto = {
    id: 1,
    cart_id: 1,
    quantity: 2,
    product: { id: 1, price: 100 } as any
  };

  it('should change quantity and reload basket', fakeAsync(() => {
    basketServiceSpy.updateQuantity.and.returnValue(
      of(new HttpResponse({ body: mockLine }))
    );
    basketServiceSpy.get.and.returnValue(of(new HttpResponse({ body: mockCart })));
    component.changeQuantity(1, 5);
    tick();
    expect(basketServiceSpy.updateQuantity).toHaveBeenCalledWith(1, { quantity: 5 });
    expect(basketServiceSpy.get).toHaveBeenCalled();
  }));

  it('should empty basket and reload', fakeAsync(() => {
    basketServiceSpy.empty.and.returnValue(of(new HttpResponse({ body: 'ok' })));
    basketServiceSpy.get.and.returnValue(of(new HttpResponse({ body: mockCart })));
    component.emptyBasket();
    tick();
    expect(basketServiceSpy.empty).toHaveBeenCalled();
    expect(basketServiceSpy.get).toHaveBeenCalled();
  }));

  it('should handle error during loadBasket', fakeAsync(() => {
    spyOn(console, 'error');
    basketServiceSpy.get.and.returnValue(throwError(() => new Error('fail')));
    component.loadBasket();
    tick();
    expect(console.error).toHaveBeenCalledWith('Error during basket recuperation', jasmine.any(Error));
  }));
});
