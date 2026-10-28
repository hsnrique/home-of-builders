import pt from "./locales/pt.json";

type Lang = "en" | "pt";

// English lives in index.html; Portuguese comes from the locale file. Text only, never HTML.
const PT: Record<string, string> = pt;
const english = new Map<HTMLElement, string>();
const toggle = document.querySelector<HTMLButtonElement>("[data-lang-toggle]")!;

function apply(lang: Lang): void {
  document.documentElement.lang = lang === "pt" ? "pt-BR" : "en";
  for (const el of document.querySelectorAll<HTMLElement>("[data-i18n]")) {
    if (!english.has(el)) english.set(el, el.textContent ?? "");
    const key = el.dataset.i18n ?? "";
    el.textContent = lang === "pt" ? (PT[key] ?? english.get(el)!) : english.get(el)!;
  }
  toggle.textContent = lang === "pt" ? "EN" : "PT";
  toggle.setAttribute("aria-label", lang === "pt" ? "Switch to English" : "Mudar para português");
}

// Storage can throw (private mode, blocked site data); the toggle still works for the visit.
function savedLang(): Lang | null {
  try {
    const saved = localStorage.getItem("lang");
    return saved === "en" || saved === "pt" ? saved : null;
  } catch {
    return null;
  }
}

function saveLang(lang: Lang): void {
  try {
    localStorage.setItem("lang", lang);
  } catch {
    /* the choice lasts for this visit only */
  }
}

let current: Lang = savedLang() ?? (navigator.language.toLowerCase().startsWith("pt") ? "pt" : "en");
apply(current);

toggle.addEventListener("click", () => {
  current = current === "pt" ? "en" : "pt";
  apply(current);
  saveLang(current);
});
