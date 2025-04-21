
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

    return editor;
}
