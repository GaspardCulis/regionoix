import { HttpClient } from '@angular/common/http';
import { inject, Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { UserStatusModel } from '../models/user-status-model';

@Injectable({
  providedIn: 'root'
})
export class AuthService {
  private readonly endpoint = 'https://www.regionoix.gasdev.frapi/auth/';

  private readonly httpClient = inject(HttpClient);

  public login(email: string, password: string): Observable<object> {
    const url = this.endpoint + 'login';
    return this.httpClient.post(url, { email: email, password: password });
  }

  public logout(): Observable<object> {
    const url = this.endpoint + 'logout';
    return this.httpClient.post(url, {});
  }

  public status(): Observable<UserStatusModel> {
    const url = this.endpoint + 'status';
    return this.httpClient.get<UserStatusModel>(url);
  }
}
