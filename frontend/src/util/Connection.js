export default class Connection {
    constructor(addr, port) {
        this.addr = addr;
        this.port = port;
    };

    connect(callback, dataHandler) {
        // Create a WebSocket connection to the server
        this.ws = new WebSocket('ws://localhost:3030/ws');
        // Event listener for incoming messages from the server
        this.ws.onmessage = function(event) {
            dataHandler(event.data);
        };

        this.ws.onopen = () => callback();
    }

    disconnect() {
        this.ws.close();
    }

    isReady() {
        return this.ws
    }

    send(message) {
        if (typeof message != "string") {
            message = JSON.stringify(message);
        }
        this.ws.send(message);
    }
}