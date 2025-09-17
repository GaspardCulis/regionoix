import { ComponentFixture, fakeAsync, TestBed, tick } from '@angular/core/testing';
import { FormsModule } from '@angular/forms';
import { of, throwError, BehaviorSubject } from 'rxjs';
import { Router } from '@angular/router';
import { ConnectionPage } from './connection-page';
import { AuthentificationService, LoggedUser, Roles } from '../../generated/clients/regionoix-client';
import { AuthStateService } from '../../services/auth-state-service';
import { SnackbarService } from '../../services/snackbar-service';
import { BasketStateService } from '../../services/basket-state-service';
import { HttpResponse } from '@angular/common/http';

describe('ConnectionPage', () => {
  let component: ConnectionPage;
  let fixture: ComponentFixture<ConnectionPage>;
  let routerSpy: jasmine.SpyObj<Router>;
  let authServiceSpy: jasmine.SpyObj<AuthentificationService>;
  let authStateSpy: jasmine.SpyObj<AuthStateService>;
  let snackBarSpy: jasmine.SpyObj<SnackbarService>;
  let basketStateSpy: jasmine.SpyObj<BasketStateService>;

  beforeEach(async () => {
    routerSpy = jasmine.createSpyObj('Router', ['navigate']);
    authServiceSpy = jasmine.createSpyObj('AuthentificationService', ['login']);
    authStateSpy = jasmine.createSpyObj('AuthStateService', ['notifyAuthChanged'], {
      user$: new BehaviorSubject<any>(null)
    });
    snackBarSpy = jasmine.createSpyObj('SnackbarService', ['show']);
    basketStateSpy = jasmine.createSpyObj('BasketStateService', ['refreshCount']);

    await TestBed.configureTestingModule({
      imports: [FormsModule, ConnectionPage],
      providers: [
        { provide: Router, useValue: routerSpy },
        { provide: AuthentificationService, useValue: authServiceSpy },
        { provide: AuthStateService, useValue: authStateSpy },
        { provide: SnackbarService, useValue: snackBarSpy },
        { provide: BasketStateService, useValue: basketStateSpy },
      ]
    }).compileComponents();

    fixture = TestBed.createComponent(ConnectionPage);
    component = fixture.componentInstance;
  });

  it('should validate correct email', () => {
    component.email = 'test@example.com';
    expect(component.checkCredentials()).toBeTrue();
  });

  it('should invalidate incorrect email', () => {
    component.email = 'invalid-email';
    expect(component.checkCredentials()).toBeFalse();
  });

  it('should navigate to create-account page', () => {
    component.goToCreate();
    expect(routerSpy.navigate).toHaveBeenCalledWith(['/create-account']);
  });

  it('should login successfully and navigate based on role', fakeAsync(() => {
    component.email = 'user@example.com';
    component.password = 'password';

    const mockLoggedUser: LoggedUser = {
      id: 1,
      email: 'user@example.com',
      role: Roles.Admin
    };
    authServiceSpy.login.and.returnValue(of(new HttpResponse({ body: mockLoggedUser })));;
    const userSubject = new BehaviorSubject<LoggedUser | null>(null);
    authStateSpy.user$ = userSubject.asObservable();
    component.onSubmit();
    userSubject.next(mockLoggedUser);
    tick();

    expect(routerSpy.navigate).toHaveBeenCalledWith(['/backoffice']);
  }));

  it('should show error message on failed login', () => {
    component.email = 'user@example.com';
    component.password = 'password';
    authServiceSpy.login.and.returnValue(throwError(() => new Error('fail')));

    component.onSubmit();

    expect(snackBarSpy.show).toHaveBeenCalledWith(
      'Échec de la connexion. Veuillez vérifier vos identifiants et réessayer.', 'error'
    );
  });

  it('should show error for invalid email', () => {
    component.email = 'invalid-email';
    component.password = 'password';

    component.onSubmit();

    expect(snackBarSpy.show).toHaveBeenCalledWith('L’adresse e-mail saisie est invalide.', 'error');
  });
});
