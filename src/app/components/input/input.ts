import { Component, Output, EventEmitter } from '@angular/core';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatIconModule } from '@angular/material/icon';

@Component({
  selector: 'app-input',
  templateUrl: './input.html',
  styleUrls: ['./input.scss'],
  imports: [
    MatFormFieldModule,
    MatIconModule,
  ]
})
export class InputComponent {
  text = '';
  @Output() send = new EventEmitter<string>();
  onSend() { if (this.text.trim()) { this.send.emit(this.text); this.text = ''; }}
}