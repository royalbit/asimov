# Migrations Protocol - Functional Equivalence (Priority 2)
# Hardcoded in binary. Cannot be bypassed.
# Date: {TODAY}

principle: "Migration complete = functionally equivalent, not just compiles"

strategies:
  test_parity:
    description: "Same tests must pass on both source and target"
  contract_testing:
    description: "API contracts must be preserved exactly"
  behavioral_snapshots:
    description: "Capture I/O pairs, verify target produces same output"
  shadow_mode:
    description: "Run both in parallel, compare results"

quality_gates:
  before: ["source tests pass", "contracts documented", "snapshots captured"]
  during: ["incremental verification", "no silent failures"]
  after: ["full test parity", "shadow mode passes", "performance within bounds"]

red_flags:
  - "Skipping tests for speed"
  - "Assuming compilation = correctness"
  - "Silent behavior changes"
  - "Missing edge case coverage"

motto: "Same inputs, same outputs. Always."
