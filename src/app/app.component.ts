import { Component } from "@angular/core";
import { RouterOutlet } from "@angular/router";
import { invoke } from "@tauri-apps/api/core";
import { MatSidenavModule } from '@angular/material/sidenav';
import { ChatComponent } from "./components/chat/chat";
import { SidebarComponent } from "./components/sidebar/sidebar";
import { SidebarChat } from "./components/sidebar-chat/sidebar-chat";
import { CommonModule } from "@angular/common";
import { HeaderComponent } from "./components/header/header";


@Component({
  selector: "app-root",
  imports: [
    RouterOutlet,
    MatSidenavModule,
    CommonModule,
    
    ChatComponent,
    SidebarComponent,
    SidebarChat,
    HeaderComponent
],
  templateUrl: "./app.component.html",
  styleUrl: "./app.component.scss",
})
export class AppComponent {
  greetingMessage = "";

  greet(event: SubmitEvent, name: string): void {
    event.preventDefault();

    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    invoke<string>("greet", { name }).then((text) => {
      this.greetingMessage = text;
    });
  }
}
