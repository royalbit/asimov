//! Template generators for Forge Protocol files

use std::fmt;

/// Supported project types for template generation
#[derive(Debug, Clone, Copy, Default)]
pub enum ProjectType {
    #[default]
    Generic,
    Rust,
}

impl fmt::Display for ProjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProjectType::Generic => write!(f, "generic"),
            ProjectType::Rust => write!(f, "rust"),
        }
    }
}

impl std::str::FromStr for ProjectType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "generic" => Ok(ProjectType::Generic),
            "rust" => Ok(ProjectType::Rust),
            _ => Err(format!(
                "Unknown project type: '{}'. Available: generic, rust",
                s
            )),
        }
    }
}

/// Generate a starter warmup.yaml template
pub fn warmup_template(project_name: &str, project_type: ProjectType) -> String {
    match project_type {
        ProjectType::Generic => warmup_generic(project_name),
        ProjectType::Rust => warmup_rust(project_name),
    }
}

fn warmup_generic(project_name: &str) -> String {
    format!(
        r#"# Forge Protocol - Session Bootstrap
# https://github.com/royalbit/forge-protocol

identity:
  project: "{}"
  tagline: "Brief project description"
  version: "0.1.0"

mission:
  problem: "What problem does this solve?"
  solution: "How does it solve it?"
  principles:
    - "Principle one"
    - "Principle two"

# ═══════════════════════════════════════════════════════════════════════════════
# GREEN CODING - Zero tokens. Zero emissions.
# ═══════════════════════════════════════════════════════════════════════════════
green_coding:
  philosophy: "Local-first tools over cloud AI for routine tasks"
  practices:
    - "Use CLI tools for validation, linting, formatting"
    - "Reserve AI for complex reasoning tasks"
    - "Prefer compiled languages or efficient runtimes"
    - "Minimize dependencies and binary sizes"
  why:
    - "Local validation: $0 and ~0.002g CO₂"
    - "Cloud AI validation: $0.02+ and ~0.5g CO₂"
    - "99.6% carbon reduction with local tools"

files:
  source:
    - "src/ - Source code"
  config:
    - "Configuration files"
  docs:
    - "README.md - Documentation"

session:
  start:
    - "Read warmup.yaml"
    - "git status"
  during:
    - "Track progress"
    - "Test frequently"
  end:
    - "Run tests"
    - "Update documentation"

quality:
  tests: "All tests must pass"
  lint: "Run linter"

style:
  code:
    - "Follow project conventions"
  docs:
    - "Keep documentation concise"
"#,
        project_name
    )
}

fn warmup_rust(project_name: &str) -> String {
    format!(
        r#"# Forge Protocol - Session Bootstrap
# https://github.com/royalbit/forge-protocol

identity:
  project: "{}"
  tagline: "Brief project description"
  version: "0.1.0"

mission:
  problem: "What problem does this solve?"
  solution: "How does it solve it?"
  principles:
    - "Principle one"
    - "Principle two"

# ═══════════════════════════════════════════════════════════════════════════════
# GREEN CODING - Zero tokens. Zero emissions.
# ═══════════════════════════════════════════════════════════════════════════════
green_coding:
  philosophy: "Local-first tools over cloud AI for routine tasks"
  practices:
    - "Use cargo test/clippy/fmt for validation (not AI)"
    - "Reserve AI for complex reasoning tasks"
    - "UPX compress release binaries (70%+ smaller)"
    - "Enable LTO and strip symbols in release profile"
  why:
    - "Rust: Zero runtime, minimal memory footprint"
    - "Local validation: $0 and ~0.002g CO₂"
    - "Cloud AI validation: $0.02+ and ~0.5g CO₂"
    - "99.6% carbon reduction with local tools"
  release_profile: |
    [profile.release]
    opt-level = 3
    lto = true
    codegen-units = 1
    strip = true
    panic = "abort"

files:
  source:
    - "src/main.rs - Entry point"
    - "src/lib.rs - Library root"
  config:
    - "Cargo.toml - Dependencies"
  docs:
    - "README.md - Documentation"

session:
  start:
    - "Read warmup.yaml"
    - "git status"
    - "cargo test (verify baseline)"
  during:
    - "Track progress"
    - "Test frequently"
    - "Small, logical commits"
  end:
    - "cargo test (all pass)"
    - "cargo clippy -- -D warnings"
    - "Update documentation"

quality:
  tests: "cargo test"
  warnings: "cargo clippy -- -D warnings"
  formatting: "cargo fmt --all -- --check"

style:
  rust:
    - "Result<T, E> for errors, no panics"
    - "thiserror for custom errors"
    - "No unwrap() in library code"
  docs:
    - "Keep documentation concise"
"#,
        project_name
    )
}

/// Generate a starter sprint.yaml template
pub fn sprint_template() -> String {
    r#"# Forge Protocol - Sprint Tracking
# https://github.com/royalbit/forge-protocol

sprint:
  current: "Initial setup"
  started: "2025-01-01"
  status: in_progress

  tasks:
    - "[ ] Task one"
    - "[ ] Task two"
    - "[ ] Task three"

  blockers: []

  notes: |
    Add any relevant context here.
"#
    .to_string()
}

/// Generate a starter roadmap.yaml template
pub fn roadmap_template() -> String {
    r#"# Forge Protocol - Roadmap
# https://github.com/royalbit/forge-protocol

metadata:
  current_version: "0.1.0"
  last_updated: "2025-01-01"

current:
  version: "0.1.0"
  status: in_progress
  summary: "Initial Release"
  highlights:
    - "Core functionality"
    - "Basic documentation"

next:
  version: "0.2.0"
  status: planned
  summary: "Next Milestone"
  features:
    - "Feature one"
    - "Feature two"

backlog:
  - "Future idea one"
  - "Future idea two"
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== ProjectType Tests ==========

    #[test]
    fn test_project_type_default() {
        let pt: ProjectType = Default::default();
        assert!(matches!(pt, ProjectType::Generic));
    }

    #[test]
    fn test_project_type_display() {
        assert_eq!(ProjectType::Generic.to_string(), "generic");
        assert_eq!(ProjectType::Rust.to_string(), "rust");
    }

    #[test]
    fn test_project_type_from_str_valid() {
        assert!(matches!(
            "generic".parse::<ProjectType>(),
            Ok(ProjectType::Generic)
        ));
        assert!(matches!(
            "rust".parse::<ProjectType>(),
            Ok(ProjectType::Rust)
        ));
        assert!(matches!(
            "GENERIC".parse::<ProjectType>(),
            Ok(ProjectType::Generic)
        ));
        assert!(matches!(
            "RUST".parse::<ProjectType>(),
            Ok(ProjectType::Rust)
        ));
        assert!(matches!(
            "Rust".parse::<ProjectType>(),
            Ok(ProjectType::Rust)
        ));
    }

    #[test]
    fn test_project_type_from_str_invalid() {
        let result = "invalid".parse::<ProjectType>();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Unknown project type"));
        assert!(err.contains("invalid"));
    }

    // ========== warmup_template Tests ==========

    #[test]
    fn test_warmup_template_generic_contains_project_name() {
        let template = warmup_template("my-project", ProjectType::Generic);
        assert!(template.contains("my-project"));
        assert!(template.contains("identity:"));
        assert!(template.contains("project:"));
    }

    #[test]
    fn test_warmup_template_generic_is_generic() {
        let template = warmup_template("test", ProjectType::Generic);
        // Should NOT contain Rust-specific content
        assert!(!template.contains("cargo"));
        assert!(!template.contains("Cargo.toml"));
        assert!(!template.contains("clippy"));
        assert!(!template.contains("main.rs"));
        // Should contain generic content
        assert!(template.contains("src/ - Source code"));
        assert!(template.contains("Run linter"));
        // Should contain green_coding (core protocol requirement)
        assert!(template.contains("green_coding:"));
        assert!(template.contains("Local-first"));
        assert!(template.contains("99.6% carbon reduction"));
    }

    #[test]
    fn test_warmup_template_rust_contains_rust_specific() {
        let template = warmup_template("rust-project", ProjectType::Rust);
        assert!(template.contains("rust-project"));
        // Should contain Rust-specific content
        assert!(template.contains("cargo test"));
        assert!(template.contains("cargo clippy"));
        assert!(template.contains("Cargo.toml"));
        assert!(template.contains("src/main.rs"));
        assert!(template.contains("src/lib.rs"));
        assert!(template.contains("Result<T, E>"));
        assert!(template.contains("thiserror"));
        // Should contain green_coding with Rust-specific practices
        assert!(template.contains("green_coding:"));
        assert!(template.contains("UPX compress"));
        assert!(template.contains("LTO"));
        assert!(template.contains("[profile.release]"));
    }

    #[test]
    fn test_warmup_template_is_valid_yaml() {
        for project_type in [ProjectType::Generic, ProjectType::Rust] {
            let template = warmup_template("test", project_type);
            let result: Result<serde_yaml::Value, _> = serde_yaml::from_str(&template);
            assert!(
                result.is_ok(),
                "Template should be valid YAML for {:?}",
                project_type
            );
        }
    }

    #[test]
    fn test_warmup_template_has_required_fields() {
        for project_type in [ProjectType::Generic, ProjectType::Rust] {
            let template = warmup_template("test", project_type);
            let yaml: serde_yaml::Value = serde_yaml::from_str(&template).unwrap();

            // Check required identity section
            assert!(
                yaml.get("identity").is_some(),
                "Should have identity section"
            );
            let identity = yaml.get("identity").unwrap();
            assert!(
                identity.get("project").is_some(),
                "Should have project field"
            );
        }
    }

    // ========== sprint_template Tests ==========

    #[test]
    fn test_sprint_template_is_valid_yaml() {
        let template = sprint_template();
        let result: Result<serde_yaml::Value, _> = serde_yaml::from_str(&template);
        assert!(result.is_ok(), "Sprint template should be valid YAML");
    }

    #[test]
    fn test_sprint_template_has_required_fields() {
        let template = sprint_template();
        let yaml: serde_yaml::Value = serde_yaml::from_str(&template).unwrap();

        assert!(yaml.get("sprint").is_some(), "Should have sprint section");
        let sprint = yaml.get("sprint").unwrap();
        assert!(sprint.get("current").is_some(), "Should have current field");
    }

    #[test]
    fn test_sprint_template_has_valid_status() {
        let template = sprint_template();
        let yaml: serde_yaml::Value = serde_yaml::from_str(&template).unwrap();
        let status = yaml["sprint"]["status"].as_str().unwrap();
        assert!(
            ["planned", "in_progress", "blocked", "done"].contains(&status),
            "Status should be valid enum value"
        );
    }

    // ========== roadmap_template Tests ==========

    #[test]
    fn test_roadmap_template_is_valid_yaml() {
        let template = roadmap_template();
        let result: Result<serde_yaml::Value, _> = serde_yaml::from_str(&template);
        assert!(result.is_ok(), "Roadmap template should be valid YAML");
    }

    #[test]
    fn test_roadmap_template_has_sections() {
        let template = roadmap_template();
        let yaml: serde_yaml::Value = serde_yaml::from_str(&template).unwrap();

        assert!(
            yaml.get("metadata").is_some(),
            "Should have metadata section"
        );
        assert!(yaml.get("current").is_some(), "Should have current section");
        assert!(yaml.get("next").is_some(), "Should have next section");
        assert!(yaml.get("backlog").is_some(), "Should have backlog section");
    }

    #[test]
    fn test_roadmap_template_has_valid_statuses() {
        let template = roadmap_template();
        let yaml: serde_yaml::Value = serde_yaml::from_str(&template).unwrap();

        let current_status = yaml["current"]["status"].as_str().unwrap();
        let next_status = yaml["next"]["status"].as_str().unwrap();

        let valid_statuses = ["planned", "in_progress", "released"];
        assert!(
            valid_statuses.contains(&current_status),
            "Current status should be valid"
        );
        assert!(
            valid_statuses.contains(&next_status),
            "Next status should be valid"
        );
    }
}
