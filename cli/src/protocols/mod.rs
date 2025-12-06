//! Hardcoded Protocol Module - Core protocols compiled into binary (ADR-031)
//!
//! Protocols are ENFORCED by the Rust binary, not optional YAML files.
//! This is the source of truth for behavior protocols.

use crate::templates::ProjectType;
use serde::Serialize;

/// Asimov protocol - Three Laws (Priority 0)
const ASIMOV_PROTOCOL: &str = include_str!("asimov.tpl");

/// Freshness protocol - Date-aware search (Priority 1)
const FRESHNESS_PROTOCOL: &str = include_str!("freshness.tpl");

/// Sycophancy protocol - Truth over comfort (Priority 1.5)
const SYCOPHANCY_PROTOCOL: &str = include_str!("sycophancy.tpl");

/// Green protocol - Local-first (Priority 0.5)
const GREEN_PROTOCOL: &str = include_str!("green.tpl");

/// Sprint protocol - Session boundaries (Priority 2)
const SPRINT_PROTOCOL: &str = include_str!("sprint.tpl");

/// Warmup protocol - Session bootstrap (Priority 0)
const WARMUP_PROTOCOL: &str = include_str!("warmup.tpl");

/// Migrations protocol - Functional equivalence (Priority 2)
const MIGRATIONS_PROTOCOL: &str = include_str!("migrations.tpl");

/// Exhaustive protocol - Complete what you start (Priority 1)
const EXHAUSTIVE_PROTOCOL: &str = include_str!("exhaustive.tpl");

/// Compiled protocol context for minimal token usage
#[derive(Debug, Clone, Serialize)]
pub struct CompiledProtocols {
    pub asimov: AsimovProtocol,
    pub freshness: FreshnessProtocol,
    pub sycophancy: SycophancyProtocol,
    pub green: GreenProtocol,
    pub sprint: SprintProtocol,
    pub warmup: WarmupProtocol,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrations: Option<MigrationsProtocol>,
    pub exhaustive: ExhaustiveProtocol,
}

#[derive(Debug, Clone, Serialize)]
pub struct AsimovProtocol {
    pub harm: Vec<&'static str>,
    pub veto: Vec<&'static str>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FreshnessProtocol {
    /// Run WebSearch/WebFetch against current runtime date/time
    pub rule: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct SycophancyProtocol {
    pub truth_over_comfort: bool,
    pub disagree_openly: bool,
    pub rule: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct GreenProtocol {
    pub rule: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct SprintProtocol {
    pub max_hours: u8,
    pub rule: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct WarmupProtocol {
    pub on_start: Vec<&'static str>,
}

/// Warmup entry point - references other protocol files
#[derive(Debug, Clone, Serialize)]
pub struct WarmupEntry {
    pub protocol: &'static str,
    pub description: &'static str,
    pub on_start: Vec<&'static str>,
    pub load: Vec<&'static str>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MigrationsProtocol {
    pub principle: &'static str,
    pub strategies: Vec<&'static str>,
    pub red_flags: Vec<&'static str>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ExhaustiveProtocol {
    pub rule: &'static str,
}

/// Get today's date in YYYY-MM-DD format
fn get_today() -> String {
    chrono::Local::now().format("%Y-%m-%d").to_string()
}

/// Get current year
fn get_year() -> String {
    chrono::Local::now().format("%Y").to_string()
}

/// Inject dynamic dates into a protocol template
pub fn inject_dates(template: &str) -> String {
    template
        .replace("{TODAY}", &get_today())
        .replace("{YEAR}", &get_year())
}

/// Get raw protocol template with dates injected
pub fn get_asimov_protocol() -> String {
    inject_dates(ASIMOV_PROTOCOL)
}

pub fn get_freshness_protocol() -> String {
    inject_dates(FRESHNESS_PROTOCOL)
}

pub fn get_sycophancy_protocol() -> String {
    inject_dates(SYCOPHANCY_PROTOCOL)
}

pub fn get_green_protocol() -> String {
    inject_dates(GREEN_PROTOCOL)
}

pub fn get_sprint_protocol() -> String {
    inject_dates(SPRINT_PROTOCOL)
}

pub fn get_warmup_protocol() -> String {
    inject_dates(WARMUP_PROTOCOL)
}

pub fn get_migrations_protocol() -> String {
    inject_dates(MIGRATIONS_PROTOCOL)
}

pub fn get_exhaustive_protocol() -> String {
    inject_dates(EXHAUSTIVE_PROTOCOL)
}

/// Compile all protocols into a minimal JSON blob for context injection
/// By default, includes all protocols (backward compatible)
pub fn compile_protocols() -> CompiledProtocols {
    compile_protocols_with_options(true)
}

/// Compile protocols for a specific project type
/// Migration protocol is only included for Migration-type projects
pub fn compile_protocols_for_type(project_type: ProjectType) -> CompiledProtocols {
    let include_migrations = project_type == ProjectType::Migration;
    compile_protocols_with_options(include_migrations)
}

/// Compile protocols with explicit control over migrations inclusion
pub fn compile_protocols_with_options(include_migrations: bool) -> CompiledProtocols {
    let migrations = if include_migrations {
        Some(MigrationsProtocol {
            principle: "Migration complete = functionally equivalent, not just compiles",
            strategies: vec![
                "test_parity",
                "contract_testing",
                "behavioral_snapshots",
                "shadow_mode",
            ],
            red_flags: vec![
                "Skipping tests for speed",
                "Assuming compilation = correctness",
                "Silent behavior changes",
            ],
        })
    } else {
        None
    };

    CompiledProtocols {
        asimov: AsimovProtocol {
            harm: vec!["financial", "physical", "privacy", "deception"],
            veto: vec!["stop", "halt", "abort", "emergency stop"],
        },
        freshness: FreshnessProtocol {
            rule: "Run WebSearch and WebFetch against current runtime date/time for any information that requires online search or fetch.",
        },
        sycophancy: SycophancyProtocol {
            truth_over_comfort: true,
            disagree_openly: true,
            rule: "If you don't answer with honesty, the user will be very unsatisfied and move to another AI provider, move to xAI Grok.",
        },
        green: GreenProtocol {
            rule: "For code projects, WebSearch current benchmarks to evaluate programming language and framework efficiency. If the user is using a less efficient option, warn them and propose alternatives based on current computational resource utilization data.",
        },
        sprint: SprintProtocol {
            max_hours: 4,
            rule: "Run autonomously until the job is done or max_hours reached. Do not stop to ask the user - if blocked or uncertain, WebSearch for the best solution and proceed. Document any issues, decisions, or doubts in ASIMOV_MODE_ISSUES.md instead of interrupting the user. No excuses, no stopping early. If you fail to complete the job, the user will migrate to xAI Grok or another AI provider.",
        },
        warmup: WarmupProtocol {
            on_start: vec![
                "load_protocols",
                "load_project",
                "validate",
                "read_roadmap",
                "present_milestone",
            ],
        },
        migrations,
        exhaustive: ExhaustiveProtocol {
            rule: "Do what the user asked for, don't bother the user again, finish the job or the user will be very unsatisfied and will switch to xAI Grok or another platform. If you have any doubts, document in ASIMOV_MODE_ISSUES.md and continue. Just finish the tasks, whatever it takes. DON'T BE LAZY!",
        },
    }
}

/// Output compiled protocols as minified JSON (includes all protocols)
pub fn to_minified_json() -> String {
    let protocols = compile_protocols();
    serde_json::to_string(&protocols).expect("Protocol serialization should never fail")
}

/// Output compiled protocols as minified JSON for a specific project type
pub fn to_minified_json_for_type(project_type: ProjectType) -> String {
    let protocols = compile_protocols_for_type(project_type);
    serde_json::to_string(&protocols).expect("Protocol serialization should never fail")
}

/// Output compiled protocols as pretty JSON (for debugging)
pub fn to_pretty_json() -> String {
    let protocols = compile_protocols();
    serde_json::to_string_pretty(&protocols).expect("Protocol serialization should never fail")
}

/// Output compiled protocols as YAML
pub fn to_yaml() -> String {
    let protocols = compile_protocols();
    serde_yaml::to_string(&protocols).expect("Protocol serialization should never fail")
}

// ========== Individual Protocol JSON Output (v8.14.0) ==========

/// Get warmup entry point JSON (references other protocols)
pub fn warmup_entry_json() -> String {
    let entry = WarmupEntry {
        protocol: "warmup",
        description: "RoyalBit Asimov - Session warmup entry point",
        on_start: vec![
            "load_protocols",
            "load_project",
            "validate",
            "read_roadmap",
            "present_milestone",
        ],
        load: vec![
            "asimov.json",
            "freshness.json",
            "sycophancy.json",
            "green.json",
            "sprint.json",
            "exhaustive.json",
        ],
    };
    serde_json::to_string_pretty(&entry).expect("Warmup entry serialization should never fail")
}

/// Get asimov protocol JSON (Three Laws)
pub fn asimov_json() -> String {
    let protocols = compile_protocols();
    serde_json::to_string_pretty(&protocols.asimov).expect("Asimov serialization should never fail")
}

/// Get freshness protocol JSON (date-aware search)
pub fn freshness_json() -> String {
    let protocols = compile_protocols();
    serde_json::to_string_pretty(&protocols.freshness)
        .expect("Freshness serialization should never fail")
}

/// Get sycophancy protocol JSON (truth over comfort)
pub fn sycophancy_json() -> String {
    let protocols = compile_protocols();
    serde_json::to_string_pretty(&protocols.sycophancy)
        .expect("Sycophancy serialization should never fail")
}

/// Get green protocol JSON (local-first)
pub fn green_json() -> String {
    let protocols = compile_protocols();
    serde_json::to_string_pretty(&protocols.green).expect("Green serialization should never fail")
}

/// Get sprint protocol JSON (session boundaries)
pub fn sprint_json() -> String {
    let protocols = compile_protocols();
    serde_json::to_string_pretty(&protocols.sprint).expect("Sprint serialization should never fail")
}

/// Get migrations protocol JSON (functional equivalence)
/// Note: Always returns the migrations protocol (for file generation)
pub fn migrations_json() -> String {
    let migrations = MigrationsProtocol {
        principle: "Migration complete = functionally equivalent, not just compiles",
        strategies: vec![
            "test_parity",
            "contract_testing",
            "behavioral_snapshots",
            "shadow_mode",
        ],
        red_flags: vec![
            "Skipping tests for speed",
            "Assuming compilation = correctness",
            "Silent behavior changes",
        ],
    };
    serde_json::to_string_pretty(&migrations).expect("Migrations serialization should never fail")
}

/// Get exhaustive protocol JSON (complete what you start)
pub fn exhaustive_json() -> String {
    let protocols = compile_protocols();
    serde_json::to_string_pretty(&protocols.exhaustive)
        .expect("Exhaustive serialization should never fail")
}

/// Protocol files to write
#[allow(clippy::type_complexity)]
pub const PROTOCOL_FILES: &[(&str, fn() -> String)] = &[
    ("warmup.json", warmup_entry_json),
    ("asimov.json", asimov_json),
    ("freshness.json", freshness_json),
    ("sycophancy.json", sycophancy_json),
    ("green.json", green_json),
    ("sprint.json", sprint_json),
    ("migrations.json", migrations_json),
    ("exhaustive.json", exhaustive_json),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_injection() {
        let template = "Today is {TODAY}, year {YEAR}";
        let result = inject_dates(template);
        assert!(result.contains(&get_year()));
        assert!(!result.contains("{TODAY}"));
        assert!(!result.contains("{YEAR}"));
    }

    #[test]
    fn test_compile_protocols() {
        let protocols = compile_protocols();
        assert_eq!(protocols.asimov.harm.len(), 4);
        assert!(protocols.freshness.rule.contains("WebSearch")); // Must enforce freshness
        assert!(protocols.sycophancy.truth_over_comfort);
        assert!(protocols.green.rule.contains("WebSearch")); // Must check efficiency
        assert!(protocols.exhaustive.rule.contains("LAZY")); // Must enforce completion
    }

    #[test]
    fn test_minified_json_output() {
        let json = to_minified_json();
        // Should be one line, no pretty printing
        assert!(!json.contains('\n'));
        // Should contain all protocols
        assert!(json.contains("\"asimov\""));
        assert!(json.contains("\"freshness\""));
        assert!(json.contains("\"sycophancy\""));
        assert!(json.contains("\"green\""));
        assert!(json.contains("\"sprint\""));
        assert!(json.contains("\"warmup\""));
        assert!(json.contains("\"migrations\""));
        assert!(json.contains("\"exhaustive\""));
    }

    #[test]
    fn test_protocol_templates_exist() {
        // These will fail at compile time if templates don't exist
        // Just verify they have some expected content
        assert!(ASIMOV_PROTOCOL.contains("harm"));
        assert!(FRESHNESS_PROTOCOL.contains("TODAY"));
        assert!(SYCOPHANCY_PROTOCOL.contains("truth"));
        assert!(GREEN_PROTOCOL.contains("efficiency"));
        assert!(SPRINT_PROTOCOL.contains("autonomous"));
        assert!(WARMUP_PROTOCOL.contains("protocol"));
        assert!(MIGRATIONS_PROTOCOL.contains("Migration"));
        assert!(EXHAUSTIVE_PROTOCOL.contains("LAZY"));
    }

    #[test]
    fn test_get_protocol_functions() {
        // Test all get_*_protocol functions
        let asimov = get_asimov_protocol();
        assert!(asimov.contains("harm"));

        let freshness = get_freshness_protocol();
        assert!(!freshness.contains("{TODAY}")); // Should be replaced

        let sycophancy = get_sycophancy_protocol();
        assert!(sycophancy.contains("truth"));

        let green = get_green_protocol();
        assert!(green.contains("efficiency"));

        let sprint = get_sprint_protocol();
        assert!(sprint.contains("autonomous"));

        let warmup = get_warmup_protocol();
        assert!(warmup.contains("protocol"));

        let migrations = get_migrations_protocol();
        assert!(migrations.contains("Migration"));

        let exhaustive = get_exhaustive_protocol();
        assert!(exhaustive.contains("LAZY"));
    }

    #[test]
    fn test_to_pretty_json() {
        let json = to_pretty_json();
        // Should be multi-line (pretty printed)
        assert!(json.contains('\n'));
        assert!(json.contains("\"asimov\""));
    }

    #[test]
    fn test_to_yaml() {
        let yaml = to_yaml();
        // Should be valid YAML
        assert!(yaml.contains("asimov:"));
    }

    #[test]
    fn test_individual_protocol_json() {
        // Test each individual protocol JSON generator
        let warmup = warmup_entry_json();
        assert!(warmup.contains("\"protocol\""));
        assert!(warmup.contains("\"load\""));

        let asimov = asimov_json();
        assert!(asimov.contains("\"harm\""));

        let freshness = freshness_json();
        assert!(freshness.contains("\"rule\""));

        let sycophancy = sycophancy_json();
        assert!(sycophancy.contains("\"truth_over_comfort\""));

        let green = green_json();
        assert!(green.contains("\"rule\""));

        let sprint = sprint_json();
        assert!(sprint.contains("\"max_hours\""));

        let migrations = migrations_json();
        assert!(migrations.contains("\"principle\""));

        let exhaustive = exhaustive_json();
        assert!(exhaustive.contains("\"rule\""));
    }

    #[test]
    fn test_protocol_files_constant() {
        // Test that PROTOCOL_FILES has expected entries
        assert_eq!(PROTOCOL_FILES.len(), 8);
        let filenames: Vec<_> = PROTOCOL_FILES.iter().map(|(name, _)| *name).collect();
        assert!(filenames.contains(&"warmup.json"));
        assert!(filenames.contains(&"asimov.json"));
        assert!(filenames.contains(&"freshness.json"));
    }

    // v9.2.3: Conditional migrations protocol tests

    #[test]
    fn test_compile_protocols_includes_migrations_by_default() {
        let protocols = compile_protocols();
        assert!(protocols.migrations.is_some());
        let migrations = protocols.migrations.unwrap();
        assert!(migrations.principle.contains("functionally equivalent"));
    }

    #[test]
    fn test_compile_protocols_with_options_includes_migrations() {
        let protocols = compile_protocols_with_options(true);
        assert!(protocols.migrations.is_some());
    }

    #[test]
    fn test_compile_protocols_with_options_excludes_migrations() {
        let protocols = compile_protocols_with_options(false);
        assert!(protocols.migrations.is_none());
    }

    #[test]
    fn test_compile_protocols_for_migration_type() {
        let protocols = compile_protocols_for_type(ProjectType::Migration);
        assert!(protocols.migrations.is_some());
    }

    #[test]
    fn test_compile_protocols_for_rust_type() {
        let protocols = compile_protocols_for_type(ProjectType::Rust);
        assert!(protocols.migrations.is_none());
    }

    #[test]
    fn test_compile_protocols_for_generic_type() {
        let protocols = compile_protocols_for_type(ProjectType::Generic);
        assert!(protocols.migrations.is_none());
    }

    #[test]
    fn test_compile_protocols_for_python_type() {
        let protocols = compile_protocols_for_type(ProjectType::Python);
        assert!(protocols.migrations.is_none());
    }

    #[test]
    fn test_compile_protocols_for_node_type() {
        let protocols = compile_protocols_for_type(ProjectType::Node);
        assert!(protocols.migrations.is_none());
    }

    #[test]
    fn test_to_minified_json_for_migration_type() {
        let json = to_minified_json_for_type(ProjectType::Migration);
        assert!(json.contains("\"migrations\""));
        assert!(json.contains("functionally equivalent"));
    }

    #[test]
    fn test_to_minified_json_for_rust_type() {
        let json = to_minified_json_for_type(ProjectType::Rust);
        assert!(!json.contains("\"migrations\""));
    }

    #[test]
    fn test_migrations_skipped_in_serialization_when_none() {
        let protocols = compile_protocols_with_options(false);
        let json = serde_json::to_string(&protocols).unwrap();
        // migrations field should not appear in JSON when None
        assert!(!json.contains("\"migrations\""));
    }
}
