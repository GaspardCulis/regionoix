import { inject, Injectable } from '@angular/core';
import { CanActivate, Router, UrlTree } from '@angular/router';
import { AuthService } from './auth-service';
import { Observable, of } from 'rxjs';
import { map, catchError } from 'rxjs/operators';

@Injectable({
  providedIn: 'root',
})
export class NoAuthGuard implements CanActivate {
  private readonly router = inject(Router);
  private readonly authService = inject(AuthService);

  canActivate(): Observable<boolean | UrlTree> {
    return this.authService.status().pipe(
      map(() => {
        return this.router.createUrlTree(['/connection']);
      }),
      catchError(() => {
        return of(true);
      })
    );
  }
}
