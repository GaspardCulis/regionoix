import { HttpClient } from '@angular/common/http';
import { inject, Injectable } from '@angular/core';
import { Observable } from 'rxjs';

export interface Category {
  id: number;
  name: string;
  description?: string | null;
  category_parent?: number | null;
  childs?: Category[];
}

@Injectable({ providedIn: 'root' })
export class CategoryService {
  private readonly http = inject(HttpClient);
  private readonly apiUrl = '/api/categories';

  getCategories(): Observable<Category[]> {
    return this.http.get<Category[]>(this.apiUrl);
  }

  getHierarchy(): Observable<Category[]> {
    return this.http.get<Category[]>(`${this.apiUrl}/hierarchy`);
  }
}
