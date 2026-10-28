# Contributing

Thanks for helping. The most valuable contributions are better agents: sharper playbooks, a missing role, a new area.

## Set up

Requires Node 24, Rust stable and [Claude Code](https://docs.claude.com/en/docs/claude-code/overview) or [Codex](https://github.com/openai/codex) logged in.

```bash
cd app
npm ci
npm run tauri dev
```

Before opening a pull request:

```bash
cd app
npm run build                       # type-check and bundle the UI
npm test                            # i18n and UI checks
cd src-tauri && cargo test          # backend tests
```

`cargo test -- --ignored generates_a_real_agent` runs one real generation with each engine, through your own logins. Use it when you change the prompt.

## Website

The landing page in `site/` is a small Vite project. On Vercel, set the project's **Root Directory** to `site`; the build settings and security headers come from `site/vercel.json`.

```bash
cd site
npm ci
npm run dev       # http://127.0.0.1:4173
npm test          # every translatable string has a Portuguese version
```

English copy lives in `site/index.html`; Portuguese lives in `site/src/locales/pt.json` under the same `data-i18n` key.

## Add or improve a base agent

1. Read [STANDARD.md](STANDARD.md). Every agent keeps the same ten sections in the same order.
2. Write in English. Use a kebab-case `name` that matches the file name.
3. Make it specific: numbers, thresholds, decision rules, templates. Ground the playbooks in the reference books of the field and name them; paraphrase, never paste excerpts.
4. No invented statistics, testimonials or case studies.
5. Only reference agents that exist in `agents/core/` under **Handoffs**.

## Add an area

Add an entry to [`catalog/areas.json`](catalog/areas.json):

```json
{ "id": "pet-shops", "kind": "niche", "label": { "en": "Pet shops", "pt": "Pet shops" }, "hint": "What changes for agents in this area: buying cycle, channels, regulation." }
```

`kind` is `niche` or `channel`. The `hint` goes straight into the generation prompt, so make it concrete. `cargo test` validates the file.

## App changes

- Keep UI text in `app/src/locales/en.json` and `pt.json`; `npm test` fails if a key is missing in either.
- Render file contents and model output as text, never as HTML.
- Pin new dependencies to exact versions.

## Commits

Use [Conventional Commits](https://www.conventionalcommits.org): `feat:`, `fix:`, `docs:`, `agents:`, `chore:`. One change per commit.

## Releasing (maintainers)

1. Bump the version in `app/package.json`, `app/src-tauri/Cargo.toml` and `app/src-tauri/tauri.conf.json`.
2. Add the changes to [CHANGELOG.md](CHANGELOG.md).
3. Tag and push: `git tag -a v0.2.0 -m "Home of Builders 0.2.0" && git push origin v0.2.0`.
4. On a Mac with a Developer ID certificate, check out the tag and run `scripts/release-macos.sh` with `APPLE_SIGNING_IDENTITY` and `NOTARY_PROFILE` set. It builds a universal DMG, signs and notarizes it, and uploads it to a draft release. Review it and publish.

Release downloads are macOS only. Windows and Linux users build from source.
