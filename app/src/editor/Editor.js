import React from "react";
import './Editor.css';
import AceEditor from "react-ace";
import "ace-builds/src-noconflict/mode-batchfile";
import "ace-builds/src-noconflict/theme-github";

class Editor extends React.Component {
    render() {
        return <>
            <div className="Editor">
                <AceEditor
                    mode="batchfile"
                    width={window.outerWidth < 500 ? window.outerWidth - 50 : window.outerWidth/2}
                    theme="github"
                    name="editor"
                    editorProps={{$blockScrolling: true}}/>

            </div>
            <div className="ControlPanel">
                <button>RUN CODE</button>
            </div>
        </>
    }
}

export {Editor}