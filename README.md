# RoyalBit Asimov

[![CI](https://github.com/royalbit/asimov/actions/workflows/ci.yml/badge.svg)](https://github.com/royalbit/asimov/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/code-ELv2-blue.svg)](LICENSE)
[![Docs License](https://img.shields.io/badge/docs-CC%20BY--NC--ND%204.0-lightgrey.svg)](https://creativecommons.org/licenses/by-nc-nd/4.0/)

> **Dynamic Swarm + HITM beats Fixed Agentic by 34x. We have the math.**

## The Architecture

```
Human (HITM)
    ↓ oversight
Orchestrator (~200K tokens, extended thinking)
    ↓ spawns dynamically at runtime
    ├── Sub-Agent 1 (~200K tokens)
    ├── Sub-Agent 2 (~200K tokens)
    └── Sub-Agent N (~200K tokens)
```

**Dynamic Swarm** (Asimov + Claude Code): Each agent has full context (~200K tokens). Spawning is AI-decided at runtime. Human-in-the-middle (HITM) approves/guides.

**Fixed Agentic** (LangChain, CrewAI, AutoGen): Pre-defined roles. Fragmented context. No human gate. 17.2x error amplification.

Source: [Claude Code Subagents](https://code.claude.com/docs/en/sub-agents) — Official Anthropic documentation

---

## The Insight

**Monte Carlo simulation (10K trials, 95% CI):**

| Steps | Dynamic Swarm + HITM | Fixed Multi-Agent | Advantage |
|-------|---------------------|-------------------|-----------|
| 10 | **81.5%** ± 0.8% | 2.4% ± 0.3% | **34x** |
| 20 | **66.4%** ± 0.9% | 0.06% ± 0.05% | **1,106x** |
| 50 | **36.0%** ± 0.9% | ~0% | **∞** |

Source: [monte-carlo-agents.yaml](models/monte-carlo-agents.yaml) — validated against R and Gnumeric via Forge.

**Why Dynamic Swarm wins:**
- Each sub-agent has **full context** (~200K tokens), not fragmented
- **HITM oversight** prevents error cascades between steps
- **AI-decided spawning** avoids fixed topology overhead
- Extended thinking enables **70% in-context error detection**

**Why Fixed Agentic fails:**
- Context fragmented across agents (10-20% effective utilization)
- O(n^1.724) communication overhead
- 17.2x error amplification without human gate
- Pre-defined roles can't adapt to task requirements

📖 [ADR-056: Extended Thinking vs RAG+Agentic](docs/adr/056-extended-thinking-vs-rag-agentic.md) — Full research

---

## Quick Start

```bash
# Install
curl -L https://github.com/royalbit/asimov/releases/latest/download/asimov-$(uname -m)-unknown-linux-gnu.tar.gz | tar xz
sudo mv asimov /usr/local/bin/

# Initialize project
asimov init

# Launch Claude Code with optimal settings (Dynamic Swarm enabled)
asimov
```

**Requires [Claude Code](https://claude.ai/code).** Protocol files work anywhere (paste them).

---

## What It Does

Eight protocol files in `.asimov/` ground AI in file-based truth:

```
.asimov/
├── asimov.json         # Ethics: harm categories + veto commands
├── sprint.json         # Autonomous execution until done
├── sycophancy.json     # Truth over comfort
├── freshness.json      # WebSearch for current information
├── green.json          # Efficiency benchmarking
└── ...
```

**The pattern:** File truth (stable, deterministic) beats AI memory (lossy, probabilistic).

---

## CLI

```bash
asimov              # Launch Claude Code with MAX_THINKING_TOKENS=200000 + Dynamic Swarm
asimov init         # Initialize project
asimov warmup       # Output complete context as JSON
asimov doctor       # Diagnose setup issues
asimov validate     # Validate protocol files
asimov update       # Self-update
```

**Platforms:** Linux, macOS, Windows | **Binary:** 1.5MB | **Dependencies:** Zero

---

## The Research

| Finding | Source |
|---------|--------|
| Dynamic Swarm: **34x advantage** over fixed agentic at 10 steps | [Monte Carlo Model](models/monte-carlo-agents.yaml) |
| Sub-agents run with **full context (~200K tokens)** | [Claude Code Docs](https://code.claude.com/docs/en/sub-agents) |
| Max **3-4 effective agents** before overhead dominates | [Google/MIT 2024](https://venturebeat.com/orchestration/research-shows-more-agents-isnt-a-reliable-path-to-better-enterprise-ai) |
| **17.2x error amplification** with independent fixed agents | Google/MIT |
| RLI benchmark: **97.5% failure** on real work (vs 70% SWE-bench) | [arXiv:2504.02189](https://arxiv.org/abs/2504.02189) |
| Fixed multi-agent: **fragile systems, dispersed decision-making** | [Cognition (Devin)](https://cognition.ai/blog/dont-build-multi-agents) |

### Architecture Decisions

- [ADR-056: Extended Thinking vs RAG+Agentic](docs/adr/056-extended-thinking-vs-rag-agentic.md) — Monte Carlo proof
- [ADR-054: Dynamic Swarm vs Fixed Frameworks](docs/adr/054-dynamic-swarm-vs-fixed-agentic-frameworks.md) — 50+ references
- [ADR-055: Balanced Architecture Critique](docs/adr/055-balanced-architecture-critique.md) — Trade-offs acknowledged

---

## Proven at Scale

| Project | LOC | Tests | Releases |
|---------|-----|-------|----------|
| Forge (private) | 45,700 | 2,486 | 46 |
| Asimov | 19,000+ | 437 | 62+ |
| **Total** | **65,000+** | **2,900+** | **108+** |

**1 human. 1 AI. 12 days.**

📖 [Origin Story](docs/ORIGIN_STORY.md) — How we built it

---

## Documentation

- [Value Proposition](docs/VALUE_PROPOSITION.md) — Why this matters
- [Setup Guide](docs/SETUP.md) — Detailed installation
- [AI Reality](docs/AI_REALITY.md) — Why AI "hallucinates"
- [Full Specification](docs/SPECIFICATION.md) — Protocol schema

---

## License

- **Code:** [Elastic License 2.0 (ELv2)](LICENSE) — Free for most uses, restrictions on competing SaaS
- **Documentation:** [CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/)

| Use Case | Allowed? |
|----------|----------|
| Personal/internal use | ✅ Yes |
| Commercial use (non-competing) | ✅ Yes |
| Modify for internal use | ✅ Yes |
| Provide as managed service | ❌ No |
| Circumvent license keys | ❌ No |

---

*Built with [RoyalBit Asimov](https://github.com/royalbit/asimov) — Dynamic Swarm + HITM beats Fixed Agentic.*
