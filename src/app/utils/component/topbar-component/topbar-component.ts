import { Component, Input, Output } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';

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
}
