import { invoke } from "@tauri-apps/api/core";

export type Lang = "en" | "pt";

export interface AgentSummary {
  path: string;
  name: string;
  description: string;
  group: string;
}

export interface Area {
  id: string;
  kind: "niche" | "channel";
  label: Record<Lang, string>;
  hint: string;
}

export interface GenerateRequest {
  workspace: string;
  basePath: string;
  areaId: string | null;
  needs: string;
  lang: Lang;
}

export const MAX_NEEDS_CHARS = 4000;

export const api = {
  /** Prepares the library (the saved folder, or the default one when null) and returns its path. */
  openLibrary: (path: string | null) => invoke<string>("open_library", { path }),
  claudeStatus: () => invoke<string | null>("claude_status"),
  listAgents: (workspace: string) => invoke<AgentSummary[]>("list_agents", { workspace }),
  readAgent: (workspace: string, path: string) => invoke<string>("read_agent", { workspace, path }),
  listAreas: (workspace: string) => invoke<Area[]>("list_areas", { workspace }),
  generateAgent: (request: GenerateRequest) => invoke<string>("generate_agent", { request }),
  saveAgent: (workspace: string, lang: Lang, markdown: string) =>
    invoke<AgentSummary>("save_agent", { workspace, lang, markdown }),
  installAgent: (workspace: string, path: string, overwrite: boolean) =>
    invoke<string>("install_agent", { workspace, path, overwrite }),
};
