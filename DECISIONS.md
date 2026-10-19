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
