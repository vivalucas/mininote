import { invoke } from "@tauri-apps/api/core";
import { open, save } from "@tauri-apps/plugin-dialog";
import { beforeEach, describe, expect, test, vi } from "vitest";
import { exportMarkdownNote, importMarkdownNote } from "./api";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

vi.mock("@tauri-apps/plugin-dialog", () => ({
  open: vi.fn(),
  save: vi.fn(),
}));

const mockedInvoke = vi.mocked(invoke);
const mockedOpen = vi.mocked(open);
const mockedSave = vi.mocked(save);

const textDocumentFilters = [
  { name: "Mint 文档", extensions: ["mint"] },
  { name: "Markdown 文档", extensions: ["md", "markdown"] },
  { name: "文本文件", extensions: ["txt"] },
];

describe("importExport api", () => {
  beforeEach(() => {
    mockedInvoke.mockReset();
    mockedOpen.mockReset();
    mockedSave.mockReset();
  });

  test("imports the selected text document path through Rust", async () => {
    mockedOpen.mockResolvedValue("D:\\notes\\外部笔记.mint");
    mockedInvoke.mockResolvedValue({
      id: "note-1",
      title: "外部笔记",
      fileName: "note-1.md",
      createdAt: "2026-04-28T00:00:00Z",
      updatedAt: "2026-04-28T00:00:00Z",
      wordCount: 4,
      content: "# 标题\n正文",
    });

    const note = await importMarkdownNote();

    expect(open).toHaveBeenCalledWith({
      title: "导入文件",
      multiple: false,
      directory: false,
      filters: textDocumentFilters,
    });
    expect(invoke).toHaveBeenCalledWith("notes_import_markdown", {
      path: "D:\\notes\\外部笔记.mint",
      category: "",
    });
    expect(note?.id).toBe("note-1");
  });

  test("returns null when the file picker is cancelled", async () => {
    mockedOpen.mockResolvedValue(null);

    await expect(importMarkdownNote()).resolves.toBeNull();
    expect(invoke).not.toHaveBeenCalled();
  });

  test("exports a note to the selected text document path", async () => {
    mockedSave.mockResolvedValue("D:\\exports\\读书笔记.mint");
    mockedInvoke.mockResolvedValue(undefined);

    await expect(exportMarkdownNote({ id: "note-1", title: "读书笔记" })).resolves.toBe(true);

    expect(save).toHaveBeenCalledWith({
      title: "导出 Markdown",
      defaultPath: "读书笔记.md",
      filters: textDocumentFilters,
    });
    expect(invoke).toHaveBeenCalledWith("notes_export_markdown", {
      id: "note-1",
      path: "D:\\exports\\读书笔记.mint",
    });
  });

  test("uses a safe markdown file name for export", async () => {
    mockedSave.mockResolvedValue(null);

    await exportMarkdownNote({ id: "note-1", title: "A/B:Test" });
    await exportMarkdownNote({ id: "note-2", title: "" });
    await exportMarkdownNote({ id: "note-3", title: `${"x".repeat(79)}😀` });

    expect(save).toHaveBeenNthCalledWith(1, {
      title: "导出 Markdown",
      defaultPath: "A_B_Test.md",
      filters: textDocumentFilters,
    });
    expect(save).toHaveBeenNthCalledWith(2, {
      title: "导出 Markdown",
      defaultPath: "无标题笔记.md",
      filters: textDocumentFilters,
    });
    expect(save).toHaveBeenNthCalledWith(3, {
      title: "导出 Markdown",
      defaultPath: `${"x".repeat(79)}😀.md`,
      filters: textDocumentFilters,
    });
    expect(invoke).not.toHaveBeenCalled();
  });

  test("uses a mint file name for mint export", async () => {
    mockedSave.mockResolvedValue("D:\\exports\\读书笔记.mint");
    mockedInvoke.mockResolvedValue(undefined);

    await expect(exportMarkdownNote({ id: "note-1", title: "读书笔记" }, "mint")).resolves.toBe(
      true,
    );

    expect(save).toHaveBeenCalledWith({
      title: "导出 Mint",
      defaultPath: "读书笔记.mint",
      filters: textDocumentFilters,
    });
    expect(invoke).toHaveBeenCalledWith("notes_export_markdown", {
      id: "note-1",
      path: "D:\\exports\\读书笔记.mint",
    });
  });
});
