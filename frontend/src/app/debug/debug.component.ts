import {Component, OnDestroy, OnInit} from '@angular/core';
import {WebSocketService} from "../services/web-socket.service";
import {Subscription} from "rxjs";

@Component({
    selector: 'app-debug',
    templateUrl: './debug.component.html',
    styleUrls: ['./debug.component.scss']
})
export class DebugComponent implements OnInit, OnDestroy {
    receivedMsgs: string[] = [];
    private messageSubscription: Subscription | undefined;
    inputFieldValue = "";

    constructor(private webSocketService: WebSocketService) {
    }

    initiateConnection(){
        this.webSocketService.establishConnection();

        this.messageSubscription = this.webSocketService.getMessages().subscribe(
            (message) => {
                console.log(`Received`, message);
                this.receivedMsgs.push(JSON.stringify(message));
            }
        )
    }

    destroyConnection() {
        this.messageSubscription?.unsubscribe();
        this.webSocketService.closeConnection();
    }

    connected(): boolean {
        return this.webSocketService.connectionEstablished();
    }

    ngOnInit(): void {
    }

    sendMessage(msg:string) {
        console.log(`Sending msg ${msg}`)
        this.webSocketService.sendMessage(msg);
    }

    ngOnDestroy(): void {
        this.destroyConnection();
    }
}
