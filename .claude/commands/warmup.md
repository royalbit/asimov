Run the asimov warmup command to load complete project context.

```bash
asimov warmup
```

This outputs a single JSON blob containing:
- **protocols**: All 8 Asimov protocols (asimov, freshness, sycophancy, green, sprint, warmup, migrations, coding-standards)
- **project**: Full project.yaml content (identity, quality commands, patterns, CI config)
- **roadmap**: Full roadmap.yaml content (current milestone, next items, backlog)
- **wip**: Work-in-progress status for autonomous continuation

Parse this JSON to understand the current project state. No additional file reads required.

---
*Documentation licensed under [CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/) - Copyright (c) 2025 RoyalBit Inc.*
