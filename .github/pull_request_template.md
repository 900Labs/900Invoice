## PR Title

Use this format:

`<type>(<scope>): <clear change summary>`

Example:
`feat(invoices): add recurring draft generation with missed-run recovery`

---

## Summary

Describe what changed in 3-6 bullets.

-
-
-

## Problem Statement

What user, product, or engineering problem does this PR solve?

## Scope

### In Scope

-
-

### Out of Scope

-
-

## Implementation Notes

Explain the key design decisions and tradeoffs in this PR.

## Validation

List the exact commands run and their results.

```bash
# Example
# cargo test --manifest-path src-tauri/Cargo.toml
# npm run check
```

## Risks and Mitigations

- **Risk**:
- **Mitigation**:

## Documentation Impact

List every documentation file updated and why.

-
-

## Sprint Record

Link the sprint doc updated/created by this PR:

`docs/sprints/sprint-XXX-<slug>.md`

## Checklist

- [ ] PR title is descriptive and follows `<type>(<scope>): <summary>`
- [ ] PR body is complete (no placeholder sections left blank)
- [ ] Scope is clear and bounded
- [ ] Relevant tests/checks were run or explicitly justified if skipped
- [ ] Documentation updated for behavior/API/process changes
- [ ] No floating-point money logic introduced (amounts remain integer minor units)
- [ ] Tax rates remain basis points (`i32`) with integer arithmetic
- [ ] User-facing strings are localized (`i18n`) when applicable
- [ ] I am requesting **squash merge** for this PR
