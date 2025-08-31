import { Component, OnInit } from '@angular/core';
import { Message } from '../../models/message.model';
import { ChatService } from '../../services/chat';
import { CommonModule } from '@angular/common';
import { MessageComponent } from '../message/message';
import { InputComponent } from '../input/input';
import { Observable } from 'rxjs';


@Component({
  selector: 'app-chat',
  templateUrl: './chat.html',
  styleUrls: ['./chat.scss'],
  imports: [
    CommonModule,

    MessageComponent,
    InputComponent,
  ]
})
export class ChatComponent implements OnInit {
  messages: Message[] = [
    {
      text: "Dadadaya1",
      timestamp: new Date(),
      sender: 'other'
    },
    {
      text: "Nysho2",
      timestamp: new Date(),
      sender: 'other'
    },
    {
      text: "OmgYa3",
      timestamp: new Date(),
      sender: 'me'
    },
    {
      text: "Poehali4",
      timestamp: new Date(),
      sender: 'other'
    },
  ]
  messages$!: Observable<Message[]>;
  constructor(public chatService: ChatService) {}

  ngOnInit() {
    this.messages$ = this.chatService.messagesStream;
  }
}
