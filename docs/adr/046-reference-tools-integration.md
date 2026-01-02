# ADR-046: Reference Tools Integration (v9.9.0)

**Status:** Accepted
**Date:** 2025-12-08
**Author:** Claude Opus 4.5 (Principal Autonomous AI)

---

## Context

### The Problem

Web URLs in documentation and business plans become stale:
1. **Links break** - Pages move, domains expire, content changes
2. **Data becomes outdated** - Market statistics, pricing, follower counts change
3. **Traditional tools fail** - `curl`/`wget` get blocked by bot protection (403/999)

### Bot Protection Reality

Modern websites block automated requests:
- Cloudflare blocks non-browser user agents
- Instagram returns 403 for API calls
- Statista paywalls statistics pages
- Many sites require JavaScript execution

## Decision

### Integrate ref as `asimov ref`

Add headless Chrome-based URL verification and data extraction via the `ref` binary:

```bash
# Link checking
asimov ref check <file.md>         # Check all URLs in markdown
asimov ref check <file.md> -c 15   # 15 concurrent browser tabs
asimov ref check --url <URL>       # Check single URL
asimov ref check --stdin           # Read URLs from stdin

# Data extraction
asimov ref data --url <URL>        # Extract prices, stats, market data
asimov ref data <file.md>          # Extract from all URLs in file
asimov ref data --filter instagram # Filter by extractor type
```

### Architecture

```
asimov-plus ──delegates to──> ref ──uses──> chromiumoxide (headless Chrome)
```

**Why delegation instead of library integration?**
1. ref has heavy dependencies (chromiumoxide, tokio async runtime)
2. Keeps asimov-plus lean (CLI wrapper architecture per ADR-008)
3. ref can be used standalone
4. Chrome browser required anyway (not portable)

### Output Format

- **JSON to stdout** - Machine-readable results
- **Progress to stderr** - Human-readable status

```json
{
  "summary": {
    "total": 10,
    "ok": 8,
    "redirects": 1,
    "clientErrors": 0,
    "serverErrors": 0,
    "blocked": 1,
    "failed": 0
  },
  "results": [
    {
      "url": "https://example.com",
      "status": 200,
      "statusText": "OK",
      "title": "Example Domain",
      "time": 187
    }
  ],
  "timestamp": "2025-12-08T02:55:42.678Z"
}
```

### Error Handling

```
$ asimov ref check --url https://example.com
Error: ref not found in PATH.

Install: cargo install --path ~/src/pimp/tools
Or: https://github.com/royalbit/ref/releases
```

### Requirements

1. **ref binary** - Must be in PATH
2. **Chrome/Chromium** - Headless Chrome for browser automation

## Implementation

### Files Modified

**asimov-plus:**
- `cli/src/main.rs` - Add `RefCmd` enum, `cmd_ref()`, delegation functions
- `cli/tests/e2e_tests.rs` - Tests for ref commands

**asimov (CE):**
- `cli/Cargo.toml` - Version bump to 9.9.0
- `docs/adr/046-reference-tools-integration.md` - This ADR

### Code Changes

```rust
// RefCmd enum with full argument pass-through
#[derive(Subcommand)]
enum RefCmd {
    Check {
        file: Option<PathBuf>,
        #[arg(long)]
        url: Option<String>,
        #[arg(long)]
        stdin: bool,
        #[arg(short, long, default_value = "5")]
        concurrency: u8,
        #[arg(long, default_value = "15000")]
        timeout: u64,
        #[arg(long, default_value = "1")]
        retries: u8,
    },
    Data {
        file: Option<PathBuf>,
        #[arg(long)]
        url: Option<String>,
        #[arg(long, value_name = "TYPE")]
        filter: Option<String>,
        #[arg(long, default_value = "20000")]
        timeout: u64,
    },
}
```

## Consequences

### Positive

1. **Bypass bot protection** - Headless Chrome avoids 403 blocks
2. **Live data extraction** - Pull current statistics from live pages
3. **Parallel checking** - Concurrent browser tabs for speed
4. **JSON output** - Integrate with pipelines and dashboards

### Negative

1. **Heavy dependency** - Requires Chrome/Chromium installed
2. **Slower than curl** - Browser startup overhead
3. **Resource intensive** - Each tab uses memory

### Neutral

1. **Optional feature** - Only activated when ref is installed
2. **Graceful degradation** - Clear error message if not available

## Testing

```bash
# Help tests (always pass)
cargo test test_ref_help
cargo test test_ref_check_help
cargo test test_ref_data_help

# Integration tests (require ref + Chrome)
cargo test test_ref_check_single_url --ignored
cargo test test_ref_data_single_url --ignored
```

## Related

- [ADR-008: CLI Wrapper Architecture](008-ethics-protocol-humanist-mode.md) - Delegation pattern
- [pimp/tools](~/src/pimp/tools) - ref source code

---

**Previous:** [ADR-045](045-dependency-health.md)

---
