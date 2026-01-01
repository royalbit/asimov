# Glossary

Quick reference for RoyalBit Asimov terminology.

---

## AI Oversight Models

| Term | Expansion | Definition | Example |
|------|-----------|------------|---------|
| **HOTL** | Human-on-the-Loop | Human monitors execution, intervenes when needed | **Asimov** |
| **HITL** | Human-in-the-Loop | Human validates every decision/step | BMAD |
| **Fixed Agentic** | - | Pre-defined agent topology, no human oversight | LangChain, CrewAI |

### HOTL vs HITL

```
HOTL: AI runs → Human watches → (optional) Human vetoes → AI stops
HITL: AI proposes → Human validates → AI proceeds → repeat
```

**Asimov uses HOTL.** The AI runs autonomously while you watch. Veto keywords give immediate control.

See [ADR-027](adr/027-bmad-incompatibility.md) for full comparison.

---

## Architecture Terms

| Term | Definition |
|------|------------|
| **Dynamic Swarm** | AI-decided agent spawning at runtime, full context (~200K) per agent |
| **Context Fragmentation** | Splitting context across agents (8-32K each vs 200K unified) |
| **Extended Thinking** | Using thinking tokens for sequential reasoning, not storage |
| **Self-Correction** | Error detection + fix via "Wait" tokens and human oversight |

---

## Protocol Terms

| Term | Definition |
|------|------------|
| **Veto Keywords** | `stop`, `halt`, `abort`, `emergency stop` - immediate AI halt |
| **Harm Categories** | `financial`, `physical`, `privacy`, `deception` - prohibited actions |
| **Warmup** | Session initialization - loads protocols, project state, roadmap |
| **Sprint** | Autonomous execution mode - run until done or vetoed |

---

## Metrics

| Term | Definition | Source |
|------|------------|--------|
| **38x Advantage** | Dynamic Swarm vs Fixed Agentic success rate at 10 steps | [Forge Model](../models/agent-formulas.yaml) |
| **17.2x Error Amplification** | Independent agents vs single-agent baseline | MIT/Google 2024 |
| **74% Error Reduction** | HOTL oversight vs no oversight | MIT/Google 2024 |
| **O(n^1.724)** | Communication overhead scaling for multi-agent | MIT/Google 2024 |

---

## Abbreviations

| Abbrev | Meaning |
|--------|---------|
| ADR | Architecture Decision Record |
| MCP | Model Context Protocol |
| RAG | Retrieval Augmented Generation |
| SWE-bench | Software Engineering Benchmark |
| RLI | Real-world LLM Intelligence (benchmark) |

---

*See [Full Specification](SPECIFICATION.md) for protocol schemas.*
