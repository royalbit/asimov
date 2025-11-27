# Component 4: Self-Healing

> **Survive context compaction - re-read rules from disk**

## Overview

Self-Healing solves a critical problem: **AI forgets rules after context compaction**.

During long sessions (2-4+ hours), Claude Code compresses conversation history. Rules get summarized away. The AI "forgets" project conventions.

Self-Healing fixes this by **re-reading rules from disk** instead of trying to preserve them in memory.

## The Problem

```
Session Start                    After Compaction
─────────────                    ────────────────
Full rules loaded                Rules summarized
Quality gates clear       →      "There were some rules"
Patterns understood              Patterns forgotten
AI follows conventions           AI makes mistakes
```

As documented by DoltHub:
> "Claude Code is definitely dumber after the compaction. It doesn't know what files it was looking at and needs to re-read them."

## The Solution

```
┌─────────────────────────────────────────────────────────────┐
│                    SELF-HEALING PROTOCOL                    │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│   CLAUDE.md              warmup.yaml         checkpoint.yaml│
│  (auto-loaded)           (full rules)        (session state)│
│  ┌───────────┐          ┌───────────┐       ┌───────────┐  │
│  │ Short     │          │ Complete  │       │ Progress  │  │
│  │ rules +   │─────────▶│ protocol  │──────▶│ + next    │  │
│  │ "re-read" │          │ + gates   │       │ steps     │  │
│  └───────────┘          └───────────┘       └───────────┘  │
│       │                       │                   │        │
│       └───────────────────────┴───────────────────┘        │
│                    ALL ON DISK                              │
│                (survives compaction)                        │
└─────────────────────────────────────────────────────────────┘
```

## The Three Files

### 1. CLAUDE.md (Auto-loaded)

Short, survives summarization. Contains the critical instruction:

```markdown
## CRITICAL: Self-Healing Protocol

After ANY compaction or confusion, RE-READ:
1. warmup.yaml - Full protocol and rules
2. .claude_checkpoint.yaml - Session state (if exists)
```

### 2. warmup.yaml (Full Rules)

Complete protocol on disk. Re-read when needed:

```yaml
quality:
  tests: "cargo test"
  warnings: "cargo clippy -- -D warnings"

self_healing:
  checkpoint_interval: "2 hours"
  on_confusion: "Re-read warmup.yaml"
```

### 3. .claude_checkpoint.yaml (Session State)

Written every 2 hours. Contains progress and next steps:

```yaml
timestamp: "2025-11-26T15:30:00Z"
milestone: "Add user authentication"
completed:
  - "Created auth middleware"
  - "Added JWT generation"
in_progress: "Writing login tests"
next_steps:
  - "Implement logout"
```

## Checkpoint Triggers

| Trigger | Action |
|---------|--------|
| Every 2 hours | Write checkpoint, re-read warmup.yaml |
| Before commit | Re-read quality gates |
| After compaction | Re-read warmup.yaml + checkpoint |
| When confused | STOP, re-read everything |

## Platform Requirement

**Self-Healing requires Claude Code.** Other AI tools lack:

| Capability | Required | ChatGPT | Copilot | Claude Code |
|------------|----------|---------|---------|-------------|
| Auto-load config | Yes | No | No | Yes |
| File system access | Yes | No | No | Yes |
| Re-read mid-session | Yes | No | No | Yes |

## Relationship to Other Components

| Component | Connection |
|-----------|------------|
| Protocol Files | warmup.yaml is the source of truth |
| Sprint Autonomy | 2hr checkpoint aligns with sprint checks |
| Quality Gates | Re-read before running gates |
| Release Discipline | Checkpoint before release |

---

**Next:** [Component 5: Release Discipline](5-RELEASE_DISCIPLINE.md)
