

export interface Message {
    text: string;
    timestamp: Date;
    sender: 'me' | 'other';
  }