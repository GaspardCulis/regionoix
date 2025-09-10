import { Component } from '@angular/core';
import { SnackbarService } from '../../../services/snackbar-service';
import { CommonModule } from '@angular/common';

@Component({
  selector: 'app-snackbar',
  imports: [CommonModule],  
  templateUrl: './snackbar-component.html',
  styleUrl: './snackbar-component.css',
})
export class SnackbarComponent {
  message = '';
  type: 'success' | 'error' | 'info' = 'info';
  timeoutId: any;

  constructor(private snackbarService: SnackbarService) {
    this.snackbarService.snackbarState.subscribe((state) => {
      this.show(state.message, state.type, state.duration);
    });
  }

  show(message: string, type: 'success' | 'error' | 'info' = 'info', duration = 3000) {
    this.message = message;
    this.type = type;

    if (this.timeoutId) clearTimeout(this.timeoutId);
    this.timeoutId = setTimeout(() => (this.message = ''), duration);
  }

  close() {
    this.message = '';
    if (this.timeoutId) clearTimeout(this.timeoutId);
  }
}