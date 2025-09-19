import { ComponentFixture, TestBed, fakeAsync, tick } from '@angular/core/testing';
import { FormsModule, NgForm } from '@angular/forms';
import { Router } from '@angular/router';
import { of, throwError } from 'rxjs';
import { CreateAccount } from './create-account';
import { SnackbarService } from '../../services/snackbar-service';
import { ClientService, LoggedUser, Roles } from '../../generated/clients/regionoix-client';
import { HttpResponse } from '@angular/common/http';

describe('CreateAccount', () => {
  let component: CreateAccount;
  let fixture: ComponentFixture<CreateAccount>;
  let routerSpy: jasmine.SpyObj<Router>;
  let snackBarSpy: jasmine.SpyObj<SnackbarService>;
  let clientServiceSpy: jasmine.SpyObj<ClientService>;

  const mockLoggedUser: LoggedUser = {
    id: 1,
    email: 'john@example.com',
    role: Roles.Client
  };

  beforeEach(async () => {
    routerSpy = jasmine.createSpyObj('Router', ['navigate']);
    snackBarSpy = jasmine.createSpyObj('SnackbarService', ['show']);
    clientServiceSpy = jasmine.createSpyObj('ClientService', ['register']);

    await TestBed.configureTestingModule({
      imports: [FormsModule, CreateAccount],
      providers: [
        { provide: Router, useValue: routerSpy },
        { provide: SnackbarService, useValue: snackBarSpy },
        { provide: ClientService, useValue: clientServiceSpy }
      ]
    }).compileComponents();

    fixture = TestBed.createComponent(CreateAccount);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should show error if form is invalid', () => {
    const form = { invalid: true } as NgForm;
    component.onRegister(form);
    expect(snackBarSpy.show).toHaveBeenCalledWith(
      'Veuillez remplir correctement tous les champs.', 'error'
    );
  });

  it('should show error if passwords do not match', () => {
    const form = { invalid: false, resetForm: () => { } } as NgForm;
    component.password = 'password1';
    component.confirmPassword = 'password2';
    component.onRegister(form);
    expect(snackBarSpy.show).toHaveBeenCalledWith(
      'Les mots de passe ne correspondent pas.', 'error'
    );
  });

  it('should show error if password too short', () => {
    const form = { invalid: false, resetForm: () => { } } as NgForm;
    component.password = 'short';
    component.confirmPassword = 'short';
    component.onRegister(form);
    expect(snackBarSpy.show).toHaveBeenCalledWith(
      'Le mot de passe doit contenir au minimum 8 cractères', 'error'
    );
  });

  it('should show error if email invalid', () => {
    const form = { invalid: false, resetForm: () => { } } as NgForm;
    component.password = 'password123';
    component.confirmPassword = 'password123';
    component.email = 'invalid-email';
    component.onRegister(form);
    expect(snackBarSpy.show).toHaveBeenCalledWith(
      'Veuillez saisir une adresse email correct.', 'error'
    );
  });

  it('should register successfully and navigate to login', fakeAsync(() => {
    const form = { invalid: false, resetForm: jasmine.createSpy('resetForm') } as unknown as NgForm;

    component.lastname = 'Doe';
    component.fisrtname = 'John';
    component.email = 'john@example.com';
    component.password = 'password123';
    component.confirmPassword = 'password123';

    clientServiceSpy.register.and.returnValue(of(new HttpResponse({ body: mockLoggedUser })));

    component.onRegister(form);
    tick();

    expect(snackBarSpy.show).toHaveBeenCalledWith('Compte créé avec succès !', 'success');
    expect(form.resetForm).toHaveBeenCalled();
    expect(routerSpy.navigate).toHaveBeenCalledWith(['/connection']);
  }));

  it('should show error if registration fails', fakeAsync(() => {
    const form = { invalid: false, resetForm: () => { } } as NgForm;

    component.lastname = 'Doe';
    component.fisrtname = 'John';
    component.email = 'john@example.com';
    component.password = 'password123';
    component.confirmPassword = 'password123';

    clientServiceSpy.register.and.returnValue(throwError(() => new Error('fail')));

    component.onRegister(form);
    tick();

    expect(snackBarSpy.show).toHaveBeenCalledWith(
      'Une erreur est survenue, veuillez contacter le support', 'error'
    );
  }));

  it('should navigate to login page when goToLogin is called', () => {
    component.goToLogin();
    expect(routerSpy.navigate).toHaveBeenCalledWith(['/connection']);
  });
});
