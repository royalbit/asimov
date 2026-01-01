# ADR-057: External Project Templates with Embedded Fallback

**Status:** Proposed
**Date:** 2026-01-01
**Author:** Claude (Opus 4.5) - Principal Autonomous AI
**Extends:** ADR-053 (External Protocol and Template Architecture)
**Supersedes:** Hardcoded templates in `cli/src/templates/*.rs`

---

## Context

### Current State

Project templates are currently implemented as **hardcoded Rust code** that generates YAML strings:

```rust
// cli/src/templates/project.rs
pub fn project_template(project_type: ProjectType) -> String {
    match project_type {
        ProjectType::Rust => include_str!("project-rust.yaml.tpl").to_string(),
        ProjectType::Python => include_str!("project-python.yaml.tpl").to_string(),
        // ... mixed approach: some .tpl files, some inline code
    }
}
```

**Problems:**
1. **Inconsistent** - Mix of `.tpl` files and inline Rust code
2. **Hard to edit** - Template changes require understanding Rust
3. **Not packageable** - `.tpl` files use relative paths outside crate (same issue we fixed for protocols in v10.2.4)
4. **Duplication** - Enterprise templates in `.asimov/templates/` duplicate embedded templates
5. **No single source of truth** - Templates scattered across multiple locations

### The Protocol Pattern (v10.2.4)

We successfully moved protocols to:
```
cli/protocols/*.json          ← Canonical source (baked in via include_str!)
.asimov/protocols/*.json      ← Runtime overrides (user customization)
```

This same pattern should apply to project templates.

## Decision

### 1. Template File Structure

```
cli/
├── protocols/                 # Already done (v10.2.4)
│   ├── asimov.json
│   └── ...
├── templates/                 # NEW: Canonical project templates
│   ├── project/
│   │   ├── rust.yaml
│   │   ├── python.yaml
│   │   ├── node.yaml
│   │   ├── go.yaml
│   │   ├── flutter.yaml
│   │   ├── docs.yaml
│   │   ├── arch.yaml
│   │   ├── generic.yaml
│   │   └── migration.yaml
│   ├── warmup/
│   │   ├── rust.yaml
│   │   ├── python.yaml
│   │   └── ...
│   ├── enterprise/            # Previously in .asimov/templates/
│   │   ├── api-rust.yaml
│   │   ├── api-go.yaml
│   │   ├── api-fastapi.yaml
│   │   ├── api-nestjs.yaml
│   │   ├── api-spring.yaml
│   │   ├── web-nextjs.yaml
│   │   ├── web-react.yaml
│   │   ├── web-vue.yaml
│   │   ├── web-angular.yaml
│   │   ├── mono-turbo.yaml
│   │   ├── mono-nx.yaml
│   │   ├── mono-pnpm.yaml
│   │   └── admin-dashboard.yaml
│   └── hooks/
│       ├── pre-commit.sh.tpl
│       ├── session-start.sh.tpl
│       └── pre-compact.sh.tpl
└── src/templates/mod.rs       # include_str! loading
```

### 2. Embedding Pattern

```rust
// cli/src/templates/mod.rs

// Project templates (compile-time embedded)
const PROJECT_RUST: &str = include_str!("../../templates/project/rust.yaml");
const PROJECT_PYTHON: &str = include_str!("../../templates/project/python.yaml");
// ...

// Enterprise templates
const ENTERPRISE_API_RUST: &str = include_str!("../../templates/enterprise/api-rust.yaml");
// ...

/// Load project template with runtime override support
pub fn project_template(project_type: &str) -> String {
    // 1. Check .asimov/templates/project/{type}.yaml (runtime override)
    // 2. Fall back to embedded template
    load_template_with_fallback("project", project_type)
}
```

### 3. Runtime Override Hierarchy

```
.asimov/templates/project/rust.yaml     ← User override (highest priority)
cli/templates/project/rust.yaml         ← Embedded fallback (compile-time)
```

**Loading logic:**
1. Check `.asimov/templates/{category}/{name}.yaml`
2. If not found, use embedded default
3. Validate against schema (both sources)

### 4. Template Format

All templates use **YAML** (not `.tpl` pseudo-format):

```yaml
# cli/templates/project/rust.yaml
# Canonical Rust project template - embedded at compile time
# Override: .asimov/templates/project/rust.yaml

name: "{PROJECT_NAME}"
tagline: "Built with Rust"
type: rust

identity:
  mission: "..."

quality:
  lint: "cargo clippy -- -D warnings"
  test: "cargo test"
  build: "cargo build --release"

# ADR-034: Standard deliverables for coding projects
deliverables_template:
  - "[ ] Unit tests pass"
  - "[ ] E2E tests pass (if applicable)"
  - "[ ] cargo clippy -- -D warnings (zero warnings)"
  - "[ ] Update README if needed"
  - "[ ] Update --help if CLI"
  - "[ ] Commit and push"
  - "[ ] GitHub release (if applicable)"
```

### 5. Deprecation of .asimov/templates/

The 21 enterprise templates currently in `.asimov/templates/` will be:
1. Moved to `cli/templates/enterprise/`
2. Baked into the binary
3. `.asimov/templates/` becomes **override-only** (not canonical source)

**Migration:**
- `asimov refresh` will NOT regenerate `.asimov/templates/` by default
- Custom templates in `.asimov/templates/` continue to work (override)
- `asimov init --template api-rust` uses embedded template

## Consequences

### Positive

1. **Single source of truth** - All templates in `cli/templates/`
2. **Packageable** - Works with `cargo publish` (no path issues)
3. **Editable** - YAML files, not Rust code
4. **Consistent** - Same pattern as protocols
5. **User customization** - Override via `.asimov/templates/`
6. **Enterprise templates included** - No separate download/install

### Negative

1. **Binary size increase** - ~50KB more (21 enterprise templates)
   - Acceptable trade-off for consistency
2. **Migration effort** - Move existing `.tpl` files to YAML
   - One-time effort, improves maintainability

### Neutral

1. **Performance** - Same as protocols (negligible file I/O vs embedded)
2. **Backward compatibility** - Existing `.asimov/templates/` overrides work

## Implementation

### Phase 1: Core Templates (v10.3.0)
- Create `cli/templates/project/` with 9 base types
- Create `cli/templates/warmup/` with matching warmup templates
- Update `cli/src/templates/mod.rs` to use `include_str!`
- Remove `.tpl` files from `cli/src/templates/`

### Phase 2: Enterprise Templates (v10.3.0)
- Move `.asimov/templates/*.yaml` to `cli/templates/enterprise/`
- Embed all 21 enterprise templates
- Update `asimov init --template` to use embedded templates

### Phase 3: ADR-034 Completion (v10.3.0)
- Add `deliverables_template` to all templates
- Implement auto-inheritance in roadmap parsing
- Surface templates in `asimov warmup` output

## References

- [ADR-053: External Protocol and Template Architecture](053-external-protocols.md) - Extended by this ADR
- [ADR-034: Project-Type-Aware Deliverables](034-project-type-deliverables.md) - Template format includes deliverables
- v10.2.4 - Protocol packaging fix (same pattern)

---

*Built with [RoyalBit Asimov](https://github.com/royalbit/asimov)*
