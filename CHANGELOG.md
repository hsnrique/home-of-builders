# Changelog

All notable changes to this project are documented here. The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the project uses [Semantic Versioning](https://semver.org).

## [0.1.2] - 2026-10-28

### Fixed

- macOS no longer reports the app as damaged. The bundle is now signed ad hoc, so Gatekeeper offers **Open Anyway** instead.

## [0.1.1] - 2026-10-28

### Fixed

- Agent frontmatter is now valid YAML. Descriptions use a folded block, so the colons in them no longer break GitHub's preview or strict loaders. Generated and saved agents get the same format automatically.
- The website deploys from `site/` on Vercel.

## [0.1.0] - 2026-10-28

### Added

- 18 base marketing agents for Claude Code, written to a shared standard.
- 28 areas of interest (niches and channels) to specialize them.
- Desktop app for macOS, Windows and Linux: browse the library, generate specialized agents in English or Portuguese through your local Claude Code, review, save and install them in one click.
- Interface in English and Portuguese.
- Website at homeofbuilders.com.
