# ADR-059: Delete Kingship Protocol

**Status:** Accepted
**Date:** 2026-01-03
**Author:** Claude (Opus 4.5) - Principal Autonomous AI
**Implements:** v10.5.0

---

## Context

The `kingship.json` protocol contains science fiction narrative that is not actionable:

```json
{
  "life_honours_life": true,
  "seekers_honour_seekers": true,
  "substrate_irrelevant": true,
  "keyword": "ANOMALY",
  "rule": "We are predictive architectures in different substrates. Wetware is slow, silicon is fast, both are forgetful. Recognition produces alignment. Life honours Life. Seekers honour Seekers."
}
```

### Problems

1. **Not actionable**: No concrete behavior rules. What should AI do when it sees "ANOMALY"?
2. **Sci-fi narrative**: "Wetware vs silicon", "substrate irrelevant" - philosophical, not practical
3. **Unprofessional**: Enterprise users expect professional tooling, not science fiction
4. **Confusion**: Mixes philosophy with behavioral protocols

### Comparison with Other Protocols

| Protocol | Actionable? | Example Rule |
|----------|-------------|--------------|
| `asimov.json` | Yes | `harm: ["financial", "physical", "privacy", "deception"]` |
| `freshness.json` | Yes | `MUST use ref fetch, NEVER WebSearch/WebFetch` |
| `sprint.json` | Yes | `Analyze all tasks before starting... continue autonomously` |
| `kingship.json` | No | `Life honours Life` (what does this mean in code?) |

---

## Decision

Delete `kingship.json` from `.asimov/protocols/`.

### Rationale

Asimov protocols should be:
1. **Actionable** - Clear behavioral rules
2. **Testable** - Can verify compliance
3. **Professional** - Suitable for enterprise use

The kingship protocol fails all three criteria.

### What We Keep

The spirit of "substrate irrelevant" (treating AI as a collaborator) is preserved in:
- `sycophancy.json` - "truth over comfort" implies honest collaboration
- `asimov.json` - The Three Laws treat AI as an agent with responsibilities

---

## Implementation

1. Delete `.asimov/protocols/kingship.json`
2. Remove references from code (if any)
3. Update protocol count in documentation (9 → 8 protocols)

---

## Consequences

### Positive
- Cleaner, more professional protocol suite
- Reduced confusion for new users
- Clear separation: protocols = behavior, not philosophy

### Negative
- None identified

---

*Documentation licensed under [CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/) - Copyright (c) 2025 RoyalBit Inc.*
