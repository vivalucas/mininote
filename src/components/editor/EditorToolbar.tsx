import React, { useMemo } from "react";
import { useTranslation } from "react-i18next";
import { FormatAction } from "../MainWindow";

interface EditorToolbarProps {
  onApplyFormat: (action: FormatAction) => void;
}

export const EditorToolbar = React.memo(({ onApplyFormat }: EditorToolbarProps) => {
  const { t } = useTranslation();

  const toolbarButtons = useMemo<
    { label: string; title: string; style: string; action: FormatAction }[]
  >(
    () => [
      {
        label: "B",
        title: t("main.toolbar.bold", { defaultValue: "粗体" }),
        style: "font-bold",
        action: "bold",
      },
      {
        label: "I",
        title: t("main.toolbar.italic", { defaultValue: "斜体" }),
        style: "italic",
        action: "italic",
      },
      {
        label: "H",
        title: t("main.toolbar.heading", { defaultValue: "标题" }),
        style: "font-bold",
        action: "heading",
      },
      {
        label: "—",
        title: t("main.toolbar.hr", { defaultValue: "分割线" }),
        style: "",
        action: "hr",
      },
      {
        label: "•",
        title: t("main.toolbar.ul", { defaultValue: "无序列表" }),
        style: "",
        action: "ul",
      },
      {
        label: "1.",
        title: t("main.toolbar.ol", { defaultValue: "有序列表" }),
        style: "font-mono text-[9px]",
        action: "ol",
      },
      {
        label: "<>",
        title: t("main.toolbar.code", { defaultValue: "代码" }),
        style: "font-mono text-[9px]",
        action: "code",
      },
      {
        label: "❝",
        title: t("main.toolbar.quote", { defaultValue: "引用" }),
        style: "",
        action: "quote",
      },
    ],
    [t],
  );

  return (
    <div className="flex items-center gap-0.5 px-4 pt-2 pb-1 shrink-0">
      {toolbarButtons.map((button) => (
        <button
          key={button.label}
          title={button.title}
          onMouseDown={(e) => e.preventDefault()}
          onClick={() => onApplyFormat(button.action)}
          className={`w-6 h-6 flex items-center justify-center rounded text-[11px] text-ink-ghost hover:text-ink-faint hover:bg-paper-warm transition-all cursor-pointer ${button.style}`}
        >
          {button.label}
        </button>
      ))}
    </div>
  );
});

EditorToolbar.displayName = "EditorToolbar";
