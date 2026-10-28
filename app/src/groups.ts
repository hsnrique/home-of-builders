import type { AgentSummary } from "./api";
import { t } from "./i18n";

const LABELS = {
  core: "group.core",
  "generated/pt": "group.generatedPt",
  "generated/en": "group.generatedEn",
} as const;

export function groupLabel(group: string): string {
  return group in LABELS ? t(LABELS[group as keyof typeof LABELS]) : group;
}

/** Agents arrive sorted by group, so grouping keeps that order. */
export function groupAgents(agents: AgentSummary[]): Map<string, AgentSummary[]> {
  const groups = new Map<string, AgentSummary[]>();
  for (const agent of agents) {
    groups.set(agent.group, [...(groups.get(agent.group) ?? []), agent]);
  }
  return groups;
}
