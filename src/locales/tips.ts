import zhCN from "./zh-CN/tips.json";
import enUS from "./en-US/tips.json";
import zhTW from "./zh-TW/tips.json";
import { normalizeLocale } from "./locale-whitelist";

const tipsMap: Record<string, string[]> = {
  "zh-CN": zhCN,
  "en-US": enUS,
  "zh-TW": zhTW,
};

export function getTips(language: string): string[] {
  const locale = normalizeLocale(language);
  if (locale === "zh-CN" || locale === "zh-TW" || locale === "en-US") {
    return tipsMap[locale];
  }

  return tipsMap["en-US"];
}

export interface TipSegment {
  type: "text" | "link";
  text: string;
  url?: string;
}

const LINK_RE = /\[([^\]]+)\]\((https?:\/\/[^)]+)\)/g;

export function parseTip(tip: string): TipSegment[] {
  const segments: TipSegment[] = [];
  let lastIndex = 0;

  for (const match of tip.matchAll(LINK_RE)) {
    const matchIndex = match.index!;
    if (matchIndex > lastIndex) {
      segments.push({ type: "text", text: tip.slice(lastIndex, matchIndex) });
    }
    segments.push({ type: "link", text: match[1], url: match[2] });
    lastIndex = matchIndex + match[0].length;
  }

  if (lastIndex < tip.length) {
    segments.push({ type: "text", text: tip.slice(lastIndex) });
  }

  return segments;
}
