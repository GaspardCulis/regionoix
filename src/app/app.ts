import { Component, inject, OnInit, signal } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { TopbarComponent } from './utils/component/topbar-component/topbar-component';
import { SnackbarComponent } from "./utils/component/snackbar-component/snackbar-component";
import { SnackbarService } from './services/snackbar-service';
import { BasketStateService } from './services/basket-state-service';
import { AdminMenu } from "./utils/admin-menu/admin-menu";
import { AuthStateService } from './services/auth-state-service';
import { LoggedUser } from './generated/clients/regionoix-client';

@Component({
  selector: 'app-root',
  imports: [RouterOutlet, TopbarComponent, SnackbarComponent, AdminMenu],
  templateUrl: './app.html',
  styleUrl: './app.css'
})
export class App implements OnInit {
  basketItemCount = 0;
  protected readonly title = signal('regionoix');

  private readonly snackBar = inject(SnackbarService);
  private readonly basketState = inject(BasketStateService);
  private readonly authState = inject(AuthStateService);
  user: LoggedUser | null = null;

  ngOnInit(): void {
    this.authState.user$.subscribe((u) => (this.user = u));
    this.basketState.basketCount$.subscribe({
      next: (count) => this.basketItemCount = count,
      error: () => {
        this.snackBar.show('Erreur lors de la récupération du panier.', 'error');
        this.basketItemCount = 0;
      }
    });
    this.basketState.refreshCount();
  }
}
