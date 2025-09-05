import { Component, inject, Input, Output } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { Router } from '@angular/router';

@Component({
  selector: 'app-topbar',
  standalone: true,
  imports: [
    CommonModule,
    FormsModule
  ],
  templateUrl: './topbar-component.html',
  styleUrl: './topbar-component.css'
})
export class TopbarComponent {
  @Input() pathLogo!: string;
  @Input() title!: string;
  @Input() basketCount = 0;
  @Input() user!: string | null;
  @Output() searchText = '';

  private router = inject(Router);

  onProfileClick() {
    if (this.user) {
      this.router.navigate(['/profile']); //TODO: replace with profile page
    }
    else {
      this.router.navigate(['/connexion']);
    }
  }
}
