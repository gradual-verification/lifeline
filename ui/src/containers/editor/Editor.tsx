import React from 'react';
import './Editor.scss';
import AceEditor from "react-ace";
import "ace-builds/src-noconflict/mode-java";
import "ace-builds/src-noconflict/theme-solarized_dark";
import "ace-builds/src-noconflict/ext-language_tools";
function Editor() {
  return (
    <AceEditor
            mode="java"
            theme="solarized dark"
            name="ace"
            editorProps={{ $blockScrolling: true }}
            />
  );
}

export default Editor;
