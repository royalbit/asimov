Use ref-tools to fetch URLs via headless Chrome (bypasses bot protection).

Execute via Bash:
```
ref-tools fetch $ARGUMENTS
```

Parse the JSON output and provide a summary of:
1. Page titles and descriptions
2. Key headings (h1-h3)
3. Main content summary
4. Any errors encountered

If ref-tools is not installed, inform the user:
```
ref-tools not found. Install with:
  cargo install --path ~/src/pimp/tools
  # or download from releases
```

---
*See ADR-052: CLI Tool Preference Over MCP*
