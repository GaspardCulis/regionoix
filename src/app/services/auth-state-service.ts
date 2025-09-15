import { inject, Injectable } from '@angular/core';
import { BehaviorSubject, of } from 'rxjs';
import { catchError } from 'rxjs/operators';
import { AuthentificationService, LoggedUser } from '../generated/clients/regionoix-client';

@Injectable({ providedIn: 'root' })
export class AuthStateService {
  private readonly userSubject = new BehaviorSubject<LoggedUser | null>(null);
  user$ = this.userSubject.asObservable();

  authService = inject(AuthentificationService);

  constructor() {
    this.refreshStatus();
  }

  refreshStatus(): void {
    this.authService
      .status()
      .pipe(catchError(() => of(null)))
      .subscribe((user) => this.userSubject.next(user));
  }

  // Call at each login or logout
  notifyAuthChanged(): void {
    this.refreshStatus();
  }

  get currentUser(): LoggedUser | null {
    return this.userSubject.value;
  }
}
