import { t, type TFunction } from "i18next";

export type NoteContextMenuAction = "exportMarkdown" | "exportMint" | "move" | "delete";

export interface NoteContextMenuItem {
  action: NoteContextMenuAction;
  label: string;
  tone?: "danger";
}

export function getNoteContextMenuItems(translate: TFunction = t): NoteContextMenuItem[] {
  return [
    {
      action: "exportMarkdown",
      label: translate("noteMenu.exportMarkdown", { defaultValue: "导出 Markdown" }),
    },
    {
      action: "exportMint",
      label: translate("noteMenu.exportMint", { defaultValue: "导出 Mint" }),
    },
    {
      action: "move",
      label: translate("noteMenu.moveToCategory", { defaultValue: "移动到分类…" }),
    },
    {
      action: "delete",
      label: translate("noteMenu.delete", { defaultValue: "删除笔记" }),
      tone: "danger",
    },
  ];
}
