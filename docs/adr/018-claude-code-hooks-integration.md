# ADR-018: Claude Code Hooks Integration

## Status

**Accepted** - 2025-11-29

## Context

Forge Protocol relies on CLAUDE.md `@import` syntax to load protocol files (warmup.yaml,
ethics.yaml, green.yaml) into Claude's context. However, this approach has critical gaps:

1. **No Auto-Start**: `@import` loads content as static context but doesn't trigger any
   automatic execution. Users must manually say "run warmup" to initialize the protocol.

2. **Post-Compaction Loss**: Context compaction happens every ~15 minutes (with
   MAX_THINKING_TOKENS=200000). After compaction, protocol rules may be summarized away,
   leaving Claude operating without ethical constraints or sprint boundaries.

3. **No Recovery Mechanism**: The self-healing directive in warmup.yaml
   (`on_confusion: "STOP → re-read warmup.yaml"`) only works if Claude remembers it exists.
   After aggressive compaction, even this instruction may be lost.

### Research: Competitor Analysis

Investigation of major AI coding assistants (November 2025) revealed:

| AI | Session Init | Post-Compact |
|----|-------------|--------------|
| **Claude Code** | SessionStart hook | SessionStart (compact matcher) |
| Cursor | .cursorrules (static) | /summarize (manual) |
| GitHub Copilot | .github/copilot-instructions.md | None |
| Windsurf | .windsurfrules + Memories | None |
| Gemini Code Assist | Context Drawer + MCP | None |
| Grok | Prompt caching | None |

**Finding**: Claude Code's lifecycle hooks are unique. No other AI coding assistant provides
hooks for session initialization or post-compaction recovery.

## Decision

Implement Claude Code hooks to enable true autonomous operation:

### 1. SessionStart Hook

**File**: `.claude/hooks/session-start.sh`

**Triggers**: `startup`, `resume`, `clear`

**Behavior**:
- Outputs protocol initialization message
- Instructs Claude to read roadmap.yaml, sprint.yaml
- Presents next milestone
- Waits for user confirmation ("go")

### 2. PostCompact Hook

**File**: `.claude/hooks/post-compact.sh`

**Triggers**: `compact` (via SessionStart matcher)

**Behavior**:
- Outputs protocol refresh message
- Re-injects core rules (4hr max, 1 milestone, tests pass)
- Instructs Claude to re-read warmup.yaml, sprint.yaml
- Reminds to check TodoWrite for in-progress tasks
- Includes ethics reminder

### 3. Configuration

**File**: `.claude/hooks.json`

```json
{
  "hooks": {
    "SessionStart": [
      {
        "matcher": "startup",
        "hooks": [{ "type": "command", "command": ".claude/hooks/session-start.sh" }]
      },
      {
        "matcher": "resume",
        "hooks": [{ "type": "command", "command": ".claude/hooks/session-start.sh" }]
      },
      {
        "matcher": "clear",
        "hooks": [{ "type": "command", "command": ".claude/hooks/session-start.sh" }]
      },
      {
        "matcher": "compact",
        "hooks": [{ "type": "command", "command": ".claude/hooks/post-compact.sh" }]
      }
    ]
  }
}
```

## Consequences

### Positive

1. **True Auto-Start**: Protocol initializes automatically on every session without manual
   "run warmup" command.

2. **Mid-Session Recovery**: PostCompact hook ensures protocol rules survive compaction.
   This is the missing piece for true SKYNET MODE autonomy.

3. **Unique Differentiator**: No other AI coding assistant has this capability. Forge
   Protocol on Claude Code offers autonomous operation that competitors cannot match.

4. **Self-Healing Complete**: Combined with v4.1.5 file auto-regeneration, the protocol
   now has defense-in-depth:
   - Missing files → auto-regenerate (v4.1.5)
   - Session start → auto-initialize (this ADR)
   - Post-compaction → auto-recover (this ADR)

### Negative

1. **Claude Code Exclusive**: This feature only works with Claude Code. Other AI assistants
   cannot use lifecycle hooks (they only support static rules files).

2. **User Must Accept Hooks**: Claude Code requires manual review in `/hooks` menu before
   hook changes take effect. Users must explicitly approve the hooks.

3. **30-Second Timeout**: Hooks have execution limits. Complex initialization must be
   kept simple.

### Vendor Neutrality Impact

Forge Protocol remains vendor-neutral at the **file format** level:
- warmup.yaml, ethics.yaml, sprint.yaml, green.yaml work anywhere as static context

But **autonomous operation** (SKYNET MODE) requires Claude Code:
- SessionStart hook for auto-initialization
- PostCompact hook for mid-session recovery

This is acceptable because:
1. File-based protocols still provide value on all platforms
2. Full autonomy is opt-in, not required
3. MCP Server Mode (v4.3.0) will provide alternative integration path

## Implementation

### Files Created

```
.claude/
├── hooks.json           # Hook configuration
└── hooks/
    ├── session-start.sh # SessionStart hook (startup, resume, clear)
    └── post-compact.sh  # PostCompact hook (after compaction)
```

### User Activation

After cloning/updating the repo, users must:

1. Review hooks: Run `/hooks` in Claude Code
2. Accept the hooks configuration
3. Restart session for hooks to take effect

### Testing

Hooks can be tested manually:

```bash
# Test SessionStart hook
.claude/hooks/session-start.sh
# Should output protocol initialization message

# Test PostCompact hook
.claude/hooks/post-compact.sh
# Should output protocol refresh message
```

## References

- [Claude Code Hooks Documentation](https://docs.anthropic.com/claude-code/hooks)
- [ADR-003: Compaction Reality](docs/adr/003-compaction-reality.md)
- [ADR-017: Protocol Self-Healing](docs/adr/017-protocol-self-healing.md)
- Nature 2025: AI Sycophancy Research
