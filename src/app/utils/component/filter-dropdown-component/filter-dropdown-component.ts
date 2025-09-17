import { CommonModule } from '@angular/common';
import { Component, EventEmitter, Input, Output } from '@angular/core';

@Component({
  selector: 'app-filter-dropdown-component',
  templateUrl: './filter-dropdown-component.html',
  imports: [CommonModule],
  styleUrls: ['./filter-dropdown-component.css']
})
export class FilterDropdownComponent {
  isOpen = false;

  @Input() title!: string;
  @Input() selectedOptions!: [string, boolean][];
  @Output() optionToggled = new EventEmitter<{ name: string; checked: boolean }>();

  toggleDropdown() {
    this.isOpen = !this.isOpen;
  }

  onOptionChange(index: number) {
    const [name, checked] = this.selectedOptions[index];
    this.selectedOptions[index][1] = !checked;
    this.optionToggled.emit({ name, checked: !checked });
  }
}
