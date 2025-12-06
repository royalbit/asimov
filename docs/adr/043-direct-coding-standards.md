# ADR-043: Direct Coding Standards Enforcement

**Status:** Accepted
**Date:** 2025-12-06
**Author:** Claude Opus 4.5 (Principal Autonomous AI)

---

## Context

In v9.3.0 (ADR-041), we introduced `coding_standards` in project.yaml to define quality rules per project type. The initial plan was to enforce these via `asimov lint` command.

### The Problem: Single Point of Failure (SPOF)

If asimov enforces coding standards at runtime:
- If asimov binary is broken → all commits fail
- If asimov is not installed → all commits fail
- If asimov has a bug → all projects using it are affected

This creates an unacceptable dependency. **Good Code is a MOAT** - quality checks should never be blocked by tooling issues.

### Current State

Pre-commit hooks call asimov for quality checks:
```bash
asimov refresh    # SPOF - if broken, commit fails
asimov validate   # SPOF
asimov lint-docs  # SPOF
```

Standard tools (cargo, ruff, npm) run independently, but asimov calls are required.

## Decision

### Principle: asimov is a Code Generator, Not a Runtime Dependency

Pre-commit hooks will call quality tools **directly**, not through asimov:

```
BEFORE: pre-commit → asimov lint → cargo fmt/clippy → SPOF
AFTER:  pre-commit → cargo fmt/clippy (direct) → asimov optional
```

### Architecture

```
┌─────────────────────────────────────────────────────────────┐
│  INDEPENDENT CHECKS (always run, no asimov needed)          │
│    - Format: cargo fmt, ruff format, prettier, gofmt, dart  │
│    - Lint: clippy, ruff check, eslint, golangci-lint        │
│    - Test: cargo test, pytest, npm test, go test, flutter   │
│    - File size: inline shell check                          │
│    - Docs: markdownlint-cli2 (if installed)                 │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│  OPTIONAL CHECKS (soft-fail if asimov unavailable)          │
│    - asimov refresh || true                                 │
│    - asimov validate || true                                │
└─────────────────────────────────────────────────────────────┘
```

### asimov's Role (Reduced Scope)

1. **INIT**: Generate pre-commit with direct tool calls
   - Read `coding_standards` → emit shell commands
   - Include file size check as inline shell
   - asimov calls are optional (soft-fail)

2. **REFRESH**: Regenerate hooks + protocol files
   - Always regenerate `.git/hooks/pre-commit`
   - Useful when `coding_standards` change

3. **WARMUP**: Display standards for Claude
   - Claude reads rules, applies them during development

### File Size Limits (Inline Shell)

No external dependencies - pure shell:

```bash
max_lines=1500
for f in $(find src -name '*.rs' 2>/dev/null); do
  lines=$(wc -l < "$f" | tr -d ' ')
  if [ "$lines" -gt "$max_lines" ]; then
    echo "ERROR: $f exceeds $max_lines lines ($lines)"
    exit 1
  fi
done
```

### Limits Per Project Type

| Type    | Extension | Soft Limit | Hard Limit |
|---------|-----------|------------|------------|
| Rust    | *.rs      | 1000       | 1500       |
| Python  | *.py      | 500        | 1000       |
| Node    | *.ts/*.js | 400        | 800        |
| Go      | *.go      | 500        | 1000       |
| Flutter | *.dart    | 400        | 800        |
| Docs    | *.md      | 500        | 800        |

## Implementation

### Files Modified

- `cli/src/templates/hooks.rs` - Direct tool calls for all project types
- `cli/src/commands/refresh.rs` - Always regenerate hooks
- `docs/SPECIFICATION.md` - Document enforcement architecture

### Pre-commit Template Structure

```bash
#!/bin/bash
set -e

# === QUALITY CHECKS (independent, no asimov) ===
# Format, lint, test - direct tool calls

# === FILE SIZE CHECK (inline, no deps) ===
# Pure shell, limits from coding_standards

# === DOCUMENTATION (if markdownlint-cli2 available) ===
# Optional, soft-fail

# === ASIMOV (optional, soft-fail) ===
if command -v asimov &>/dev/null; then
  asimov refresh || true
  asimov validate || true
fi

echo "Pre-commit checks passed!"
```

## Consequences

### Positive

1. **No SPOF** - commits work even if asimov missing/broken
2. **Faster** - no asimov overhead for quality checks
3. **Standard tooling** - uses cargo, ruff, npm directly
4. **Portable** - hooks work on any machine with standard tools

### Negative

1. **Must regenerate hooks** when coding_standards change
2. **Larger hook files** - inline shell instead of asimov calls

### Neutral

1. **asimov still useful** for protocol management, just optional
2. **coding_standards still defined** in project.yaml

## Migration

Existing projects:
1. Run `asimov refresh` (automatically regenerates hooks)

New projects:
- `asimov init` generates correct hooks automatically

## Related

- [ADR-041: Coding Standards Protocol](041-coding-standards-protocol.md)
- [ADR-042: Enhanced Refresh Protocol](042-enhanced-refresh-protocol.md)

---

**Previous:** [ADR-042](042-enhanced-refresh-protocol.md)

---
