//! Java-specific pattern detectors

use super::{PatternDetector, PatternInstance, FixSuggestion};
use crate::{CodeGraph, CodeNode, Location, Severity};
use std::sync::Arc;

pub struct JavaPatterns {
    detectors: Vec<Arc<dyn PatternDetector>>,
}

impl JavaPatterns {
    pub fn new() -> Self {
        let detectors: Vec<Arc<dyn PatternDetector>> = vec![
            Arc::new(NullCheckPattern::new()),
            Arc::new(ExceptionCatchingPattern::new()),
            Arc::new(EmptyCatchBlockPattern::new()),
            Arc::new(ResourceLeakPattern::new()),
            Arc::new(StreamNotClosedPattern::new()),
            Arc::new(OptionalMisusePattern::new()),
            Arc::new(StringConcatenationPattern::new()),
            Arc::new(EqualsHashCodePattern::new()),
            Arc::new(FinalizeMethodPattern::new()),
            Arc::new(SynchronizedPattern::new()),
            Arc::new(VolatilePattern::new()),
            Arc::new(DoubleLockingPattern::new()),
            Arc::new(RawTypePattern::new()),
            Arc::new(UncheckedCastPattern::new()),
            Arc::new(SerializablePattern::new()),
            Arc::new(CloneablePattern::new()),
            Arc::new(CompareToPattern::new()),
            Arc::new(CollectionToArrayPattern::new()),
            Arc::new(BigDecimalEqualsPattern::new()),
            Arc::new(DateTimeUsagePattern::new()),
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

impl Default for JavaPatterns {
    fn default() -> Self {
        Self::new()
    }
}

fn node_location(node: &CodeNode) -> Location {
    Location {
        file_path: node.file_path.clone().unwrap_or_else(|| "unknown".to_string()),
        start_line: node.start_line,
        end_line: node.end_line,
        start_col: node.start_col,
        end_col: node.end_col,
    }
}

struct NullCheckPattern;
impl NullCheckPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for NullCheckPattern {
    fn name(&self) -> &str {
        "null_check"
    }
    fn description(&self) -> &str {
        "Potential NullPointerException - add null checks"
    }
    fn severity(&self) -> Severity {
        Severity::Warning
    }
    fn detect(&self, graph: &CodeGraph) -> Vec<PatternInstance> {
        let mut instances = Vec::new();
        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];
            if let Some(sig) = &node.signature {
                if sig.contains(".") && !sig.contains("== null") && !sig.contains("!= null") {
                    instances.push(
                        PatternInstance::new(
                            self.name(),
                            node_location(node),
                            self.severity(),
                            "Potential null dereference. Add null check or use Optional.".to_string(),
                        )
                        .with_confidence(0.6),
                    );
                }
            }
        }
        instances
    }
    fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
        Some(
            FixSuggestion::new(
                self.name(),
                "Add null check or use Optional",
                "obj.method()",
                "if (obj != null) {\n    obj.method();\n}\n// OR\nOptional.ofNullable(obj).ifPresent(o -> o.method());",
            )
            .with_confidence(0.7),
        )
    }
}

struct ExceptionCatchingPattern;
impl ExceptionCatchingPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for ExceptionCatchingPattern {
    fn name(&self) -> &str {
        "broad_exception_catch"
    }
    fn description(&self) -> &str {
        "Catching Exception or Throwable too broadly"
    }
    fn severity(&self) -> Severity {
        Severity::Warning
    }
    fn detect(&self, graph: &CodeGraph) -> Vec<PatternInstance> {
        let mut instances = Vec::new();
        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];
            if let Some(sig) = &node.signature {
                if sig.contains("catch (Exception") || sig.contains("catch (Throwable") {
                    instances.push(
                        PatternInstance::new(
                            self.name(),
                            node_location(node),
                            self.severity(),
                            "Catching Exception/Throwable too broadly. Catch specific exceptions.".to_string(),
                        )
                        .with_confidence(0.85),
                    );
                }
            }
        }
        instances
    }
    fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
        Some(
            FixSuggestion::new(
                self.name(),
                "Catch specific exception types",
                "catch (Exception e)",
                "catch (IOException | SQLException e)",
            )
            .with_confidence(0.8),
        )
    }
}

struct EmptyCatchBlockPattern;
impl EmptyCatchBlockPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for EmptyCatchBlockPattern {
    fn name(&self) -> &str {
        "empty_catch"
    }
    fn description(&self) -> &str {
        "Empty catch block swallows exceptions"
    }
    fn severity(&self) -> Severity {
        Severity::Error
    }
    fn detect(&self, _graph: &CodeGraph) -> Vec<PatternInstance> {
        Vec::new()
    }
    fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
        Some(
            FixSuggestion::new(
                self.name(),
                "Log exceptions or handle appropriately",
                "catch (Exception e) { }",
                "catch (Exception e) {\n    logger.error(\"Error occurred\", e);\n}",
            )
            .with_confidence(0.9),
        )
    }
}

struct ResourceLeakPattern;
impl ResourceLeakPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for ResourceLeakPattern {
    fn name(&self) -> &str {
        "resource_leak"
    }
    fn description(&self) -> &str {
        "Resource not closed - use try-with-resources"
    }
    fn severity(&self) -> Severity {
        Severity::Error
    }
    fn detect(&self, _graph: &CodeGraph) -> Vec<PatternInstance> {
        Vec::new()
    }
    fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
        Some(
            FixSuggestion::new(
                self.name(),
                "Use try-with-resources",
                "FileReader fr = new FileReader(\"file.txt\");\n// use fr\nfr.close();",
                "try (FileReader fr = new FileReader(\"file.txt\")) {\n    // use fr\n}",
            )
            .with_confidence(0.95)
            .automated(),
        )
    }
}

macro_rules! impl_java_pattern {
    ($name:ident, $pattern_name:expr, $desc:expr, $severity:expr) => {
        struct $name;
        impl $name {
            fn new() -> Self {
                Self
            }
        }
        impl PatternDetector for $name {
            fn name(&self) -> &str {
                $pattern_name
            }
            fn description(&self) -> &str {
                $desc
            }
            fn severity(&self) -> Severity {
                $severity
            }
            fn detect(&self, _graph: &CodeGraph) -> Vec<PatternInstance> {
                Vec::new()
            }
            fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
                None
            }
        }
    };
}

impl_java_pattern!(
    StreamNotClosedPattern,
    "stream_not_closed",
    "Stream should be closed after use",
    Severity::Warning
);

impl_java_pattern!(
    OptionalMisusePattern,
    "optional_misuse",
    "Optional.get() without isPresent() check",
    Severity::Error
);

impl_java_pattern!(
    StringConcatenationPattern,
    "string_concatenation",
    "Use StringBuilder for string concatenation in loops",
    Severity::Warning
);

impl_java_pattern!(
    EqualsHashCodePattern,
    "equals_hashcode",
    "Override both equals() and hashCode() together",
    Severity::Warning
);

impl_java_pattern!(
    FinalizeMethodPattern,
    "finalize_method",
    "finalize() is deprecated - use try-with-resources",
    Severity::Warning
);

impl_java_pattern!(
    SynchronizedPattern,
    "synchronized_usage",
    "Review synchronized block scope",
    Severity::Info
);

impl_java_pattern!(
    VolatilePattern,
    "volatile_usage",
    "Ensure volatile is necessary and sufficient",
    Severity::Info
);

impl_java_pattern!(
    DoubleLockingPattern,
    "double_checked_locking",
    "Double-checked locking may be broken",
    Severity::Warning
);

impl_java_pattern!(
    RawTypePattern,
    "raw_type",
    "Use generic type parameters instead of raw types",
    Severity::Warning
);

impl_java_pattern!(
    UncheckedCastPattern,
    "unchecked_cast",
    "Unchecked cast may fail at runtime",
    Severity::Warning
);

impl_java_pattern!(
    SerializablePattern,
    "serializable_issues",
    "Serializable implementation issues",
    Severity::Info
);

impl_java_pattern!(
    CloneablePattern,
    "cloneable_issues",
    "Cloneable implementation issues",
    Severity::Info
);

impl_java_pattern!(
    CompareToPattern,
    "compareto_implementation",
    "Ensure compareTo() is consistent with equals()",
    Severity::Warning
);

impl_java_pattern!(
    CollectionToArrayPattern,
    "collection_to_array",
    "Use typed array in toArray() call",
    Severity::Info
);

impl_java_pattern!(
    BigDecimalEqualsPattern,
    "bigdecimal_equals",
    "Use compareTo() instead of equals() for BigDecimal",
    Severity::Warning
);

impl_java_pattern!(
    DateTimeUsagePattern,
    "legacy_datetime",
    "Use java.time API instead of Date/Calendar",
    Severity::Info
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_java_patterns_creation() {
        let patterns = JavaPatterns::new();
        assert!(patterns.all_detectors().len() >= 15);
    }
}
