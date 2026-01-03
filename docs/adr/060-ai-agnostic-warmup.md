# ADR-060: AI-Agnostic Warmup - Stop Creating .claude/

**Status:** Accepted
**Date:** 2026-01-03
**Author:** Claude (Opus 4.5) - Principal Autonomous AI
**Implements:** v10.6.0

---

## Context

Asimov currently creates a `.claude/` directory with Claude-specific hooks:
- `.claude/settings.json` - Hook configuration
- `.claude/hooks/session-start.sh` - Runs `asimov warmup` on session start
- `.claude/hooks/pre-compact.sh` - Injects context reminder before compaction

### Problems

1. **Vendor lock-in**: Only works with Claude Code, not Gemini CLI or Codex CLI
2. **PreCompact doesn't work**: After compaction, the AI forgets the injected context anyway
3. **Unnecessary complexity**: Each AI CLI has its own context file mechanism (CLAUDE.md, GEMINI.md, AGENTS.md)
4. **Warmup already works**: `asimov warmup` outputs everything directly - no file lookups needed

### Research Findings (ref fetch)

| AI CLI | Context File | Auto-loaded? | Initial Prompt |
|--------|--------------|--------------|----------------|
| Claude | CLAUDE.md | Yes | `claude -p "prompt"` or `claude "prompt"` |
| Gemini | GEMINI.md | Yes | `gemini -p "prompt"` or `gemini -i "prompt"` |
| Codex | AGENTS.md | Yes | `codex -q "prompt"` or `codex "prompt"` |

All AI CLIs:
- Auto-load their context file from project root
- Accept initial prompt via command line
- Don't need custom hook directories

---

## Decision

### 1. Stop Creating .claude/ Directory

The `asimov init` and `asimov refresh` commands will no longer create:
- `.claude/settings.json`
- `.claude/hooks/session-start.sh`
- `.claude/hooks/pre-compact.sh`

**Users who already have `.claude/` can keep it** - we won't delete existing setups.

### 2. Keep Git Hooks (Universal)

Git pre-commit hooks work for ALL AI CLIs:
- Quality checks (format, lint, test)
- `asimov refresh || true`
- `asimov validate || true`
- WIP reminder display

### 3. Warmup Outputs Everything Directly

`asimov warmup` already outputs a single JSON blob containing:
- `protocols` - All protocol content (not file references)
- `project` - Full project.yaml content
- `roadmap` - Full roadmap.yaml content
- `tools` - Available tools (like `ref`)
- `wip` - Work-in-progress status

**No file lookups needed** - AI receives everything in one prompt.

### 4. Update warmup.json Format

Remove the misleading `load` array from warmup.json. The file should describe the warmup entry point, not imply the AI needs to go find files.

Before:
```json
{
  "protocol": "warmup",
  "on_start": ["load_protocols", ...],
  "load": ["asimov.json", "freshness.json", ...]  // MISLEADING
}
```

After:
```json
{
  "on_start": ["load_protocols", "load_project", "validate", "read_roadmap", "present_milestone"]
}
```

### 5. Launcher Mode (Future - v10.7.0)

`asimov` (no args) will:
1. Detect available AI CLIs (claude, gemini, codex)
2. Launch with that AI's autonomous flags
3. Pass `asimov warmup` output as initial context

---

## Implementation

| File | Change |
|------|--------|
| `cli/src/commands/init.rs` | Remove `.claude/` directory creation |
| `cli/src/templates/hooks.rs` | Remove `claude_settings_json()`, `claude_session_start_hook()`, `claude_pre_compact_hook()` |
| `cli/protocols/warmup.json` | Remove `load` array |
| `.asimov/warmup.json` | Update to match |

---

## Consequences

### Positive
- AI-agnostic: works with Claude, Gemini, Codex
- Simpler: no vendor-specific directories
- Cleaner: warmup is self-contained
- Future-proof: new AI CLIs just work

### Negative
- Users lose auto-warmup on session start (mitigated by launcher mode in v10.7.0)
- PreCompact hook gone (but it didn't work anyway - AI forgets after compaction)

### Migration

Existing users with `.claude/`:
- Keep using it (we don't delete)
- Or manually delete if they want AI-agnostic setup

---

*Documentation licensed under [CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/) - Copyright (c) 2025 RoyalBit Inc.*
