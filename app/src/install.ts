import { ask, message } from "@tauri-apps/plugin-dialog";

import { api, ENGINE_NAMES, type AgentSummary, type Engine } from "./api";
import { h } from "./dom";
import { errorText, t } from "./i18n";

/** "Install in [Claude Code] [Codex]", for wherever an agent can be installed. */
export function installRow(workspace: string, agent: AgentSummary): HTMLElement {
  const button = (engine: Engine) =>
    h("button", { class: "btn", onclick: () => installAgent(workspace, agent, engine) }, ENGINE_NAMES[engine]);
  return h("div", { class: "actions install" }, h("span", { class: "muted small" }, t("install.label")), button("claude"), button("codex"));
}

/** Writes the agent where the engine reads user agents, asking before replacing an installed one. */
async function installAgent(workspace: string, agent: AgentSummary, engine: Engine): Promise<void> {
  const vars = { name: agent.name, engine: ENGINE_NAMES[engine] };
  const install = (overwrite: boolean) => api.installAgent(workspace, agent.path, engine, overwrite);
  const done = (path: string) => message(t("install.done", { ...vars, path }), { kind: "info" });
  try {
    await done(await install(false));
  } catch (error) {
    if (String(error) !== "error.installExists") return showError(error);
    if (!(await ask(t("install.confirmOverwrite", vars), { kind: "warning" }))) return;
    await install(true).then(done, showError);
  }
}

async function showError(error: unknown): Promise<void> {
  await message(errorText(error), { kind: "error" });
}
