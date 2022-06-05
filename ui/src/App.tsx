import React from 'react';
import './App.scss';
import Navbar from './containers/navbar/Navbar';
import Editor from './containers/editor/Editor';
import Output from './containers/output/Output';
import { Button } from './components/Button';
function App() {
  return (
    <div className="App">
      <div className='flex-stack'>
        <Navbar>
          <h1>Lifeline</h1>
          
        </Navbar>
        <div className="flex-main">
        <div className="col-editor">
          <div className="editor-body">
            <div className="toolbar">
                <Button className="green-b">
                  <i className="ri-play-line"></i>
                </Button>
            </div>
            <Editor></Editor>
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
