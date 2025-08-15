import { Injectable } from '@angular/core';
import { Observable } from "rxjs";
import {WebSocketSubject, webSocket} from "rxjs/webSocket";

@Injectable({
  providedIn: 'root'
})
export class WebSocketService {
  private socket$: WebSocketSubject<any> | undefined

  establishConnection(){
    this.socket$ = webSocket('ws://localhost:9001')
  }

  sendMessage(message: any){
    if (!this.socket$) {
      console.log("Socket not initialized")
      return;
    }
    this.socket$.next(message);
    console.log(`sent ${message}`)
  }

  getMessages(): Observable<any> {
    if (!this.socket$) {
      console.log("Socket not initialized")
      return new Observable<void>;
    }
    return this.socket$.asObservable()
  }

  connectionEstablished(): boolean {
    return !!this.socket$
  }

  closeConnection() {
    if (!this.socket$) {
      console.log("Socket not initialized")
      return;
    }
    this.socket$.complete();
  }

}
