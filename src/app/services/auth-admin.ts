import { inject, Injectable } from '@angular/core';
import { CanActivate, Router, UrlTree } from '@angular/router';
import { AuthService } from './auth-service';
import { Observable, of } from 'rxjs';
import { map, catchError } from 'rxjs/operators';
import { Roles } from '../generated/clients/regionoix-client';

@Injectable({ providedIn: 'root' })
export class AdminGuard implements CanActivate {
    private readonly router = inject(Router);

    private readonly authService = inject(AuthService);


    canActivate(): Observable<boolean | UrlTree> {
        return this.authService.status().pipe(
            map((data) => {
                if (data.role == Roles.Admin) {
                    return true;
                } else {
                    return false;
                }
            }),
            catchError(() => {
                return of(this.router.createUrlTree(['/']));
            })
        );
    }
}