import { useCallback, useMemo, useRef, useState, forwardRef, useImperativeHandle } from "react";
import type { TFunction } from "i18next";
import { LazyMarkdownPreview } from "../../features/markdown/LazyMarkdownPreview";
import { EditorToolbar } from "./EditorToolbar";
import { applyFormat } from "./editorUtils";
import { countNoteChars } from "../../features/notes/noteUtils";
import { useImagePaste } from "../../features/images/useImagePaste";
import type { AppConfig, ViewMode } from "../../features/settings/types";

export interface MainEditorRef {
  undo: () => void;
  redo: () => void;
  focus: () => void;
}

interface MainEditorProps {
  initialContent: string;
  selectedId: string | null;
  viewMode: ViewMode;
  splitRatio: number;
  isResizingSplit: boolean;
  setIsResizingSplit: (resizing: boolean) => void;
  settingsConfig: AppConfig | null;
  imageBaseDir: string | null;
  documentFormat: string | null;
  onChange: (content: string) => void;
  onEnsureNoteSaved: () => Promise<string | null>;
  onCleanUnusedImages: () => void;
  t: TFunction;
}

export function runEditorCommand(
  textarea: HTMLTextAreaElement | null,
  command: "undo" | "redo",
): boolean {
  if (!textarea || textarea.disabled) return false;
  textarea.focus();
  return document.execCommand(command);
}

export function getCursorPosition(value: string, offset: number): { line: number; column: number } {
  const safeOffset = Math.max(0, Math.min(offset, value.length));
  const beforeCursor = value.slice(0, safeOffset);
  const lastLineBreak = beforeCursor.lastIndexOf("\n");
  return {
    line: beforeCursor.split("\n").length,
    column: safeOffset - lastLineBreak,
  };
}

export function detectNewline(value: string): "LF" | "CRLF" | "Mixed" {
  const hasCrlf = /\r\n/.test(value);
  const hasBareLf = /(^|[^\r])\n/.test(value);
  if (hasCrlf && hasBareLf) return "Mixed";
  if (hasCrlf) return "CRLF";
  return "LF";
}

export const MainEditor = forwardRef<MainEditorRef, MainEditorProps>(function MainEditor(
  {
    initialContent,
    selectedId,
    viewMode,
    splitRatio,
    isResizingSplit,
    setIsResizingSplit,
    settingsConfig,
    imageBaseDir,
    documentFormat,
    onChange,
    onEnsureNoteSaved,
    onCleanUnusedImages,
    t,
  },
  ref,
) {
  const contentRef = useRef<HTMLTextAreaElement>(null);
  const [contentSnapshot, setContentSnapshot] = useState(initialContent);
  const contentUpdateTimer = useRef<number>(0);

  const [cursorPosition, setCursorPosition] = useState({ line: 1, column: 1 });
  const cursorUpdateTimer = useRef<number>(0);

  const syncCursorPositionDebounced = useCallback((pos: { line: number; column: number }) => {
    if (cursorUpdateTimer.current) window.clearTimeout(cursorUpdateTimer.current);
    cursorUpdateTimer.current = window.setTimeout(() => {
      setCursorPosition(pos);
    }, 10000);
  }, []);

  const syncCursorPosition = useCallback(() => {
    const textarea = contentRef.current;
    if (!textarea) return;
    setCursorPosition(getCursorPosition(textarea.value, textarea.selectionStart));
  }, []);

  const syncContentState = useCallback((newContent: string) => {
    if (contentUpdateTimer.current) window.clearTimeout(contentUpdateTimer.current);
    contentUpdateTimer.current = window.setTimeout(() => {
      setContentSnapshot(newContent);
    }, 5000);
  }, []);

  const updateEditorContent = useCallback(
    (newContent: string) => {
      if (contentRef.current && contentRef.current.value !== newContent) {
        contentRef.current.value = newContent;
      }
      onChange(newContent);
      if (contentUpdateTimer.current) window.clearTimeout(contentUpdateTimer.current);
      setContentSnapshot(newContent);
    },
    [onChange],
  );

  const markDirty = useCallback(() => {
    onChange(contentRef.current?.value ?? "");
  }, [onChange]);

  useImperativeHandle(ref, () => ({
    undo: () => {
      const textarea = contentRef.current;
      if (runEditorCommand(textarea, "undo")) {
        updateEditorContent(textarea?.value ?? "");
      }
    },
    redo: () => {
      const textarea = contentRef.current;
      if (runEditorCommand(textarea, "redo")) {
        updateEditorContent(textarea?.value ?? "");
      }
    },
    focus: () => {
      contentRef.current?.focus();
    },
  }));

  const {
    handlePaste: imagePasteHandler,
    handleDrop: imageDropHandler,
    handleDragOver: imageDragOverHandler,
  } = useImagePaste({
    noteId: selectedId,
    textareaRef: contentRef,
    setContent: updateEditorContent,
    markDirty,
    onEnsureNoteSaved,
    disabled: false,
  });

  const lineCount = useMemo(() => contentSnapshot.split("\n").length, [contentSnapshot]);
  const newlineFormat = useMemo(() => detectNewline(contentSnapshot), [contentSnapshot]);
  const byteSize = useMemo(
    () => (new TextEncoder().encode(contentSnapshot).length / 1024).toFixed(1),
    [contentSnapshot],
  );
  const charCount = useMemo(() => countNoteChars(contentSnapshot), [contentSnapshot]);

  return (
    <>
      {(viewMode === "edit" || viewMode === "split") && (
        <div
          className="flex flex-col min-h-0 shrink-0"
          style={{ width: viewMode === "split" ? `${splitRatio * 100}%` : "100%" }}
        >
          <EditorToolbar
            onApplyFormat={(action) => {
              if (contentRef.current) {
                applyFormat(contentRef.current, action, t, updateEditorContent, markDirty);
              }
            }}
          />

          <div className="flex-1 overflow-hidden px-5 pb-4">
            <textarea
              ref={contentRef}
              data-tab-indent="true"
              defaultValue={initialContent}
              onChange={(event) => {
                onChange(event.target.value);
                syncContentState(event.target.value);
                syncCursorPositionDebounced(
                  getCursorPosition(event.target.value, event.target.selectionStart),
                );
              }}
              onClick={syncCursorPosition}
              onKeyUp={syncCursorPosition}
              onSelect={syncCursorPosition}
              onPaste={imagePasteHandler}
              onDrop={imageDropHandler}
              onDragOver={imageDragOverHandler}
              className="w-full h-full text-ink-soft font-body placeholder:text-ink-ghost/40"
              style={{
                fontSize: `${settingsConfig?.fontSize ?? 14}px`,
                lineHeight: "normal",
                tabSize: `var(--tab-indent-size, 2)`,
              }}
              placeholder={t("main.editor.contentPlaceholder", {
                defaultValue: "开始写作……",
              })}
              spellCheck={false}
              disabled={!selectedId}
            />
          </div>
        </div>
      )}

      {viewMode === "split" && (
        <div
          className={`w-1.5 shrink-0 cursor-col-resize group relative flex items-center justify-center ${isResizingSplit ? "bg-bamboo/30" : "hover:bg-bamboo/20"} transition-colors`}
          onMouseDown={(e) => {
            e.preventDefault();
            setIsResizingSplit(true);
          }}
        >
          <div
            className={`absolute inset-y-0 -left-1.5 -right-1.5 ${isResizingSplit ? "" : "group-hover:bg-bamboo/5"}`}
          />
          {/* 拖拽手柄指示器 */}
          <div className="relative z-10 flex flex-col gap-[3px] opacity-0 group-hover:opacity-100 transition-opacity">
            <div className="w-[3px] h-[3px] rounded-full bg-ink-ghost/60" />
            <div className="w-[3px] h-[3px] rounded-full bg-ink-ghost/60" />
            <div className="w-[3px] h-[3px] rounded-full bg-ink-ghost/60" />
          </div>
        </div>
      )}

      {(viewMode === "preview" || viewMode === "split") && (
        <div className="flex flex-col min-h-0 min-w-0 flex-1">
          {viewMode === "split" && (
            <div className="px-4 pt-2.5 pb-1 shrink-0">
              <span className="text-[10px] text-ink-ghost/60 font-mono tracking-widest uppercase">
                {t("main.editor.previewLabel", { defaultValue: "Preview" })}
              </span>
            </div>
          )}
          <div
            className={`flex-1 overflow-y-auto px-6 pb-6 ${
              viewMode === "preview" ? "pt-3" : "pt-1"
            }`}
          >
            <LazyMarkdownPreview
              content={contentSnapshot}
              fontSize={settingsConfig?.fontSize ?? 14}
              renderHtml={settingsConfig?.renderHtmlMarkdown ?? false}
              imageBaseDir={imageBaseDir ?? undefined}
            />
          </div>
        </div>
      )}

      {/* Footer bar */}
      <div className="absolute bottom-0 left-0 right-0 flex items-center justify-between px-4 h-7 border-t border-paper-deep/20 bg-paper/30 shrink-0">
        <div className="flex items-center gap-3 min-w-0">
          <span className="text-[10px] text-ink-ghost font-mono tabular-nums">
            {t("main.statusBar.lineNumber", {
              count: cursorPosition.line,
              defaultValue: "Ln {{count}}",
            })}
          </span>
          <span className="text-[10px] text-ink-ghost font-mono tabular-nums">
            {t("main.statusBar.columnNumber", {
              count: cursorPosition.column,
              defaultValue: "Col {{count}}",
            })}
          </span>
          <span className="text-[10px] text-ink-ghost/40">|</span>
          <span className="text-[10px] text-ink-ghost font-mono tabular-nums">
            {t("main.statusBar.totalLines", {
              count: lineCount,
              defaultValue: "{{count}} lines",
            })}
          </span>
          <span className="text-[10px] text-ink-ghost/40">|</span>
          <span className="text-[10px] text-ink-ghost font-mono tabular-nums">
            {t("common.wordCount", { count: charCount, defaultValue: "{{count}} 字" })}
          </span>
        </div>
        <div className="flex items-center gap-3 min-w-0">
          {selectedId && contentSnapshot.includes("images/") && (
            <>
              <button
                type="button"
                onClick={onCleanUnusedImages}
                className="text-[10px] text-ink-ghost hover:text-bamboo font-mono cursor-pointer transition-colors"
              >
                {t("main.images.cleanUnused", { defaultValue: "清理未使用图片" })}
              </button>
              <span className="text-[10px] text-ink-ghost/40">|</span>
            </>
          )}
          <span className="text-[10px] text-ink-ghost font-mono whitespace-nowrap">
            {documentFormat || t("main.statusBar.format", { defaultValue: "Markdown" })}
          </span>
          <span className="text-[10px] text-ink-ghost/40">|</span>
          <span className="text-[10px] text-ink-ghost font-mono">{newlineFormat}</span>
          <span className="text-[10px] text-ink-ghost/40">|</span>
          <span className="text-[10px] text-ink-ghost font-mono">
            {t("main.statusBar.encoding", { defaultValue: "UTF-8" })}
          </span>
          <span className="text-[10px] text-ink-ghost/40">|</span>
          <span className="text-[10px] text-ink-ghost font-mono tabular-nums">
            {t("main.statusBar.byteSize", { size: byteSize, defaultValue: "{{size}} KB" })}
          </span>
        </div>
      </div>
    </>
  );
});
