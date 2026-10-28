import type { AgentSummary, Area, Lang } from "./api";

export type View = "library" | "create" | "settings";
export type CreateStatus = "idle" | "generating" | "review" | "saved";

export interface CreateState {
  basePath: string | null;
  /** "" means no area. */
  areaId: string;
  needs: string;
  lang: Lang;
  status: CreateStatus;
  startedAt: number;
  draft: string;
  /** Language the draft was generated in; the form may change while it is reviewed. */
  draftLang: Lang;
  saved: AgentSummary | null;
  error: string | null;
}

export interface AppState {
  view: View;
  workspace: string | null;
  claudePath: string | null;
  agents: AgentSummary[];
  areas: Area[];
  loadError: string | null;
  query: string;
  selectedPath: string | null;
  selectedContent: string | null;
  create: CreateState;
}

export interface Ctx {
  state: AppState;
  render(): void;
  reload(): Promise<void>;
  go(view: View): void;
  select(path: string): Promise<void>;
  /** Prepares the folder as the library and switches to it; throws an error key if it cannot. */
  openLibrary(path: string | null): Promise<void>;
}

export const DEFAULT_BASE = "agents/core/traffic-manager.md";

export function initialState(uiLang: Lang): AppState {
  return {
    view: "library",
    workspace: null,
    claudePath: null,
    agents: [],
    areas: [],
    loadError: null,
    query: "",
    selectedPath: null,
    selectedContent: null,
    create: {
      basePath: null,
      areaId: "",
      needs: "",
      lang: uiLang,
      status: "idle",
      startedAt: 0,
      draft: "",
      draftLang: uiLang,
      saved: null,
      error: null,
    },
  };
}
