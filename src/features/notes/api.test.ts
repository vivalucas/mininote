import { i18n } from "../../locales";
import { describe, expect, test, vi } from "vitest";
import { getErrorMessage, reloadNoteSourceFile, syncNoteSourceFile } from "./api";

const invoke = vi.hoisted(() => vi.fn());

vi.mock("@tauri-apps/api/core", () => ({
  invoke,
}));

describe("notes api error localization", () => {
  test("localizes structured backend errors with interpolation details", () => {
    expect(
      getErrorMessage({
        code: "categoryAlreadyExists",
        message: "分类「工作」已存在",
        details: { category: "工作" },
      }),
    ).toBe("分类「工作」已存在");
  });

  test("localizes shortcut configuration errors with settings labels", () => {
    expect(
      getErrorMessage({
        code: "unsupportedShortcut",
        message: "unsupported globalShortcut shortcut config: Ctrl+",
        details: { field: "globalShortcut" },
      }),
    ).toBe("快捷记录快捷键 配置无效");
  });

  test("parses serialized backend error strings when a structured payload is unavailable", () => {
    expect(getErrorMessage("noteNotFound: Note note-1 was not found")).toBe("找不到该笔记");
  });

  test("localizes serialized category errors when interpolation details can be recovered", () => {
    const translate = i18n.getFixedT("en-US");

    expect(getErrorMessage("categoryNotFound: 分类「工作」不存在", translate)).toBe(
      'Category "工作" not found',
    );
    expect(getErrorMessage("categoryAlreadyExists: 分类「工作」已存在", translate)).toBe(
      'Category "工作" already exists',
    );
  });

  test("falls back to the backend message for unknown error codes", () => {
    expect(
      getErrorMessage({
        code: "mysteryError",
        message: "something went wrong",
      }),
    ).toBe("something went wrong");
  });

  test("localizes image validation errors from backend commands", () => {
    expect(getErrorMessage({ code: "unsupportedImageFormat", message: "unsupported" })).toBe(
      "不支持的图片格式",
    );
    expect(
      getErrorMessage({
        code: "imageTooLarge",
        message: "too large",
        details: { maxMb: "30" },
      }),
    ).toBe("图片文件过大（上限 30 MB）");
    expect(
      getErrorMessage({
        code: "fileTooLarge",
        message: "too large",
        details: { maxMb: "10" },
      }),
    ).toBe("文件过大（上限 10 MB）");
  });
});

describe("notes api commands", () => {
  test("reloads a source file by note id instead of arbitrary path", async () => {
    const note = {
      id: "note-1",
      title: "A",
      fileName: "note-1_A.md",
      category: "",
      createdAt: "2026-01-01T00:00:00Z",
      updatedAt: "2026-01-01T00:00:00Z",
      wordCount: 4,
      content: "disk",
    };
    invoke.mockResolvedValueOnce(note);

    await expect(reloadNoteSourceFile("note-1")).resolves.toBe(note);
    expect(invoke).toHaveBeenCalledWith("notes_reload_source_file", { id: "note-1" });
  });

  test("syncs a linked source file through the guarded backend command", async () => {
    const metadata = {
      id: "note-1",
      title: "A",
      fileName: "note-1_A.md",
      category: "",
      sourcePath: "/tmp/source.md",
      sourceModifiedTime: 1234,
      createdAt: "2026-01-01T00:00:00Z",
      updatedAt: "2026-01-01T00:00:00Z",
      wordCount: 4,
      preview: "body",
    };
    invoke.mockResolvedValueOnce(metadata);

    await expect(
      syncNoteSourceFile("note-1", {
        content: "body",
        expectedModifiedTime: 1000,
        force: true,
      }),
    ).resolves.toBe(metadata);
    expect(invoke).toHaveBeenCalledWith("notes_sync_source_file", {
      id: "note-1",
      request: {
        content: "body",
        expectedModifiedTime: 1000,
        force: true,
      },
    });
  });
});
