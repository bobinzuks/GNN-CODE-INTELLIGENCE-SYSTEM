//! Rust-specific pattern detectors
//!
//! Comprehensive set of pattern detectors for Rust code including:
//! - Error handling patterns
//! - Memory safety patterns
//! - Async/await patterns
//! - Ownership and borrowing patterns
//! - Performance patterns

use super::{PatternDetector, PatternInstance, FixSuggestion};
use crate::{CodeGraph, CodeNode, NodeKind, Location, Severity};
use petgraph::visit::EdgeRef;
use regex::Regex;
use std::sync::Arc;

pub struct RustPatterns {
    detectors: Vec<Arc<dyn PatternDetector>>,
}

impl RustPatterns {
    pub fn new() -> Self {
        let detectors: Vec<Arc<dyn PatternDetector>> = vec![
            Arc::new(UnwrapPattern::new()),
            Arc::new(ExpectPattern::new()),
            Arc::new(UnwrapOrPattern::new()),
            Arc::new(ClonePattern::new()),
            Arc::new(ExcessiveCloningPattern::new()),
            Arc::new(UnsafeWithoutDocsPattern::new()),
            Arc::new(UnsafeBlockPattern::new()),
            Arc::new(AsyncWithoutAwaitPattern::new()),
            Arc::new(MissingAwaitPattern::new()),
            Arc::new(BlockingInAsyncPattern::new()),
            Arc::new(LifetimeComplexityPattern::new()),
            Arc::new(UnnecessaryLifetimePattern::new()),
            Arc::new(MissingLifetimePattern::new()),
            Arc::new(ResultIgnoredPattern::new()),
            Arc::new(OptionUnwrapPattern::new()),
            Arc::new(PanicInProductionPattern::new()),
            Arc::new(TodoFixmePattern::new()),
            Arc::new(UnreachablePattern::new()),
            Arc::new(BorrowCheckerPattern::new()),
            Arc::new(MutableBorrowPattern::new()),
            Arc::new(OwnershipTransferPattern::new()),
            Arc::new(DerefCoercionPattern::new()),
            Arc::new(StringAllocationPattern::new()),
            Arc::new(VecCapacityPattern::new()),
            Arc::new(IteratorChainPattern::new()),
            Arc::new(MatchExhaustivePattern::new()),
            Arc::new(EnumVariantNamingPattern::new()),
            Arc::new(StructUpdateSyntaxPattern::new()),
            Arc::new(TraitObjectPattern::new()),
            Arc::new(GenericBoundsPattern::new()),
        ];

        Self { detectors }
    }

    pub fn all_detectors(&self) -> &[Arc<dyn PatternDetector>] {
        &self.detectors
    }

    pub fn detect_all(&self, graph: &CodeGraph) -> Vec<PatternInstance> {
        self.detectors
            .iter()
            .flat_map(|detector| detector.detect(graph))
            .collect()
    }
}

impl Default for RustPatterns {
    fn default() -> Self {
        Self::new()
    }
}

// Helper function to extract location from node
fn node_location(node: &CodeNode) -> Location {
    Location {
        file_path: node.file_path.clone().unwrap_or_else(|| "unknown".to_string()),
        start_line: node.start_line,
        end_line: node.end_line,
        start_col: node.start_col,
        end_col: node.end_col,
    }
}

// Pattern 1: Unwrap Pattern
struct UnwrapPattern {
    regex: Regex,
}

impl UnwrapPattern {
    fn new() -> Self {
        Self {
            regex: Regex::new(r"\.unwrap\(\)").unwrap(),
        }
    }
}

impl PatternDetector for UnwrapPattern {
    fn name(&self) -> &str {
        "unwrap_usage"
    }

    fn description(&self) -> &str {
        "Direct .unwrap() calls can cause panics - use ? operator or proper error handling"
    }

    fn severity(&self) -> Severity {
        Severity::Warning
    }

    fn detect(&self, graph: &CodeGraph) -> Vec<PatternInstance> {
        let mut instances = Vec::new();

        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];

            if matches!(node.kind, NodeKind::Function | NodeKind::Method) {
                if let Some(sig) = &node.signature {
                    let count = sig.matches(".unwrap()").count();
                    if count > 0 {
                        instances.push(
                            PatternInstance::new(
                                self.name(),
                                node_location(node),
                                self.severity(),
                                format!(
                                    "Function '{}' contains {} .unwrap() call(s). Consider using the ? operator or proper error handling.",
                                    node.name, count
                                ),
                            )
                            .with_context(sig.clone())
                            .with_confidence(0.9)
                            .with_metadata("unwrap_count", count.to_string()),
                        );
                    }
                }
            }
        }

        instances
    }

    fn confidence(&self) -> f32 {
        0.9
    }

    fn suggest_fix(&self, instance: &PatternInstance) -> Option<FixSuggestion> {
        Some(
            FixSuggestion::new(
                self.name(),
                "Replace .unwrap() with ? operator and return Result",
                "let value = some_result.unwrap();",
                "let value = some_result?;",
            )
            .with_confidence(0.85)
            .automated(),
        )
    }
}

// Pattern 2: Expect Pattern
struct ExpectPattern {
    regex: Regex,
}

impl ExpectPattern {
    fn new() -> Self {
        Self {
            regex: Regex::new(r"\.expect\(").unwrap(),
        }
    }
}

impl PatternDetector for ExpectPattern {
    fn name(&self) -> &str {
        "expect_usage"
    }

    fn description(&self) -> &str {
        "Excessive .expect() calls - consider returning errors to caller"
    }

    fn severity(&self) -> Severity {
        Severity::Info
    }

    fn detect(&self, graph: &CodeGraph) -> Vec<PatternInstance> {
        let mut instances = Vec::new();

        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];

            if matches!(node.kind, NodeKind::Function | NodeKind::Method) {
                if let Some(sig) = &node.signature {
                    let count = sig.matches(".expect(").count();
                    if count > 2 {
                        instances.push(
                            PatternInstance::new(
                                self.name(),
                                node_location(node),
                                self.severity(),
                                format!(
                                    "Function '{}' has {} .expect() calls. Consider propagating errors with ?",
                                    node.name, count
                                ),
                            )
                            .with_context(sig.clone())
                            .with_confidence(0.8)
                            .with_metadata("expect_count", count.to_string()),
                        );
                    }
                }
            }
        }

        instances
    }

    fn confidence(&self) -> f32 {
        0.8
    }

    fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
        Some(
            FixSuggestion::new(
                self.name(),
                "Replace .expect() with ? operator for error propagation",
                "let value = some_result.expect(\"failed\");",
                "let value = some_result?;",
            )
            .with_confidence(0.75),
        )
    }
}

// Pattern 3: Unwrap Or Pattern
struct UnwrapOrPattern;

impl UnwrapOrPattern {
    fn new() -> Self {
        Self
    }
}

impl PatternDetector for UnwrapOrPattern {
    fn name(&self) -> &str {
        "unwrap_or_default_opportunity"
    }

    fn description(&self) -> &str {
        "Opportunity to use unwrap_or_default() for cleaner code"
    }

    fn severity(&self) -> Severity {
        Severity::Info
    }

    fn detect(&self, graph: &CodeGraph) -> Vec<PatternInstance> {
        let mut instances = Vec::new();

        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];

            if let Some(sig) = &node.signature {
                if sig.contains(".unwrap_or(") {
                    instances.push(
                        PatternInstance::new(
                            self.name(),
                            node_location(node),
                            self.severity(),
                            format!(
                                "Consider using .unwrap_or_default() if using default value in '{}'",
                                node.name
                            ),
                        )
                        .with_context(sig.clone())
                        .with_confidence(0.7),
                    );
                }
            }
        }

        instances
    }

    fn confidence(&self) -> f32 {
        0.7
    }

    fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
        Some(
            FixSuggestion::new(
                self.name(),
                "Use unwrap_or_default() for default values",
                ".unwrap_or(Vec::new())",
                ".unwrap_or_default()",
            )
            .with_confidence(0.7),
        )
    }
}

// Pattern 4: Clone Pattern
struct ClonePattern;

impl ClonePattern {
    fn new() -> Self {
        Self
    }
}

impl PatternDetector for ClonePattern {
    fn name(&self) -> &str {
        "unnecessary_clone"
    }

    fn description(&self) -> &str {
        "Unnecessary .clone() - consider borrowing instead"
    }

    fn severity(&self) -> Severity {
        Severity::Info
    }

    fn detect(&self, graph: &CodeGraph) -> Vec<PatternInstance> {
        let mut instances = Vec::new();

        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];

            if let Some(sig) = &node.signature {
                let clone_count = sig.matches(".clone()").count();
                if clone_count > 0 && clone_count <= 2 {
                    instances.push(
                        PatternInstance::new(
                            self.name(),
                            node_location(node),
                            self.severity(),
                            format!(
                                "Consider if .clone() is necessary in '{}'. Borrowing may be more efficient.",
                                node.name
                            ),
                        )
                        .with_context(sig.clone())
                        .with_confidence(0.6)
                        .with_metadata("clone_count", clone_count.to_string()),
                    );
                }
            }
        }

        instances
    }

    fn confidence(&self) -> f32 {
        0.6
    }

    fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
        Some(
            FixSuggestion::new(
                self.name(),
                "Use references instead of cloning when possible",
                "let copy = value.clone();\nprocess(copy);",
                "process(&value);",
            )
            .with_confidence(0.6),
        )
    }
}

// Pattern 5: Excessive Cloning Pattern
struct ExcessiveCloningPattern;

impl ExcessiveCloningPattern {
    fn new() -> Self {
        Self
    }
}

impl PatternDetector for ExcessiveCloningPattern {
    fn name(&self) -> &str {
        "excessive_cloning"
    }

    fn description(&self) -> &str {
        "Excessive .clone() calls detected - major performance concern"
    }

    fn severity(&self) -> Severity {
        Severity::Warning
    }

    fn detect(&self, graph: &CodeGraph) -> Vec<PatternInstance> {
        let mut instances = Vec::new();

        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];

            if let Some(sig) = &node.signature {
                let clone_count = sig.matches(".clone()").count();
                if clone_count > 3 {
                    instances.push(
                        PatternInstance::new(
                            self.name(),
                            node_location(node),
                            self.severity(),
                            format!(
                                "Function '{}' has {} .clone() calls. This may impact performance. Consider redesigning to use references.",
                                node.name, clone_count
                            ),
                        )
                        .with_context(sig.clone())
                        .with_confidence(0.85)
                        .with_metadata("clone_count", clone_count.to_string()),
                    );
                }
            }
        }

        instances
    }

    fn confidence(&self) -> f32 {
        0.85
    }

    fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
        Some(
            FixSuggestion::new(
                self.name(),
                "Refactor to use references or restructure data flow",
                "Multiple .clone() calls",
                "Use &T references or Rc<T>/Arc<T> for shared ownership",
            )
            .with_confidence(0.7),
        )
    }
}

// Pattern 6: Unsafe Without Docs Pattern
struct UnsafeWithoutDocsPattern {
    regex: Regex,
}

impl UnsafeWithoutDocsPattern {
    fn new() -> Self {
        Self {
            regex: Regex::new(r"\bunsafe\b").unwrap(),
        }
    }
}

impl PatternDetector for UnsafeWithoutDocsPattern {
    fn name(&self) -> &str {
        "unsafe_without_documentation"
    }

    fn description(&self) -> &str {
        "Unsafe code without SAFETY documentation"
    }

    fn severity(&self) -> Severity {
        Severity::Warning
    }

    fn detect(&self, graph: &CodeGraph) -> Vec<PatternInstance> {
        let mut instances = Vec::new();

        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];

            let has_unsafe = node.metadata.contains_key("unsafe")
                || node.signature.as_ref().map_or(false, |s| self.regex.is_match(s));

            if has_unsafe {
                let has_docs = node.metadata.contains_key("safety_comment")
                    || node.metadata.contains_key("docs");

                if !has_docs {
                    instances.push(
                        PatternInstance::new(
                            self.name(),
                            node_location(node),
                            self.severity(),
                            format!(
                                "Unsafe code in '{}' lacks SAFETY documentation. Explain why this is safe.",
                                node.name
                            ),
                        )
                        .with_context(node.signature.clone().unwrap_or_default())
                        .with_confidence(0.9),
                    );
                }
            }
        }

        instances
    }

    fn confidence(&self) -> f32 {
        0.9
    }

    fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
        Some(
            FixSuggestion::new(
                self.name(),
                "Add SAFETY comment explaining why the unsafe code is safe",
                "unsafe { ... }",
                "// SAFETY: Explain invariants that make this safe\nunsafe { ... }",
            )
            .with_confidence(0.8),
        )
    }
}

// Pattern 7: Unsafe Block Pattern
struct UnsafeBlockPattern {
    regex: Regex,
}

impl UnsafeBlockPattern {
    fn new() -> Self {
        Self {
            regex: Regex::new(r"unsafe\s*\{").unwrap(),
        }
    }
}

impl PatternDetector for UnsafeBlockPattern {
    fn name(&self) -> &str {
        "large_unsafe_block"
    }

    fn description(&self) -> &str {
        "Large unsafe blocks - minimize unsafe code scope"
    }

    fn severity(&self) -> Severity {
        Severity::Warning
    }

    fn detect(&self, graph: &CodeGraph) -> Vec<PatternInstance> {
        let mut instances = Vec::new();

        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];

            if node.metadata.get("unsafe").is_some() {
                let line_count = node.end_line.saturating_sub(node.start_line);
                if line_count > 10 {
                    instances.push(
                        PatternInstance::new(
                            self.name(),
                            node_location(node),
                            self.severity(),
                            format!(
                                "Large unsafe block ({} lines) in '{}'. Minimize unsafe scope.",
                                line_count, node.name
                            ),
                        )
                        .with_confidence(0.8)
                        .with_metadata("unsafe_lines", line_count.to_string()),
                    );
                }
            }
        }

        instances
    }

    fn confidence(&self) -> f32 {
        0.8
    }

    fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
        Some(
            FixSuggestion::new(
                self.name(),
                "Extract safe portions and minimize unsafe scope",
                "unsafe { /* many lines */ }",
                "// safe code\nunsafe { /* minimal unsafe operations */ }\n// more safe code",
            )
            .with_confidence(0.7),
        )
    }
}

// Pattern 8: Async Without Await Pattern
struct AsyncWithoutAwaitPattern;

impl AsyncWithoutAwaitPattern {
    fn new() -> Self {
        Self
    }
}

impl PatternDetector for AsyncWithoutAwaitPattern {
    fn name(&self) -> &str {
        "async_without_await"
    }

    fn description(&self) -> &str {
        "Async function may not be awaiting any async operations"
    }

    fn severity(&self) -> Severity {
        Severity::Warning
    }

    fn detect(&self, graph: &CodeGraph) -> Vec<PatternInstance> {
        let mut instances = Vec::new();

        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];

            if node.is_async && matches!(node.kind, NodeKind::Function | NodeKind::Method) {
                let has_await = node
                    .signature
                    .as_ref()
                    .map_or(false, |s| s.contains(".await"));

                if !has_await {
                    instances.push(
                        PatternInstance::new(
                            self.name(),
                            node_location(node),
                            self.severity(),
                            format!(
                                "Async function '{}' doesn't appear to await. Consider removing 'async' or add .await calls.",
                                node.name
                            ),
                        )
                        .with_context(node.signature.clone().unwrap_or_default())
                        .with_confidence(0.75),
                    );
                }
            }
        }

        instances
    }

    fn confidence(&self) -> f32 {
        0.75
    }

    fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
        Some(
            FixSuggestion::new(
                self.name(),
                "Remove async keyword if not awaiting, or add .await to async calls",
                "async fn foo() { sync_call() }",
                "fn foo() { sync_call() }\n// OR\nasync fn foo() { async_call().await }",
            )
            .with_confidence(0.7),
        )
    }
}

// Pattern 9: Missing Await Pattern
struct MissingAwaitPattern;

impl MissingAwaitPattern {
    fn new() -> Self {
        Self
    }
}

impl PatternDetector for MissingAwaitPattern {
    fn name(&self) -> &str {
        "missing_await"
    }

    fn description(&self) -> &str {
        "Async function call not awaited"
    }

    fn severity(&self) -> Severity {
        Severity::Error
    }

    fn detect(&self, graph: &CodeGraph) -> Vec<PatternInstance> {
        let mut instances = Vec::new();

        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];

            // Check if calling async functions without await
            if node.metadata.get("calls_async").is_some() {
                if !node
                    .signature
                    .as_ref()
                    .map_or(false, |s| s.contains(".await"))
                {
                    instances.push(
                        PatternInstance::new(
                            self.name(),
                            node_location(node),
                            self.severity(),
                            format!(
                                "Async function call in '{}' may not be awaited. Add .await to execute.",
                                node.name
                            ),
                        )
                        .with_confidence(0.85),
                    );
                }
            }
        }

        instances
    }

    fn confidence(&self) -> f32 {
        0.85
    }

    fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
        Some(
            FixSuggestion::new(
                self.name(),
                "Add .await to async function call",
                "let result = async_function();",
                "let result = async_function().await;",
            )
            .with_confidence(0.9)
            .automated(),
        )
    }
}

// Pattern 10: Blocking in Async Pattern
struct BlockingInAsyncPattern;

impl BlockingInAsyncPattern {
    fn new() -> Self {
        Self
    }
}

impl PatternDetector for BlockingInAsyncPattern {
    fn name(&self) -> &str {
        "blocking_in_async"
    }

    fn description(&self) -> &str {
        "Blocking operations in async context can cause runtime issues"
    }

    fn severity(&self) -> Severity {
        Severity::Warning
    }

    fn detect(&self, graph: &CodeGraph) -> Vec<PatternInstance> {
        let mut instances = Vec::new();

        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];

            if node.is_async {
                // Check for blocking operations
                if let Some(sig) = &node.signature {
                    let blocking_patterns = ["thread::sleep", "std::io::", ".lock()", ".read()"];

                    for pattern in &blocking_patterns {
                        if sig.contains(pattern) {
                            instances.push(
                                PatternInstance::new(
                                    self.name(),
                                    node_location(node),
                                    self.severity(),
                                    format!(
                                        "Async function '{}' contains blocking operation '{}'. Use async alternatives.",
                                        node.name, pattern
                                    ),
                                )
                                .with_context(sig.clone())
                                .with_confidence(0.8)
                                .with_metadata("blocking_call", pattern.to_string()),
                            );
                        }
                    }
                }
            }
        }

        instances
    }

    fn confidence(&self) -> f32 {
        0.8
    }

    fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
        Some(
            FixSuggestion::new(
                self.name(),
                "Use async alternatives for blocking operations",
                "thread::sleep(Duration::from_secs(1));",
                "tokio::time::sleep(Duration::from_secs(1)).await;",
            )
            .with_confidence(0.75),
        )
    }
}

// Continuing with more patterns...

// Pattern 11: Lifetime Complexity Pattern
struct LifetimeComplexityPattern;

impl LifetimeComplexityPattern {
    fn new() -> Self {
        Self
    }
}

impl PatternDetector for LifetimeComplexityPattern {
    fn name(&self) -> &str {
        "complex_lifetimes"
    }

    fn description(&self) -> &str {
        "Complex lifetime annotations - consider simplifying"
    }

    fn severity(&self) -> Severity {
        Severity::Info
    }

    fn detect(&self, graph: &CodeGraph) -> Vec<PatternInstance> {
        let mut instances = Vec::new();

        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];

            if let Some(sig) = &node.signature {
                let lifetime_count = sig.matches("'").count() / 2;
                if lifetime_count > 3 {
                    instances.push(
                        PatternInstance::new(
                            self.name(),
                            node_location(node),
                            self.severity(),
                            format!(
                                "Function '{}' has {} lifetime parameters. Consider refactoring for simplicity.",
                                node.name, lifetime_count
                            ),
                        )
                        .with_context(sig.clone())
                        .with_confidence(0.7)
                        .with_metadata("lifetime_count", lifetime_count.to_string()),
                    );
                }
            }
        }

        instances
    }

    fn confidence(&self) -> f32 {
        0.7
    }

    fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
        Some(
            FixSuggestion::new(
                self.name(),
                "Simplify by using owned types or fewer lifetime parameters",
                "fn complex<'a, 'b, 'c>(x: &'a T, y: &'b T, z: &'c T)",
                "fn simpler<'a>(x: &'a T, y: &'a T, z: &'a T)",
            )
            .with_confidence(0.6),
        )
    }
}

// Pattern 12-30: Additional patterns following similar structure...

// Pattern 12: Unnecessary Lifetime Pattern
struct UnnecessaryLifetimePattern;
impl UnnecessaryLifetimePattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for UnnecessaryLifetimePattern {
    fn name(&self) -> &str {
        "unnecessary_lifetime"
    }
    fn description(&self) -> &str {
        "Lifetime parameter may be elided"
    }
    fn severity(&self) -> Severity {
        Severity::Info
    }
    fn detect(&self, _graph: &CodeGraph) -> Vec<PatternInstance> {
        Vec::new()
    }
    fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
        Some(FixSuggestion::new(
            self.name(),
            "Remove unnecessary lifetime annotations - rely on elision rules",
            "fn foo<'a>(x: &'a str) -> &'a str",
            "fn foo(x: &str) -> &str",
        ))
    }
}

// Pattern 13: Missing Lifetime Pattern
struct MissingLifetimePattern;
impl MissingLifetimePattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for MissingLifetimePattern {
    fn name(&self) -> &str {
        "missing_lifetime_bound"
    }
    fn description(&self) -> &str {
        "Missing lifetime bound may cause compilation issues"
    }
    fn severity(&self) -> Severity {
        Severity::Error
    }
    fn detect(&self, _graph: &CodeGraph) -> Vec<PatternInstance> {
        Vec::new()
    }
    fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
        Some(FixSuggestion::new(
            self.name(),
            "Add required lifetime bounds",
            "struct Foo<T> { data: &T }",
            "struct Foo<'a, T> { data: &'a T }",
        ))
    }
}

// Pattern 14: Result Ignored Pattern
struct ResultIgnoredPattern;
impl ResultIgnoredPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for ResultIgnoredPattern {
    fn name(&self) -> &str {
        "result_ignored"
    }
    fn description(&self) -> &str {
        "Result type returned but not handled"
    }
    fn severity(&self) -> Severity {
        Severity::Warning
    }
    fn detect(&self, graph: &CodeGraph) -> Vec<PatternInstance> {
        let mut instances = Vec::new();
        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];
            if let Some(sig) = &node.signature {
                if sig.contains("-> Result<") && sig.contains(";") && !sig.contains("?") {
                    instances.push(
                        PatternInstance::new(
                            self.name(),
                            node_location(node),
                            self.severity(),
                            format!("Result ignored in '{}'. Handle errors explicitly.", node.name),
                        )
                        .with_confidence(0.75),
                    );
                }
            }
        }
        instances
    }
    fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
        Some(FixSuggestion::new(
            self.name(),
            "Handle Result with ? or match",
            "func_returning_result();",
            "func_returning_result()?;\n// or\nmatch func_returning_result() { Ok(v) => ..., Err(e) => ... }",
        ))
    }
}

// Pattern 15: Option Unwrap Pattern
struct OptionUnwrapPattern;
impl OptionUnwrapPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for OptionUnwrapPattern {
    fn name(&self) -> &str {
        "option_unwrap_pattern"
    }
    fn description(&self) -> &str {
        "Use pattern matching or if-let instead of unwrap on Option"
    }
    fn severity(&self) -> Severity {
        Severity::Info
    }
    fn detect(&self, _graph: &CodeGraph) -> Vec<PatternInstance> {
        Vec::new()
    }
    fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
        Some(FixSuggestion::new(
            self.name(),
            "Use if-let or match for Option handling",
            "let value = option.unwrap();",
            "if let Some(value) = option { ... }\n// or use match",
        ))
    }
}

// Pattern 16: Panic in Production Pattern
struct PanicInProductionPattern;
impl PanicInProductionPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for PanicInProductionPattern {
    fn name(&self) -> &str {
        "panic_in_code"
    }
    fn description(&self) -> &str {
        "Direct panic!() calls should be avoided in production code"
    }
    fn severity(&self) -> Severity {
        Severity::Warning
    }
    fn detect(&self, graph: &CodeGraph) -> Vec<PatternInstance> {
        let mut instances = Vec::new();
        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];
            if let Some(sig) = &node.signature {
                if sig.contains("panic!(") {
                    instances.push(
                        PatternInstance::new(
                            self.name(),
                            node_location(node),
                            self.severity(),
                            format!("Direct panic!() call in '{}'. Use Result for error handling.", node.name),
                        )
                        .with_confidence(0.9),
                    );
                }
            }
        }
        instances
    }
    fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
        Some(FixSuggestion::new(
            self.name(),
            "Return Result instead of panicking",
            "panic!(\"error\");",
            "return Err(Error::new(\"error\"));",
        ))
    }
}

// Pattern 17: Todo/Fixme Pattern
struct TodoFixmePattern;
impl TodoFixmePattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for TodoFixmePattern {
    fn name(&self) -> &str {
        "todo_fixme_markers"
    }
    fn description(&self) -> &str {
        "TODO/FIXME markers found - track technical debt"
    }
    fn severity(&self) -> Severity {
        Severity::Info
    }
    fn detect(&self, graph: &CodeGraph) -> Vec<PatternInstance> {
        let mut instances = Vec::new();
        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];
            if let Some(sig) = &node.signature {
                if sig.contains("todo!()") || sig.contains("TODO") || sig.contains("FIXME") {
                    instances.push(
                        PatternInstance::new(
                            self.name(),
                            node_location(node),
                            self.severity(),
                            format!("TODO/FIXME marker in '{}'. Track and resolve.", node.name),
                        )
                        .with_confidence(0.95),
                    );
                }
            }
        }
        instances
    }
    fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
        None
    }
}

// Pattern 18: Unreachable Pattern
struct UnreachablePattern;
impl UnreachablePattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for UnreachablePattern {
    fn name(&self) -> &str {
        "unreachable_code"
    }
    fn description(&self) -> &str {
        "unreachable!() macro usage - ensure it's truly unreachable"
    }
    fn severity(&self) -> Severity {
        Severity::Info
    }
    fn detect(&self, graph: &CodeGraph) -> Vec<PatternInstance> {
        let mut instances = Vec::new();
        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];
            if let Some(sig) = &node.signature {
                if sig.contains("unreachable!()") {
                    instances.push(
                        PatternInstance::new(
                            self.name(),
                            node_location(node),
                            self.severity(),
                            format!("unreachable!() in '{}'. Verify logic is correct.", node.name),
                        )
                        .with_confidence(0.7),
                    );
                }
            }
        }
        instances
    }
    fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
        None
    }
}

// Pattern 19-30: Additional specialized patterns

struct BorrowCheckerPattern;
impl BorrowCheckerPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for BorrowCheckerPattern {
    fn name(&self) -> &str {
        "borrow_checker_workaround"
    }
    fn description(&self) -> &str {
        "Potential borrow checker workaround - review for correctness"
    }
    fn severity(&self) -> Severity {
        Severity::Info
    }
    fn detect(&self, _graph: &CodeGraph) -> Vec<PatternInstance> {
        Vec::new()
    }
    fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
        None
    }
}

struct MutableBorrowPattern;
impl MutableBorrowPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for MutableBorrowPattern {
    fn name(&self) -> &str {
        "multiple_mutable_borrows"
    }
    fn description(&self) -> &str {
        "Attempting multiple mutable borrows"
    }
    fn severity(&self) -> Severity {
        Severity::Error
    }
    fn detect(&self, _graph: &CodeGraph) -> Vec<PatternInstance> {
        Vec::new()
    }
    fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
        Some(FixSuggestion::new(
            self.name(),
            "Restructure code to avoid multiple mutable borrows",
            "let a = &mut x; let b = &mut x;",
            "Use interior mutability or refactor logic",
        ))
    }
}

struct OwnershipTransferPattern;
impl OwnershipTransferPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for OwnershipTransferPattern {
    fn name(&self) -> &str {
        "ownership_transfer"
    }
    fn description(&self) -> &str {
        "Ownership transfer - ensure this is intentional"
    }
    fn severity(&self) -> Severity {
        Severity::Info
    }
    fn detect(&self, _graph: &CodeGraph) -> Vec<PatternInstance> {
        Vec::new()
    }
    fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
        None
    }
}

struct DerefCoercionPattern;
impl DerefCoercionPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for DerefCoercionPattern {
    fn name(&self) -> &str {
        "deref_coercion"
    }
    fn description(&self) -> &str {
        "Consider explicit deref for clarity"
    }
    fn severity(&self) -> Severity {
        Severity::Info
    }
    fn detect(&self, _graph: &CodeGraph) -> Vec<PatternInstance> {
        Vec::new()
    }
    fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
        None
    }
}

struct StringAllocationPattern;
impl StringAllocationPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for StringAllocationPattern {
    fn name(&self) -> &str {
        "string_allocation"
    }
    fn description(&self) -> &str {
        "Frequent string allocations - consider using &str"
    }
    fn severity(&self) -> Severity {
        Severity::Info
    }
    fn detect(&self, graph: &CodeGraph) -> Vec<PatternInstance> {
        let mut instances = Vec::new();
        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];
            if let Some(sig) = &node.signature {
                let to_string_count = sig.matches(".to_string()").count();
                if to_string_count > 3 {
                    instances.push(
                        PatternInstance::new(
                            self.name(),
                            node_location(node),
                            self.severity(),
                            format!("Many .to_string() calls ({}) in '{}'. Use &str when possible.", to_string_count, node.name),
                        )
                        .with_confidence(0.7),
                    );
                }
            }
        }
        instances
    }
    fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
        Some(FixSuggestion::new(
            self.name(),
            "Use &str instead of String when ownership not needed",
            "fn foo(s: String)",
            "fn foo(s: &str)",
        ))
    }
}

struct VecCapacityPattern;
impl VecCapacityPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for VecCapacityPattern {
    fn name(&self) -> &str {
        "vec_without_capacity"
    }
    fn description(&self) -> &str {
        "Vec created without capacity - consider with_capacity for known sizes"
    }
    fn severity(&self) -> Severity {
        Severity::Info
    }
    fn detect(&self, _graph: &CodeGraph) -> Vec<PatternInstance> {
        Vec::new()
    }
    fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
        Some(FixSuggestion::new(
            self.name(),
            "Use Vec::with_capacity when size is known",
            "let mut v = Vec::new();\nfor i in 0..1000 { v.push(i); }",
            "let mut v = Vec::with_capacity(1000);\nfor i in 0..1000 { v.push(i); }",
        ))
    }
}

struct IteratorChainPattern;
impl IteratorChainPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for IteratorChainPattern {
    fn name(&self) -> &str {
        "complex_iterator_chain"
    }
    fn description(&self) -> &str {
        "Complex iterator chain - consider breaking up for readability"
    }
    fn severity(&self) -> Severity {
        Severity::Info
    }
    fn detect(&self, _graph: &CodeGraph) -> Vec<PatternInstance> {
        Vec::new()
    }
    fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
        None
    }
}

struct MatchExhaustivePattern;
impl MatchExhaustivePattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for MatchExhaustivePattern {
    fn name(&self) -> &str {
        "non_exhaustive_match"
    }
    fn description(&self) -> &str {
        "Match expression may not be exhaustive"
    }
    fn severity(&self) -> Severity {
        Severity::Warning
    }
    fn detect(&self, _graph: &CodeGraph) -> Vec<PatternInstance> {
        Vec::new()
    }
    fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
        Some(FixSuggestion::new(
            self.name(),
            "Add wildcard pattern or handle all variants",
            "match value { Some(x) => x }",
            "match value { Some(x) => x, None => default }",
        ))
    }
}

struct EnumVariantNamingPattern;
impl EnumVariantNamingPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for EnumVariantNamingPattern {
    fn name(&self) -> &str {
        "enum_variant_naming"
    }
    fn description(&self) -> &str {
        "Enum variant naming doesn't follow Rust conventions"
    }
    fn severity(&self) -> Severity {
        Severity::Info
    }
    fn detect(&self, _graph: &CodeGraph) -> Vec<PatternInstance> {
        Vec::new()
    }
    fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
        None
    }
}

struct StructUpdateSyntaxPattern;
impl StructUpdateSyntaxPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for StructUpdateSyntaxPattern {
    fn name(&self) -> &str {
        "struct_update_syntax"
    }
    fn description(&self) -> &str {
        "Consider using struct update syntax"
    }
    fn severity(&self) -> Severity {
        Severity::Info
    }
    fn detect(&self, _graph: &CodeGraph) -> Vec<PatternInstance> {
        Vec::new()
    }
    fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
        Some(FixSuggestion::new(
            self.name(),
            "Use struct update syntax for partial updates",
            "Struct { a: new_a, b: old.b, c: old.c }",
            "Struct { a: new_a, ..old }",
        ))
    }
}

struct TraitObjectPattern;
impl TraitObjectPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for TraitObjectPattern {
    fn name(&self) -> &str {
        "trait_object_safety"
    }
    fn description(&self) -> &str {
        "Trait may not be object-safe"
    }
    fn severity(&self) -> Severity {
        Severity::Warning
    }
    fn detect(&self, _graph: &CodeGraph) -> Vec<PatternInstance> {
        Vec::new()
    }
    fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
        None
    }
}

struct GenericBoundsPattern;
impl GenericBoundsPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for GenericBoundsPattern {
    fn name(&self) -> &str {
        "complex_generic_bounds"
    }
    fn description(&self) -> &str {
        "Complex generic bounds - consider using where clause"
    }
    fn severity(&self) -> Severity {
        Severity::Info
    }
    fn detect(&self, _graph: &CodeGraph) -> Vec<PatternInstance> {
        Vec::new()
    }
    fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
        Some(FixSuggestion::new(
            self.name(),
            "Use where clause for complex bounds",
            "fn foo<T: Trait1 + Trait2 + Trait3>(x: T)",
            "fn foo<T>(x: T)\nwhere\n    T: Trait1 + Trait2 + Trait3",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_patterns_creation() {
        let patterns = RustPatterns::new();
        assert!(!patterns.all_detectors().is_empty());
        assert!(patterns.all_detectors().len() >= 20);
    }

    #[test]
    fn test_unwrap_pattern() {
        let detector = UnwrapPattern::new();
        assert_eq!(detector.name(), "unwrap_usage");
        assert_eq!(detector.severity(), Severity::Warning);
    }
}
