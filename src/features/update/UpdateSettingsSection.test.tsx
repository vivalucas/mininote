import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, test, vi } from "vitest";
import { UpdateSettingsSection } from "./UpdateSettingsSection";
import type { UpdateState } from "./types";

vi.mock("@tauri-apps/plugin-opener", () => ({
  openUrl: vi.fn(),
}));

vi.mock("@tauri-apps/api/event", () => ({
  listen: vi.fn(),
}));

const status: UpdateState = {
  status: "available",
  currentVersion: "1.0.4",
  latestVersion: "1.0.5",
  channel: "stable",
  assetName: "mininote-1.0.5-macos-arm64.dmg",
  assetPath: null,
  assetSha256: "abc",
  assetSize: 12345678,
  source: "github",
  checkedAt: "2026-05-26T12:00:00Z",
  downloadedAt: null,
  installLogPath: null,
  installMode: null,
  installStartedAt: null,
  installScheduledAt: null,
  lastError: null,
};

describe("UpdateSettingsSection", () => {
  test("renders manual update controls", () => {
    const markup = renderToStaticMarkup(<UpdateSettingsSection initialStatus={status} />);

    expect(markup).toContain("更新");
    expect(markup).toContain("当前版本：1.0.4");
    expect(markup).toContain("检查更新");
    expect(markup).toContain("待更新版本：1.0.5");
    expect(markup).toContain("请前往 GitHub Releases 下载并安装最新版本");
    expect(markup).toContain("打开 Release 页面");
  });

  test("does not render automatic update or in-app install controls", () => {
    const downloadedStatus: UpdateState = {
      ...status,
      status: "downloaded",
      assetPath: "/tmp/mininote-1.0.5-macos-arm64.dmg",
      downloadedAt: "2026-05-26T12:05:00Z",
    };

    const markup = renderToStaticMarkup(<UpdateSettingsSection initialStatus={downloadedStatus} />);

    expect(markup).not.toContain("自动检查更新");
    expect(markup).not.toContain("有新版本时自动下载");
    expect(markup).not.toContain("下载更新");
    expect(markup).not.toContain("安装并重启");
    expect(markup).not.toContain("取消下载");
  });
});
