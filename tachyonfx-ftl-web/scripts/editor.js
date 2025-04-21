// import tfxCustomCompleter from './completions.mjs';

function setupEditor() {
    const editor = ace.edit("editor");
    editor.setTheme("ace/theme/gruvbox");
    editor.session.setMode("ace/mode/rust");
    editor.setFontSize("14px");

    // Enable basic autocompletion, snippets, and live autocompletion
    editor.setOptions({
        enableBasicAutocompletion: true,
        enableSnippets: true,
        enableLiveAutocompletion: true
    });

    // console.log("editor initialized; loading completions");
    //
    // // add the custom completer to the editor
    // editor.completers = [tfxCustomCompleter];
    // console.log("completions loaded", tfxCustomCompleter);

    return editor;
}
