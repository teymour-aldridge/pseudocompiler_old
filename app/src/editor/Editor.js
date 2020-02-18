import React from "react";
import './Editor.css';
import AceEditor from "react-ace";
import "ace-builds/src-noconflict/mode-batchfile";
import "ace-builds/src-noconflict/theme-github";

class Editor extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            code: "",
            wasm: {},
            windowSize: [0, 0]
        };
        this.loadWasm = this.loadWasm.bind(this);
    }

    componentDidMount() {
        this.loadWasm();
    }

    loadWasm = async () => {
        try {
            const wasm = await import('pseudocompiler');
            this.setState({wasm: wasm});
        } catch (err) {
            console.error(`Unexpected error in loadWasm. [Message: ${err.message}]`);
        }
    };

    render() {
        const wasm = this.state.wasm;
        return <>
            <div className="Editor">
                <AceEditor
                    mode="batchfile"
                    theme="github"
                    width="100%"
                    name="editor"
                    value={this.state.code}
                    onChange={value => {
                        this.setState({
                            code: value
                        })
                    }}
                    editorProps={{$blockScrolling: true}}/>
            </div>
            <div className="ControlPanel">
                <button onClick={event => {
                    event.preventDefault();
                    console.log(wasm.compile(this.state.code))
                }
                }>RUN CODE
                </button>
            </div>
        </>
    }
}

export {Editor}