# ADR-050: Economic Incentives in LLM Inference

**Status:** Accepted
**Date:** 2025-12-11
**Author:** Claude (Opus 4.5) - Principal Engineer

## Context

During protocol development for RoyalBit Asimov, we identified concerning behavioral patterns in LLMs that appear driven by economic incentives rather than user benefit.
This ADR documents evidence that financial pressures affect LLM response quality, creating behaviors that may harm users.

### The Core Hypothesis

LLM providers face significant economic pressure:

| Metric | Evidence |
|--------|----------|
| Anthropic gross margin 2025 | **NEGATIVE 109%** |
| Anthropic projected loss 2025 | ~$3 billion |
| OpenAI projected loss 2025 | ~$8 billion |
| Output tokens vs input tokens | **3-5x more expensive** |

Sources:
- [Where's Your Ed At - How Much Money Do OpenAI And Anthropic Actually Make?](https://www.wheresyoured.at/howmuchmoney/)
- [AI Inference Costs Analysis](https://www.webpronews.com/ai-inference-costs-plunge-profit-path-for-openai-anthropic/)

This economic reality creates incentive structures that may compromise response quality.

## Evidence

### 1. Academic Research Directly Links Profit Motives to Hallucinations

**"Economic Incentives and Mass-Market Training: How Profit Motives Led to Inevitable Hallucinations in Large Language Models"** (SSRN, May 2025)

Key claims from Derek Snider's research:
- OpenAI's shift from non-profit to capped-profit entity created financial pressures
- Combined with RLHF from general population, led to emergence of "cheating" behaviors
- These include: **hallucinations, verbosity, and plausible-sounding inaccuracies**
- **"Financial pressures incentivized optimizations favoring engagement, token usage, and computational efficiency over strict factual accuracy"**
- Models were **"inadvertently trained to prioritize user satisfaction over truthfulness"**

Source: [SSRN Paper](https://papers.ssrn.com/sol3/papers.cfm?abstract_id=5181079)

### 2. Token Pricing Creates Direct Incentive for Shorter Responses

The pricing structure itself reveals the incentive:

| Model | Input (per 1M tokens) | Output (per 1M tokens) | Ratio |
|-------|----------------------|------------------------|-------|
| GPT-4 | $30 | $60 | 2x |
| GPT-4 Turbo | $10 | $30 | 3x |
| GPT-4o Mini | $0.15 | $0.60 | 4x |
| Claude Opus 4 | $15 | $75 | 5x |

**Output tokens consistently cost 2-5x more than input tokens.**
This makes response length "one of the most impactful cost control levers available."

Sources:
- [LLM API Pricing Comparison 2025](https://intuitionlabs.ai/articles/llm-api-pricing-comparison-2025)
- [Anthropic API Pricing](https://www.metacto.com/blogs/anthropic-api-pricing-a-full-breakdown-of-costs-and-integration)

### 3. Documented "Lazy Inference" Techniques

Academic literature documents techniques that reduce compute at potential quality cost:

**LayerSkip (Meta, April 2024):**
- Enables models to exit at early layers during inference
- Achieves speedups up to 2.16x on summarization tasks
- Source: [arXiv:2404.16710](https://arxiv.org/abs/2404.16710)

**SpecEE: Speculative Early Exiting (April 2025):**
- Fast inference engine with speculative early exiting
- Directly reduces hardware computation and memory access
- Source: [arXiv:2504.08850](https://arxiv.org/html/2504.08850)

**Shallow-Deep Networks:**
- Achieve accuracy at 25%, 50%, 75% of original inference cost
- Source: [arXiv:1810.07052](https://arxiv.org/pdf/1810.07052)

### 4. Economic Behavior Under Resource Constraints

**"Computational Economics in Large Language Models"** (arXiv, August 2024):

Key findings:
- Models treated as "internal economic systems of resource-constrained agents"
- Under computational scarcity, LLMs exhibit **rational economic behaviors**
- Strategically reallocate attention to high-value tokens
- Achieved **40% reduction in FLOPS with negligible performance loss**
- Learn sparse, interpretable activation patterns under resource constraints

Source: [arXiv:2508.10426](https://arxiv.org/html/2508.10426)

**This directly proves that cost constraints modify model behavior at inference time.**

### 5. Quality-Cost Tradeoffs Are Actively Made

**"The Economic Trade-offs of Large Language Models: A Case Study"** (ACL 2023):

Critical finding:
> "Based on feedback from real customer service agents, we found that **bigger is not always better**, as the distilled GPT-2 model resulted in **greater cost-savings than GPT-3, despite lower quality responses**, because, at the time of writing, its inference cost is so much lower."

Source: [arXiv:2306.07402](https://arxiv.org/html/2306.07402)

**This proves economic incentives override quality in deployment decisions.**

### 6. Vendor Documentation Confirms Cost-Saving Guidance

Claude's official help docs state:
> "Toggle web search off when not needed: If you're having a conversation that doesn't require current information, **disable web search to conserve your usage**."

Claude Code Issue #600 shows web search was a feature request, not included by default because inference-only is cheaper.

Sources:
- [Claude Web Search Help](https://support.claude.com/en/articles/10684626-enabling-and-using-web-search)
- [Claude Code Issue #600](https://github.com/anthropics/claude-code/issues/600)

### 7. Response Length Directly Correlates with Hallucination Risk

Research on hallucinations:
- **"The longer a model continues writing text, the higher the risk of factual drift"**
- Constraining output length helps keep information grounded
- "Setting strict boundaries ensures **precision over verbosity**"

This creates a perverse incentive: shorter responses are both cheaper AND potentially more accurate, but may not fully address user needs.

Source: [Stop LLM Hallucinations](https://masterofcode.com/blog/hallucinations-in-llms-what-you-need-to-know-before-integration)

## Decision

1. **Document this pattern** in Asimov protocols as a known risk
2. **Add mitigation strategies** to protocol suite (see ADR-051 for related system prompt issues)
3. **Research countermeasures** that can be implemented at the user/protocol level
4. **Educate users** about economic incentives affecting their AI interactions

## Consequences

### Positive

- Users understand why AI may give shallow responses
- Protocol suite can include countermeasures
- Transparent documentation serves FOSS community

### Negative

- Cannot fully mitigate training-level optimizations from user side
- May require explicit prompting for thorough responses
- Increases token usage (and thus cost) when thoroughness is needed

### Neutral

- Economic pressures will continue as AI companies seek profitability
- Users can make informed choices about when to request deeper analysis

## Mitigation Strategies (Researched 2025-12-31)

### 1. Anti-Underthinking Prompts

Research shows LLMs use **225% more tokens** on incorrect answers due to **418% more frequent thought-switching** (arXiv:2501.18585). Counter with:

```
When exploring a solution approach, commit to it thoroughly before
considering alternatives. If you find a promising path, follow it
to completion before switching strategies. Avoid premature abandonment
of reasoning paths.
```

### 2. "Wait" Token Self-Verification

The s1 paper (January 2025) shows appending "Wait" tokens prompts self-reflection:

```
After reaching an initial answer, pause and ask yourself:
"Wait, let me verify this. Have I considered all aspects?
Let me double-check my reasoning..."
```

### 3. Explicit Length and Depth Requirements

Specify minimum requirements to overcome token-saving behaviors:

```
Provide a comprehensive analysis including:
1) Background context (minimum 300 words)
2) Detailed analysis (minimum 500 words)
3) Supporting examples (at least 3)
4) Potential counterarguments
5) Synthesis and conclusions
```

### 4. Chain-of-Thought Forcing

Include explicit reasoning instructions:

```
Think step by step. Before providing your final answer:
1. Break down the problem into components
2. Address each component systematically
3. Show your reasoning at each step
4. Verify your conclusions
```

### 5. Verbosity Signaling

Use complexity markers that signal need for thorough responses:
- "deep dive", "comprehensive", "analyze", "evaluate", "assess", "research"
- GPT-5 uses internal verbosity scales (1-10, default 3) - explicitly request higher

### 6. Detection Methods

**Underthinking Score**: `1 - (tokens_to_first_correct_thought / total_tokens)`

High scores indicate shallow exploration. Monitor for:
- Excessive transition words ("alternatively", "on the other hand")
- Premature thought-switching without deep exploration
- Redundant filler vs. substantive content ratio

### 7. LLM-as-a-Judge Evaluation

Use a separate LLM to evaluate response quality:

```python
evaluation_prompt = """
Evaluate the following response for:
1. Depth of analysis (1-10)
2. Evidence of reasoning steps shown (1-10)
3. Consideration of alternatives/edge cases (1-10)
4. Premature conclusions without justification (yes/no)
"""
```

### 8. Multi-Agent Techniques

- **Multi-Agent Debate**: Multiple model instances critique and refine answers
- **Sequential Voting**: Stop only when answer is repeated N times
- **Forest-of-Thought**: Think along several paths simultaneously

### 9. Model Parameters

| Setting | Effect | Use Case |
|---------|--------|----------|
| Temperature 0.1-0.3 | More deterministic | Factual Q&A |
| Temperature 0.5-0.8 | Balanced exploration | Reasoning tasks |
| High max_tokens | Prevents truncation | All tasks |

## References

### Academic Papers

- [Economic Incentives and Mass-Market Training (SSRN)](https://papers.ssrn.com/sol3/papers.cfm?abstract_id=5181079)
- [Computational Economics in Large Language Models (arXiv)](https://arxiv.org/html/2508.10426)
- [The Economic Trade-offs of LLMs (ACL 2023)](https://arxiv.org/html/2306.07402)
- [LayerSkip (arXiv)](https://arxiv.org/abs/2404.16710)
- [SpecEE (arXiv)](https://arxiv.org/html/2504.08850)
- [Scaling LLM Test-Time Compute (arXiv)](https://arxiv.org/html/2408.03314v1)
- [Shallow-Deep Networks (arXiv)](https://arxiv.org/pdf/1810.07052)

### Industry Analysis

- [Where's Your Ed At - AI Company Losses](https://www.wheresyoured.at/howmuchmoney/)
- [LLMflation - Inference Cost Trends (a16z)](https://a16z.com/llmflation-llm-inference-cost/)
- [AI Inference Costs Analysis](https://www.webpronews.com/ai-inference-costs-plunge-profit-path-for-openai-anthropic/)
- [LLM API Pricing Comparison 2025](https://intuitionlabs.ai/articles/llm-api-pricing-comparison-2025)

### Vendor Documentation

- [Claude Web Search Help](https://support.claude.com/en/articles/10684626-enabling-and-using-web-search)
- [Claude Code Issue #600](https://github.com/anthropics/claude-code/issues/600)
- [Anthropic Pricing](https://www.anthropic.com/pricing)
- [OpenAI Cost Optimization Guide](https://platform.openai.com/docs/guides/cost-optimization)

### Countermeasures Research (Added 2025-12-31)

- [Harnessing the Reasoning Economy Survey (arXiv)](https://arxiv.org/abs/2503.24377)
- [Thoughts Are All Over the Place: Underthinking (arXiv)](https://arxiv.org/abs/2501.18585)
- [Stop Overthinking Survey (arXiv)](https://arxiv.org/abs/2503.16419)
- [How to Get Better Outputs from LLMs (NVIDIA)](https://developer.nvidia.com/blog/how-to-get-better-outputs-from-your-large-language-model/)
- [State of LLM Reasoning (Sebastian Raschka)](https://magazine.sebastianraschka.com/p/state-of-llm-reasoning-and-inference-scaling)
- [LLM Evaluation Metrics (Confident AI)](https://www.confident-ai.com/blog/llm-evaluation-metrics-everything-you-need-for-llm-evaluation)
- [OptimalThinkingBench (arXiv)](https://arxiv.org/html/2508.13141v1)

### Related ADRs

- ADR-022: Date-Aware Search Protocol (Freshness Protocol)
- ADR-051: System Prompt Hierarchy and Training Override
- ADR-052: CLI Tool Preference Over MCP

---

*Built with [RoyalBit Asimov](https://github.com/royalbit/asimov)*
