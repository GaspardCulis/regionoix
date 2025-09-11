import { Injectable } from '@angular/core';
import { BehaviorSubject } from 'rxjs';

@Injectable({ providedIn: 'root' })
export class AuthCookieService {
    private userSubject = new BehaviorSubject<string | null>(null);
    user$ = this.userSubject.asObservable();

    constructor() {
        this.checkUserCookie();
    }

    private checkUserCookie(): void {
        const cookies = document.cookie.split(';').map(cookie => cookie.trim());
        console.log(`Cookies: ${document.cookie}`);
        const userCookie = cookies.find(cookie => cookie.startsWith('id='));
        const user = userCookie ? decodeURIComponent(userCookie.split('=')[1]) : null;
        this.userSubject.next(user);
    }

    // Call this method after connecting/disconnecting
    public updateUserCookie(): void {
        this.checkUserCookie();
    }
}
