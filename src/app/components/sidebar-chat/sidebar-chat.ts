import { CommonModule } from '@angular/common';
import { Component, Input } from '@angular/core';
import { MatBadgeModule } from '@angular/material/badge';
import { MatCardModule } from '@angular/material/card';
import { MatIconModule } from '@angular/material/icon';


@Component({
  selector: 'app-sidebar-chat',
  imports: [
    CommonModule,
    MatCardModule,
    MatIconModule,
    MatBadgeModule
  ],
  templateUrl: './sidebar-chat.html',
  styleUrl: './sidebar-chat.scss'
})
export class SidebarChat {
  badgeColor = 'green';

}
