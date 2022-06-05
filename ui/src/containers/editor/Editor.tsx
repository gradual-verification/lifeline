import React from 'react';
import './Editor.scss';
import AceEditor from "react-ace";

function Editor() {
  return (
    <div className="Editor">
          <AceEditor
            mode="java"
            theme="github"
            name="UNIQUE_ID_OF_DIV"
            editorProps={{ $blockScrolling: true }}
            />,
    </div>
  );
}

export default Editor;
