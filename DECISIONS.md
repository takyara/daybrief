# Decision Log — daybrief

Every non-trivial decision gets an entry: date, decision, reason, alternatives considered.
Never rewrite history: if a decision changes, add a new entry that supersedes the old one.

## Template

```
### D-XXX — Title (YYYY-MM-DD)
- **Decision:**
- **Reason:**
- **Alternatives considered:**
- **Supersedes / superseded by:**
```

---

### D-001 — Blocking HTTP with `ureq` (2026-10-19)
- **Decision:** use `ureq` for HTTP instead of `reqwest`.
- **Reason:** first Rust project; avoiding async and `tokio` keeps the focus on ownership, traits and error handling. daybrief makes a few sequential requests, so async brings little here.
- **Alternatives considered:** `reqwest` (standard, but requires async or its blocking feature on top of tokio).
- **Supersedes / superseded by:** async is introduced later in another project (sentinel).

### D-002 — Calendar through a shared ICS link, no Proton Bridge (2026-10-19)
- **Decision:** read the calendar from a local ICS file or a shared ICS link.
- **Reason:** Proton Bridge requires a paid plan and adds IMAP complexity the project does not need.
- **Alternatives considered:** Proton Bridge for mail features (revisit only if a mail-related feature appears).
- **Supersedes / superseded by:** —

### D-003 — Conventional Commits, one feature per commit (2026-10-19)
- **Decision:** Conventional Commits 1.0.0, small commits, one feature each.
- **Reason:** readable history, easy changelogs and reverts, good signal in a public repo.
- **Alternatives considered:** free-form messages.
- **Supersedes / superseded by:** —

### D-004 — Repository layout and branch name (2026-10-19)
- **Decision:** this project is its own git repository under `~/Documents/projects/`, default branch `main`, `Cargo.lock` committed.
- **Reason:** one repository per project; `daybrief` is an application, so a committed lockfile gives reproducible builds.
- **Alternatives considered:** keeping git's default branch name `master`.
- **Supersedes / superseded by:** —

### D-005 — clap with the derive API (2026-11-02)
- **Decision:** parse the command line with `clap` using its derive API (`#[derive(Parser)]`, `#[derive(Subcommand)]`).
- **Reason:** the CLI shape is a plain Rust type checked by the compiler; doc comments become help text; less code than the builder API. Adding a subcommand means adding an enum variant, and the exhaustive `match` forces handling it.
- **Alternatives considered:** clap builder API (more flexible at runtime, more verbose); `argh` or `lexopt` (lighter, but less documented and less common).
- **Supersedes / superseded by:** —

### D-006 — Dual license MIT OR Apache-2.0, public repository (2026-11-02)
- **Decision:** publish the repository publicly on GitHub under `MIT OR Apache-2.0`.
- **Reason:** portfolio project meant to be read; dual licensing is the Rust ecosystem convention (rustc, serde, clap): MIT for compatibility with GPLv2 projects, Apache-2.0 for its explicit patent grant. Users pick the one they need.
- **Alternatives considered:** MIT only (simpler, no patent clause); Apache-2.0 only (incompatible with GPLv2); private repository until v0.1.0.
- **Supersedes / superseded by:** —
