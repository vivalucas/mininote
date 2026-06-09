import { t, type TFunction } from "i18next";
import { invoke } from "@tauri-apps/api/core";
import { open, save } from "@tauri-apps/plugin-dialog";
import type { Note } from "../notes/types";

function textDocumentFilters(translate: TFunction = t) {
  return [
    {
      name: translate("dialogs.filters.mint", { defaultValue: "MiniNote" }),
      extensions: ["mint"],
    },
    {
      name: translate("dialogs.filters.markdown", { defaultValue: "Markdown" }),
      extensions: ["md", "markdown"],
    },
    {
      name: translate("dialogs.filters.text", { defaultValue: "Text" }),
      extensions: ["txt"],
    },
  ];
}

interface ExportableNote {
  id: string;
  title: string;
}

type ExportDocumentFormat = "markdown" | "mint";

export async function importMarkdownNote(category = ""): Promise<Note | null> {
  const path = await open({
    title: t("dialogs.import.title", { defaultValue: "导入文件" }),
    multiple: false,
    directory: false,
    filters: textDocumentFilters(),
  });

  if (typeof path !== "string") {
    return null;
  }

  return importMarkdownPath(path, category);
}

export function importMarkdownPath(path: string, category = ""): Promise<Note> {
  return invoke("notes_import_markdown", { path, category });
}

export async function exportMarkdownNote(
  note: ExportableNote,
  format: ExportDocumentFormat = "markdown",
): Promise<boolean> {
  const path = await save({
    title: t(format === "mint" ? "dialogs.exportMint.title" : "dialogs.exportMarkdown.title", {
      defaultValue: format === "mint" ? "导出 Mint" : "导出 Markdown",
    }),
    defaultPath: exportFileName(note.title, format),
    filters: textDocumentFilters(),
  });

  if (typeof path !== "string") {
    return false;
  }

  await invoke("notes_export_markdown", { id: note.id, path });
  return true;
}

function exportFileName(
  title: string,
  format: ExportDocumentFormat,
  translate: TFunction = t,
): string {
  const safeTitle =
    safeFileStem(title) || translate("common.untitledNote", { defaultValue: "无标题笔记" });
  return `${safeTitle}.${format === "mint" ? "mint" : "md"}`;
}

function safeFileStem(value: string): string {
  const sanitized = Array.from(value.trim(), (char) => {
    const code = char.codePointAt(0) ?? 0;

    if ('<>:"/\\|?*'.includes(char) || code < 0x20) {
      return "_";
    }

    return char;
  }).join("");

  const normalized = sanitized
    .replace(/\s+/g, "_")
    .replace(/_+/g, "_")
    .replace(/^_+|_+$/g, "");

  return Array.from(normalized).slice(0, 80).join("");
}
