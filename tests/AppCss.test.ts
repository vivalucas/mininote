import { readFileSync } from "node:fs";
import { describe, expect, test } from "vitest";

describe("app font stacks", () => {
  test("uses system font stacks without bundling custom app fonts", () => {
    const css = readFileSync(new URL("../src/App.css", import.meta.url), "utf8");

    expect(css).not.toContain("@font-face");
    expect(css).toMatch(/--font-body:[\s\S]*-apple-system[\s\S]*system-ui[\s\S]*sans-serif;/);
    expect(css).toMatch(/--font-mono:[\s\S]*"SF Mono"[\s\S]*monospace;/);
  });
});
