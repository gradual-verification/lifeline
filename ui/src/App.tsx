import React from 'react';
import './App.scss';
import Navbar from './containers/navbar/Navbar';
import Editor from './containers/editor/Editor';
import Output from './containers/output/Output';
function App() {
  return (
    <div className="App">
      <Navbar>
        <h1>Lifeline</h1>
      </Navbar>
      <div className="splitpane">
        <div>
          <Editor></Editor>
        </div>
        <div>
          <Output></Output>
        </div>
      </div>
    </div>
  );
}

export default App;
