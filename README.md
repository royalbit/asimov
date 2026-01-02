# Asimov

> **Dynamic Agentic AI with Human-on-the-Loop.**
> 39x better than fixed frameworks at 10 steps. 1502x at 20.

[![CI](https://github.com/royalbit/asimov/actions/workflows/ci.yml/badge.svg)](https://github.com/royalbit/asimov/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/code-ELv2-blue.svg)](LICENSE)
[![Docs License](https://img.shields.io/badge/docs-CC%20BY--NC--ND%204.0-lightgrey.svg)](https://creativecommons.org/licenses/by-nc-nd/4.0/)

---

## The Problem

Fixed agentic frameworks fail at scale:

| Issue | Impact |
|-------|--------|
| **17.2x error amplification** | Independent agents multiply mistakes |
| **Max 3-4 effective agents** | O(n^1.724) communication overhead |
| **8-32K context per agent** | Fragmented understanding |
| **Pre-defined roles** | Can't adapt to task requirements |

> "Multi-agent systems in 2025 result in fragile systems. Decision-making too dispersed."
> — [Cognition (Devin)](https://cognition.ai/blog/dont-build-multi-agents)

---

## The Solution

**Dynamic Swarm + HOTL:**

```
Human (HOTL)
    ↓ oversight (can intervene at any step)
Orchestrator (~200K tokens, extended thinking)
    ↓ spawns dynamically at runtime
    ├── Sub-Agent 1 (~200K tokens)
    ├── Sub-Agent 2 (~200K tokens)
    └── Sub-Agent N (~200K tokens)
```

| Dynamic Swarm + HOTL | Fixed Agentic |
|---------------------|---------------|
| ~200K tokens per agent | 8-32K fragmented |
| AI-decided spawning | Pre-defined roles |
| HOTL catches errors | No human gate |
| 75% error detection | 17.2x amplification |

Source: [Claude Code Subagents](https://code.claude.com/docs/en/sub-agents) — Official Anthropic documentation

---

## The Math

Monte Carlo simulation. 10,000 trials. Validated against R and Gnumeric.

| Steps | Dynamic Swarm + HOTL | Fixed Agentic | Advantage |
|-------|---------------------|---------------|-----------|
| 5 | **95.2%** | 15.3% | 6x |
| 10 | **90.7%** | 2.3% | **39x** |
| 20 | **82.2%** | 0.05% | **1,502x** |
| 50 | **61.3%** | ~0% | **∞** |

Source: [agent-formulas.yaml](models/agent-formulas.yaml) — validate with `forge calculate models/agent-formulas.yaml`

Research: [Google/MIT 2024](https://venturebeat.com/orchestration/research-shows-more-agents-isnt-a-reliable-path-to-better-enterprise-ai)

---

## Quick Start

```bash
# Install
curl -L https://github.com/royalbit/asimov/releases/latest/download/asimov-$(uname -m)-unknown-linux-musl.tar.gz | tar xz
sudo mv asimov /usr/local/bin/

# Initialize project
asimov init

# Launch Claude Code with Dynamic Swarm
asimov
```

**Requires [Claude Code](https://claude.ai/code).**

---

## How It Works

Asimov implements Dynamic Swarm + HOTL through 8 protocol files:

```
.asimov/
├── asimov.json         # Ethics: harm categories + veto commands
├── sprint.json         # Autonomous execution until done
├── sycophancy.json     # Truth over comfort
├── freshness.json      # Use ref for web fetching
├── warmup.json         # Session initialization
├── green.json          # Efficiency benchmarking
├── coding-standards.json
└── migrations.json     # For migration projects
```

**The pattern:** File truth (stable, deterministic) beats AI memory (lossy, probabilistic).

**Warmup** injects complete context at session start — zero file reads required:

```bash
asimov warmup  # Outputs JSON with project + roadmap + protocols
```

---

## CLI

```bash
asimov              # Launch Claude Code with MAX_THINKING_TOKENS=200000
asimov init         # Initialize project
asimov warmup       # Output complete context as JSON
asimov doctor       # Diagnose setup issues
asimov validate     # Validate protocol files
asimov update       # Self-update
```

**Platforms:** Linux, macOS, Windows | **Binary:** 1.5MB | **Dependencies:** Zero

---

## Limitations

| Limitation | Impact | Mitigation |
|------------|--------|------------|
| **HOTL bottleneck** | Human approval can slow work | Batch approvals, trust calibration |
| **Claude dependency** | No GPT/Gemini interop | Protocol files are model-agnostic |
| **Token cost** | ~200K context costs more | Fewer errors, less rework |
| **Spawn latency** | ~2-5s per sub-agent | Parallelizable, amortized |

**Research transparency:**
- O(n^1.724) exponent from [VentureBeat summary](https://venturebeat.com/orchestration/research-shows-more-agents-isnt-a-reliable-path-to-better-enterprise-ai) of MIT/Google research
- Monte Carlo simulations reproducible: `python models/simulate.py`
- See [Formula Derivations](docs/FORMULA_DERIVATIONS.md) for methodology

---

## The Research

| Finding | Source |
|---------|--------|
| **39x advantage** at 10 steps | [Forge Model](models/agent-formulas.yaml) |
| Sub-agents run with **~200K tokens** | [Claude Code Docs](https://code.claude.com/docs/en/sub-agents) |
| HOTL reduces error by **74%** | [MIT/Google 2024](https://venturebeat.com/orchestration/research-shows-more-agents-isnt-a-reliable-path-to-better-enterprise-ai) |
| Max **3-4 agents** before overhead dominates | MIT/Google 2024 |
| RLI: **97.5% failure** on real work | [arXiv:2504.02189](https://arxiv.org/abs/2504.02189) |

### ADRs

- [ADR-056: Extended Thinking vs RAG+Agentic](docs/adr/056-extended-thinking-vs-rag-agentic.md)
- [ADR-054: Dynamic Swarm vs Fixed Frameworks](docs/adr/054-dynamic-swarm-vs-fixed-agentic-frameworks.md)
- [ADR-055: Balanced Architecture Critique](docs/adr/055-balanced-architecture-critique.md)

---

## Documentation

- [Value Proposition](docs/VALUE_PROPOSITION.md)
- [Setup Guide](docs/SETUP.md)
- [Glossary](docs/GLOSSARY.md) — HOTL, HITL, Dynamic Swarm
- [Formula Derivations](docs/FORMULA_DERIVATIONS.md)
- [Full Specification](docs/SPECIFICATION.md)

---

## License

- **Code:** [Elastic License 2.0](LICENSE) — Free for most uses, restrictions on competing SaaS
- **Documentation:** [CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/)

| Use Case | Allowed? |
|----------|----------|
| Personal/internal use | Yes |
| Commercial use (non-competing) | Yes |
| Modify for internal use | Yes |
| Provide as managed service | No |
| Circumvent license keys | No |

For commercial licensing, [open a GitHub issue](https://github.com/royalbit/asimov/issues).

---

*Dynamic Swarm + HOTL beats Fixed Agentic. We have the math.*
