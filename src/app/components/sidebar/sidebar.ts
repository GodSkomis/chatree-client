import { Component } from '@angular/core';
import { MatSidenavModule } from '@angular/material/sidenav';
import { MatToolbarModule } from '@angular/material/toolbar';
import { MatListModule } from '@angular/material/list';
import { CommonModule } from '@angular/common';
import { SidebarChat } from "../sidebar-chat/sidebar-chat";


@Component({
  selector: 'app-sidebar',
  templateUrl: './sidebar.html',
  styleUrls: ['./sidebar.scss'],
  imports: [
    CommonModule,
    MatSidenavModule,
    MatListModule,

    SidebarChat
]
})
export class SidebarComponent {
  chats = [1,2,3,4,5,6,7,8,9,10,11,12,113,14,15,16,17];
  selectChat(id: string) {
    console.log('Selected chat', id);
  }
}
