<p align="center">
  <img src="site/public/favicon.svg" width="72" height="72" alt="Home of Builders">
</p>

<h1 align="center">Home of Builders</h1>

<p align="center">
  Elite marketing agents for Claude Code, and a small desktop app that builds new ones for your niche.
</p>

<p align="center">
  <a href="https://github.com/hsnrique/home-of-builders/releases/latest">Download</a> ·
  <a href="https://homeofbuilders.com">Website</a> ·
  <a href="README.pt-BR.md">Português</a> ·
  <a href="https://buymeacoffee.com/hsnrique">Sponsor</a>
</p>

<p align="center">
  <a href="https://github.com/hsnrique/home-of-builders/actions/workflows/ci.yml"><img src="https://github.com/hsnrique/home-of-builders/actions/workflows/ci.yml/badge.svg" alt="CI"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-b4441c" alt="MIT license"></a>
</p>

---

Pick one of the 18 base agents, cross it with an area (28 niches and channels) and your specific needs, choose English or Portuguese, and the app asks your local Claude Code to rewrite the whole agent for that context. You review the draft, save it to your library, and install it in Claude Code with one click.

**Real example:** [`gestor-trafego-implante-odontologico.md`](agents/generated/pt/gestor-trafego-implante-odontologico.md) came from `traffic-manager` × *Clinics & healthcare* × "dental implant clinic in Curitiba, R$5k/month, leads go to WhatsApp", in Portuguese, in 74 seconds.

## The base agents

Every agent is a single Markdown file in [`agents/core`](agents/core), written to the same [standard](STANDARD.md): identity, mission, operating principles, intake questions, workflow, playbooks, deliverables, quality bar, guardrails and handoffs.

| Agent | What it does |
|---|---|
| [`traffic-manager`](agents/core/traffic-manager.md) | Media plans, campaign structure, creative tests and cost diagnosis on Meta, Google, TikTok and LinkedIn |
| [`landing-page-optimizer`](agents/core/landing-page-optimizer.md) | Conversion research, page structure, checkout trust and A/B tests sized to your traffic |
| [`launch-strategist`](agents/core/launch-strategist.md) | Launch calendars, offer stacks and lead goals worked back from the revenue target |
| [`copywriter`](agents/core/copywriter.md) | Direct-response copy in your own voice, built from a voice profile of your samples |
| [`headline-writer`](agents/core/headline-writer.md) | Headlines, hooks and subject lines in volume, scored and fitted to each channel |
| [`email-marketer`](agents/core/email-marketer.md) | Weekly newsletters, sequences, automations and deliverability |
| [`video-scriptwriter`](agents/core/video-scriptwriter.md) | Reels, TikTok, UGC ads, YouTube and VSL scripts with shot lists |
| [`social-media-manager`](agents/core/social-media-manager.md) | Content pillars, a monthly calendar, captions and community replies |
| [`linkedin-ghostwriter`](agents/core/linkedin-ghostwriter.md) | LinkedIn posts in your voice, ready to publish |
| [`x-thread-writer`](agents/core/x-thread-writer.md) | A topic turned into a thread for X |
| [`seo-strategist`](agents/core/seo-strategist.md) | Keyword research, content briefs, technical fixes and visibility in AI answers |
| [`competitor-analyst`](agents/core/competitor-analyst.md) | Ad libraries, offers, reviews and positioning gaps turned into a battlecard |
| [`market-researcher`](agents/core/market-researcher.md) | Sourced research with confidence levels, interview guides and market sizing |
| [`marketing-analyst`](agents/core/marketing-analyst.md) | Reads your exports and delivers the report in your format |
| [`brand-strategist`](agents/core/brand-strategist.md) | Positioning, messaging and a voice guide with real examples |
| [`sdr`](agents/core/sdr.md) | Qualifies leads, writes outbound sequences and books meetings |
| [`sales-assistant`](agents/core/sales-assistant.md) | Follows a lead to the close, answering from your own FAQ |
| [`negotiation-specialist`](agents/core/negotiation-specialist.md) | Negotiation one-sheet, calibrated questions and an offer plan with exact numbers |

The playbooks apply the frameworks of each field's reference books and name them inline: *Breakthrough Advertising*, *Influence*, *Tested Advertising Methods*, *Never Split the Difference*, *Getting to Yes*, *Positioning*, *Obviously Awesome*, *How Brands Grow*, *StoryBrand*, *The Mom Test*, *Contagious*, *Hook Point*, *Don't Make Me Think*, *Predictable Revenue*, *Lean Analytics*, *Everybody Writes* and more. The agents paraphrase and apply these ideas; they contain no excerpts.

## Install

1. Install [Claude Code](https://docs.claude.com/en/docs/claude-code/overview) and run `claude` once to log in.
2. Download the app for your system from [Releases](https://github.com/hsnrique/home-of-builders/releases/latest).
   - **macOS:** builds are not notarized yet. The first time, open the app, then go to **System Settings → Privacy & Security** and click **Open Anyway**.
   - **Windows:** if SmartScreen appears, choose **More info**, then **Run anyway**.
3. Open it. Your library is created at `Documents/Home of Builders` with all base agents inside.

Only want the agents? Copy any file from [`agents/`](agents) into `~/.claude/agents/` and Claude Code will use it.

## How generation works

- The app runs `claude -p --safe-mode --tools ""` on your machine. It uses your Claude Code login: no API key, no cost beyond your plan.
- `--tools ""` means the model can only answer with text; it cannot read or change your files.
- `--safe-mode` keeps your personal `CLAUDE.md`, hooks and plugins out of the generation, so every agent comes out the same way for everyone.
- Nothing else leaves your computer. The app has no telemetry and no account.

## Library layout

```
agents/core/              base agents, in English
agents/generated/en|pt/   agents you create, by language
catalog/areas.json        niches and channels you can cross with
STANDARD.md               the format every agent follows
app/                      desktop app (Tauri 2, TypeScript, Rust)
site/                     homeofbuilders.com (Vite)
```

Add an area by editing `catalog/areas.json`. Add a base agent by dropping a file in `agents/core/` that follows [STANDARD.md](STANDARD.md). The app picks both up on the next launch.

## Build from source

Requires Node 24, Rust stable and Claude Code.

```bash
cd app
npm ci
npm run tauri dev      # run in development
npm run tauri build    # produce an installer for your OS
```

Run the tests with `npm test`, `npm run build` and `cargo test` (inside `app/src-tauri`).

## Contributing

New agents, areas, translations and fixes are welcome. Start with [CONTRIBUTING.md](CONTRIBUTING.md).

## Sponsor

Home of Builders is free and stays free. If it saves you a hire or a weekend, [buy me a coffee](https://buymeacoffee.com/hsnrique): it pays for new agents and keeps the app current with Claude Code.

## Credits

Created and maintained by **Henrique Braga** ([@hsnrique](https://github.com/hsnrique), [hsnrique.com](https://hsnrique.com)).

Released under the [MIT License](LICENSE).
