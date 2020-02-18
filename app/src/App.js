import React from 'react';
import './App.css';
import {Editor} from "./editor/Editor";

function App() {
    return (
        <div className="App">
            <h1>Pseudocompiler</h1>
            <h3>This doesn't work yet. Will possibly cause your computer to crash through infinite looping. Be
                careful!</h3>
            <Editor/>
        </div>
    );
}

export default App;
