import { useCallback, useEffect, useRef, useState } from "react";
import type { MouseEvent } from "react";
import { useMemo } from "react";
import { useTranslation } from "react-i18next";
import {
  createNote,
  getErrorMessage,
  getNote,
  listNotes,
  reloadNoteSourceFile,
  syncNoteSourceFile,
  updateNote,
} from "../features/notes/api";
import { useImagePaste } from "../features/images/useImagePaste";
import { useImageBaseDir } from "../features/images/useImageBaseDir";
import { reportInstallPreparation } from "../features/update/api";
import type { UpdateInstallPrepareRequest } from "../features/update/types";
import type { Note, NoteMetadata, SourceFileChangedPayload } from "../features/notes/types";
import {
  countNoteChars,
  formatShortDate,
  getDisplayTitle,
  metadataFromNote,
} from "../features/notes/noteUtils";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import {
  animateCurrentWindowBounds,
  closeCurrentWindow,
  getCurrentWindowBounds,
  recycleCurrentNotepad,
  setCurrentWindowAlwaysOnTop,
  showCurrentWindow,
  startCurrentWindowDrag,
  startCurrentWindowResize,
} from "../features/windows/controls";
import { openNoteInEditor, reportAppQuitPreparation } from "../features/windows/api";
import type { ResizeDirection } from "../features/windows/controls";
import { getConfig } from "../features/settings/api";
import {
  DEFAULT_TILE_COLOR,
  normalizeTileColor,
  resolveTileColor,
} from "../features/settings/tileColor";
import type { TileColorMode } from "../features/settings/types";
import { shouldSaveBeforeSwitchingToTile } from "../features/windows/noteSurfaceSavePolicy";
import {
  NOTE_SURFACE_ACTION_EVENT,
  surfaceActionFromEvent,
} from "../features/windows/surfaceActions";
import {
  NOTE_SURFACE_MODE_EVENT,
  getSurfaceTargetBounds,
  surfaceModeFromEvent,
} from "../features/windows/surfaceMode";
import type { NoteSurfaceMode } from "../features/windows/surfaceMode";
import {
  emitTileWindowUnpinned,
  tileSurfaceModeUnpinNoteId,
} from "../features/windows/tileWindowEvents";
import { Tile } from "./Tile";

type OpenMode = "new" | "open";
type NotePadStatus = "empty" | "opened" | "saved" | "dirty" | "saveFailed" | "copied";

interface NotePadProps {
  initialNoteId?: string;
  initialSurfaceMode?: NoteSurfaceMode;
  initialAutoSave?: boolean;
  initialTileColor?: string;
}

interface SourceConflictState {
  noteId: string;
  path: string;
  content: string;
  expectedModifiedTime?: number;
}

const surfaceResizeHandles: Array<{
  direction: ResizeDirection;
  className: string;
  size: string;
}> = [
  {
    direction: "NorthWest",
    size: "w-8 h-8",
    className: "top-0 left-0 cursor-nwse-resize",
  },
  {
    direction: "NorthEast",
    size: "w-5 h-5",
    className: "top-0 right-0 cursor-nesw-resize",
  },
  {
    direction: "SouthWest",
    size: "w-8 h-8",
    className: "bottom-0 left-0 cursor-nesw-resize",
  },
  {
    direction: "SouthEast",
    size: "w-5 h-5",
    className: "bottom-0 right-0 cursor-nwse-resize",
  },
];

function SurfaceResizeHandles() {
  return (
    <>
      {surfaceResizeHandles.map((handle) => (
        <div
          key={handle.direction}
          aria-hidden="true"
          data-surface-resize-handle="true"
          data-resize-direction={handle.direction}
          onMouseDown={(event) => {
            event.stopPropagation();
            void startCurrentWindowResize(handle.direction).catch(() => undefined);
          }}
          className={`absolute ${handle.size} opacity-0 ${handle.className}`}
        />
      ))}
    </>
  );
}

export function NotePad({
  initialNoteId,
  initialSurfaceMode = "pad",
  initialAutoSave = true,
  initialTileColor = DEFAULT_TILE_COLOR,
}: NotePadProps) {
  const { t } = useTranslation();
  const [surfaceMode, setSurfaceMode] = useState<NoteSurfaceMode>(initialSurfaceMode);
  const [mode, setMode] = useState<OpenMode>("new");
  const [notes, setNotes] = useState<NoteMetadata[]>([]);
  const [editingNoteId, setEditingNoteId] = useState<string | null>(null);
  const [title, setTitle] = useState("");
  const [content, setContent] = useState("");
  const [hoveredNote, setHoveredNote] = useState<string | null>(null);
  const [status, setStatus] = useState<NotePadStatus>("empty");
  const [errorMessage, setErrorMessage] = useState<string | null>(null);
  const [noteSurfaceAutoSave, setNoteSurfaceAutoSave] = useState(initialAutoSave);
  const [tileColorRaw, setTileColorRaw] = useState(normalizeTileColor(initialTileColor));
  const [tileColorMode, setTileColorMode] = useState<TileColorMode>("system");
  const [surfaceFontSize, setSurfaceFontSize] = useState(14);
  const [tileRenderMarkdown, setTileRenderMarkdown] = useState(false);
  const [tileColor, setTileColor] = useState(() =>
    resolveTileColor("system", normalizeTileColor(initialTileColor)),
  );
  const [isExiting, setIsExiting] = useState(false);
  const [sourceConflict, setSourceConflict] = useState<SourceConflictState | null>(null);
  const [hasSourceUpdate, setHasSourceUpdate] = useState<string | null>(null);
  const [sourceUpdateConfirming, setSourceUpdateConfirming] = useState(false);
  const [pendingSourceUpdates, setPendingSourceUpdates] = useState<Set<string>>(() => new Set());
  const titleRef = useRef<HTMLInputElement>(null);
  const contentRef = useRef<HTMLTextAreaElement>(null);
  const windowLabelRef = useRef("");
  const statusRef = useRef<NotePadStatus>("empty");
  const sourceConflictRef = useRef(sourceConflict);
  const pendingSourceUpdatesRef = useRef(pendingSourceUpdates);
  const allowNextWindowCloseRef = useRef(false);
  const isStandby = useRef(
    typeof window !== "undefined" &&
      new URLSearchParams(window.location.search).get("standby") === "1",
  );
  const hasEnteredOnce = useRef(false);
  const statusLabel = useMemo<Record<NotePadStatus, string>>(
    () => ({
      empty: t("notepad.status.empty", { defaultValue: "空" }),
      opened: t("notepad.status.opened", { defaultValue: "已打开" }),
      saved: t("notepad.status.saved", { defaultValue: "已保存" }),
      dirty: t("notepad.status.unsaved", { defaultValue: "未保存" }),
      saveFailed: t("notepad.status.saveFailed", { defaultValue: "保存失败" }),
      copied: t("notepad.status.copied", { defaultValue: "已复制" }),
    }),
    [t],
  );
  const tabLabels = useMemo(
    () => ({
      new: t("notepad.tab.new", { defaultValue: "新建" }),
      edit: t("notepad.tab.edit", { defaultValue: "编辑" }),
      open: t("notepad.tab.open", { defaultValue: "打开" }),
    }),
    [t],
  );
  statusRef.current = status;
  sourceConflictRef.current = sourceConflict;
  pendingSourceUpdatesRef.current = pendingSourceUpdates;

  function getAppErrorCode(error: unknown): string | null {
    if (error && typeof error === "object" && "code" in error) {
      const code = (error as { code?: unknown }).code;
      return typeof code === "string" ? code : null;
    }
    if (typeof error === "string") {
      return error.match(/^([A-Za-z][A-Za-z0-9]*):/)?.[1] ?? null;
    }
    return null;
  }

  const refreshNotes = useCallback(async () => {
    const loadedNotes = await listNotes();
    setNotes(loadedNotes);
    return loadedNotes;
  }, []);

  const applyNote = useCallback((note: Note) => {
    setEditingNoteId(note.id);
    setTitle(note.title);
    setContent(note.content);
    setMode("new");
    setStatus("opened");
  }, []);

  useEffect(() => {
    let cancelled = false;

    async function bootstrap() {
      try {
        const [loadedConfig] = await Promise.all([getConfig(), refreshNotes()]);
        if (!cancelled) {
          setNoteSurfaceAutoSave(loadedConfig.noteSurfaceAutoSave);
          setSurfaceFontSize(loadedConfig.surfaceFontSize ?? 14);
          setTileRenderMarkdown(loadedConfig.tileRenderMarkdown ?? false);
          setTileColorRaw(normalizeTileColor(loadedConfig.tileColor));
          setTileColorMode(loadedConfig.tileColorMode ?? "system");
          setTileColor(
            resolveTileColor(loadedConfig.tileColorMode ?? "system", loadedConfig.tileColor),
          );
        }
        if (initialNoteId) {
          const note = await getNote(initialNoteId);
          if (!cancelled) {
            applyNote(note);
            setHasSourceUpdate(
              pendingSourceUpdatesRef.current.has(initialNoteId) ? initialNoteId : null,
            );
            setSourceUpdateConfirming(false);
          }
        }
      } catch (error) {
        if (!cancelled) setErrorMessage(getErrorMessage(error));
      }
    }

    void bootstrap();
    return () => {
      cancelled = true;
    };
  }, [applyNote, initialNoteId, refreshNotes]);

  useEffect(() => {
    const unlisten = listen("notes-changed", () => {
      void refreshNotes().catch(() => undefined);
    });
    return () => {
      void unlisten.then((fn) => fn());
    };
  }, [refreshNotes]);

  useEffect(() => {
    const unlisten = listen<SourceFileChangedPayload>("source-file-changed", (event) => {
      const { noteId } = event.payload;
      setPendingSourceUpdates((current) => {
        const next = new Set(current);
        next.add(noteId);
        return next;
      });
      if (editingNoteId !== noteId) return;

      setHasSourceUpdate(noteId);
      setSourceUpdateConfirming(false);
    });
    return () => {
      void unlisten.then((fn) => fn());
    };
  }, [editingNoteId, applyNote]);

  useEffect(() => {
    if (isStandby.current) return;
    let cancelled = false;
    requestAnimationFrame(() => {
      requestAnimationFrame(() => {
        if (!cancelled) {
          hasEnteredOnce.current = true;
          void showCurrentWindow()
            .then(() => contentRef.current?.focus())
            .catch(() => undefined);
        }
      });
    });
    return () => {
      cancelled = true;
    };
  }, []);

  useEffect(() => {
    const unlisten = listen<{
      tileColor?: string;
      tileColorMode?: TileColorMode;
      surfaceFontSize?: number;
      tileRenderMarkdown?: boolean;
    }>("config-changed", (event) => {
      const mode = event.payload.tileColorMode ?? tileColorMode;
      const raw = event.payload.tileColor ?? tileColorRaw;
      setTileColorMode(mode);
      setTileColorRaw(normalizeTileColor(raw));
      setTileColor(resolveTileColor(mode, raw));
      if (event.payload.surfaceFontSize != null) setSurfaceFontSize(event.payload.surfaceFontSize);
      if (event.payload.tileRenderMarkdown != null)
        setTileRenderMarkdown(event.payload.tileRenderMarkdown);
    });
    return () => {
      void unlisten.then((fn) => fn());
    };
  }, [tileColorMode, tileColorRaw]);

  useEffect(() => {
    if (tileColorMode !== "system") return;
    const observer = new MutationObserver(() => {
      setTileColor(resolveTileColor("system", tileColorRaw));
    });
    observer.observe(document.documentElement, {
      attributes: true,
      attributeFilter: ["data-theme"],
    });
    return () => observer.disconnect();
  }, [tileColorMode, tileColorRaw]);

  useEffect(() => {
    let myLabel = "";
    try {
      myLabel = getCurrentWindow().label;
      windowLabelRef.current = myLabel;
    } catch {
      // not in Tauri environment (tests)
    }

    const unlisten = listen<string>("notepad:activate", (event) => {
      if (event.payload !== myLabel) return;

      isStandby.current = false;
      hasEnteredOnce.current = true;
      setEditingNoteId(null);
      setTitle("");
      setContent("");
      setMode("new");
      setStatus("empty");
      setErrorMessage(null);
      setIsExiting(false);
      setSurfaceMode("pad");
      void refreshNotes().catch(() => undefined);
      void showCurrentWindow()
        .then(() => contentRef.current?.focus())
        .catch(() => undefined);
    });
    return () => {
      void unlisten.then((fn) => fn());
    };
  }, [refreshNotes]);

  const saveNote = useCallback(async () => {
    const existingCategory = notes.find((n) => n.id === editingNoteId)?.category ?? "";
    const request = { title, content, category: existingCategory };
    const note = editingNoteId
      ? await updateNote(editingNoteId, request)
      : await createNote(request);

    let syncedSourceModifiedTime = note.sourceModifiedTime;
    if (note.sourcePath) {
      try {
        const synced = await syncNoteSourceFile(note.id, {
          content,
          expectedModifiedTime: note.sourceModifiedTime,
        });
        syncedSourceModifiedTime = synced.sourceModifiedTime;
        setPendingSourceUpdates((current) => {
          const next = new Set(current);
          next.delete(note.id);
          return next;
        });
        setHasSourceUpdate((current) => (current === note.id ? null : current));
        setSourceUpdateConfirming(false);
      } catch (syncError) {
        const errorCode = getAppErrorCode(syncError);
        if (errorCode === "sourceFileConflict") {
          setSourceConflict({
            noteId: note.id,
            path: note.sourcePath,
            content,
            expectedModifiedTime: note.sourceModifiedTime,
          });
          setStatus("dirty");
          throw syncError;
        }
        if (errorCode === "sourceFileMissing") {
          setErrorMessage(getErrorMessage(syncError));
        } else {
          throw syncError;
        }
      }
    }

    const savedNote = {
      ...note,
      sourceModifiedTime: syncedSourceModifiedTime,
    };

    setEditingNoteId(note.id);
    setNotes((current) => {
      const metadata = metadataFromNote(savedNote);
      const exists = current.some((item) => item.id === note.id);
      const next = exists
        ? current.map((item) => (item.id === note.id ? metadata : item))
        : [metadata, ...current];
      return [...next].sort((left, right) => right.updatedAt.localeCompare(left.updatedAt));
    });
    setStatus("saved");
    return savedNote;
  }, [content, editingNoteId, notes, title]);

  const hasDraftContent = useCallback(
    () => Boolean(editingNoteId || title.trim() || content.trim()),
    [content, editingNoteId, title],
  );

  const ensureDraftSaved = useCallback(async () => {
    if (sourceConflictRef.current) return false;
    if (statusRef.current !== "dirty" || !hasDraftContent()) return true;

    try {
      await saveNote();
      return true;
    } catch (error) {
      setStatus("saveFailed");
      setErrorMessage(getErrorMessage(error));
      return false;
    }
  }, [hasDraftContent, saveNote]);

  useEffect(() => {
    const unlisten = getCurrentWindow().onCloseRequested(async (event) => {
      if (allowNextWindowCloseRef.current) {
        allowNextWindowCloseRef.current = false;
        return;
      }

      if (sourceConflictRef.current) {
        event.preventDefault();
        return;
      }

      if (statusRef.current !== "dirty") return;

      event.preventDefault();
      if (!(await ensureDraftSaved())) return;

      allowNextWindowCloseRef.current = true;
      await closeCurrentWindow();
    });

    return () => {
      void unlisten.then((fn) => fn());
    };
  }, [ensureDraftSaved]);

  useEffect(() => {
    const unlisten = listen<UpdateInstallPrepareRequest>("update://prepare-install", (event) => {
      const respond = async () => {
        const windowLabel = windowLabelRef.current || "notepad";
        if (statusRef.current !== "dirty") {
          await reportInstallPreparation(event.payload.requestId, windowLabel, "ready");
          return;
        }

        try {
          await saveNote();
          await reportInstallPreparation(event.payload.requestId, windowLabel, "ready");
        } catch (error) {
          setStatus("saveFailed");
          setErrorMessage(getErrorMessage(error));
          await reportInstallPreparation(
            event.payload.requestId,
            windowLabel,
            "failed",
            getErrorMessage(error),
          );
        }
      };

      void respond().catch(async (error) => {
        await reportInstallPreparation(
          event.payload.requestId,
          windowLabelRef.current || "notepad",
          "failed",
          getErrorMessage(error),
        ).catch(() => undefined);
      });
    });
    return () => {
      void unlisten.then((fn) => fn());
    };
  }, [saveNote]);

  useEffect(() => {
    const unlisten = listen<{ requestId: string }>("app-quit-requested", (event) => {
      const respond = async () => {
        const windowLabel = windowLabelRef.current || "notepad";
        if (sourceConflictRef.current) {
          await reportAppQuitPreparation(
            event.payload.requestId,
            windowLabel,
            "failed",
            t("main.sourceConflict.title", { defaultValue: "原文件已被修改" }),
          );
          return;
        }

        const saved = await ensureDraftSaved();
        await reportAppQuitPreparation(
          event.payload.requestId,
          windowLabel,
          saved ? "ready" : "failed",
          saved
            ? undefined
            : t("settings.update.error.installSaveFailed", {
                defaultValue: "安装前自动保存失败，请先处理当前笔记后重试",
              }),
        );
      };

      void respond().catch((error) => {
        setErrorMessage(getErrorMessage(error));
        void reportAppQuitPreparation(
          event.payload.requestId,
          windowLabelRef.current || "notepad",
          "failed",
          getErrorMessage(error),
        ).catch(() => undefined);
      });
    });
    return () => {
      void unlisten.then((fn) => fn());
    };
  }, [ensureDraftSaved, t]);

  const imageBaseDir = useImageBaseDir();

  const ensureNoteSaved = useCallback(async (): Promise<string | null> => {
    if (editingNoteId) return editingNoteId;
    try {
      const note = await saveNote();
      return note.id;
    } catch {
      return null;
    }
  }, [editingNoteId, saveNote]);

  const {
    handlePaste: imagePasteHandler,
    handleDrop: imageDropHandler,
    handleDragOver: imageDragOverHandler,
  } = useImagePaste({
    noteId: editingNoteId,
    textareaRef: contentRef,
    setContent,
    markDirty: () => setStatus("dirty"),
    onEnsureNoteSaved: ensureNoteSaved,
    onError: setErrorMessage,
    t,
  });

  const tileNoteId = editingNoteId ?? initialNoteId ?? "";

  const switchSurfaceMode = useCallback(
    async (nextMode: NoteSurfaceMode) => {
      const unpinnedNoteId = tileSurfaceModeUnpinNoteId(surfaceMode, nextMode, tileNoteId);
      setSurfaceMode(nextMode);
      if (unpinnedNoteId) {
        void emitTileWindowUnpinned(unpinnedNoteId).catch(() => undefined);
      }

      try {
        if (nextMode === "tile") {
          await setCurrentWindowAlwaysOnTop(true);
        }

        const currentBounds = await getCurrentWindowBounds();
        await animateCurrentWindowBounds(getSurfaceTargetBounds(nextMode, currentBounds));
      } catch (error) {
        setErrorMessage(getErrorMessage(error));
      }
    },
    [surfaceMode, tileNoteId],
  );

  useEffect(() => {
    function handleSurfaceModeRequest(event: Event) {
      const nextMode = surfaceModeFromEvent(event);
      if (!nextMode) return;
      void switchSurfaceMode(nextMode);
    }

    window.addEventListener(NOTE_SURFACE_MODE_EVENT, handleSurfaceModeRequest);
    return () => {
      window.removeEventListener(NOTE_SURFACE_MODE_EVENT, handleSurfaceModeRequest);
    };
  }, [switchSurfaceMode]);

  useEffect(() => {
    if (surfaceMode !== "tile") return;
    void setCurrentWindowAlwaysOnTop(true).catch(() => undefined);
  }, [surfaceMode]);

  const handleSave = useCallback(async () => {
    setErrorMessage(null);
    try {
      await saveNote();
    } catch (error) {
      setStatus("saveFailed");
      setErrorMessage(getErrorMessage(error));
    }
  }, [saveNote]);

  const handleReloadSourceFile = useCallback(async () => {
    if (!sourceConflict) return;
    setErrorMessage(null);
    try {
      const note = await reloadNoteSourceFile(sourceConflict.noteId);
      applyNote(note);
      setNotes((current) => {
        const metadata = metadataFromNote(note);
        return current.map((item) => (item.id === metadata.id ? metadata : item));
      });
      setPendingSourceUpdates((current) => {
        const next = new Set(current);
        next.delete(sourceConflict.noteId);
        return next;
      });
      setHasSourceUpdate(null);
      setSourceUpdateConfirming(false);
      setSourceConflict(null);
    } catch (error) {
      setErrorMessage(getErrorMessage(error));
    }
  }, [applyNote, sourceConflict]);

  const handleForceOverwriteSourceFile = useCallback(async () => {
    if (!sourceConflict) return;
    setErrorMessage(null);
    try {
      const synced = await syncNoteSourceFile(sourceConflict.noteId, {
        content: sourceConflict.content,
        expectedModifiedTime: sourceConflict.expectedModifiedTime,
        force: true,
      });
      setNotes((current) =>
        current.map((item) => (item.id === synced.id ? { ...item, ...synced } : item)),
      );
      setPendingSourceUpdates((current) => {
        const next = new Set(current);
        next.delete(sourceConflict.noteId);
        return next;
      });
      setHasSourceUpdate(null);
      setSourceUpdateConfirming(false);
      setSourceConflict(null);
      setStatus("saved");
    } catch (error) {
      setErrorMessage(getErrorMessage(error));
    }
  }, [sourceConflict]);

  const handleKeepLocalOnlyForNow = useCallback(() => {
    setSourceConflict(null);
    setStatus("saved");
    setErrorMessage(
      t("main.sourceConflict.keptLocal", {
        defaultValue: "已保留 MiniNote 中的内容，暂未写回原文件",
      }),
    );
  }, [t]);

  const handleViewSourceUpdate = useCallback(async () => {
    if (!hasSourceUpdate) return;
    if (statusRef.current === "dirty") {
      setSourceUpdateConfirming(true);
      return;
    }
    try {
      const note = await reloadNoteSourceFile(hasSourceUpdate);
      applyNote(note);
      setPendingSourceUpdates((current) => {
        const next = new Set(current);
        next.delete(hasSourceUpdate);
        return next;
      });
      setHasSourceUpdate(null);
      setSourceUpdateConfirming(false);
    } catch (error) {
      setErrorMessage(getErrorMessage(error));
    }
  }, [hasSourceUpdate, applyNote]);

  const handleConfirmSourceUpdate = useCallback(async () => {
    if (!hasSourceUpdate) return;
    try {
      const note = await reloadNoteSourceFile(hasSourceUpdate);
      applyNote(note);
      setPendingSourceUpdates((current) => {
        const next = new Set(current);
        next.delete(hasSourceUpdate);
        return next;
      });
      setHasSourceUpdate(null);
      setSourceUpdateConfirming(false);
    } catch (error) {
      setErrorMessage(getErrorMessage(error));
    }
  }, [hasSourceUpdate, applyNote]);

  const handleDismissSourceUpdate = useCallback(() => {
    setPendingSourceUpdates((current) => {
      if (!hasSourceUpdate) return current;
      const next = new Set(current);
      next.delete(hasSourceUpdate);
      return next;
    });
    setHasSourceUpdate(null);
    setSourceUpdateConfirming(false);
  }, [hasSourceUpdate]);

  useEffect(() => {
    function handleKeyDown(event: KeyboardEvent) {
      if ((event.ctrlKey || event.metaKey) && event.key === "s") {
        event.preventDefault();
        void handleSave();
      }
    }

    document.addEventListener("keydown", handleKeyDown);
    return () => document.removeEventListener("keydown", handleKeyDown);
  }, [handleSave]);

  const handleOpenNote = async (noteId: string) => {
    setErrorMessage(null);
    if (!(await ensureDraftSaved())) return;
    try {
      const note = await getNote(noteId);
      applyNote(note);
      setHasSourceUpdate(pendingSourceUpdatesRef.current.has(noteId) ? noteId : null);
      setSourceUpdateConfirming(false);
      await switchSurfaceMode("pad");
    } catch (error) {
      setErrorMessage(getErrorMessage(error));
    }
  };

  const handlePin = async () => {
    setErrorMessage(null);
    try {
      if (shouldSaveBeforeSwitchingToTile(noteSurfaceAutoSave) || statusRef.current === "dirty") {
        await saveNote();
      }
      await switchSurfaceMode("tile");
    } catch (error) {
      setErrorMessage(getErrorMessage(error));
    }
  };

  const handleClose = useCallback(() => {
    setIsExiting(true);
    void (async () => {
      if (!(await ensureDraftSaved())) {
        setIsExiting(false);
        return;
      }
      const closeSurface = surfaceMode === "tile" ? closeCurrentWindow : recycleCurrentNotepad;
      await closeSurface();
    })().catch((error) => {
      setIsExiting(false);
      setErrorMessage(getErrorMessage(error));
    });
  }, [ensureDraftSaved, surfaceMode]);

  const copyTileContent = useCallback(async () => {
    setErrorMessage(null);
    try {
      const clipboard = navigator.clipboard;
      if (!clipboard?.writeText) {
        throw new Error(t("notepad.error.copyUnsupported", { defaultValue: "当前环境不支持复制" }));
      }
      await clipboard.writeText(content);
      setStatus("copied");
    } catch (error) {
      setErrorMessage(getErrorMessage(error));
    }
  }, [content, t]);

  useEffect(() => {
    function handleSurfaceActionRequest(event: Event) {
      const action = surfaceActionFromEvent(event);
      if (!action) return;

      if (action === "copy") {
        void copyTileContent();
        return;
      }

      if (action === "save") {
        void handleSave();
        return;
      }

      if (action === "close") {
        void handleClose();
        return;
      }

      void switchSurfaceMode("pad");
    }

    window.addEventListener(NOTE_SURFACE_ACTION_EVENT, handleSurfaceActionRequest);
    return () => {
      window.removeEventListener(NOTE_SURFACE_ACTION_EVENT, handleSurfaceActionRequest);
    };
  }, [copyTileContent, handleClose, handleSave, switchSurfaceMode]);

  useEffect(() => {
    if (!noteSurfaceAutoSave || mode !== "new" || status !== "dirty") {
      return undefined;
    }
    if (!hasDraftContent()) return undefined;

    const timer = window.setTimeout(() => {
      void handleSave();
    }, 900);

    return () => window.clearTimeout(timer);
  }, [handleSave, hasDraftContent, mode, noteSurfaceAutoSave, status]);

  const handleDrag = (event: MouseEvent<HTMLElement>) => {
    const target = event.target as HTMLElement;
    if (target.closest("button,input,textarea")) return;
    void startCurrentWindowDrag().catch(() => undefined);
  };

  const resetDraft = () => {
    setEditingNoteId(null);
    setTitle("");
    setContent("");
    setMode("new");
    setStatus("empty");
    setErrorMessage(null);
  };

  const handleClearDraft = () => {
    if (statusRef.current === "dirty" && hasDraftContent()) {
      const confirmed = window.confirm(
        t("notepad.confirmDiscard", { defaultValue: "当前内容尚未保存，确定清空吗？" }),
      );
      if (!confirmed) return;
    }
    resetDraft();
  };

  const isTile = surfaceMode === "tile";
  const tileTitle = title.trim();
  const enterClass = hasEnteredOnce.current ? "" : "animate-window-enter";
  const surfaceWrapperClassName = `w-full h-screen flex flex-col bg-transparent p-0 ${isExiting ? "animate-window-exit" : enterClass}`;
  const padSurfaceClassName =
    "app-surface-frame relative noise-bg w-full h-full min-h-0 bg-cloud overflow-hidden flex flex-col flex-1 border border-paper-deep/70 shadow-[0_1px_10px_rgba(26,26,24,0.06)] transition-all duration-200 ease-out";

  return (
    <div className={surfaceWrapperClassName}>
      {sourceConflict && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-ink/18 backdrop-blur-[2px] px-4">
          <div className="w-full max-w-[360px] rounded-xl border border-paper-deep/50 bg-paper/95 shadow-[0_18px_60px_rgba(47,40,32,0.18)] px-5 py-4">
            <h2 className="text-[15px] font-display font-semibold text-ink">
              {t("main.sourceConflict.title", { defaultValue: "原文件已被修改" })}
            </h2>
            <p className="mt-2 text-[12px] leading-relaxed text-ink-soft">
              {t("main.sourceConflict.description", {
                defaultValue:
                  "MiniNote 已保存当前内容；原文件也被其他程序改过。请选择下一步，避免覆盖不想丢的内容。",
              })}
            </p>
            <p className="mt-2 text-[11px] leading-relaxed text-ink-ghost break-all">
              {sourceConflict.path}
            </p>
            <div className="mt-4 flex flex-wrap justify-end gap-2">
              <button
                type="button"
                onClick={handleKeepLocalOnlyForNow}
                className="h-8 px-3 rounded-lg text-[12px] text-ink-ghost hover:text-ink-soft hover:bg-paper-warm transition-colors cursor-pointer"
              >
                {t("main.sourceConflict.keepLocal", { defaultValue: "只保存到 MiniNote" })}
              </button>
              <button
                type="button"
                onClick={() => void handleReloadSourceFile()}
                className="h-8 px-3 rounded-lg text-[12px] text-ink-soft bg-paper-warm/80 hover:bg-paper-warm border border-paper-deep/40 transition-colors cursor-pointer"
              >
                {t("main.sourceConflict.reloadDisk", { defaultValue: "载入原文件内容" })}
              </button>
              <button
                type="button"
                onClick={() => void handleForceOverwriteSourceFile()}
                className="h-8 px-3 rounded-lg text-[12px] text-paper bg-bamboo hover:bg-bamboo-light transition-colors cursor-pointer"
              >
                {t("main.sourceConflict.forceOverwrite", { defaultValue: "覆盖原文件" })}
              </button>
            </div>
          </div>
        </div>
      )}
      {hasSourceUpdate && (
        <div className="flex items-center gap-2 px-4 py-1.5 bg-amber-50/80 border-b border-amber-200/50 text-amber-800 shrink-0">
          <svg
            width="13"
            height="13"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            strokeLinecap="round"
            strokeLinejoin="round"
            className="shrink-0"
          >
            <path d="M21 12a9 9 0 1 1-6.22-8.56" />
            <path d="M21 3v5h-5" />
          </svg>
          {sourceUpdateConfirming ? (
            <>
              <span className="text-[11px] flex-1">
                {t("main.sourceUpdate.confirmDiscard", {
                  defaultValue: "当前编辑还未保存，查看更新会替换掉现有内容",
                })}
              </span>
              <button
                type="button"
                onClick={() => void handleConfirmSourceUpdate()}
                className="text-[11px] px-2 py-0.5 rounded bg-amber-200 hover:bg-amber-300 transition-colors cursor-pointer font-medium"
              >
                {t("main.sourceUpdate.confirmYes", { defaultValue: "仍然查看" })}
              </button>
              <button
                type="button"
                onClick={() => setSourceUpdateConfirming(false)}
                className="text-[11px] px-2 py-0.5 rounded hover:bg-amber-200/60 transition-colors cursor-pointer"
              >
                {t("main.sourceUpdate.cancel", { defaultValue: "取消" })}
              </button>
            </>
          ) : (
            <>
              <span className="text-[11px] flex-1">
                {t("main.sourceUpdate.available", { defaultValue: "外部文件已更新" })}
              </span>
              <button
                type="button"
                onClick={() => void handleViewSourceUpdate()}
                className="text-[11px] px-2 py-0.5 rounded bg-amber-100 hover:bg-amber-200 transition-colors cursor-pointer"
              >
                {t("main.sourceUpdate.view", { defaultValue: "查看更新" })}
              </button>
              <button
                type="button"
                onClick={handleDismissSourceUpdate}
                className="text-[11px] px-2 py-0.5 rounded hover:bg-amber-200/60 transition-colors cursor-pointer"
              >
                {t("main.sourceUpdate.ignore", { defaultValue: "忽略" })}
              </button>
            </>
          )}
        </div>
      )}
      {isTile ? (
        <Tile
          title={tileTitle || undefined}
          content={errorMessage || content}
          color={tileColor}
          fontSize={surfaceFontSize}
          renderMarkdown={!errorMessage && tileRenderMarkdown}
          imageBaseDir={imageBaseDir ?? undefined}
          width="100%"
          className="h-full cursor-default"
          data-surface-mode={surfaceMode}
          data-context-menu="tile"
          data-note-id={tileNoteId}
          onMouseDown={handleDrag}
        >
          <button
            type="button"
            aria-label={t("contextMenu.tile.close", { defaultValue: "取消钉屏" })}
            title={t("contextMenu.tile.close", { defaultValue: "取消钉屏" })}
            onMouseDown={(event) => event.stopPropagation()}
            onClick={() => void handleClose()}
            className="absolute top-2 right-2 z-10 w-6 h-6 flex items-center justify-center rounded-full text-ink-ghost/70 hover:text-red-400 hover:bg-danger-bg/80 transition-colors cursor-pointer"
          >
            <svg
              width="12"
              height="12"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2.5"
              strokeLinecap="round"
            >
              <path d="M18 6L6 18M6 6l12 12" />
            </svg>
          </button>
          <SurfaceResizeHandles />
        </Tile>
      ) : (
        <div className={padSurfaceClassName} data-surface-mode={surfaceMode}>
          <>
            <div
              className="flex items-center justify-between px-4 pt-3 pb-0 cursor-default"
              onMouseDown={handleDrag}
            >
              <div className="flex items-center gap-0.5">
                <button
                  onClick={() => {
                    if (mode !== "new") setMode("new");
                  }}
                  className={`relative px-3.5 py-1.5 text-[13px] rounded-t-lg transition-all duration-200 cursor-pointer ${
                    mode === "new"
                      ? "text-bamboo font-medium"
                      : "text-ink-ghost hover:text-ink-faint"
                  }`}
                >
                  {editingNoteId ? tabLabels.edit : tabLabels.new}
                  {mode === "new" && (
                    <div className="absolute bottom-0 left-3 right-3 h-[2px] bg-bamboo rounded-full" />
                  )}
                </button>
                <button
                  onClick={() => setMode("open")}
                  className={`relative px-3.5 py-1.5 text-[13px] rounded-t-lg transition-all duration-200 cursor-pointer ${
                    mode === "open"
                      ? "text-bamboo font-medium"
                      : "text-ink-ghost hover:text-ink-faint"
                  }`}
                >
                  {tabLabels.open}
                  {mode === "open" && (
                    <div className="absolute bottom-0 left-3 right-3 h-[2px] bg-bamboo rounded-full" />
                  )}
                </button>
              </div>

              <div className="flex items-center gap-1.5">
                <button
                  onClick={() => void handlePin()}
                  className="group w-7 h-7 flex items-center justify-center rounded-lg transition-all duration-200 cursor-pointer text-ink-ghost hover:text-ink-faint hover:bg-paper-warm"
                  title={t("notepad.tooltip.pinToTile", { defaultValue: "转为磁贴" })}
                >
                  <svg
                    width="14"
                    height="14"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    strokeWidth="2"
                    strokeLinecap="round"
                    strokeLinejoin="round"
                  >
                    <path d="M12 17v5" />
                    <path d="M9 10.76a2 2 0 0 1-1.11 1.79l-1.78.9A2 2 0 0 0 5 15.24V16a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1v-.76a2 2 0 0 0-1.11-1.79l-1.78-.9A2 2 0 0 1 15 10.76V7a1 1 0 0 1 1-1 1 1 0 0 0 1-1V4a1 1 0 0 0-1-1H8a1 1 0 0 0-1 1v1a1 1 0 0 0 1 1 1 1 0 0 1 1 1z" />
                  </svg>
                </button>

                <button
                  onClick={() => void handleClose()}
                  className="group w-7 h-7 flex items-center justify-center rounded-lg text-ink-ghost hover:bg-danger-bg hover:text-red-400 transition-all duration-200 cursor-pointer"
                  title={t("notepad.tooltip.close", { defaultValue: "关闭" })}
                >
                  <svg
                    width="13"
                    height="13"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    strokeWidth="2.5"
                    strokeLinecap="round"
                  >
                    <path d="M18 6L6 18M6 6l12 12" />
                  </svg>
                </button>
              </div>
            </div>

            <div className="mx-4 mt-1 h-px bg-paper-deep/50" />

            {mode === "new" ? (
              <div
                data-pad-editor-body="true"
                className="px-4 pt-3 pb-2 flex flex-col flex-1 min-h-0"
              >
                <input
                  ref={titleRef}
                  type="text"
                  value={title}
                  onChange={(event) => {
                    setTitle(event.target.value);
                    setStatus("dirty");
                  }}
                  onKeyDown={(event) => {
                    if (event.key === "Enter" || event.key === "ArrowDown") {
                      event.preventDefault();
                      contentRef.current?.focus();
                    }
                  }}
                  placeholder={t("notepad.placeholder.title", { defaultValue: "标题（可选）" })}
                  className="w-full font-display font-medium text-ink placeholder:text-ink-ghost/60 mb-2 tracking-wide shrink-0"
                  style={{ fontSize: `${surfaceFontSize}px` }}
                />

                <textarea
                  ref={contentRef}
                  data-tab-indent="true"
                  value={content}
                  onChange={(event) => {
                    setContent(event.target.value);
                    setStatus("dirty");
                  }}
                  onPaste={imagePasteHandler}
                  onDrop={imageDropHandler}
                  onDragOver={imageDragOverHandler}
                  onKeyDown={(event) => {
                    if (event.key === "ArrowUp") {
                      const ta = contentRef.current;
                      if (ta && ta.selectionStart === ta.selectionEnd) {
                        const textBeforeCursor = content.slice(0, ta.selectionStart);
                        if (!textBeforeCursor.includes("\n")) {
                          event.preventDefault();
                          titleRef.current?.focus();
                        }
                      }
                    }
                  }}
                  placeholder={t("notepad.placeholder.content", { defaultValue: "写点什么……" })}
                  className="w-full flex-1 min-h-0 pb-2 text-ink-soft font-body placeholder:text-ink-ghost/50"
                  style={{
                    fontSize: `${surfaceFontSize}px`,
                    lineHeight: "normal",
                    tabSize: `var(--tab-indent-size, 2)`,
                  }}
                />

                <div className="flex items-center justify-between mt-auto pt-2 border-t border-paper-deep/30 shrink-0">
                  <span className="text-[11px] text-ink-ghost font-mono tabular-nums truncate max-w-[170px]">
                    {errorMessage ??
                      `${countNoteChars(content)} ${t("common.wordCountUnit", { defaultValue: "字" })} · ${statusLabel[status]}`}
                  </span>
                  <div className="flex items-center gap-2">
                    <button
                      onClick={handleClearDraft}
                      className="px-4 py-1.5 text-[12px] text-ink-faint hover:text-ink-soft rounded-lg hover:bg-paper-warm transition-all duration-200 cursor-pointer"
                    >
                      {t("notepad.button.clear", { defaultValue: "清空" })}
                    </button>
                    <button
                      onClick={() => void handleSave()}
                      className="px-4 py-1.5 text-[12px] text-cloud bg-bamboo hover:bg-bamboo-light rounded-lg transition-all duration-200 font-medium cursor-pointer"
                    >
                      {t("common.save", { defaultValue: "保存" })}
                    </button>
                  </div>
                </div>
              </div>
            ) : (
              <div className="p-2 flex-1 min-h-0 overflow-y-auto">
                <div className="space-y-0.5">
                  {notes.map((note) => (
                    <button
                      key={note.id}
                      onClick={() => void handleOpenNote(note.id)}
                      onMouseEnter={() => setHoveredNote(note.id)}
                      onMouseLeave={() => setHoveredNote(null)}
                      className="w-full text-left px-3.5 py-3 rounded-xl transition-all duration-200 cursor-pointer group hover:bg-paper-warm/70"
                    >
                      <div className="flex items-center justify-between mb-0.5">
                        <span className="text-[13px] font-display font-medium text-ink-soft group-hover:text-ink transition-colors truncate pr-2">
                          {getDisplayTitle(note)}
                        </span>
                        <div className="flex items-center gap-1.5 shrink-0">
                          <button
                            onClick={(e) => {
                              e.stopPropagation();
                              void (async () => {
                                if (!(await ensureDraftSaved())) return;
                                await openNoteInEditor(note.id);
                              })();
                            }}
                            className="w-6 h-6 flex items-center justify-center rounded-md text-ink-ghost hover:text-bamboo hover:bg-bamboo-mist/50 transition-all duration-200 opacity-0 group-hover:opacity-100 cursor-pointer"
                            title={t("notepad.tooltip.openInEditor", {
                              defaultValue: "在编辑器中打开",
                            })}
                          >
                            <svg
                              width="13"
                              height="13"
                              viewBox="0 0 24 24"
                              fill="none"
                              stroke="currentColor"
                              strokeWidth="2"
                              strokeLinecap="round"
                              strokeLinejoin="round"
                            >
                              <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" />
                              <polyline points="15 3 21 3 21 9" />
                              <line x1="10" y1="14" x2="21" y2="3" />
                            </svg>
                          </button>
                          <span className="text-[11px] text-ink-ghost font-mono tabular-nums">
                            {formatShortDate(note.updatedAt)}
                          </span>
                        </div>
                      </div>
                      <p className="text-[12px] text-ink-ghost leading-relaxed line-clamp-1 group-hover:text-ink-faint transition-colors">
                        {note.preview || t("common.blankNote", { defaultValue: "空白笔记" })}
                      </p>
                      {hoveredNote === note.id && (
                        <div className="mt-1.5 h-px bg-bamboo/10 transition-all duration-300" />
                      )}
                    </button>
                  ))}
                  {notes.length === 0 && (
                    <div className="px-4 py-8 text-center text-[12px] text-ink-ghost">
                      {t("notepad.emptyState", { defaultValue: "还没有可打开的笔记" })}
                    </div>
                  )}
                </div>
              </div>
            )}
          </>
          <SurfaceResizeHandles />
        </div>
      )}
    </div>
  );
}
