import { Component, inject, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { AuthentificationService, LoggedUser, OrderDto, OrdersService } from '../../generated/clients/regionoix-client';
import { SnackbarService } from '../../services/snackbar-service';
import { Router } from '@angular/router';

@Component({
  selector: 'app-profile-page',
  standalone: true,
  imports: [CommonModule, FormsModule],
  templateUrl: './profile-page.html',
  styleUrls: ['./profile-page.css'],
})
export class ProfilePage implements OnInit {
  private readonly authService = inject(AuthentificationService);
  private readonly ordersService = inject(OrdersService);
  private readonly snackBar = inject(SnackbarService);
  private readonly router = inject(Router);

  informationPage = true;
  user!: LoggedUser;
  orders: OrderDto[] = [];

  ngOnInit(): void {
    this.loadUser();
  }

  loadUser() {
    this.authService.status().subscribe({
      next: (user: LoggedUser) => (this.user = user),
      error: () => this.snackBar.show('Erreur lors de la récupération des informations utilisateur', 'error'),
    });
  }

  loadOrders() {
    this.ordersService.get().subscribe({
      next: (orders: OrderDto[]) => (this.orders = orders),
      error: () => this.snackBar.show('Erreur lors de la récupération des commandes', 'error'),
    });
  }

  save() {
    this.snackBar.show('Service de sauvegarde des informations non implémenté', 'info');
  }

  changePassword() {
    this.snackBar.show('Service de changement de mot de passe non implémenté', 'info');
  }

  showInformationPage() {
    this.informationPage = true;
  }

  showOrdersPage() {
    this.informationPage = false;
    this.loadOrders();
  }

  isDelivered(order: OrderDto): boolean {
    if (!order.arrival_date) return false;

    const today = new Date();
    const arrival = new Date(order.arrival_date);

    const todayDateOnly = new Date(today.getFullYear(), today.getMonth(), today.getDate());
    const arrivalDateOnly = new Date(arrival.getFullYear(), arrival.getMonth(), arrival.getDate());

    return arrivalDateOnly < todayDateOnly;
  }

  logout() {
    this.authService.logout().subscribe({
      next: () => {
        this.router.navigate(['/connection']);
      },
      error: () => this.snackBar.show('Erreur lors de la déconnexion', 'error'),
    });
  }
}
