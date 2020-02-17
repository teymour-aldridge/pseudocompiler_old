import React from 'react';
import './App.css';
import {Editor} from "./editor/Editor";

function App() {
    return (
        <div className="App">
            <h1>Pseudocompiler</h1>
            <Editor/>
        </div>
    );
}

export default App;
