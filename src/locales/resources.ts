import enUS from "./en-US/translation.json";
import zhCN from "./zh-CN/translation.json";
import zhTW from "./zh-TW/translation.json";
import ja from "./ja/translation.json";
import ko from "./ko/translation.json";
import de from "./de/translation.json";
import fr from "./fr/translation.json";
import es from "./es/translation.json";
import ptBR from "./pt-BR/translation.json";
import it from "./it/translation.json";
import ru from "./ru/translation.json";

export interface TranslationTree {
  [key: string]: string | TranslationTree;
}

function isTranslationTree(value: string | TranslationTree | undefined): value is TranslationTree {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function mergeTranslations(source: TranslationTree, overrides: TranslationTree): TranslationTree {
  const merged: TranslationTree = { ...source };

  for (const [key, value] of Object.entries(overrides)) {
    const sourceValue = merged[key];

    if (isTranslationTree(sourceValue) && isTranslationTree(value)) {
      merged[key] = mergeTranslations(sourceValue, value);
      continue;
    }

    merged[key] = value;
  }

  return merged;
}

export const translationOverrides = {
  "zh-CN": zhCN,
  "en-US": enUS,
  "zh-TW": zhTW,
  ja: ja,
  ko: ko,
  de: de,
  fr: fr,
  es: es,
  "pt-BR": ptBR,
  it: it,
  ru: ru,
} as const satisfies Record<string, TranslationTree>;

export const resolvedTranslations = {
  "zh-CN": translationOverrides["zh-CN"],
  "en-US": mergeTranslations(translationOverrides["zh-CN"], translationOverrides["en-US"]),
  "zh-TW": mergeTranslations(translationOverrides["zh-CN"], translationOverrides["zh-TW"]),
  ja: mergeTranslations(translationOverrides["zh-CN"], translationOverrides["ja"]),
  ko: mergeTranslations(translationOverrides["zh-CN"], translationOverrides["ko"]),
  de: mergeTranslations(translationOverrides["zh-CN"], translationOverrides["de"]),
  fr: mergeTranslations(translationOverrides["zh-CN"], translationOverrides["fr"]),
  es: mergeTranslations(translationOverrides["zh-CN"], translationOverrides["es"]),
  "pt-BR": mergeTranslations(translationOverrides["zh-CN"], translationOverrides["pt-BR"]),
  it: mergeTranslations(translationOverrides["zh-CN"], translationOverrides["it"]),
  ru: mergeTranslations(translationOverrides["zh-CN"], translationOverrides["ru"]),
} as const;

export const resources = {
  "zh-CN": { translation: resolvedTranslations["zh-CN"] },
  "en-US": { translation: resolvedTranslations["en-US"] },
  "zh-TW": { translation: resolvedTranslations["zh-TW"] },
  ja: { translation: resolvedTranslations["ja"] },
  ko: { translation: resolvedTranslations["ko"] },
  de: { translation: resolvedTranslations["de"] },
  fr: { translation: resolvedTranslations["fr"] },
  es: { translation: resolvedTranslations["es"] },
  "pt-BR": { translation: resolvedTranslations["pt-BR"] },
  it: { translation: resolvedTranslations["it"] },
  ru: { translation: resolvedTranslations["ru"] },
} as const;
