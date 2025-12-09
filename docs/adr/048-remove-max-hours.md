# ADR-048: Remove max_hours from Sprint Protocol

## Status

**ACCEPTED** - 2025-12-09

## Context

The sprint protocol had a `max_hours: 4` constraint designed to prevent runaway sessions. In practice:

1. **Sessions rarely reach 4 hours** - Most milestones complete in minutes to an hour
2. **Artificial ceiling creates false urgency** - The limit was never actually hit
3. **Natural stopping points exist** - Roadmap exhaustion, blocking issues, or user intervention

### ADR-028 Already Removed Milestone Limits

ADR-028 removed the "1 milestone per session" constraint, allowing continuous shipping. The max_hours constraint is now the last artificial limit.

## Decision

### Remove max_hours Entirely

**OLD:**
```yaml
rules:
  max_hours: 4
  must_ship: true
```

**NEW:**
```yaml
rules:
  must_ship: true
```

### Updated Sprint Rule

**OLD:**
```
Run autonomously until the job is done or max_hours reached.
```

**NEW:**
```
Analyze all tasks before starting. Use agents for parallel analysis if needed.
Plan and run autonomously until the job is done, using roadmap.yaml to track progress.
If analysis indicates you can work in parallel without conflicts, use agents to prevent
running out of context window tokens. Do not stop to ask the user - if blocked or uncertain,
WebSearch for the best solution, document decisions in an ADR if needed, and continue autonomously.
```

### Natural Stop Conditions

The sprint now stops when:

| Condition | Description |
|-----------|-------------|
| Roadmap exhausted | All deliverables completed |
| Blocked | External dependency or decision needed |
| Human says stop | User intervention (veto words) |
| Context compaction | Session naturally ends, WIP continuity resumes |

## Consequences

### Positive

1. **True autonomy** - No artificial time ceiling
2. **Simpler protocol** - One less field to track
3. **Natural flow** - Work until done, not until a timer expires
4. **Better documentation** - Decisions go to ADRs, not ASIMOV_MODE_ISSUES.md

### Negative

1. **Theoretically unbounded** - Could run very long
   - Mitigated by: Context compaction naturally limits sessions
   - Mitigated by: Roadmap provides natural stopping points

## Implementation

Files updated:
- `.asimov/sprint.json` - Removed max_hours field
- `cli/src/protocols/mod.rs` - Removed max_hours from SprintProtocol struct
- `cli/src/protocols/sprint.tpl` - Removed max_hours reference
- `cli/src/schemas/sprint.rs` - Made max_hours optional (deprecated)
- `cli/src/templates/protocols.rs` - Updated templates
- `cli/src/validator.rs` - Updated tests

## References

- [ADR-028: Velocity Reality - Continuous Shipping](028-velocity-reality-continuous-shipping.md)
- [ADR-047: WIP Continuity Protocol](047-wip-continuity-protocol.md)

---
