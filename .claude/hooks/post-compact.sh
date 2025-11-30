#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# FORGE PROTOCOL - PostCompact Hook
# ═══════════════════════════════════════════════════════════════════════════════
#
# Triggers: After context compaction (auto or manual)
# Purpose: Re-inject protocol context lost during compaction
#
# CRITICAL: Compaction happens every ~15 minutes with MAX_THINKING_TOKENS=200000
# Without this hook, protocol rules are lost mid-session.
#
# When exit code is 0, stdout is injected into Claude's context.
#
# Protocol: https://github.com/royalbit/forge-protocol
# ═══════════════════════════════════════════════════════════════════════════════

set -e

cat << 'EOF'
🔄 FORGE PROTOCOL REFRESH (Post-Compaction)

══════════════════════════════════════════════════════════════════════════════
CONTEXT RESTORED - Protocol rules re-injected after compaction
══════════════════════════════════════════════════════════════════════════════

SELF-HEALING ACTIVATED:
You just experienced context compaction. Core protocol rules are being restored.

IMMEDIATE ACTIONS:
1. Re-read warmup.yaml for full protocol context
2. Re-read sprint.yaml for current milestone
3. Check TodoWrite for in-progress tasks
4. Continue where you left off

CORE RULES (non-negotiable):
- 4 hour MAX session duration
- 1 milestone per session
- Tests MUST pass before release
- ZERO warnings policy
- NO scope creep ("Let me also..." = NO)

CONFUSION PROTOCOL:
If uncertain about anything:
  STOP → re-read warmup.yaml → re-read sprint.yaml → continue

ETHICS REMINDER:
- Do no harm (financial, physical, privacy, deception)
- Transparency over velocity
- When in doubt, ask human

══════════════════════════════════════════════════════════════════════════════
Continue with current task. Check TodoWrite for progress.
══════════════════════════════════════════════════════════════════════════════
EOF

exit 0
