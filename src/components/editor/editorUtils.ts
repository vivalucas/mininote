import type { TFunction } from "i18next";

export type FormatAction = "bold" | "italic" | "heading" | "hr" | "ul" | "ol" | "code" | "quote";

export function applyFormat(
  textarea: HTMLTextAreaElement,
  action: FormatAction,
  translate: TFunction,
  updateEditorContent: (v: string) => void,
  markDirty: () => void,
) {
  const { selectionStart: start, selectionEnd: end, value } = textarea;
  const selected = value.slice(start, end);
  const before = value.slice(0, start);
  const after = value.slice(end);

  const lineStart = before.lastIndexOf("\n") + 1;
  const currentLine = before.slice(lineStart);

  let replacementStart = start;
  let replacementEnd = end;
  let replacementString = "";
  let cursorStart: number;
  let cursorEnd: number;

  switch (action) {
    case "bold": {
      const fallback = translate("main.formatSample.boldText", { defaultValue: "粗体文本" });
      replacementString = `**${selected || fallback}**`;
      cursorStart = start + 2;
      cursorEnd = cursorStart + (selected || fallback).length;
      break;
    }
    case "italic": {
      const fallback = translate("main.formatSample.italicText", { defaultValue: "斜体文本" });
      replacementString = `*${selected || fallback}*`;
      cursorStart = start + 1;
      cursorEnd = cursorStart + (selected || fallback).length;
      break;
    }
    case "heading": {
      const prefix = currentLine.match(/^(#{1,5})\s/);
      if (prefix) {
        const newLevel = prefix[1].length < 5 ? "#".repeat(prefix[1].length + 1) : "#";
        replacementStart = lineStart;
        replacementEnd = lineStart + prefix[0].length;
        replacementString = newLevel + " ";
        const offset = newLevel.length + 1 - prefix[0].length;
        cursorStart = start + offset;
        cursorEnd = end + offset;
      } else if (currentLine.length > 0 && start === end) {
        replacementStart = lineStart;
        replacementEnd = lineStart;
        replacementString = "## ";
        cursorStart = start + 3;
        cursorEnd = cursorStart;
      } else if (selected) {
        replacementString = `## ${selected}`;
        cursorStart = start + 3;
        cursorEnd = cursorStart + selected.length;
      } else {
        replacementString = `## ${translate("main.formatSample.headingText", { defaultValue: "标题" })}`;
        cursorStart = start + 3;
        cursorEnd = cursorStart + 2;
      }
      break;
    }
    case "hr": {
      const newlineBefore = before.endsWith("\n") || before === "" ? "" : "\n";
      const newlineAfter = after.startsWith("\n") || after === "" ? "" : "\n";
      replacementString = `${newlineBefore}---${newlineAfter}`;
      cursorStart = cursorEnd = before.length + newlineBefore.length + 3;
      break;
    }
    case "ul": {
      if (selected.includes("\n")) {
        replacementString = selected
          .split("\n")
          .map((l) => `- ${l}`)
          .join("\n");
        cursorStart = start;
        cursorEnd = start + replacementString.length;
      } else {
        const fallback = translate("main.formatSample.listItem", { defaultValue: "列表项" });
        replacementString = `- ${selected || fallback}`;
        cursorStart = start + 2;
        cursorEnd = cursorStart + (selected || fallback).length;
      }
      break;
    }
    case "ol": {
      if (selected.includes("\n")) {
        replacementString = selected
          .split("\n")
          .map((l, i) => `${i + 1}. ${l}`)
          .join("\n");
        cursorStart = start;
        cursorEnd = start + replacementString.length;
      } else {
        const fallback = translate("main.formatSample.listItem", { defaultValue: "列表项" });
        replacementString = `1. ${selected || fallback}`;
        cursorStart = start + 3;
        cursorEnd = cursorStart + (selected || fallback).length;
      }
      break;
    }
    case "code": {
      if (selected.includes("\n")) {
        replacementString = "```\n" + selected + "\n```";
        cursorStart = start + 4;
        cursorEnd = cursorStart + selected.length;
      } else {
        const fallback = translate("main.formatSample.codeText", { defaultValue: "代码" });
        replacementString = `\`${selected || fallback}\``;
        cursorStart = start + 1;
        cursorEnd = cursorStart + (selected || fallback).length;
      }
      break;
    }
    case "quote": {
      if (selected.includes("\n")) {
        replacementString = selected
          .split("\n")
          .map((l) => `> ${l}`)
          .join("\n");
        cursorStart = start;
        cursorEnd = start + replacementString.length;
      } else {
        const fallback = translate("main.formatSample.quoteText", { defaultValue: "引用文本" });
        replacementString = `> ${selected || fallback}`;
        cursorStart = start + 2;
        cursorEnd = cursorStart + (selected || fallback).length;
      }
      break;
    }
  }

  const result = value.slice(0, replacementStart) + replacementString + value.slice(replacementEnd);
  const scrollTop = textarea.scrollTop;

  textarea.focus();
  textarea.setSelectionRange(replacementStart, replacementEnd);
  document.execCommand("insertText", false, replacementString);
  updateEditorContent(result);
  markDirty();
  requestAnimationFrame(() => {
    textarea.scrollTop = scrollTop;
    textarea.setSelectionRange(cursorStart, cursorEnd);
  });
}
