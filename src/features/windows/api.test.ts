import { invoke } from "@tauri-apps/api/core";
import { beforeEach, describe, expect, test, vi } from "vitest";
import {
  openNotepadWindow,
  openTileWindow,
  reportAppQuitPreparation,
  requestAppExit,
  requestMainWindowClose,
  takeStartupFiles,
  toggleTileWindow,
  type WindowBounds,
} from "./api";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

describe("window api", () => {
  beforeEach(() => {
    vi.mocked(invoke).mockReset();
  });

  test("passes optional bounds when opening tile and notepad windows", async () => {
    const bounds: WindowBounds = { x: 12, y: 34, width: 320, height: 240 };
    vi.mocked(invoke).mockResolvedValue("ok");

    await openTileWindow("note-1", bounds);
    await openNotepadWindow("note-1", bounds);

    expect(invoke).toHaveBeenNthCalledWith(1, "open_tile_window", {
      noteId: "note-1",
      bounds,
    });
    expect(invoke).toHaveBeenNthCalledWith(2, "open_notepad_window", {
      noteId: "note-1",
      bounds,
    });
  });

  test("toggles a tile window for a note", async () => {
    vi.mocked(invoke).mockResolvedValue(false);

    await expect(toggleTileWindow("note-1")).resolves.toBe(false);

    expect(invoke).toHaveBeenCalledWith("toggle_tile_window", {
      noteId: "note-1",
      bounds: null,
    });
  });

  test("requests approved close and exit through backend commands", async () => {
    vi.mocked(invoke).mockResolvedValue(undefined);

    await requestMainWindowClose();
    await requestAppExit();
    await reportAppQuitPreparation("request-1", "main", "failed", "save failed");

    expect(invoke).toHaveBeenNthCalledWith(1, "request_main_window_close");
    expect(invoke).toHaveBeenNthCalledWith(2, "request_app_exit");
    expect(invoke).toHaveBeenNthCalledWith(3, "report_app_quit_preparation", {
      requestId: "request-1",
      windowLabel: "main",
      status: "failed",
      message: "save failed",
    });
  });

  test("takes queued external file-open requests through backend", async () => {
    vi.mocked(invoke).mockResolvedValue(["/tmp/note.md"]);

    await expect(takeStartupFiles()).resolves.toEqual(["/tmp/note.md"]);

    expect(invoke).toHaveBeenCalledWith("take_startup_files");
  });
});
