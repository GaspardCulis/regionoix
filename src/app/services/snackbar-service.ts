import { Injectable } from '@angular/core';
import { Subject } from 'rxjs';
import { SnackbarState } from '../models/snackbar-state';

@Injectable({ providedIn: 'root' })
export class SnackbarService {
  private state = new Subject<SnackbarState>();
  snackbarState = this.state.asObservable();

  show(message: string, type: 'success' | 'error' | 'info' = 'info', duration = 3000) {
    this.state.next({ message, type, duration });
  }
}
