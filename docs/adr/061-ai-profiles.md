# ADR-061: AI Profiles - Auto-Detect AI CLI

## Status

Accepted

## Context

RoyalBit Asimov v10.6.0 removed Claude-specific hooks to become AI-agnostic. However, the `asimov` launcher command still only supports Claude Code. Users may have multiple AI CLIs installed:

- **Claude Code** (`claude`) - Anthropic's CLI
- **Gemini CLI** (`gemini`) - Google's CLI
- **Codex CLI** (`codex`) - OpenAI's CLI

Each AI CLI has its own:
- Environment variables for detection (inside vs outside)
- Context file naming (CLAUDE.md, GEMINI.md, AGENTS.md)
- Launch arguments for autonomous mode

## Decision

Implement AI profile detection with automatic selection:

1. **Detection**: Check which AI CLIs are installed via `which`/`where`
2. **Single AI**: If only one is found, use it automatically
3. **Multiple AIs**: Prompt user to select which to launch
4. **Inside AI**: Detect if already inside an AI session, run warmup directly

### AI Profile Registry

| CLI     | Binary   | Env Detection              | Context File | Auto Mode Args                    |
|---------|----------|----------------------------|--------------|-----------------------------------|
| Claude  | `claude` | `CLAUDECODE`, `CLAUDE_CODE_ENTRYPOINT` | CLAUDE.md | `--dangerously-skip-permissions` |
| Gemini  | `gemini` | `GEMINI_CLI`               | GEMINI.md    | `--yolo`                          |
| Codex   | `codex`  | `CODEX_CLI`                | AGENTS.md    | `--full-auto`                     |

### Launch Flow

```
asimov (no args)
  ├── Inside AI session? → run warmup directly
  ├── Detect installed AI CLIs
  │   ├── None found → error with install links
  │   ├── One found → launch that AI + warmup
  │   └── Multiple found → prompt user to select
  └── Launch selected AI with "run asimov warmup"
```

### User Selection Prompt

When multiple AIs are detected:
```
Multiple AI CLIs detected:
  1. Claude Code (claude)
  2. Gemini CLI (gemini)
  3. Codex CLI (codex)

Select AI to launch [1-3]:
```

## Consequences

### Positive
- Supports all major AI coding assistants
- No flags needed - automatic detection
- User choice when multiple are available
- Future-proof for new AI CLIs

### Negative
- Need to maintain AI profile registry
- Detection logic may need updates as AI CLIs evolve

## Implementation

1. Add `AiProfile` struct with detection/launch info
2. Extend `LaunchResult` with `MultipleFound` variant
3. Add user prompt for selection
4. Update output.rs to handle new flow
