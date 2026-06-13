const fs = require("fs");
let code = fs.readFileSync("src/components/MainWindow.tsx", "utf8");

// fix applyFormat signature
code = code.replace(
  /setContent: \(v: string\) => void,\n  markDirty: \(\) => void,\n\) {/g,
  "updateEditorContent: (v: string) => void,\n  markDirty: () => void,\n) {",
);

// fix useImagePaste config
code = code.replace(
  /updateEditorContent,\n                                  markDirty,/g,
  "setContent: updateEditorContent,\n    markDirty,",
);

// fix undo/redo content reference
code = code.replace(
  /updateEditorContent\(textarea\?.value \?\? content\);/g,
  "updateEditorContent(textarea?.value ?? contentStateRef.current);",
);

fs.writeFileSync("src/components/MainWindow.tsx", code);
