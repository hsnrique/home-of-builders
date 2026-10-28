import { revealItemInDir } from "@tauri-apps/plugin-opener";

import { api } from "../api";
import { h } from "../dom";
import { errorText, t } from "../i18n";
import { installAgent } from "../install";
import { skeleton } from "../skeleton";
import type { Ctx } from "../state";

/** Right-hand panel of the create view: hint → progress → editable draft → saved. */
export function renderOutput(ctx: Ctx): HTMLElement {
  switch (ctx.state.create.status) {
    case "generating":
      return progress(ctx.state.create.startedAt);
    case "review":
      return review(ctx);
    case "saved":
      return saved(ctx);
    default:
      return h("div", { class: "panel output empty muted" }, t("create.idle"));
  }
}

function progress(startedAt: number): HTMLElement {
  const elapsed = h("span", { class: "mono muted" }, secondsSince(startedAt));
  const timer = window.setInterval(() => {
    if (!elapsed.isConnected) return window.clearInterval(timer);
    elapsed.textContent = secondsSince(startedAt);
  }, 1000);
  return h(
    "div",
    { class: "panel output", role: "status" },
    h("div", { class: "output-head" }, h("span", {}, t("create.generating")), elapsed),
    h("p", { class: "muted small" }, t("create.hint")),
    skeleton(),
  );
}

const secondsSince = (start: number) => `${Math.max(0, Math.round((Date.now() - start) / 1000))}s`;

function review(ctx: Ctx): HTMLElement {
  const create = ctx.state.create;
  const editor = h("textarea", {
    class: "markdown editor",
    spellcheck: "false",
    "aria-label": t("create.review"),
    value: create.draft,
    oninput: (event) => (create.draft = (event.target as HTMLTextAreaElement).value),
  });
  const discard = () => {
    Object.assign(create, { status: "idle", draft: "", error: null });
    ctx.render();
  };
  return h(
    "div",
    { class: "panel output" },
    h("div", { class: "output-head" }, h("span", {}, t("create.review")), h("span", { class: "tag" }, create.draftLang.toUpperCase())),
    editor,
    create.error ? h("p", { class: "error", role: "alert" }, errorText(create.error)) : null,
    h(
      "div",
      { class: "actions end" },
      h("button", { class: "btn quiet", onclick: discard }, t("action.discard")),
      h("button", { class: "btn primary", onclick: () => save(ctx) }, t("action.save")),
    ),
  );
}

async function save(ctx: Ctx): Promise<void> {
  const create = ctx.state.create;
  try {
    create.saved = await api.saveAgent(ctx.state.workspace!, create.draftLang, create.draft);
    Object.assign(create, { status: "saved", error: null });
    await ctx.reload();
  } catch (error) {
    create.error = String(error);
  }
  ctx.render();
}

function saved(ctx: Ctx): HTMLElement {
  const agent = ctx.state.create.saved!;
  const workspace = ctx.state.workspace!;
  const open = () => {
    ctx.go("library");
    void ctx.select(agent.path);
  };
  const another = () => {
    Object.assign(ctx.state.create, { status: "idle", draft: "", saved: null });
    ctx.render();
  };
  return h(
    "div",
    { class: "panel output empty" },
    h("p", { class: "success" }, t("create.saved", { path: agent.path })),
    h(
      "div",
      { class: "actions" },
      h("button", { class: "btn quiet", onclick: () => revealItemInDir(`${workspace}/${agent.path}`) }, t("action.showFile")),
      h("button", { class: "btn", onclick: open }, t("action.openInLibrary")),
      h("button", { class: "btn", onclick: another }, t("action.createAnother")),
      h("button", { class: "btn primary", onclick: () => installAgent(workspace, agent) }, t("action.install")),
    ),
  );
}
