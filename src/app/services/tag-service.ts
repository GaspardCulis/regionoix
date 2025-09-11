import { Injectable, inject } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';

export interface Tag {
  id: number;
  name: string;
}

@Injectable({ providedIn: 'root' })
export class TagService {
  private http = inject(HttpClient);
  private apiUrl = '/api/tags';

  getTags(): Observable<Tag[]> {
    return this.http.get<Tag[]>(this.apiUrl);
  }
}
