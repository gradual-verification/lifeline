import { useState } from 'react';
import './App.scss';
import Navbar from './containers/navbar/Navbar';
import Editor from './containers/editor/Editor';
import Output from './containers/output/Output';
import { Button } from './components/Button';
import init, {analyze, pretty_print} from './lifeline';
function App() {

  const [text, setText] = useState("");

  const prettyPrint = () => {
    init().then(() => {
      setText(pretty_print(text));
    })  }

  const runAnalysis = () => {
   init().then(() => {
      analyze(text);
    })
  }

  return (
    <div className="App">
      <div className='flex-stack'>
        <Navbar>
          <h1>Lifeline</h1>
          <a id="gh-link"
          href="https://github.com/gradual-verification/lifeline" 
          target="_blank" 
          rel="noreferrer">
            <i className="ri-github-fill"></i>
          </a>
        </Navbar>
        <div className="flex-main">
        <div className="col-editor">
          <div className="editor-body">
            <div className="toolbar">
                <Button className="blue-b" onClick={prettyPrint}>
                  <i className="ri-braces-fill"></i>
                </Button>
                <Button className="green-b" onClick={runAnalysis}>
                  <i className="ri-play-line"></i>
                </Button>
            </div>
            <Editor onChange={setText} contents={text}></Editor>
          </div>
        </div>
        <div className="col-display">
          <Output></Output>
        </div>
      </div>
      </div>
    </div>
  );
}

export default App;
