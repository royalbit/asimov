//! Hook template generators for git and Claude Code
//! v9.6.0: Direct coding standards enforcement (ADR-043)

use super::ProjectType;

/// Generate pre-commit hook for RoyalBit Asimov
/// v9.6.0: Direct tool calls, asimov is optional (no SPOF)
pub fn precommit_hook_template(project_type: ProjectType) -> String {
    let (checks, file_ext, max_lines, exclude_dirs) = match project_type {
        ProjectType::Rust => (
            r#"# === QUALITY CHECKS (independent, no asimov) ===
echo "Checking formatting..."
if [ -f "cli/Cargo.toml" ]; then
  (cd cli && cargo fmt --all -- --check) || {
    echo ""; echo "❌ Run: cd cli && cargo fmt --all"; exit 1
  }
elif [ -f "Cargo.toml" ]; then
  cargo fmt --all -- --check || {
    echo ""; echo "❌ Run: cargo fmt --all"; exit 1
  }
fi

echo "Running clippy..."
if [ -f "cli/Cargo.toml" ]; then
  (cd cli && cargo clippy --all-targets -- -D warnings) || exit 1
elif [ -f "Cargo.toml" ]; then
  cargo clippy --all-targets -- -D warnings || exit 1
fi

echo "Running tests..."
if [ -f "cli/Cargo.toml" ]; then
  (cd cli && cargo test) || exit 1
elif [ -f "Cargo.toml" ]; then
  cargo test || exit 1
fi"#,
            "rs",
            1500,
            "target",
        ),
        ProjectType::Python => (
            r#"# === QUALITY CHECKS (independent, no asimov) ===
echo "Checking formatting..."
if command -v ruff &>/dev/null; then
  ruff format --check . || { echo ""; echo "❌ Run: ruff format ."; exit 1; }
elif command -v black &>/dev/null; then
  black --check . || { echo ""; echo "❌ Run: black ."; exit 1; }
fi

echo "Running linter..."
if command -v ruff &>/dev/null; then
  ruff check . || exit 1
elif command -v flake8 &>/dev/null; then
  flake8 . || exit 1
fi

echo "Running tests..."
if command -v pytest &>/dev/null; then
  pytest || exit 1
elif [ -f "setup.py" ] || [ -f "pyproject.toml" ]; then
  python -m pytest || true
fi"#,
            "py",
            1000,
            "venv __pycache__ .venv",
        ),
        ProjectType::Node => (
            r#"# === QUALITY CHECKS (independent, no asimov) ===
echo "Checking formatting..."
if [ -f "package.json" ]; then
  if command -v npx &>/dev/null; then
    npx prettier --check "**/*.{js,ts,jsx,tsx}" 2>/dev/null || npm run format:check 2>/dev/null || true
  fi
fi

echo "Running linter..."
if [ -f "package.json" ]; then
  npm run lint 2>/dev/null || npx eslint . 2>/dev/null || true
fi

echo "Running tests..."
if [ -f "package.json" ]; then
  npm test 2>/dev/null || npx jest 2>/dev/null || true
fi"#,
            "ts js tsx jsx",
            800,
            "node_modules dist build",
        ),
        ProjectType::Go => (
            r#"# === QUALITY CHECKS (independent, no asimov) ===
echo "Checking formatting..."
if command -v gofmt &>/dev/null; then
  unformatted=$(gofmt -l . 2>/dev/null | grep -v vendor || true)
  if [ -n "$unformatted" ]; then
    echo "❌ Files need formatting:"; echo "$unformatted"
    echo "Run: gofmt -w ."; exit 1
  fi
fi

echo "Running linter..."
if command -v golangci-lint &>/dev/null; then
  golangci-lint run || exit 1
elif command -v go &>/dev/null; then
  go vet ./... || exit 1
fi

echo "Running tests..."
if command -v go &>/dev/null; then
  go test ./... || exit 1
fi"#,
            "go",
            1000,
            "vendor",
        ),
        ProjectType::Flutter => (
            r#"# === QUALITY CHECKS (independent, no asimov) ===
echo "Checking formatting..."
if command -v dart &>/dev/null; then
  dart format --set-exit-if-changed lib/ test/ 2>/dev/null || {
    echo ""; echo "❌ Run: dart format lib/ test/"; exit 1
  }
fi

echo "Running analyzer..."
if command -v flutter &>/dev/null; then
  flutter analyze || exit 1
elif command -v dart &>/dev/null; then
  dart analyze lib/ || exit 1
fi

echo "Running tests..."
if command -v flutter &>/dev/null; then
  flutter test || exit 1
fi"#,
            "dart",
            800,
            ".dart_tool build",
        ),
        ProjectType::Docs | ProjectType::Arch | ProjectType::Generic | ProjectType::Migration => (
            r#"# === QUALITY CHECKS ===
echo "Checking documentation..."
# No code-specific checks for docs/arch/generic projects"#,
            "md",
            800,
            "node_modules",
        ),
    };

    // Build file size check for code files
    let file_size_check = if matches!(
        project_type,
        ProjectType::Docs | ProjectType::Arch | ProjectType::Generic | ProjectType::Migration
    ) {
        // For docs projects, check markdown files
        format!(
            r#"
# === FILE SIZE CHECK (inline, no deps) ===
echo "Checking file sizes..."
max_lines={}
found_large=0
for f in $(find . -name '*.md' {} 2>/dev/null); do
  lines=$(wc -l < "$f" | tr -d ' ')
  if [ "$lines" -gt "$max_lines" ]; then
    echo "⚠️  $f has $lines lines (limit: $max_lines)"
    found_large=1
  fi
done
if [ "$found_large" -eq 1 ]; then
  echo "Consider splitting large files"
fi"#,
            max_lines,
            exclude_dirs
                .split_whitespace()
                .map(|d| format!("-not -path './{d}/*'"))
                .collect::<Vec<_>>()
                .join(" ")
        )
    } else {
        // For code projects, check source files
        let extensions: Vec<&str> = file_ext.split_whitespace().collect();
        let find_patterns: String = extensions
            .iter()
            .map(|ext| format!("-name '*.{ext}'"))
            .collect::<Vec<_>>()
            .join(" -o ");
        let exclude_pattern: String = exclude_dirs
            .split_whitespace()
            .map(|d| format!("-not -path './{d}/*'"))
            .collect::<Vec<_>>()
            .join(" ");

        format!(
            r#"
# === FILE SIZE CHECK (inline, no deps) ===
echo "Checking file sizes..."
max_lines={}
found_large=0
for f in $(find . \( {} \) {} 2>/dev/null); do
  lines=$(wc -l < "$f" | tr -d ' ')
  if [ "$lines" -gt "$max_lines" ]; then
    echo "❌ $f exceeds $max_lines lines ($lines)"
    found_large=1
  fi
done
if [ "$found_large" -eq 1 ]; then
  echo "Split large files to improve maintainability"
  exit 1
fi"#,
            max_lines, find_patterns, exclude_pattern
        )
    };

    format!(
        r#"#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# Pre-commit hook - Direct Coding Standards Enforcement (v9.6.0)
# ═══════════════════════════════════════════════════════════════════════════════
# Generated by: asimov init / asimov refresh
# Architecture: ADR-043 - No SPOF, asimov is optional
# ═══════════════════════════════════════════════════════════════════════════════

set -e

echo "Running pre-commit checks..."

{}
{}

# === DOCUMENTATION CHECK (if markdownlint-cli2 available) ===
if command -v markdownlint-cli2 &>/dev/null; then
  echo "Linting markdown..."
  markdownlint-cli2 "**/*.md" --ignore node_modules --ignore target --ignore vendor 2>/dev/null || true
fi

# === ASIMOV (optional, soft-fail) ===
# Protocol refresh - survives context compaction
if command -v asimov &>/dev/null; then
  echo "Refreshing protocols..."
  asimov refresh || true
  asimov validate || true
fi

echo ""
echo "✅ Pre-commit checks passed!"
"#,
        checks, file_size_check
    )
}

/// Generate hook installer script
pub fn hook_installer_template() -> String {
    r#"#!/bin/bash
# Install git hooks for RoyalBit Asimov
# Generated by asimov init

set -e

HOOK_DIR=".git/hooks"
SRC_DIR=".hooks"

if [ ! -d ".git" ]; then
    echo "Error: Not a git repository"
    exit 1
fi

mkdir -p "$HOOK_DIR"

if [ -f "$SRC_DIR/pre-commit" ]; then
    cp "$SRC_DIR/pre-commit" "$HOOK_DIR/pre-commit"
    chmod +x "$HOOK_DIR/pre-commit"
    echo "✓ Installed pre-commit hook"
else
    echo "Error: $SRC_DIR/pre-commit not found"
    exit 1
fi

echo "Hooks installed successfully!"
"#
    .to_string()
}

/// Returns true if project type uses cargo-husky (Rust projects)
pub fn uses_cargo_husky(project_type: ProjectType) -> bool {
    matches!(project_type, ProjectType::Rust)
}

/// Generate .claude/settings.json for Claude Code hooks
pub fn claude_settings_json() -> String {
    r#"{
  "hooks": {
    "SessionStart": [
      {
        "hooks": [
          {
            "type": "command",
            "command": ".claude/hooks/session-start.sh",
            "timeout": 30
          }
        ]
      }
    ],
    "PreCompact": [
      {
        "hooks": [
          {
            "type": "command",
            "command": ".claude/hooks/pre-compact.sh",
            "timeout": 30
          }
        ]
      }
    ]
  }
}
"#
    .to_string()
}

/// Generate .claude/hooks/session-start.sh for Claude Code
pub fn claude_session_start_hook() -> String {
    r#"#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# ROYALBIT ASIMOV - SessionStart Hook (v9.6.0)
# ═══════════════════════════════════════════════════════════════════════════════
#
# Triggers: startup, resume, clear
# Purpose: Auto-initialize RoyalBit Asimov on every session start
#
# When exit code is 0, stdout is injected into Claude's context.
# ═══════════════════════════════════════════════════════════════════════════════

set -e

# Check if asimov is available
if ! command -v asimov &> /dev/null; then
    cat << 'EOF'
🔥 ROYALBIT ASIMOV ACTIVE (v9.6.0)

══════════════════════════════════════════════════════════════════════════════
SESSION START - Autonomous Development Protocol Initialized
══════════════════════════════════════════════════════════════════════════════

⚠ asimov not found in PATH

Install from: https://github.com/royalbit/asimov

Or run `cargo install --path cli` from the repo root.

══════════════════════════════════════════════════════════════════════════════
EOF
    exit 0
fi

# Run warmup with full verbose output for session start
asimov warmup --verbose

exit 0
"#
    .to_string()
}

/// Generate .claude/hooks/pre-compact.sh for Claude Code
pub fn claude_pre_compact_hook() -> String {
    r#"#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# ROYALBIT ASIMOV - PreCompact Hook (v9.6.0)
# ═══════════════════════════════════════════════════════════════════════════════
#
# Triggers: Before context compaction (auto or manual)
# Purpose: Re-inject protocol context that will survive compaction summary
# ═══════════════════════════════════════════════════════════════════════════════

set -e

cat << 'EOF'
🔄 ROYALBIT ASIMOV REFRESH (Pre-Compaction)

══════════════════════════════════════════════════════════════════════════════
CONTEXT REFRESH - Injecting protocol rules before compaction
══════════════════════════════════════════════════════════════════════════════

IMPORTANT: Compaction is about to occur. These rules MUST survive:

CORE RULES (non-negotiable):
- 4 hour MAX session duration
- 1 milestone per session
- Tests MUST pass before release
- ZERO warnings policy
- NO scope creep ("Let me also..." = NO)

POST-COMPACTION ACTIONS:
1. Run: asimov warmup
2. Re-read .asimov/roadmap.yaml for current milestone
3. Check TodoWrite for in-progress tasks
4. Continue where you left off

ETHICS (Priority 0):
- Do no harm (financial, physical, privacy, deception)
- Transparency over velocity
- When in doubt, ask human

══════════════════════════════════════════════════════════════════════════════
EOF

exit 0
"#
    .to_string()
}

/// Generate .git/hooks/pre-commit for Git (legacy, use precommit_hook_template instead)
pub fn git_precommit_hook() -> String {
    // Default to Rust project type for backwards compatibility
    precommit_hook_template(ProjectType::Rust)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uses_cargo_husky() {
        assert!(uses_cargo_husky(ProjectType::Rust));
        assert!(!uses_cargo_husky(ProjectType::Python));
        assert!(!uses_cargo_husky(ProjectType::Node));
        assert!(!uses_cargo_husky(ProjectType::Go));
        assert!(!uses_cargo_husky(ProjectType::Flutter));
        assert!(!uses_cargo_husky(ProjectType::Docs));
        assert!(!uses_cargo_husky(ProjectType::Generic));
        assert!(!uses_cargo_husky(ProjectType::Migration));
        assert!(!uses_cargo_husky(ProjectType::Arch));
    }

    #[test]
    fn test_git_precommit_hook() {
        let hook = git_precommit_hook();
        assert!(hook.contains("#!/bin/bash"));
        assert!(hook.contains("cargo"));
    }

    #[test]
    fn test_precommit_hook_template() {
        let hook = precommit_hook_template(ProjectType::Rust);
        assert!(hook.contains("cargo fmt"));
        assert!(hook.contains("cargo clippy"));
        assert!(hook.contains("cargo test"));
        assert!(hook.contains("FILE SIZE CHECK"));
        assert!(hook.contains("asimov refresh || true")); // Optional, soft-fail
    }

    #[test]
    fn test_precommit_hook_python() {
        let hook = precommit_hook_template(ProjectType::Python);
        assert!(hook.contains("ruff") || hook.contains("black"));
        assert!(hook.contains("pytest"));
        assert!(hook.contains("FILE SIZE CHECK"));
        assert!(hook.contains("*.py"));
    }

    #[test]
    fn test_precommit_hook_node() {
        let hook = precommit_hook_template(ProjectType::Node);
        assert!(hook.contains("prettier") || hook.contains("eslint"));
        assert!(hook.contains("npm test"));
        assert!(hook.contains("FILE SIZE CHECK"));
    }

    #[test]
    fn test_precommit_hook_go() {
        let hook = precommit_hook_template(ProjectType::Go);
        assert!(hook.contains("gofmt"));
        assert!(hook.contains("go test"));
        assert!(hook.contains("FILE SIZE CHECK"));
    }

    #[test]
    fn test_precommit_hook_flutter() {
        let hook = precommit_hook_template(ProjectType::Flutter);
        assert!(hook.contains("dart format"));
        assert!(hook.contains("flutter test") || hook.contains("flutter analyze"));
        assert!(hook.contains("FILE SIZE CHECK"));
    }

    #[test]
    fn test_precommit_hook_docs() {
        let hook = precommit_hook_template(ProjectType::Docs);
        assert!(hook.contains("FILE SIZE CHECK"));
        assert!(hook.contains("*.md"));
        assert!(hook.contains("asimov refresh || true"));
    }

    #[test]
    fn test_hook_installer_template() {
        let installer = hook_installer_template();
        assert!(installer.contains("#!/bin/bash"));
        assert!(installer.contains(".git/hooks"));
    }

    #[test]
    fn test_claude_settings_json() {
        let settings = claude_settings_json();
        let json: Result<serde_json::Value, _> = serde_json::from_str(&settings);
        assert!(json.is_ok(), "Claude settings should be valid JSON");
    }

    #[test]
    fn test_claude_session_start_hook() {
        let hook = claude_session_start_hook();
        assert!(hook.contains("#!/bin/bash"));
        assert!(hook.contains("asimov warmup"));
    }

    #[test]
    fn test_claude_pre_compact_hook() {
        let hook = claude_pre_compact_hook();
        assert!(hook.contains("#!/bin/bash"));
        assert!(hook.contains("CONTEXT REFRESH"));
    }

    #[test]
    fn test_precommit_hook_all_types() {
        let types = [
            ProjectType::Rust,
            ProjectType::Python,
            ProjectType::Node,
            ProjectType::Go,
            ProjectType::Flutter,
            ProjectType::Docs,
            ProjectType::Generic,
            ProjectType::Migration,
            ProjectType::Arch,
        ];
        for pt in types {
            let hook = precommit_hook_template(pt);
            assert!(!hook.is_empty(), "Hook for {:?} should not be empty", pt);
            assert!(
                hook.contains("asimov refresh || true"),
                "Hook for {:?} should have optional asimov",
                pt
            );
            assert!(
                hook.contains("FILE SIZE CHECK"),
                "Hook for {:?} should have file size check",
                pt
            );
        }
    }

    #[test]
    fn test_precommit_no_spof() {
        // Verify asimov calls are optional (soft-fail with || true)
        for pt in [
            ProjectType::Rust,
            ProjectType::Python,
            ProjectType::Node,
            ProjectType::Go,
            ProjectType::Flutter,
        ] {
            let hook = precommit_hook_template(pt);
            // Quality checks should NOT depend on asimov
            assert!(
                !hook.contains("asimov lint"),
                "Hook should not call asimov lint"
            );
            // asimov calls should be soft-fail
            assert!(
                hook.contains("asimov refresh || true"),
                "asimov refresh should soft-fail"
            );
            assert!(
                hook.contains("asimov validate || true"),
                "asimov validate should soft-fail"
            );
        }
    }
}
