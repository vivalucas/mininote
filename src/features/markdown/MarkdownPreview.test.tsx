import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, test, vi } from "vitest";
import { MarkdownPreview } from "./MarkdownPreview";

vi.mock("@tauri-apps/plugin-opener", () => ({
  openUrl: vi.fn(),
}));

describe("MarkdownPreview", () => {
  test("marks rendered Markdown content as selectable", () => {
    const markup = renderToStaticMarkup(<MarkdownPreview content="# MiniNote\n\n正文" />);

    expect(markup).toContain("markdown-selectable");
    expect(markup).toContain("<h1");
    expect(markup).toContain("MiniNote");
    expect(markup).toContain("正文");
  });

  test("sanitizes raw HTML when safe HTML rendering is enabled", () => {
    const markup = renderToStaticMarkup(
      <MarkdownPreview
        renderHtml
        content={`<mark style="position:fixed" onclick="alert(1)">safe</mark>
<script>alert(1)</script>
<img src="x" onerror="alert(1)" />
<a href="javascript:alert(1)">bad link</a>`}
      />,
    );

    expect(markup).toContain("<mark>safe</mark>");
    expect(markup).not.toContain("<script");
    expect(markup).not.toContain("onclick");
    expect(markup).not.toContain("onerror");
    expect(markup).not.toContain("javascript:");
    expect(markup).not.toContain("position:fixed");
  });
});
