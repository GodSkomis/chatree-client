import { Component, HostBinding, Input, OnInit } from '@angular/core';
import { Message } from '../../models/message.model';
import { CommonModule } from '@angular/common';


@Component({
  selector: 'app-message',
  templateUrl: './message.html',
  styleUrls: ['./message.scss'],
  imports: [
    CommonModule,
  ]
})

export class MessageComponent implements OnInit {
  @Input() message!: Message;
  @HostBinding('class.me') isMe = false;

  ngOnInit(): void {
    this.isMe = this.message.sender === 'me';
  }
}
