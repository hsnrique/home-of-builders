import { message, open } from "@tauri-apps/plugin-dialog";
import { revealItemInDir } from "@tauri-apps/plugin-opener";

import { ENGINE_NAMES, type Engine } from "../api";
import { h } from "../dom";
import { errorText, t } from "../i18n";
import * as prefs from "../prefs";
import type { Ctx } from "../state";

export function renderSettings(ctx: Ctx): HTMLElement {
  return h(
    "section",
    { class: "view settings" },
    h("header", { class: "view-head" }, h("h1", {}, t("settings.title"))),
    h("div", { class: "panel settings-list" }, libraryRow(ctx), engineRow(ctx)),
  );
}

function libraryRow(ctx: Ctx): HTMLElement {
  const { workspace, loadError, agents } = ctx.state;
  const status = loadError
    ? h("p", { class: "error" }, errorText(loadError))
    : h("p", { class: "muted small" }, t("library.count", { count: agents.length }));
  return h(
    "div",
    { class: "setting" },
    h("span", { class: "field-label" }, t("settings.workspace")),
    h("code", { class: "path-box mono" }, workspace ?? t("settings.workspaceNone")),
    status,
    h(
      "div",
      { class: "actions" },
      workspace ? h("button", { class: "btn", onclick: () => revealItemInDir(workspace) }, t("action.openFolder")) : null,
      h("button", { class: "btn", onclick: () => chooseLibrary(ctx) }, t("action.changeFolder")),
    ),
  );
}

function engineRow(ctx: Ctx): HTMLElement {
  const engine = ENGINE_NAMES[ctx.state.engine];
  const path = ctx.state.enginePaths[ctx.state.engine];
  const option = (choice: Engine) =>
    h(
      "button",
      {
        type: "button",
        class: "segment",
        "aria-pressed": String(ctx.state.engine === choice),
        onclick: () => {
          ctx.state.engine = choice;
          prefs.saveEngine(choice);
          ctx.render();
        },
      },
      ENGINE_NAMES[choice],
    );
  return h(
    "div",
    { class: "setting" },
    h("span", { class: "field-label" }, t("settings.engine")),
    h("div", { class: "segmented", role: "group" }, option("claude"), option("codex")),
    path
      ? h("p", { class: "success" }, t("settings.engineFound", { path }))
      : h("p", { class: "error" }, t("settings.engineMissing", { engine })),
  );
}

async function chooseLibrary(ctx: Ctx): Promise<void> {
  const picked = await open({ directory: true, multiple: false, title: t("settings.pickTitle") });
  if (typeof picked !== "string") return;
  try {
    await ctx.openLibrary(picked);
  } catch (error) {
    await message(errorText(error), { kind: "error" });
  }
  ctx.render();
}
