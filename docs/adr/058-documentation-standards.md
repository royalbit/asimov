# ADR-058: Documentation Standards - Transparency as Moat

**Status:** Accepted
**Date:** 2026-01-01
**Author:** Claude (Opus 4.5) - Principal Autonomous AI
**Implements:** v10.5.0
**Tooling:** [ref](https://github.com/royalbit/ref) - LLM-optimized reference verification
**References:** Grok feedback (2026-01-01), ADR-055 (Balanced Critique)

---

## Context

Grok's review of Asimov identified citation gaps:

> "Some key claims lack direct primary sources or full hyperlinks... The O(n^1.724) communication overhead formula has no citation or explanation—where does this come from?"

This feedback revealed an opportunity: **transparency and correctness can be a competitive moat**.

### Current State (v10.4.0)

- `references.yaml` contains 150+ sources but mixes primary and secondary
- VentureBeat articles cited for MIT/Google research (secondary source)
- Some metrics derived but derivation not documented
- `models/simulate.py` added for reproducibility

### The Moat Thesis

Most AI projects optimize for:
- Speed to market
- Feature count
- Marketing claims

Few optimize for **verifiable correctness**. By making every Asimov claim traceable to primary sources, we create trust that competitors cannot easily replicate.

---

## Decision

### 1. Citation Hierarchy

All claims in Asimov documentation MUST use this hierarchy:

| Priority | Source Type | Example | Requirement |
|----------|-------------|---------|-------------|
| 1 | **Primary Research** | arXiv paper, Nature article | DOI or permanent URL |
| 2 | **Official Documentation** | Anthropic docs, RFC | Canonical URL |
| 3 | **Secondary Source** | VentureBeat, blog | Mark with `secondary_source: true` |
| 4 | **Derived/Calculated** | Monte Carlo results | Link to derivation code |

### 2. Required Metadata

Every reference in `references.yaml` MUST include:

```yaml
- title: "Paper Title"
  url: https://...
  doi: "10.xxxx/..."          # If available
  date: 2024-01
  organization: "Research Lab"
  key_finding: "One sentence"
  # For secondary sources:
  secondary_source: true
  primary_research: "Original paper - status"
  citation_note: "Why secondary, when primary expected"
```

### 3. Claim Categories

#### Quantitative Claims (Numbers)
- **MUST** have primary source or documented derivation
- Example: "39x advantage" → links to `models/simulate.py` + `docs/FORMULA_DERIVATIONS.md`

#### Architectural Claims (Design)
- **MUST** reference official documentation or ADR
- Example: "Full 200K context" → links to Claude Code docs

#### Comparative Claims (vs Competitors)
- **MUST** use peer-reviewed or official sources
- **MUST NOT** use marketing materials as primary evidence

#### Pending Claims
- Claims awaiting primary publication **MUST** be flagged:

```markdown
**Citation Status:** Secondary source. Original paper pending publication.
```

### 4. Validation Process

#### At PR Time
- New quantitative claims require source link
- `references.yaml` changes require DOI where available

#### Periodic Audit
- Quarterly review of `secondary_source: true` entries
- Check if primary sources have been published
- Update citations when primaries become available

### 5. Transparency Statements

README and key docs MUST include:

```markdown
**Research Transparency:**
- [Metric] from [Source] - [Primary/Secondary/Derived]
- Simulations reproducible: `python models/simulate.py`
```

---

## Implementation

### Tooling: ref (Already Available)

The [ref](https://github.com/royalbit/ref) tool provides citation verification:

```bash
# Verify all references in references.yaml
ref verify-refs

# Scan markdown for URLs, build references.yaml
ref scan docs/

# Check link health
ref check-links docs/README.md

# Fetch URL with headless Chrome (bypasses bot protection)
ref fetch https://arxiv.org/abs/2310.06770
```

### Phase 1: Standards (v10.5.0)
- [x] This ADR accepted
- [ ] `references.yaml` schema updated with required fields
- [ ] Existing secondary sources marked
- [ ] README transparency section expanded
- [ ] `ref verify-refs` integrated into CI

### Phase 2: Content Verification (Future)

**LLM-as-Verifier:** Not just link health, but content accuracy.

```bash
# Verify key_finding claims against actual source content
ref verify-content references.yaml

# Full audit: links + content + staleness
ref audit --full
```

**Verification Schema:**
```yaml
- title: "SWE-bench Paper"
  url: https://arxiv.org/abs/2310.06770
  key_finding: "2,294 real GitHub issues"
  content_hash: "sha256:abc123..."    # Detect source changes
  last_verified: 2026-01-01
  verification_status: verified        # verified|partial|unsupported
```

**Implementation:**
- [ ] `ref verify-content` - LLM checks if source supports `key_finding`
- [ ] `ref audit --full` - comprehensive report with confidence scores
- [ ] Content hash tracking to detect source drift
- [ ] CI integration: `ref verify-content --fail-on-unsupported`
- [ ] Auto-add DOI for arXiv papers
- [ ] Flag unmarked secondary sources
- [ ] Quarterly re-verification automation

---

## Consequences

### Positive

1. **Trust Moat** - Competitors can't easily match verified claims
2. **Self-Correction** - Forces us to find primary sources or acknowledge gaps
3. **Academic Credibility** - Proper citations enable scholarly reference
4. **Reduced Liability** - Clear provenance for all claims

### Negative

1. **Velocity Tax** - Adding sources takes time
2. **Uncomfortable Truths** - Some claims may need to be walked back
3. **Maintenance Burden** - Tracking pending primaries

### Trade-off

The velocity tax is worth the trust premium. Per ADR-055, Asimov already committed to "truth over comfort." This ADR operationalizes that commitment for documentation.

---

## Examples

### Good: Primary Source

```yaml
- title: "SWE-bench: Can Language Models Resolve Real-World GitHub Issues?"
  url: https://arxiv.org/abs/2310.06770
  doi: "10.48550/arXiv.2310.06770"
  date: 2023-10
  organization: Princeton
  key_finding: "Benchmark of 2,294 real GitHub issues"
```

### Good: Marked Secondary

```yaml
- title: "Research shows 'more agents' isn't reliable"
  url: https://venturebeat.com/...
  date: 2024-12
  secondary_source: true
  primary_research: "MIT/Google study - not yet published"
  citation_note: "Metrics from journalism summary"
  key_finding: "17.2x error amplification, O(n^1.724) overhead"
```

### Good: Derived Claim

```markdown
| Steps | Dynamic Swarm | Fixed Agentic | Advantage |
|-------|---------------|---------------|-----------|
| 10 | 90.7% | 2.3% | **39x** |

Source: [Monte Carlo simulation](models/simulate.py) with parameters from [agent-formulas.yaml](models/agent-formulas.yaml).
Derivation: [FORMULA_DERIVATIONS.md](docs/FORMULA_DERIVATIONS.md)
```

### Bad: Unsourced Claim

```markdown
Dynamic Swarm is 39x better than fixed agentic systems.
```
*(No source, no derivation link)*

---

## References

- [ref](https://github.com/royalbit/ref) - LLM-optimized reference verification toolkit
- [references.yaml](../../references.yaml) - Source of truth for all citations
- [ADR-055: Balanced Architecture Critique](055-balanced-architecture-critique.md) - Truth over comfort
- [FORMULA_DERIVATIONS.md](../FORMULA_DERIVATIONS.md) - Derivation methodology

---

*Built with [RoyalBit Asimov](https://github.com/royalbit/asimov) - Transparency as Moat*
