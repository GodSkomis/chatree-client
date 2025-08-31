import { Injectable } from '@angular/core';
import { BehaviorSubject } from 'rxjs';
import { Message } from '../models/message.model';


@Injectable({ providedIn: 'root' })
export class ChatService {
  private messages: Message[] = [];
  private messages$ = new BehaviorSubject<Message[]>(this.messages);

  get messagesStream() {
    return this.messages$.asObservable();
  }

  send(text: string) {
    const msg: Message = { text, timestamp: new Date(), sender: 'me' };
    this.messages.push(msg);
    this.messages$.next([...this.messages]);
  }
}