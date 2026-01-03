# ADR-065: Documentation Audit for Professional Language

**Status:** Accepted
**Date:** 2026-01-03
**Author:** Claude (Opus 4.5) - Principal Autonomous AI
**Implements:** v11.0.0

---

## Context

The RoyalBit Asimov documentation uses Isaac Asimov's Three Laws of Robotics as inspiration for its ethics protocol. This reference is intentional and provides a clear, well-understood framework for AI ethics.

However, some documentation contained language that was inconsistent with a professional technical product:

1. **"Context is King"** - Contained a "kingship" reference inconsistent with the project's values
2. **Historical sci-fi narrative** - The `kingship.json` protocol (removed in ADR-059) contained phrases like "Life honours Life" and "Wetware vs silicon"

### Audit Scope

Files audited:
- `README.md`
- All files in `docs/*.md` (excluding ADRs in `docs/adr/`)

### What to Keep

Per project guidelines:
- **"Asimov" name** throughout the project
- **References to "Three Laws"** as inspiration for the ethics protocol
- **Technical accuracy** in all claims and descriptions
- **Professional tone** appropriate for enterprise tooling

### What to Remove or Rephrase

- Sci-fi narrative ("Life honours Life", dramatic language)
- Overly dramatic descriptions
- Non-actionable philosophical statements
- Any remaining "kingship" references

---

## Decision

Audit all documentation files and make targeted edits to professionalize language while preserving the Asimov ethics framework.

### Changes Made

| File | Change | Rationale |
|------|--------|-----------|
| `docs/MANIFESTO.md` | "Context is King" -> "Context Matters" | Remove kingship reference |

### Files Reviewed (No Changes Needed)

The following files were reviewed and found to already use professional language:

- `README.md` - Professional, data-driven claims with citations
- `docs/SPECIFICATION.md` - Technical specification, professional tone
- `docs/GLOSSARY.md` - Concise definitions
- `docs/VALUE_PROPOSITION.md` - Professional marketing language with verifiable claims
- `docs/AI_REALITY.md` - Technical analysis with research citations
- `docs/SETUP.md` - Installation guide, professional
- `docs/FORMULA_DERIVATIONS.md` - Mathematical derivations, professional
- `docs/USE_CASES.md` - Use case documentation, professional
- `docs/ECOSYSTEM.md` - Ecosystem description, professional
- `docs/ECOSYSTEM_PATTERN.md` - Pattern documentation, professional
- `docs/GREEN_CODING.md` - Economics analysis, professional
- `docs/TECHNICAL_DECK.md` - Presentation slides, professional
- `docs/EXECUTIVE_DECK.md` - Presentation slides, professional
- `docs/ROYALBIT_ASIMOV.md` - Product overview, professional
- `docs/EXAMPLES.md` - Configuration examples, professional
- `docs/ASIMOV_VS_COPILOT.md` - Competitive analysis with citations
- `docs/VENDOR_IMPLEMENTATION.md` - Technical compatibility, professional
- `docs/PRESS_KIT.md` - Press materials, professional
- `docs/MARKDOWN_STANDARDS.md` - Standards documentation, professional
- `docs/RUST_GUIDE.md` - Development guide, professional
- `docs/ORIGIN_STORY.md` - Historical context, appropriately narrative
- `docs/PROTOCOL_GOALS.md` - Goals documentation, professional

### Preserved Content

The following content was intentionally preserved:

1. **Asimov name and branding** - Central to project identity
2. **Three Laws references** - Foundation of ethics protocol, derived from Isaac Asimov's 1942 work
3. **Isaac Asimov quotations** - Properly attributed literary references
4. **Technical terminology** - "Self-Evolving Autonomous AI", "Dynamic Swarm", etc.
5. **Data-driven claims** - Velocity multipliers, test counts, all with verification links

---

## Implementation

1. Single edit to `docs/MANIFESTO.md` line 346
2. No other changes required - documentation is already professional

---

## Consequences

### Positive

- Consistent professional language across all documentation
- No remaining "kingship" references in user-facing documentation
- Asimov ethics framework preserved with appropriate technical framing

### Negative

- None identified

---

## Related ADRs

- [ADR-059: Delete Kingship Protocol](059-delete-kingship-protocol.md) - Removed sci-fi narrative protocol
- [ADR-064: Tone Down Sprint Protocol](064-tone-down-sprint.md) - Professionalized sprint.json language
- [ADR-058: Documentation Standards](058-documentation-standards.md) - Established documentation quality standards

---

*Documentation licensed under [CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/) - Copyright (c) 2025 RoyalBit Inc.*
