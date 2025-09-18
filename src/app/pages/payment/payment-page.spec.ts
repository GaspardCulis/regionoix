import { ComponentFixture, TestBed } from '@angular/core/testing';
import { of } from 'rxjs';
import { Router } from '@angular/router';
import { PaymentPage } from './payment-page';
import { AuthentificationService, BasketService, LoggedUser } from '../../generated/clients/regionoix-client';
import { SnackbarService } from '../../services/snackbar-service';
import { RouterTestingModule } from '@angular/router/testing';

describe('PaymentPage', () => {
  let component: PaymentPage;
  let fixture: ComponentFixture<PaymentPage>;
  let authServiceSpy: jasmine.SpyObj<AuthentificationService>;
  let basketServiceSpy: jasmine.SpyObj<BasketService>;
  let snackBarSpy: jasmine.SpyObj<SnackbarService>;

  const mockUser: LoggedUser = {
    id: 1,
    email: 'test@example.com',
    firstname: 'John',
    lastname: 'Doe',
    role: 'Client'
  };

  beforeEach(async () => {
    authServiceSpy = jasmine.createSpyObj('AuthentificationService', ['status']);
    basketServiceSpy = jasmine.createSpyObj('BasketService', ['get', 'make']);
    snackBarSpy = jasmine.createSpyObj('SnackbarService', ['show']);

    await TestBed.configureTestingModule({
      imports: [PaymentPage, RouterTestingModule],
      providers: [
        { provide: AuthentificationService, useValue: authServiceSpy },
        { provide: BasketService, useValue: basketServiceSpy },
        { provide: SnackbarService, useValue: snackBarSpy }
      ]
    }).compileComponents();

    fixture = TestBed.createComponent(PaymentPage);
    component = fixture.componentInstance;
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should load client on init', () => {
    // ⚡ cast any pour bypass TS
    authServiceSpy.status.and.returnValue(of(mockUser as any));

    fixture.detectChanges();

    expect(component.client).toEqual(mockUser);
  });

  it('should go to next step', () => {
    component.currentStep = 1;
    component.nextStep();
    expect(component.currentStep).toBe(2);
  });

  it('should go to previous step', () => {
    component.currentStep = 3;
    component.prevStep();
    expect(component.currentStep).toBe(2);
  });

  it('should handle payment step without error', () => {
    component.currentStep = 3;
    authServiceSpy.status.and.returnValue(of(mockUser as any));
    basketServiceSpy.get.and.returnValue(of({} as any));
    basketServiceSpy.make.and.returnValue(of({} as any));

    component.nextStep();

    expect(component.currentStep).toBe(4);
    expect(snackBarSpy.show).toHaveBeenCalledWith('Paiement validé avec succès ✅', 'success');
  });
});
