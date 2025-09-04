import { Component, input, Input, Output } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';

@Component({
  selector: 'app-topbar',
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
  @Input() basketCount: number = 0;
  @Input() user!: any;
  @Output() searchText: string = '';
}
