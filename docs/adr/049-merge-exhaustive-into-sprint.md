# ADR-049: Merge Exhaustive Protocol into Sprint

## Status

**ACCEPTED** - 2025-12-10

## Context

### The Problem

The exhaustive protocol (ADR-036) and sprint protocol had overlapping purposes:

| Protocol | Core Message | Purpose |
|----------|--------------|---------|
| **sprint** | "Run autonomously until done" | HOW to work: planning, agents, roadmap.yaml, WebSearch when blocked |
| **exhaustive** | "DON'T STOP!" | Compaction survival reminder |

Both protocols ultimately said "keep working until complete" - redundant messaging that consumed extra tokens.

### Analysis

The exhaustive protocol was added in v8.10.0 to address task breadth control and compaction survival. Its key feature was the phrase:

```
REMEMBER THIS AFTER COMPACT, THIS IS IMPORTANT: IF YOU'RE RUNNING IN AUTONOMOUS MODE, OR ASIMOV MODE, CONTINUE THE WORK UNTIL IT'S ALL COMPLETED, DON'T STOP!
```

This is designed to survive context window summarization by:
1. Using ALL CAPS for emphasis
2. Including "REMEMBER THIS AFTER COMPACT" as a self-referential anchor
3. Being explicitly about continuation behavior

However, with WIP Continuity (ADR-047), we now have a more reliable mechanism:
- Pre-commit hook fires on every commit
- Hook reads fresh state from roadmap.yaml
- Outputs concrete WIP reminder with task name and progress

The exhaustive protocol's compaction survival hack is now belt-and-suspenders redundancy, not the primary mechanism.

## Decision

### Merge exhaustive into sprint as `compaction_reminder`

**Before (9 protocol files):**
- sprint.json: `{ "rule": "..." }`
- exhaustive.json: `{ "rule": "REMEMBER THIS AFTER COMPACT..." }`

**After (8 protocol files):**
- sprint.json: `{ "rule": "...", "compaction_reminder": "REMEMBER THIS AFTER COMPACT..." }`

The compaction reminder content is preserved exactly - only the container changes.

### Files Changed

1. `cli/src/protocols/sprint.tpl` - Added compaction_reminder field
2. `cli/src/protocols/mod.rs`:
   - Removed `EXHAUSTIVE_PROTOCOL` const
   - Removed `ExhaustiveProtocol` struct
   - Removed `get_exhaustive_protocol()` function
   - Removed `exhaustive_json()` function
   - Updated `SprintProtocol` struct with `compaction_reminder` field
   - Updated `CompiledProtocols` to remove `exhaustive` field
   - Updated `PROTOCOL_FILES` constant (9 → 8 entries)
   - Updated `warmup_entry_json()` load list
   - Updated tests
3. Deleted `cli/src/protocols/exhaustive.tpl`
4. Deleted `.asimov/exhaustive.json`

## Consequences

### Positive

1. **Reduced redundancy** - One less protocol saying "don't stop"
2. **Fewer tokens** - 8 protocol files instead of 9
3. **Clearer architecture** - Sprint is the single source of truth for autonomous behavior
4. **Preserved functionality** - Compaction reminder still exists in sprint

### Negative

1. **ADR-036 is now historical** - The exhaustive protocol ADR documents a removed feature
2. **Documentation updates needed** - Docs referencing "8 protocols" or "exhaustive" need updating

### Migration

Existing projects using `asimov update` or `asimov init` will automatically get the new protocol structure. The exhaustive.json file will no longer be generated.

## References

- [ADR-036: Exhaustive Execution Protocol](036-exhaustive-execution-protocol.md) - Original exhaustive protocol (now merged)
- [ADR-047: WIP Continuity Protocol](047-wip-continuity-protocol.md) - Primary compaction survival mechanism
- [ADR-048: Remove max_hours](048-remove-max-hours.md) - Related sprint simplification
