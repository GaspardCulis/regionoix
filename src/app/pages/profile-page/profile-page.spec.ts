import { ComponentFixture, fakeAsync, TestBed, tick } from '@angular/core/testing';
import { of, throwError } from 'rxjs';
import { Router } from '@angular/router';
import { RouterTestingModule } from '@angular/router/testing';
import { HttpResponse } from '@angular/common/http';

import { ProfilePage } from './profile-page';
import { AuthentificationService, LoggedUser, OrderDto, OrdersService } from '../../generated/clients/regionoix-client';
import { SnackbarService } from '../../services/snackbar-service';
import { AuthStateService } from '../../services/auth-state-service';
import { BasketStateService } from '../../services/basket-state-service';

describe('ProfilePage', () => {
  let component: ProfilePage;
  let fixture: ComponentFixture<ProfilePage>;

  let authServiceSpy: jasmine.SpyObj<AuthentificationService>;
  let authStateSpy: jasmine.SpyObj<AuthStateService>;
  let ordersServiceSpy: jasmine.SpyObj<OrdersService>;
  let snackbarSpy: jasmine.SpyObj<SnackbarService>;
  let basketStateSpy: jasmine.SpyObj<BasketStateService>;

  const mockUser: LoggedUser = {
    id: 1,
    firstname: 'John',
    lastname: 'Doe',
    email: 'john@example.com',
  } as any;

  const mockOrders: OrderDto[] = [
    {
      id: 101,
      total_price: 50,
      arrival_date: new Date().toISOString(),
      order_lines: [
        { product: { name: 'Produit X', price: 25, image: null } as any, quantity: 2 },
      ],
    } as any,
  ];

  beforeEach(async () => {
    authServiceSpy = jasmine.createSpyObj('AuthentificationService', ['status', 'logout']);
    authStateSpy = jasmine.createSpyObj('AuthStateService', ['notifyAuthChanged']);
    ordersServiceSpy = jasmine.createSpyObj('OrdersService', ['get']);
    snackbarSpy = jasmine.createSpyObj('SnackbarService', ['show']);
    basketStateSpy = jasmine.createSpyObj('BasketStateService', ['refreshCount']);

    authServiceSpy.status.and.returnValue(of(new HttpResponse({ body: mockUser })));
    ordersServiceSpy.get.and.returnValue(of(new HttpResponse({ body: mockOrders })));
    authServiceSpy.logout.and.returnValue(of(new HttpResponse({ body: {} })));

    await TestBed.configureTestingModule({
      imports: [ProfilePage, RouterTestingModule],
      providers: [
        { provide: AuthentificationService, useValue: authServiceSpy },
        { provide: OrdersService, useValue: ordersServiceSpy },
        { provide: SnackbarService, useValue: snackbarSpy },
        { provide: AuthStateService, useValue: authStateSpy },
        { provide: BasketStateService, useValue: basketStateSpy },
      ],
    }).compileComponents();

    fixture = TestBed.createComponent(ProfilePage);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should detect delivered order', () => {
    const deliveredOrder: OrderDto = { ...mockOrders[0], arrival_date: '2020-01-01' } as any;
    expect(component.isDelivered(deliveredOrder)).toBeTrue();
  });

  it('should logout successfully', () => {
    const router = TestBed.inject(Router);
    spyOn(router, 'navigate');

    component.logout();

    expect(authServiceSpy.logout).toHaveBeenCalled();
    expect(authStateSpy.notifyAuthChanged).toHaveBeenCalled();
    expect(basketStateSpy.refreshCount).toHaveBeenCalled();
    expect(router.navigate).toHaveBeenCalledWith(['/connection']);
  });

  it('should show error when status fails', () => {
    authServiceSpy.status.and.returnValue(throwError(() => new Error('fail')));
    component.loadUser();
    expect(snackbarSpy.show).toHaveBeenCalledWith(
      'Erreur lors de la récupération des informations utilisateur',
      'error'
    );
  });
});
