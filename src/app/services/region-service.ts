import { Injectable, inject } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';

export interface Region {
  id: number;
  name: string;
}

@Injectable({ providedIn: 'root' })
export class RegionService {
  private http = inject(HttpClient);
  private apiUrl = '/api/regions';

  getRegions(): Observable<Region[]> {
    return this.http.get<Region[]>(this.apiUrl);
  }
}
