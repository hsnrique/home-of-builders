import { message, open } from "@tauri-apps/plugin-dialog";
import { revealItemInDir } from "@tauri-apps/plugin-opener";

import { h } from "../dom";
import { errorText, t } from "../i18n";
import type { Ctx } from "../state";

export function renderSettings(ctx: Ctx): HTMLElement {
  return h(
    "section",
    { class: "view settings" },
    h("header", { class: "view-head" }, h("h1", {}, t("settings.title"))),
    h("div", { class: "panel settings-list" }, libraryRow(ctx), claudeRow(ctx)),
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

function claudeRow(ctx: Ctx): HTMLElement {
  const path = ctx.state.claudePath;
  return h(
    "div",
    { class: "setting" },
    h("span", { class: "field-label" }, t("settings.claude")),
    path
      ? h("p", { class: "success" }, t("settings.claudeFound", { path }))
      : h("p", { class: "error" }, t("settings.claudeMissing")),
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
