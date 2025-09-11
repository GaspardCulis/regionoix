export interface SnackbarState {
  message: string;
  type: 'success' | 'error' | 'info';
  duration?: number;
}