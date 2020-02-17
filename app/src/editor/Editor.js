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
            wasm: {}
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
                    width={window.outerWidth < 500 ? (window.outerWidth - 50).toString() + 'px' : (window.outerWidth / 2).toString() + 'px'}
                    theme="github"
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
                    eval(wasm.compile(this.state.code));
                }
                }>RUN CODE
                </button>
            </div>
        </>
    }
}

export {Editor}