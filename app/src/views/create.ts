import { api, MAX_NEEDS_CHARS, type Lang } from "../api";
import { h } from "../dom";
import { groupAgents, groupLabel } from "../groups";
import { errorText, locale, t } from "../i18n";
import { DEFAULT_BASE, type Ctx } from "../state";
import { renderOutput } from "./create-output";
import { workspaceMissing } from "./library";

export function renderCreate(ctx: Ctx): HTMLElement {
  if (!ctx.state.workspace || ctx.state.loadError) return workspaceMissing(ctx);
  ctx.state.create.basePath = resolveBase(ctx);
  return h(
    "section",
    { class: "view create" },
    h("header", { class: "view-head" }, h("h1", {}, t("create.title"))),
    h("div", { class: "create-grid" }, form(ctx), renderOutput(ctx)),
  );
}

/** Keeps the chosen base if it still exists, else falls back to the elite traffic manager. */
function resolveBase(ctx: Ctx): string | null {
  const paths = ctx.state.agents.map((a) => a.path);
  const chosen = ctx.state.create.basePath;
  if (chosen && paths.includes(chosen)) return chosen;
  return paths.includes(DEFAULT_BASE) ? DEFAULT_BASE : (paths[0] ?? null);
}

function form(ctx: Ctx): HTMLElement {
  const { create } = ctx.state;
  const submit = (event: Event) => {
    event.preventDefault();
    void generate(ctx);
  };
  return h(
    "form",
    { class: "panel form", onsubmit: submit },
    field(t("create.base"), baseSelect(ctx)),
    field(t("create.area"), areaSelect(ctx)),
    field(t("create.needs"), needsInput(ctx)),
    field(t("create.language"), languageSwitch(ctx)),
    h("div", { class: "form-foot" }, recipe(ctx), submitButton(create.status === "generating")),
    create.error && create.status === "idle" ? h("p", { class: "error", role: "alert" }, errorText(create.error)) : null,
  );
}

function field(label: string, control: HTMLElement): HTMLElement {
  return h("label", { class: "field" }, h("span", { class: "field-label" }, label), control);
}

function submitButton(busy: boolean): HTMLElement {
  return h("button", { class: "btn primary wide", type: "submit", disabled: busy }, t("action.generate"));
}

/** States the cross being made, so the user sees what the button will produce. */
function recipe(ctx: Ctx): HTMLElement {
  const { create, agents, areas } = ctx.state;
  const base = agents.find((a) => a.path === create.basePath)?.name ?? "";
  const area = areas.find((a) => a.id === create.areaId)?.label[locale()] ?? t("create.yourNeeds");
  const lang = t(create.lang === "en" ? "lang.en" : "lang.pt");
  return h("p", { class: "recipe" }, h("span", { class: "mono" }, base), " × ", area, " → ", lang);
}

function rerenderOnChange(apply: (value: string) => void, ctx: Ctx) {
  return (event: Event) => {
    apply((event.target as HTMLSelectElement).value);
    ctx.render();
  };
}

function baseSelect(ctx: Ctx): HTMLElement {
  const groups = [...groupAgents(ctx.state.agents)].map(([group, agents]) =>
    h("optgroup", { label: groupLabel(group) }, ...agents.map((a) => h("option", { value: a.path }, a.name))),
  );
  const onchange = rerenderOnChange((value) => (ctx.state.create.basePath = value), ctx);
  return h("select", { value: ctx.state.create.basePath ?? "", onchange }, ...groups);
}

function areaSelect(ctx: Ctx): HTMLElement {
  const lang = locale();
  const options = (kind: "niche" | "channel") =>
    ctx.state.areas.filter((a) => a.kind === kind).map((a) => h("option", { value: a.id }, a.label[lang]));
  const onchange = rerenderOnChange((value) => (ctx.state.create.areaId = value), ctx);
  return h(
    "select",
    { value: ctx.state.create.areaId, onchange },
    h("option", { value: "" }, t("create.areaNone")),
    h("optgroup", { label: t("create.niches") }, ...options("niche")),
    h("optgroup", { label: t("create.channels") }, ...options("channel")),
  );
}

function needsInput(ctx: Ctx): HTMLElement {
  return h("textarea", {
    rows: 5,
    maxlength: MAX_NEEDS_CHARS,
    placeholder: t("create.needsPlaceholder"),
    value: ctx.state.create.needs,
    oninput: (event) => (ctx.state.create.needs = (event.target as HTMLTextAreaElement).value),
  });
}

function languageSwitch(ctx: Ctx): HTMLElement {
  const option = (lang: Lang) =>
    h(
      "button",
      {
        type: "button",
        class: "segment",
        "aria-pressed": String(ctx.state.create.lang === lang),
        onclick: () => {
          ctx.state.create.lang = lang;
          ctx.render();
        },
      },
      t(lang === "en" ? "lang.en" : "lang.pt"),
    );
  return h("div", { class: "segmented", role: "group" }, option("en"), option("pt"));
}

async function generate(ctx: Ctx): Promise<void> {
  const create = ctx.state.create;
  if (!create.areaId && !create.needs.trim()) {
    Object.assign(create, { status: "idle", error: "error.specializationRequired" });
    return ctx.render();
  }
  Object.assign(create, { status: "generating", error: null, saved: null, startedAt: Date.now() });
  ctx.render();
  try {
    const draft = await api.generateAgent({
      workspace: ctx.state.workspace!,
      basePath: create.basePath ?? DEFAULT_BASE,
      areaId: create.areaId || null,
      needs: create.needs,
      lang: create.lang,
      engine: ctx.state.engine,
    });
    Object.assign(create, { status: "review", draft, draftLang: create.lang });
  } catch (error) {
    Object.assign(create, { status: "idle", error: String(error) });
  }
  ctx.render();
}
