# daybrief — Roadmap

Created: 2026-10-19
Estimate: ~5 weeks at 10–12 h/week. A guess: re-estimate after step 6.
Author context: first Rust project, learning as it goes (Rust Book + Rustlings in parallel).

## Goal

One command prints your day: calendar events, GitHub items assigned to you, local tasks.

## Scope

**MVP:** `daybrief today` shows events + assigned GitHub issues/PRs + tasks. `--json` flag.

**Non-goals (for now):** plugin system, TUI, notifications, Proton Bridge/IMAP.

## Stack

- `clap` (CLI), `serde` + `toml` (config), `directories` (XDG paths).
- `ureq` (blocking HTTP) instead of `reqwest`: avoids async/`tokio` while learning ownership and traits.
- `ical` or `icalendar` for ICS parsing, `rrule` for recurring events (check current maintenance before choosing).
- `thiserror` for typed errors in the library part, `anyhow` in `main`.
- Tests: `assert_cmd` + `predicates` for the CLI, a local mock HTTP server (`httpmock` or `mockito`) for the GitHub client.

## Steps

Each step is one or more small commits (Conventional Commits, one feature per commit).

- [x] 1. `chore: init cargo project` · `ci: add fmt, clippy and test workflow` · `docs: add README skeleton`
- [x] 2. `feat(cli): add clap skeleton with today subcommand` (prints a placeholder)
- [ ] 3. `feat(config): load TOML config from XDG config dir`. Clear error if missing. Check config file permissions (0600): it will hold secrets.
- [ ] 4. `feat(calendar): parse a local ICS file and list today's events`
- [ ] 5. `feat(calendar): expand recurring events`. Expected to be the hardest step (RRULE, time zones, all-day events). If it drags on: ship without it, document the limitation, open an issue.
- [ ] 6. `feat(calendar): fetch ICS from a URL`. The shared calendar link gives read access to anyone who has it: treat it as a secret (config or env var, never in the repo, never in logs).
- [ ] 7. `feat(github): list assigned issues and PRs` via REST (`GET /issues?filter=assigned`, includes PRs). Token from `GITHUB_TOKEN`, minimal scope. Handle pagination and rate-limit headers.
- [ ] 8. `feat(tasks): add local task store with add/done/list`
- [ ] 9. `refactor: introduce Source trait` (`fn fetch(&self) -> Result<Vec<Item>>`), move calendar/GitHub/tasks behind it. Log the reason in `DECISIONS.md`.
- [ ] 10. `feat(output): colored terminal rendering` and `feat(output): add --json flag`
- [ ] 11. `feat(cache): add TTL cache for remote sources`
- [ ] 12. `test: add fixtures and integration tests` (sample ICS with recurrence and time zones, recorded GitHub JSON)
- [ ] 13. `docs: architecture diagram and usage` · `ci: build release binaries` · tag `v0.1.0`

## Definition of done

- [ ] MVP scope met, non-goals explicitly left out
- [ ] Tests pass in CI; critical paths covered
- [ ] CI green: format, lint, tests, dependency audit
- [ ] README: what / why / install / usage / architecture diagram / design decisions / limitations
- [ ] `DECISIONS.md` up to date
- [ ] Tagged release with installable artifact
- [ ] Project page text ready for the personal site (problem, solution, stack, lessons learned)

**Done when:** it is run every morning for a week without touching the code.

## Rules

- `main` is always working. Work on `feat/...` branches, merge through a PR.
- Commits follow [Conventional Commits 1.0.0](https://www.conventionalcommits.org/en/v1.0.0/), one feature each.
- No secrets in the repo. Synthetic fixtures only, never real calendar or personal data.
- CI: `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, `cargo audit`.

## Extensions (after v0.1.0)

Weather/RSS/notes sources; external-executable plugins (a source = a program printing JSON on stdout); TUI with `ratatui`.
