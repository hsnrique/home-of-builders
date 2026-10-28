import type { Lang } from "./api";
import en from "./locales/en.json";
import pt from "./locales/pt.json";

export type MessageKey = keyof typeof en;

// Typing pt as the full key set makes a missing translation a compile error.
const catalogs: Record<Lang, Record<MessageKey, string>> = { en, pt };
let current: Lang = "en";

export function setLocale(lang: Lang): void {
  current = lang;
  document.documentElement.lang = lang === "pt" ? "pt-BR" : "en";
}

export const locale = (): Lang => current;

export function t(key: MessageKey, vars: Record<string, string | number> = {}): string {
  return catalogs[current][key].replace(/\{(\w+)\}/g, (match, name: string) =>
    name in vars ? String(vars[name]) : match,
  );
}

/** Backend errors are either i18n keys ("error.*") or raw messages shown as they are. */
export function errorText(error: unknown): string {
  const message = String(error);
  return message in en ? t(message as MessageKey) : message;
}
