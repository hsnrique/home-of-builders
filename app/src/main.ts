import { api, type Lang } from "./api";
import { h } from "./dom";
import { locale, setLocale, t, type MessageKey } from "./i18n";
import * as prefs from "./prefs";
import { initialState, type Ctx, type View } from "./state";
import { renderCreate } from "./views/create";
import { renderLibrary } from "./views/library";
import { renderSettings } from "./views/settings";

const VIEWS: Record<View, { label: MessageKey; render: (ctx: Ctx) => HTMLElement }> = {
  library: { label: "nav.library", render: renderLibrary },
  create: { label: "nav.create", render: renderCreate },
  settings: { label: "nav.settings", render: renderSettings },
};

const ctx: Ctx = {
  state: initialState(prefs.loadUiLang()),
  render,
  reload,
  go(view) {
    ctx.state.view = view;
    render();
  },
  async select(path) {
    Object.assign(ctx.state, { selectedPath: path, selectedContent: null });
    render();
    try {
      ctx.state.selectedContent = await api.readAgent(ctx.state.workspace!, path);
    } catch (error) {
      ctx.state.selectedContent = String(error);
    }
    render();
  },
  async openLibrary(path) {
    const workspace = await api.openLibrary(path);
    prefs.saveWorkspace(workspace);
    Object.assign(ctx.state, { workspace, selectedPath: null, selectedContent: null });
    await reload();
    const first = ctx.state.agents[0];
    if (first) void ctx.select(first.path);
  },
};

async function reload(): Promise<void> {
  const { workspace } = ctx.state;
  if (!workspace) return;
  try {
    const [agents, areas] = await Promise.all([api.listAgents(workspace), api.listAreas(workspace)]);
    Object.assign(ctx.state, { agents, areas, loadError: null });
  } catch (error) {
    Object.assign(ctx.state, { agents: [], areas: [], loadError: String(error) });
  }
}

function render(): void {
  const main = h("main", { class: "content" }, VIEWS[ctx.state.view].render(ctx));
  document.getElementById("app")!.replaceChildren(sidebar(), main);
}

function sidebar(): HTMLElement {
  const nav = (Object.keys(VIEWS) as View[]).map((view) =>
    h(
      "button",
      { class: "nav-item", "aria-current": ctx.state.view === view ? "page" : null, onclick: () => ctx.go(view) },
      t(VIEWS[view].label),
    ),
  );
  return h(
    "aside",
    { class: "sidebar" },
    h("div", { class: "brand" }, "Home of Builders"),
    h("nav", {}, ...nav),
    h("div", { class: "sidebar-foot" }, h("span", { class: "field-label" }, t("ui.language")), uiLanguageSwitch()),
  );
}

function uiLanguageSwitch(): HTMLElement {
  const option = (lang: Lang) =>
    h(
      "button",
      { class: "segment", "aria-pressed": String(locale() === lang), onclick: () => switchUiLanguage(lang) },
      lang.toUpperCase(),
    );
  return h("div", { class: "segmented small", role: "group", "aria-label": t("ui.language") }, option("en"), option("pt"));
}

function switchUiLanguage(lang: Lang): void {
  setLocale(lang);
  prefs.saveUiLang(lang);
  render();
}

async function boot(): Promise<void> {
  setLocale(ctx.state.create.lang);
  ctx.state.claudePath = await api.claudeStatus();
  try {
    await ctx.openLibrary(prefs.loadWorkspace());
  } catch (error) {
    ctx.state.loadError = String(error);
  }
  render();
}

void boot();
