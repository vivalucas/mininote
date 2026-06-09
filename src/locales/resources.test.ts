import { describe, expect, it } from "vitest";
import { DEFAULT_LOCALE, SUPPORTED_LOCALES } from "./locale-whitelist";
import { resolvedTranslations, translationOverrides, type TranslationTree } from "./resources";

function collectLeafKeys(tree: TranslationTree, prefix = ""): string[] {
  const keys: string[] = [];

  for (const [key, value] of Object.entries(tree)) {
    const nextPrefix = prefix ? `${prefix}.${key}` : key;

    if (typeof value === "string") {
      keys.push(nextPrefix);
      continue;
    }

    keys.push(...collectLeafKeys(value, nextPrefix));
  }

  return keys.sort();
}

describe("locale resources", () => {
  const sourceKeys = collectLeafKeys(translationOverrides[DEFAULT_LOCALE]);
  const sourceKeySet = new Set(sourceKeys);
  const requiredOverrideKeys = [
    "dialogs.import.title",
    "dialogs.exportMarkdown.title",
    "dialogs.exportMint.title",
    "dialogs.filters.mint",
    "dialogs.filters.markdown",
    "dialogs.filters.text",
    "main.editor.pinToTile",
    "main.editor.unpinTile",
    "main.sourceFile.label",
    "main.sourceConflict.title",
    "main.sourceConflict.keptLocal",
    "main.sourceSync.missing",
    "main.sourceSync.failed",
    "main.statusBar.columnNumber",
    "main.statusBar.totalLines",
    "notepad.confirmDiscard",
    "settings.autoSave.externalFile",
    "settings.notesDirSwitchConfirm",
    "settings.renderHtmlMarkdown",
    "settings.shortcut.check",
    "settings.shortcut.checkingShort",
  ];

  it("resolves every supported locale with complete source-locale coverage", () => {
    for (const locale of SUPPORTED_LOCALES) {
      expect(collectLeafKeys(resolvedTranslations[locale])).toEqual(sourceKeys);
    }
  });

  it("keeps non-source locale overrides within the source-locale key set", () => {
    for (const locale of SUPPORTED_LOCALES) {
      if (locale === DEFAULT_LOCALE) {
        continue;
      }

      for (const key of collectLeafKeys(translationOverrides[locale])) {
        expect(sourceKeySet.has(key)).toBe(true);
      }
    }
  });

  it("keeps user-visible recent keys explicitly translated outside the source locale", () => {
    for (const locale of SUPPORTED_LOCALES) {
      if (locale === DEFAULT_LOCALE) {
        continue;
      }

      const overrideKeySet = new Set(collectLeafKeys(translationOverrides[locale]));
      for (const key of requiredOverrideKeys) {
        expect(overrideKeySet.has(key)).toBe(true);
      }
    }
  });

  it("keeps static translation calls backed by the source locale", () => {
    const usedKeys = collectStaticTranslationKeys();
    for (const key of usedKeys) {
      expect(sourceKeySet.has(key)).toBe(true);
    }
  });
});

function collectStaticTranslationKeys(): string[] {
  const modules = import.meta.glob("../**/*.{ts,tsx}", {
    eager: true,
    import: "default",
    query: "?raw",
  }) as Record<string, string>;
  const keys = new Set<string>();
  const callPattern = /\b(?:t|translate)\(\s*(["'`])([^"'`]+)\1/g;

  for (const [file, source] of Object.entries(modules)) {
    if (file.includes("/locales/") || /\.test\.(ts|tsx)$/.test(file)) {
      continue;
    }

    for (const match of source.matchAll(callPattern)) {
      const key = match[2];
      if (!key.includes("${")) {
        keys.add(key);
      }
    }
  }

  return [...keys].sort();
}
