import { ComponentFixture, TestBed } from '@angular/core/testing';
import { ProductCardComponent } from './product-card-component';
import { RouterTestingModule } from '@angular/router/testing';

describe('ProductCardComponent', () => {
  let component: ProductCardComponent;
  let fixture: ComponentFixture<ProductCardComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ProductCardComponent, RouterTestingModule]
    }).compileComponents();

    fixture = TestBed.createComponent(ProductCardComponent);
    component = fixture.componentInstance;

    // Inputs de base
    component.id = 1;
    component.name = 'Test Product';
    component.price = 100;
    component.discount = 20;

    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should calculate final_price if discount exists', () => {
    component.ngOnInit();
    expect(component.final_price).toBe(80); // 100 - 20%
  });

  it('should emit addToBasket event', () => {
    spyOn(component.addToBasket, 'emit');
    component.onAddToBasket();
    expect(component.addToBasket.emit).toHaveBeenCalledWith(1);
  });
});
