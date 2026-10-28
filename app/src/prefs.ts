import type { Engine, Lang } from "./api";

const UI_LANG = "hob.uiLang";
const WORKSPACE = "hob.workspace";
const ENGINE = "hob.engine";

// Storage can throw (private mode, blocked site data); the app still works without it.
function read(key: string): string | null {
  try {
    return localStorage.getItem(key);
  } catch {
    return null;
  }
}

function write(key: string, value: string): void {
  try {
    localStorage.setItem(key, value);
  } catch {
    /* preference lives for this session only */
  }
}

export function loadUiLang(): Lang {
  const saved = read(UI_LANG);
  if (saved === "en" || saved === "pt") return saved;
  return navigator.language.toLowerCase().startsWith("pt") ? "pt" : "en";
}

export const saveUiLang = (lang: Lang) => write(UI_LANG, lang);
export const loadWorkspace = () => read(WORKSPACE);
export const saveWorkspace = (path: string) => write(WORKSPACE, path);

export function loadEngine(): Engine | null {
  const saved = read(ENGINE);
  return saved === "claude" || saved === "codex" ? saved : null;
}

export const saveEngine = (engine: Engine) => write(ENGINE, engine);
