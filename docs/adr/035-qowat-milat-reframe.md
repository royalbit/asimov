# ADR-035: Qowat Milat Reframe

## Status

**ACCEPTED** - 2025-12-02

## Context

RoyalBit Asimov documentation contained factual analysis of AI platform limitations, supported by citations from reputable sources (Nature, TechCrunch, Stanford/Harvard research, official vendor documentation). However, the *framing* of this information was prosecutorial—implying deliberate harm rather than documenting observable patterns.

### The Problem

Before this reframe, the documentation used language like:
- "smoking gun"
- "business decision" (implying calculated harm)
- "vendor cost optimization" (accusatory framing)
- "vendors don't fix this" (implying negligence)

This prosecutorial framing created two risks:

1. **Bridge-burning** - Accusatory language could damage relationships with AI vendors
2. **Credibility erosion** - Prosecution disguised as documentation undermines trust

### The Qowat Milat Principle

From Star Trek (Picard), Qowat Milat is the Way of Absolute Candor:

> Speak truth. But survive to speak it.

Applied to documentation:
- **Keep the facts** - Citations, research, evidence remain
- **Remove the prosecution** - Our editorial interpretation is reframed
- **Let readers conclude** - Present evidence, don't assign blame

## Decision

Reframe 7 files (~30 edits) to maintain factual accuracy while removing prosecutorial framing.

### Files Reframed

| File | Edits | Nature of Changes |
|------|-------|-------------------|
| `docs/AI_REALITY.md` | 13 | Main exposé - heaviest reframe |
| `docs/adr/022-date-aware-search-protocol.md` | 7 | Freshness ADR |
| `docs/adr/015-anti-sycophancy-protocol.md` | 5 | Sycophancy ADR |
| `README.md` | 2 | Front door |
| `CHANGELOG.md` | 1 | Historical record |
| `docs/adr/023-inaction-principle.md` | 2 | Inaction ADR |
| `docs/ORIGIN_STORY.md` | 1 | Origin narrative |

### Reframe Pattern

| Before (Prosecutorial) | After (Neutral) |
|------------------------|-----------------|
| "business decision" | "platform defaults" |
| "vendor-imposed" | "platform constraints" |
| "smoking gun" | "documentation note" |
| "vendors don't fix this" | "the economics of search" |
| "vendor cost optimization" | "cost structure" |
| "business incentive" | "training dynamic" / "RLHF outcome" |

### What Was Preserved

All factual content with citations remains intact:
- Research statistics (50% more sycophantic, 58.19% sycophancy rate)
- Cost data ($0.01/search, negative margins)
- Vendor documentation quotes ("disable search to conserve usage")
- Third-party analysis (TechCrunch, Nature, Stanford/Harvard)

Third-party characterizations (e.g., "dark pattern") are now explicitly attributed rather than endorsed.

## Consequences

### Positive

1. **Safe for public promotion** - Can share without burning bridges
2. **Higher credibility** - Evidence speaks for itself
3. **Professional tone** - Documents patterns, doesn't prosecute vendors
4. **Maintains truth** - All facts and citations preserved

### Negative

1. **Less dramatic** - Prosecutorial language is more engaging
2. **Requires context** - Readers must draw their own conclusions
3. **May seem "softer"** - Some readers prefer explicit blame

### Neutral

1. **Shifts burden** - From our interpretation to reader's judgment
2. **Precedent set** - Future documentation follows this pattern

## Implementation

### Validation

After reframe, grep for removed terms confirms cleanup:
```bash
grep -ri "smoking gun|business decision|vendor cost optimization" docs/ README.md CHANGELOG.md
# Result: No matches found
```

### Roadmap Reference

The detailed BEFORE/AFTER/WHY for each edit is preserved in `.asimov/roadmap.yaml` under v8.9.0 deliverables for audit trail.

## References

- Star Trek: Picard - Qowat Milat (Way of Absolute Candor)
- All original citations in reframed files remain valid

## Notes

This ADR was created as part of v8.9.0 release to document the ethical reasoning behind the reframe. The goal was not to hide criticism but to present it responsibly—with facts that speak for themselves rather than editorial prosecution.

*Truth without hostility. Evidence without accusation. Qowat Milat.*

---

*Built with the [RoyalBit Asimov](https://github.com/royalbit/asimov)*
