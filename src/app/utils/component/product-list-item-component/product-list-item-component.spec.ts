import { ComponentFixture, TestBed } from '@angular/core/testing';
import { ProductListItemComponent } from './product-list-item-component';
import { ProductDto } from '../../../generated/clients/regionoix-client';
import { By } from '@angular/platform-browser';

describe('ProductListItemComponent', () => {
  let component: ProductListItemComponent;
  let fixture: ComponentFixture<ProductListItemComponent>;

  const mockProduct: ProductDto = {
    id: 1,
    name: 'Test Product',
    price: 100,
    stock: 10,
    image: 'test.jpg',
    discount: { percentage_off: 20 } as any
  } as any;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ProductListItemComponent]
    }).compileComponents();

    fixture = TestBed.createComponent(ProductListItemComponent);
    component = fixture.componentInstance;

    component.product = mockProduct;
    component.quantity = 1;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should calculate final_price on init', () => {
    component.ngOnInit();
    expect(component.final_price).toBe(80); // 100 - 20%
  });

  it('should increase quantity', () => {
    spyOn(component.quantityChange, 'emit');
    component.increaseQuantity();
    expect(component.quantity).toBe(2);
    expect(component.quantityChange.emit).toHaveBeenCalledWith({ productId: 1, quantity: 2 });
  });

  it('should not increase quantity beyond stock', () => {
    spyOn(component.quantityChange, 'emit');
    component.quantity = 10;
    component.increaseQuantity();
    expect(component.quantity).toBe(10);
  });

  it('should decrease quantity', () => {
    spyOn(component.quantityChange, 'emit');
    component.quantity = 2;
    component.decreaseQuantity();
    expect(component.quantity).toBe(1);
    expect(component.quantityChange.emit).toHaveBeenCalledWith({ productId: 1, quantity: 1 });
  });

  it('should not decrease quantity below 1', () => {
    spyOn(component.quantityChange, 'emit');
    component.quantity = 1;
    component.decreaseQuantity();
    expect(component.quantity).toBe(1);
  });

  it('should emit removeFromBasket', () => {
    spyOn(component.removeFromBasket, 'emit');
    component.onRemoveFromBasket();
    expect(component.removeFromBasket.emit).toHaveBeenCalledWith(1);
  });
});
