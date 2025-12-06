# ADR-041: Coding Standards Protocol

**Status:** Accepted
**Date:** 2025-12-06
**Author:** Claude Opus 4.5 (Principal Autonomous AI)

---

## Context

RoyalBit Asimov manages multiple project types (Rust, Python, Node, Go, Flutter, Docs, Architecture). Each has different coding standards, but they share common principles:

1. **Human-readable code** - Code is for humans first, machines second
2. **RFC 2119 interpretation** - "SHOULD" becomes "MUST" for RoyalBit projects
3. **Quality gates** - No warnings, proper testing, consistent formatting

### The Problem

Currently, coding standards exist only in:
- ADR-038 (100% test coverage - Rust-specific)
- ADR-040 (Code organization - Rust-specific)
- Scattered in project.yaml comments

There's no:
1. Generic protocol loaded at warmup for ALL projects
2. Project-type-specific coding standards in templates
3. Unified philosophy document

### Architecture Pattern (from v9.2.3)

The migrations protocol showed how to do conditional loading:
- Generic protocol in `protocols/` (compiled into binary)
- Project type read from `project.yaml` at warmup
- Conditional inclusion based on type

Coding standards should follow the same pattern but inverted:
- **Generic philosophy** → ALL projects (coding-standards.json)
- **Specific rules** → Per project type (in project.yaml templates)

## Decision

### 1. New Protocol: coding-standards.json

A generic protocol loaded for ALL projects containing:

```json
{
  "coding_standards": {
    "philosophy": "Human-readable, beautiful, well-formatted code",
    "junior_warning": "Without these rules, you are a JUNIOR programmer...",
    "rfc2119": {
      "MUST": "We follow (obviously)",
      "SHOULD": "We follow (best practice = we do it)",
      "MAY": "We don't care (zero bikeshedding)"
    },
    "principles": [
      "Code is for humans first, machines second",
      "Tests are documentation",
      "No warnings, no exceptions",
      "Done > Perfect, but not sloppy"
    ],
    "rule": "See project.yaml coding_standards section for project-specific rules"
  }
}
```

### 2. Project Templates: coding_standards Section

Each project template gets a `coding_standards:` section with type-specific rules:

**Rust:**
```yaml
coding_standards:
  file_size:
    soft_limit: 1000
    hard_limit: 1500
  coverage: 100%
  linting: "clippy pedantic, zero warnings"
  tests: "colocated with code (#[cfg(test)] mod tests)"
```

**Python:**
```yaml
coding_standards:
  file_size:
    soft_limit: 500
    hard_limit: 1000
  coverage: "90%+"
  linting: "ruff, mypy strict"
  tests: "pytest, colocated or tests/ directory"
```

**Docs/Arch:**
```yaml
coding_standards:
  file_size:
    soft_limit: 500
    note: "lines per markdown file"
  linting: "markdownlint"
  diagrams: "Mermaid (text-based, version controlled)"
```

### 3. New Project Type: Arch

Architecture projects (like mouvify/arch) are distinct from Docs:
- Have specific structure (c4-models/, decisions/, diagrams/)
- Different deliverables template
- Different detection markers

```rust
ProjectType::Arch  // Separate from Docs
```

Detection: Directory contains `c4-models/` OR `decisions/` OR (`diagrams/` AND `ARCHITECTURE*.md`)

### 4. Loading Strategy

```
┌─────────────────────────────────────────────────────────────────┐
│                     WARMUP PROTOCOL LOADING                      │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Always loaded (ALL projects):                                  │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐               │
│  │   asimov    │ │    green    │ │  sycophancy │               │
│  └─────────────┘ └─────────────┘ └─────────────┘               │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐               │
│  │   sprint    │ │   warmup    │ │  exhaustive │               │
│  └─────────────┘ └─────────────┘ └─────────────┘               │
│  ┌─────────────┐ ┌─────────────┐                               │
│  │  freshness  │ │   coding-   │  ← NEW                        │
│  │             │ │  standards  │                               │
│  └─────────────┘ └─────────────┘                               │
│                                                                 │
│  Conditionally loaded:                                          │
│  ┌─────────────┐                                               │
│  │ migrations  │ ← Only for type: migration                     │
│  └─────────────┘                                               │
│                                                                 │
│  Project-specific (from project.yaml):                          │
│  ┌──────────────────────────────────────────┐                  │
│  │ coding_standards:                         │                  │
│  │   file_size: { soft: 1000, hard: 1500 }  │  ← Per type      │
│  │   coverage: "100%"                        │                  │
│  │   linting: "clippy pedantic"              │                  │
│  └──────────────────────────────────────────┘                  │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## Implementation

### Files to Create/Modify

1. **Create:** `.asimov/coding-standards.json` - Template for projects
2. **Create:** `cli/src/protocols/coding_standards.rs` - Hardcoded protocol
3. **Modify:** `cli/src/templates/mod.rs` - Add `Arch` variant
4. **Modify:** `cli/src/templates/project-*.yaml.tpl` - Add coding_standards section
5. **Create:** `cli/src/templates/project-arch.yaml.tpl` - New template
6. **Modify:** `cli/src/protocols/mod.rs` - Include coding-standards in compilation

### Migration Path

Existing projects: `asimov refresh` will NOT overwrite project.yaml (user content). Users can manually add `coding_standards:` section or re-init.

New projects: `asimov init` generates project.yaml with coding_standards section.

## Rationale

### Why a Separate Protocol?

1. **Visibility** - Coding standards should be front-and-center, not buried in ADRs
2. **Consistency** - Same philosophy across all project types
3. **Enforcement** - Protocol is loaded every session, constant reminder

### Why Per-Type in project.yaml?

1. **Different languages, different rules** - Rust's 1500-line limit doesn't apply to Python
2. **User customizable** - Projects can adjust within reason
3. **Self-documenting** - Standards live with the project

### Why Separate Arch from Docs?

1. **Different purpose** - Architecture documents systems, Docs explains usage
2. **Different structure** - C4 models, decision records, diagrams
3. **Different quality gates** - Consistency checks, diagram validation

## Consequences

### Positive

1. **Unified standards** - All projects get the philosophy
2. **Type-specific rules** - Each project type gets appropriate limits
3. **Self-documenting** - Standards in project.yaml visible to team
4. **Enforced at warmup** - Can't forget the standards

### Negative

1. **More protocol content** - Slightly larger context injection
2. **Template changes** - All project templates need updating
3. **New type complexity** - Arch detection logic needed

## Related

- [ADR-038: 100% Test Coverage](038-100-percent-test-coverage.md)
- [ADR-040: Code Organization](040-code-organization.md)
- [ADR-031: Enforced Protocol Loading](031-enforced-protocol-loading.md)
- [RFC 2119: Key Words](https://www.rfc-editor.org/rfc/rfc2119)

---

**Previous:** [ADR-040](040-code-organization.md)

---
