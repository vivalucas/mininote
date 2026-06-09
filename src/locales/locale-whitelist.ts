export const SUPPORTED_LOCALES = [
  "zh-CN",
  "en-US",
  "zh-TW",
  "ja",
  "ko",
  "de",
  "fr",
  "es",
  "pt-BR",
  "it",
  "ru",
] as const;

export type SupportedLocale = (typeof SUPPORTED_LOCALES)[number];

export interface LocaleOption {
  value: SupportedLocale;
  labelKey: string;
  defaultLabel: string;
}

export const LOCALE_OPTIONS = [
  { value: "zh-CN", labelKey: "settings.locale.zhCN", defaultLabel: "简体中文" },
  { value: "en-US", labelKey: "settings.locale.enUS", defaultLabel: "English" },
  { value: "zh-TW", labelKey: "settings.locale.zhTW", defaultLabel: "繁體中文" },
  { value: "ja", labelKey: "settings.locale.ja", defaultLabel: "日本語" },
  { value: "ko", labelKey: "settings.locale.ko", defaultLabel: "한국어" },
  { value: "de", labelKey: "settings.locale.de", defaultLabel: "Deutsch" },
  { value: "fr", labelKey: "settings.locale.fr", defaultLabel: "Français" },
  { value: "es", labelKey: "settings.locale.es", defaultLabel: "Español" },
  { value: "pt-BR", labelKey: "settings.locale.ptBR", defaultLabel: "Português" },
  { value: "it", labelKey: "settings.locale.it", defaultLabel: "Italiano" },
  { value: "ru", labelKey: "settings.locale.ru", defaultLabel: "Русский" },
] as const satisfies readonly LocaleOption[];

export const DEFAULT_LOCALE: SupportedLocale = "zh-CN";

const SUPPORTED_LOCALE_SET = new Set<string>(SUPPORTED_LOCALES);

const LOCALE_ALIASES: Record<string, SupportedLocale> = {
  en: "en-US",
  "en-us": "en-US",
  zh: "zh-CN",
  "zh-cn": "zh-CN",
  "zh-hans": "zh-CN",
  "zh-sg": "zh-CN",
  "zh-hant": "zh-TW",
  "zh-hk": "zh-TW",
  "zh-mo": "zh-TW",
  "zh-tw": "zh-TW",
  ja: "ja",
  "ja-jp": "ja",
  ko: "ko",
  "ko-kr": "ko",
  de: "de",
  "de-de": "de",
  fr: "fr",
  "fr-fr": "fr",
  es: "es",
  "es-es": "es",
  pt: "pt-BR",
  "pt-br": "pt-BR",
  "pt-bz": "pt-BR",
  it: "it",
  "it-it": "it",
  ru: "ru",
  "ru-ru": "ru",
};

function canonicalizeLocale(locale: string): string {
  try {
    return Intl.getCanonicalLocales(locale)[0] ?? locale;
  } catch {
    return locale;
  }
}

export function normalizeLocale(locale?: string | null): SupportedLocale | null {
  if (!locale) {
    return null;
  }

  const trimmed = locale.trim();
  if (!trimmed) {
    return null;
  }

  const canonical = canonicalizeLocale(trimmed);
  if (SUPPORTED_LOCALE_SET.has(canonical)) {
    return canonical as SupportedLocale;
  }

  const segments = canonical.toLowerCase().split("-");
  for (let length = segments.length; length > 0; length -= 1) {
    const candidate = segments.slice(0, length).join("-");
    const normalized = LOCALE_ALIASES[candidate];
    if (normalized) {
      return normalized;
    }
  }

  return null;
}

export function resolveAppLocale(
  preferredLocale?: string | null,
  navigatorLocale?: string | null,
): SupportedLocale {
  return normalizeLocale(preferredLocale) ?? normalizeLocale(navigatorLocale) ?? DEFAULT_LOCALE;
}
