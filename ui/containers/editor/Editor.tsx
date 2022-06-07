import React from 'react';
import './Editor.scss';
import AceEditor from "react-ace";
import "ace-builds/src-noconflict/mode-java";
import "ace-builds/src-noconflict/theme-solarized_dark";
import "ace-builds/src-noconflict/ext-language_tools";

type Props = {onChange?: (event:any) => void, contents: string};


function Editor({onChange, contents}:Props) {
  return (
    <AceEditor
            mode="java"
            theme="solarized dark"
            name="ace"
            onChange={onChange} 
            value={contents}
            editorProps={{ $blockScrolling: true }}
            />
  );
}

export default Editor;
