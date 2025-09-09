import { HttpClient } from '@angular/common/http';
import { inject, Injectable } from '@angular/core';
import { Observable } from 'rxjs';

@Injectable({
  providedIn: 'root'
})
export class AuthService {
  private readonly endpoint = 'api/auth/';

  private readonly httpClient = inject(HttpClient);

  public login(email: string, password: string): Observable<any> {
    let url = this.endpoint + 'login';
    return this.httpClient.post(url, { email: email, password: password });
  }

  public logout(): Observable<any> {
    let url = this.endpoint + 'logout';
    return this.httpClient.post(url, {});
  }

  public status(): Observable<any> {
    let url = this.endpoint + 'status';
    return this.httpClient.get(url);
  }
}
