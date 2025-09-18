import { ComponentFixture, TestBed } from '@angular/core/testing';
import { of } from 'rxjs';
import { ActivatedRoute } from '@angular/router';
import { ProductPage } from './product-page';
import { BasketService, ProductsService, ProductDto } from '../../generated/clients/regionoix-client';
import { BasketStateService } from '../../services/basket-state-service';
import { AuthStateService } from '../../services/auth-state-service';
import { SnackbarService } from '../../services/snackbar-service';
import { HttpResponse } from '@angular/common/http';

describe('ProductPage', () => {
  let component: ProductPage;
  let fixture: ComponentFixture<ProductPage>;

  // 🔹 Spies pour services
  let basketServiceSpy: jasmine.SpyObj<BasketService>;
  let basketStateServiceSpy: jasmine.SpyObj<BasketStateService>;
  let authStateServiceSpy: jasmine.SpyObj<AuthStateService>;
  let productsServiceSpy: jasmine.SpyObj<ProductsService>;
  let snackbarServiceSpy: jasmine.SpyObj<SnackbarService>;

  const mockProduct: ProductDto = {
    id: 1,
    name: 'Produit Test',
    price: 100,
    stock: 10,
    description: 'Super produit',
    image: null,
    tags: [],
  } as any;

  beforeEach(async () => {
    basketServiceSpy = jasmine.createSpyObj('BasketService', ['add']);
    basketStateServiceSpy = jasmine.createSpyObj('BasketStateService', ['refreshCount']);
    authStateServiceSpy = jasmine.createSpyObj('AuthStateService', [], { currentUser: { id: 123 } });
    productsServiceSpy = jasmine.createSpyObj('ProductsService', ['getById']);
    snackbarServiceSpy = jasmine.createSpyObj('SnackbarService', ['show']);

    productsServiceSpy.getById.and.returnValue(of(new HttpResponse({ body: mockProduct })));

    await TestBed.configureTestingModule({
      imports: [ProductPage],
      providers: [
        { provide: BasketService, useValue: basketServiceSpy },
        { provide: BasketStateService, useValue: basketStateServiceSpy },
        { provide: AuthStateService, useValue: authStateServiceSpy },
        { provide: ProductsService, useValue: productsServiceSpy },
        { provide: SnackbarService, useValue: snackbarServiceSpy },
        {
          provide: ActivatedRoute,
          useValue: { snapshot: { paramMap: new Map([['id', '1']]) } }
        }
      ]
    }).compileComponents();

    fixture = TestBed.createComponent(ProductPage);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should load product on init', () => {
    expect(productsServiceSpy.getById).toHaveBeenCalledWith(1);
    expect(component.product).toBeTruthy();
    expect(component.product.image).toBe('assets/default.png'); // car null → fallback
  });

  it('should add product to basket if user logged in', () => {
    basketServiceSpy.add.and.returnValue(of(new HttpResponse({ body: {} as any })));
    component.product = mockProduct;
    component.addItem(mockProduct.id);
    expect(basketServiceSpy.add).toHaveBeenCalledWith({ product_id: mockProduct.id, quantity: 1 });
    expect(snackbarServiceSpy.show).toHaveBeenCalledWith('Produit ajouté au panier ✅', 'success');
    expect(basketStateServiceSpy.refreshCount).toHaveBeenCalled();
  });

  it('should not add product if user not logged in', () => {
    (Object.getOwnPropertyDescriptor(authStateServiceSpy, 'currentUser')?.get as jasmine.Spy)
      .and.returnValue(null);
    component.addItem(mockProduct.id);
    expect(snackbarServiceSpy.show).toHaveBeenCalledWith(
      'Veuillez vous connecter pour ajouter au panier !',
      'error'
    );
    expect(basketServiceSpy.add).not.toHaveBeenCalled();
  });

  it('should increase and decrease quantity', () => {
    component.product = mockProduct;
    expect(component.quantity).toBe(1);
    component.increaseQuantity();
    expect(component.quantity).toBe(2);
    component.decreaseQuantity();
    expect(component.quantity).toBe(1);
  });
});
