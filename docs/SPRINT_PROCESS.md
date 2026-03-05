# Sprint Process

This document defines the required sprint workflow for 900Invoice.

The goal is simple: every sprint must leave a high-quality paper trail that helps contributors anywhere in the world understand what changed, why it changed, and how to verify it.

---

## Core Principles

1. **Document-first delivery**: Every sprint includes implementation updates and documentation updates.
2. **Traceable decisions**: The PR body must explain rationale, not just code diffs.
3. **Reviewable increments**: Keep sprint PRs focused and scoped.
4. **Clean history**: Use squash merge so `main` remains readable.

---

## Sprint Lifecycle

1. Create a branch from `main`.
2. Define sprint scope and acceptance criteria.
3. Implement code and documentation updates together.
4. Run validation commands relevant to the change.
   - Use `docs/QUALITY_GATE.md` as the default validation baseline.
5. Open a Pull Request with a complete title and body.
6. Address review feedback.
7. **Squash and merge**.
8. Share the PR URL after merge and delete the merged branch.

---

## Required Sprint Artifacts

Each sprint must include the following:

1. A sprint document in `docs/sprints/`:
   - Suggested path: `docs/sprints/sprint-XXX-<slug>.md`
   - Include scope, goals, deliverables, validation, and follow-ups.
2. A complete PR description:
   - Summary
   - Problem statement
   - Scope (in/out)
   - Validation evidence
   - Risks and mitigations
   - Documentation impact
3. Updated documentation for any behavior, API, schema, workflow, or contributor expectation that changed.

---

## Parallel Agent Safety

When using multiple automation agents in one sprint:

1. Assign each agent to an isolated workspace.
2. Prevent overlapping file ownership across agents.
3. Keep a single integration checkout for final `git` operations.
4. Merge reviewed changes into the integration checkout sequentially.

This prevents `git` metadata corruption and keeps change history auditable.

---

## PR Title Convention

Use a clear, descriptive title:

```text
<type>(<scope>): <what changed and why>
```

Examples:
- `feat(invoices): add partial-payment allocation logic for mixed-currency invoices`
- `fix(taxes): prevent negative withholding totals in inclusive mode`
- `docs(process): add sprint lifecycle and squash-merge policy`

Avoid vague titles such as "updates", "fix stuff", or "changes".

---

## Merge Policy

- Merge method: **Squash and merge only**
- Merge target: `main`
- Post-merge handoff:
  - Share merged PR URL
  - Confirm merge commit SHA
  - Delete merged branch from remote

---

## Definition of Done

A sprint is complete only when all are true:

1. Acceptance criteria are met.
2. Relevant tests/checks pass (or skipped checks are explicitly justified in the PR).
   - For PRs targeting `main`, CI quality-gate checks in `.github/workflows/ci.yml` must pass.
3. Documentation has been updated.
4. PR is reviewed and merged via squash.
5. Merge URL is shared with stakeholders.
