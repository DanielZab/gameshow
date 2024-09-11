import './App.css';
import NameInput from "./components/nameInput";
import {useState} from "react";
import RoleRadioInput from "./components/roleRadioInput";
import Connection from "./util/Connection";



const ADDR = "localhost";
const PORT = 8080
let connection = new Connection(ADDR, PORT);

function App() {
    const [name, setName] = useState("");
    const [role, setRole] = useState("host")
    const [started, setStarted] = useState(false);

    if (!started) {
        setStarted(true);
        establishConnection(onConnectionReady);
    }

    function onConnectionReady() {
        console.log("Connected");
    }

    function dataHandler(text) {
        console.log(text);
    }
    function establishConnection(callback) {
        connection.connect(callback, dataHandler);
    }

    function submitRole() {
        if (name === "") {
            // TODO: Add toastr notif.
            return;
        }

        if (!connection.isReady()) {
            // TODO: Add toast
            return;
        }

        let toSend = {"name": name, "role": role};
        connection.send(toSend);
    }

    return (
    <div className="App">
        <NameInput value={name} setValue={setName}/>
        <RoleRadioInput value={role} setValue={setRole}></RoleRadioInput>
        <button onClick={submitRole}></button>
    </div>
    );
}

export default App;
