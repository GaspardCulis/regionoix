import { Component, signal } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { TopbarComponent } from './utils/component/topbar-component/topbar-component';

@Component({
  selector: 'app-root',
  imports: [RouterOutlet, TopbarComponent],
  templateUrl: './app.html',
  styleUrl: './app.css'
})
export class App {
  protected readonly title = signal('regionoix');
}
