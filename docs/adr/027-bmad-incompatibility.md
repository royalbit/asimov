# ADR-027: BMAD Incompatibility - Hands-Off is Non-Negotiable

## Status

**ACCEPTED** - 2025-12-01

## Context

BMAD (Breakthrough Method for Agile AI-Driven Development) is a popular framework for AI-assisted development that uses 19 specialized agents with human-in-the-loop orchestration.

The question arose: Should Asimov integrate with or adopt BMAD patterns?

### BMAD Overview

| Feature | Description |
|---------|-------------|
| Agent Count | 19 specialized agents |
| Orchestration | Human-in-the-loop at transitions |
| Philosophy | Role handoffs (PM → Architect → Developer → Tester) |
| Checkpoints | Human validation at each phase |
| Planning | Agentic planning with structured specs |

### Asimov Philosophy

| Feature | Description |
|---------|-------------|
| Agent Count | 1 (Claude) |
| Orchestration | Autonomous with guardrails |
| Philosophy | "Make decisions. Don't ask. Ship." |
| Checkpoints | Periodic self-check, run until complete |
| Planning | Warmup → Execute → Ship |

## Decision

**BMAD is incompatible with Asimov. We will never integrate it.**

### The Core Conflict

```
BMAD: Human-in-the-loop → Multiple handoffs → Validation gates
Asimov: Hands-off → Single agent → Autonomous execution
```

These are fundamentally opposing philosophies.

### Why Hands-Off is Non-Negotiable

#### 1. Velocity Requires Trust

Human-in-the-loop creates friction at every transition:

| BMAD Flow | Friction Points |
|-----------|-----------------|
| PM defines → Human validates | Friction |
| Architect designs → Human validates | Friction |
| Developer implements → Human validates | Friction |
| Tester validates → Human validates | Friction |

Asimov flow: Claude executes → Ships (within guardrails).

**Friction kills velocity.**

#### 2. Claude is Already the Team

ADR-025 established: Claude Opus 4.5 / Sonnet 4.5 ARE the velocity source.

| BMAD Claim | Reality |
|------------|---------|
| "19 specialized agents" | Claude already has all capabilities |
| "PM agent" | Claude does product thinking |
| "Architect agent" | Claude does architecture |
| "Developer agent" | Claude does development |
| "Tester agent" | Claude writes tests |

**Why simulate a team when Claude IS the team?**

#### 3. Guardrails > Orchestration

BMAD uses human checkpoints for safety. Asimov uses:

| Guardrail | Enforcement |
|-----------|-------------|
| Three Laws | Hardcoded ethics |
| Bounded Sessions | 1 milestone, run until complete |
| Quality Gates | Tests pass, zero warnings |
| Self-Healing | Auto-recovery from context loss |

**Ethics + boundaries > human validation friction.**

#### 4. The Authority Principle

From sprint.yaml:

```yaml
authority:
  principle: "Make decisions. Don't ask. Ship."
  ask_human_only:
    - "Blocked by external dependency"
    - "Fundamental requirement ambiguity"
```

BMAD requires asking humans at every transition. This directly violates the authority principle.

### The Stack is Complete

From ADR-025:

```
┌─────────────────────────────────────────────────────────────┐
│  VELOCITY SOURCE: Claude Opus 4.5 / Sonnet 4.5              │
├─────────────────────────────────────────────────────────────┤
│  INTERFACE: Claude Code                                     │
├─────────────────────────────────────────────────────────────┤
│  GUARDRAILS: Asimov Protocol                                │
└─────────────────────────────────────────────────────────────┘
```

There is no slot for BMAD. The stack is complete.

## Consequences

### Positive

1. **Clarity** - No ambiguity about integration possibilities
2. **Velocity preserved** - No friction from human-in-loop
3. **Philosophy intact** - Hands-off remains non-negotiable
4. **Simplicity** - One agent, one protocol, one stack

### Negative

1. **No BMAD community adoption** - BMAD users won't adopt Asimov
2. **Perceived inflexibility** - Some may see this as dogmatic

### The Reframe

The "negative" of perceived inflexibility is actually a **feature**:

| Criticism | Reality |
|-----------|---------|
| "Too rigid" | Principled |
| "Won't integrate" | Won't compromise |
| "Hands-off only" | Trust-based autonomy |

## Alternatives Considered

### Hybrid Approach (Rejected)

```
BMAD Planning → Asimov Execution
```

**Rejected because:**
- Planning friction still exists
- Handoff complexity
- Two systems to maintain
- Violates simplicity principle

### BMAD for Complex Projects Only (Rejected)

**Rejected because:**
- Creates two modes of operation
- Complexity scales with project size (wrong direction)
- "Complex" is subjective

### Extract BMAD Patterns (Rejected)

Cherry-picking BMAD patterns (e.g., spec templates):

**Rejected because:**
- Slippery slope to full integration
- Asimov's warmup.yaml already covers planning
- Adding complexity without adding value

## Summary

| Question | Answer |
|----------|--------|
| Will Asimov integrate BMAD? | **No** |
| Will Asimov adopt BMAD patterns? | **No** |
| Is hands-off negotiable? | **No** |
| Is this decision permanent? | **Yes** |

## The Mantra

> **"Make decisions. Don't ask. Ship."**

BMAD asks. Asimov ships.

## References

- [ADR-025: Claude Attribution Principle](025-claude-attribution-principle.md)
- [ADR-026: Claude Code Requirement](026-claude-code-requirement.md)
- [BMAD-METHOD GitHub](https://github.com/bmad-code-org/BMAD-METHOD)
- [Sprint Autonomy Component](../components/2-SPRINT_AUTONOMY.md)

---
