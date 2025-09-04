import { Component } from '@angular/core';
import { TopbarComponent } from "../../utils/component/topbar-component/topbar-component";

@Component({
  selector: 'app-test-component',
  imports: [TopbarComponent],
  templateUrl: './test-component.html',
  styleUrl: './test-component.css'
})
export class TestComponent {

}
