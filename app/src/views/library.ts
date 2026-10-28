import { revealItemInDir } from "@tauri-apps/plugin-opener";

import type { AgentSummary } from "../api";
import { h } from "../dom";
import { groupAgents, groupLabel } from "../groups";
import { errorText, t } from "../i18n";
import { installRow } from "../install";
import { skeleton } from "../skeleton";
import type { Ctx } from "../state";

export function renderLibrary(ctx: Ctx): HTMLElement {
  if (!ctx.state.workspace || ctx.state.loadError) return workspaceMissing(ctx);
  const list = h("div", { class: "agent-list", role: "list" });
  fillList(ctx, list);
  return h(
    "section",
    { class: "view library" },
    h(
      "header",
      { class: "view-head" },
      h("h1", {}, t("library.title"), h("span", { class: "count" }, t("library.count", { count: ctx.state.agents.length }))),
      searchBox(ctx, list),
    ),
    h("div", { class: "split" }, list, agentDetail(ctx)),
  );
}

export function workspaceMissing(ctx: Ctx): HTMLElement {
  const text = ctx.state.loadError ? errorText(ctx.state.loadError) : t("library.noWorkspace");
  return h(
    "section",
    { class: "view empty-state" },
    h("p", {}, text),
    h("button", { class: "btn primary", onclick: () => ctx.go("settings") }, t("nav.settings")),
  );
}

// Typing filters the list in place so the search box keeps focus.
function searchBox(ctx: Ctx, list: HTMLElement): HTMLElement {
  return h("input", {
    class: "search",
    type: "search",
    placeholder: t("library.search"),
    "aria-label": t("library.search"),
    value: ctx.state.query,
    oninput: (event) => {
      ctx.state.query = (event.target as HTMLInputElement).value;
      fillList(ctx, list);
    },
  });
}

function fillList(ctx: Ctx, list: HTMLElement): void {
  const query = ctx.state.query.trim().toLowerCase();
  const visible = ctx.state.agents.filter((a) => `${a.name} ${a.description}`.toLowerCase().includes(query));
  const groups = [...groupAgents(visible)].map(([group, agents]) =>
    h(
      "div",
      { class: "group" },
      h("h2", { class: "group-title" }, groupLabel(group), h("span", {}, String(agents.length))),
      ...agents.map((agent) => agentRow(ctx, agent)),
    ),
  );
  list.replaceChildren(...(groups.length ? groups : [h("p", { class: "muted" }, t("library.empty"))]));
}

function agentRow(ctx: Ctx, agent: AgentSummary): HTMLElement {
  const selected = agent.path === ctx.state.selectedPath;
  return h(
    "button",
    { class: "agent-row", role: "listitem", "aria-current": selected ? "true" : null, onclick: () => ctx.select(agent.path) },
    h("span", { class: "agent-name mono" }, agent.name),
    h("span", { class: "agent-desc" }, agent.description),
  );
}

function agentDetail(ctx: Ctx): HTMLElement {
  const { state } = ctx;
  const agent = state.agents.find((a) => a.path === state.selectedPath);
  if (!agent) return h("div", { class: "detail empty muted" }, t("library.pick"));
  const content = state.selectedContent === null ? skeleton() : h("pre", { class: "markdown" }, state.selectedContent);
  return h(
    "article",
    { class: "detail" },
    h(
      "header",
      { class: "detail-head" },
      h("h2", { class: "mono" }, agent.name),
      h("div", { class: "actions" }, ...detailActions(ctx, agent)),
    ),
    h("p", { class: "description" }, agent.description),
    installRow(ctx.state.workspace!, agent),
    h("div", { class: "document" }, h("p", { class: "path mono" }, agent.path), content),
  );
}

function detailActions(ctx: Ctx, agent: AgentSummary): HTMLElement[] {
  const workspace = ctx.state.workspace!;
  const useAsBase = () => {
    ctx.state.create.basePath = agent.path;
    ctx.go("create");
  };
  return [
    h("button", { class: "btn quiet", onclick: () => revealItemInDir(`${workspace}/${agent.path}`) }, t("action.showFile")),
    h("button", { class: "btn primary", onclick: useAsBase }, t("action.useAsBase")),
  ];
}
