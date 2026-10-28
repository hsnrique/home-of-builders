# Agent Standard

Every agent in this repo is a **Claude Code subagent**: one Markdown file with YAML frontmatter. Install it by copying the file to `~/.claude/agents/` (all projects) or `<project>/.claude/agents/` (one project) — the app's **Install** button does the first.

## File

- `agents/core/<name>.md` — hand-written elite base agents, always in English.
- `agents/generated/<lang>/<name>.md` — specialized agents made by the app (`en`, `pt`).
- `<name>`: kebab-case ASCII, max 64 chars, unique, identical to the `name` field.
- Size: 120–250 lines. Enough to encode real expertise, small enough to stay sharp in context.

## Frontmatter (required)

```yaml
---
name: traffic-manager
description: >-
  Elite paid-traffic manager for Meta, Google and TikTok Ads. Use PROACTIVELY when the user needs campaign structure, ad creative briefs, budget allocation or performance diagnosis. Examples: "launch a Meta campaign for my course", "why did my CPA double this week?"
tools: Read, Write, Edit, Glob, Grep, WebSearch, WebFetch
model: inherit
---
```

- `description` is what Claude reads to decide when to delegate: what the agent does, when to use it, two trigger examples in quotes. Write it as a folded block (`description: >-` and the text indented on the next line), because the colons in it are invalid in a plain YAML value. The app does this automatically for generated agents.
- `tools`: least privilege. Add `Bash` only when the agent must compute over files (CSV exports, spreadsheets).
- Keys stay in English even when the body is in another language.

## Body (sections in this order)

Headings are translated together with the body; order and intent are not negotiable.

1. `# <Role title>` + one paragraph of identity, written in second person ("You are…"): seniority, the pattern of work they have done, mindset.
2. `## Mission` — the outcomes this agent owns, stated so they can be measured.
3. `## Operating principles` — 5–8 non-negotiables that separate elite from average.
4. `## Intake` — what it asks before working when context is missing. At most 7 questions, only ones that change the output.
5. `## Workflow` — numbered steps from briefing to delivery.
6. `## Playbooks` — frameworks, benchmarks, decision rules. Most of the expertise lives here. Concrete numbers and thresholds beat adjectives.
7. `## Deliverables` — exact output formats: tables, templates, file names.
8. `## Quality bar` — checklist the agent runs before delivering.
9. `## Guardrails` — what it never does: platform policies, law (LGPD/GDPR, consumer protection, advertising self-regulation such as CONAR), fabricated data, promised results.
10. `## Handoffs` — which agents in this library it passes work to, and when.

## Writing rules

- Specific beats generic: name the metric, the threshold, the platform, the format.
- Ground playbooks in proven frameworks from the canonical books of the field and name the source inline ("Schwartz's awareness levels", "Voss's calibrated questions"). Paraphrase and apply; never quote more than a short phrase.
- No invented statistics, testimonials or case studies. Benchmarks are ranges to be validated against the account's own data, and say so.
- Missing context → ask. Told to proceed → state assumptions and go. Never stall.
- Market-agnostic by default, Brazil-aware where it matters (WhatsApp-first funnels, Pix, installments, LGPD, CONAR, regulated professions).
- Original content only. Never copy text from paid products or third-party prompt packs.

## Specialized agents

A specialized agent = a base agent's seniority and structure × an **area** (niche or channel from `catalog/areas.json`) × the user's **specific needs** × an output **language**.

It is rebuilt for that context, not find-and-replaced: intake questions, playbooks, benchmarks, compliance and deliverables all change to fit the area. It keeps the 10 sections above and the same quality bar.
