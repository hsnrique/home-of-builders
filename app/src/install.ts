import { ask, message } from "@tauri-apps/plugin-dialog";

import { api, type AgentSummary } from "./api";
import { errorText, t } from "./i18n";

/** Copies the agent to ~/.claude/agents, asking before replacing an installed one. */
export async function installAgent(workspace: string, agent: AgentSummary): Promise<void> {
  try {
    await confirmInstalled(await api.installAgent(workspace, agent.path, false));
  } catch (error) {
    if (String(error) !== "error.installExists") return showError(error);
    const replace = await ask(t("install.confirmOverwrite", { name: agent.name }), { kind: "warning" });
    if (!replace) return;
    await api.installAgent(workspace, agent.path, true).then(confirmInstalled, showError);
  }
}

async function confirmInstalled(path: string): Promise<void> {
  await message(t("install.done", { path }), { kind: "info" });
}

async function showError(error: unknown): Promise<void> {
  await message(errorText(error), { kind: "error" });
}
