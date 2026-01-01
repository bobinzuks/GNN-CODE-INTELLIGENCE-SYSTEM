//! C++-specific pattern detectors

use super::{PatternDetector, PatternInstance, FixSuggestion};
use crate::{CodeGraph, CodeNode, Location, Severity};
use std::sync::Arc;

pub struct CppPatterns {
    detectors: Vec<Arc<dyn PatternDetector>>,
}

impl CppPatterns {
    pub fn new() -> Self {
        let detectors: Vec<Arc<dyn PatternDetector>> = vec![
            Arc::new(MemoryLeakPattern::new()),
            Arc::new(RawPointerPattern::new()),
            Arc::new(DeleteMismatchPattern::new()),
            Arc::new(DoubleFreePattern::new()),
            Arc::new(UseAfterFreePattern::new()),
            Arc::new(RaiiViolationPattern::new()),
            Arc::new(MoveSemanticsPattern::new()),
            Arc::new(SmartPointerPattern::new()),
            Arc::new(VirtualDestructorPattern::new()),
            Arc::new(CopyConstructorPattern::new()),
            Arc::new(AssignmentOperatorPattern::new()),
            Arc::new(ResourceOwnershipPattern::new()),
            Arc::new(ExceptionSafetyPattern::new()),
            Arc::new(SlicingPattern::new()),
            Arc::new(ConstCorrectnessPattern::new()),
            Arc::new(IncludeGuardPattern::new()),
            Arc::new(HeaderOnlyPattern::new()),
            Arc::new(NamespacePattern::new()),
            Arc::new(AutoKeywordPattern::new()),
            Arc::new(RangeBasedForPattern::new()),
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

impl Default for CppPatterns {
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

struct MemoryLeakPattern;
impl MemoryLeakPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for MemoryLeakPattern {
    fn name(&self) -> &str {
        "memory_leak"
    }
    fn description(&self) -> &str {
        "Potential memory leak - new without corresponding delete"
    }
    fn severity(&self) -> Severity {
        Severity::Error
    }
    fn detect(&self, graph: &CodeGraph) -> Vec<PatternInstance> {
        let mut instances = Vec::new();
        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];
            if let Some(sig) = &node.signature {
                if sig.contains("new ") && !sig.contains("delete") && !sig.contains("unique_ptr") {
                    instances.push(
                        PatternInstance::new(
                            self.name(),
                            node_location(node),
                            self.severity(),
                            "Potential memory leak. Use smart pointers or ensure delete is called.".to_string(),
                        )
                        .with_confidence(0.75),
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
                "Use smart pointers instead of raw new/delete",
                "int* ptr = new int(42);",
                "auto ptr = std::make_unique<int>(42);",
            )
            .with_confidence(0.9)
            .automated(),
        )
    }
}

struct RawPointerPattern;
impl RawPointerPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for RawPointerPattern {
    fn name(&self) -> &str {
        "raw_pointer_ownership"
    }
    fn description(&self) -> &str {
        "Raw pointer with unclear ownership - use smart pointers"
    }
    fn severity(&self) -> Severity {
        Severity::Warning
    }
    fn detect(&self, _graph: &CodeGraph) -> Vec<PatternInstance> {
        Vec::new()
    }
    fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
        Some(
            FixSuggestion::new(
                self.name(),
                "Use smart pointers for ownership semantics",
                "T* ptr;",
                "std::unique_ptr<T> ptr; // or std::shared_ptr<T>",
            )
            .with_confidence(0.8),
        )
    }
}

struct DeleteMismatchPattern;
impl DeleteMismatchPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for DeleteMismatchPattern {
    fn name(&self) -> &str {
        "delete_mismatch"
    }
    fn description(&self) -> &str {
        "new[] must be paired with delete[], not delete"
    }
    fn severity(&self) -> Severity {
        Severity::Error
    }
    fn detect(&self, graph: &CodeGraph) -> Vec<PatternInstance> {
        let mut instances = Vec::new();
        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];
            if let Some(sig) = &node.signature {
                if sig.contains("new[]") && sig.contains("delete ") && !sig.contains("delete[]") {
                    instances.push(
                        PatternInstance::new(
                            self.name(),
                            node_location(node),
                            self.severity(),
                            "new[] paired with delete instead of delete[]. Undefined behavior!".to_string(),
                        )
                        .with_confidence(0.95),
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
                "Use delete[] for arrays or use std::vector",
                "int* arr = new int[10];\ndelete arr;",
                "int* arr = new int[10];\ndelete[] arr;\n// OR\nstd::vector<int> arr(10);",
            )
            .with_confidence(0.95)
            .automated(),
        )
    }
}

struct DoubleFreePattern;
impl DoubleFreePattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for DoubleFreePattern {
    fn name(&self) -> &str {
        "double_free"
    }
    fn description(&self) -> &str {
        "Potential double free - pointer deleted twice"
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
                "Set pointer to nullptr after delete",
                "delete ptr;",
                "delete ptr;\nptr = nullptr;",
            )
            .with_confidence(0.85),
        )
    }
}

struct UseAfterFreePattern;
impl UseAfterFreePattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for UseAfterFreePattern {
    fn name(&self) -> &str {
        "use_after_free"
    }
    fn description(&self) -> &str {
        "Potential use-after-free - pointer used after delete"
    }
    fn severity(&self) -> Severity {
        Severity::Error
    }
    fn detect(&self, _graph: &CodeGraph) -> Vec<PatternInstance> {
        Vec::new()
    }
    fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
        None
    }
}

struct RaiiViolationPattern;
impl RaiiViolationPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for RaiiViolationPattern {
    fn name(&self) -> &str {
        "raii_violation"
    }
    fn description(&self) -> &str {
        "Resource not managed via RAII"
    }
    fn severity(&self) -> Severity {
        Severity::Warning
    }
    fn detect(&self, _graph: &CodeGraph) -> Vec<PatternInstance> {
        Vec::new()
    }
    fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
        Some(
            FixSuggestion::new(
                self.name(),
                "Wrap resource in RAII class",
                "FILE* f = fopen(\"file\", \"r\");\n// use f\nfclose(f);",
                "std::ifstream f(\"file\");\n// use f\n// automatically closed",
            )
            .with_confidence(0.8),
        )
    }
}

struct MoveSemanticsPattern;
impl MoveSemanticsPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for MoveSemanticsPattern {
    fn name(&self) -> &str {
        "move_semantics"
    }
    fn description(&self) -> &str {
        "Consider using move semantics for efficiency"
    }
    fn severity(&self) -> Severity {
        Severity::Info
    }
    fn detect(&self, _graph: &CodeGraph) -> Vec<PatternInstance> {
        Vec::new()
    }
    fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
        Some(
            FixSuggestion::new(
                self.name(),
                "Use std::move for efficiency",
                "std::vector<int> copy = original;",
                "std::vector<int> moved = std::move(original);",
            )
            .with_confidence(0.7),
        )
    }
}

macro_rules! impl_cpp_pattern {
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

impl_cpp_pattern!(
    SmartPointerPattern,
    "smart_pointer_choice",
    "Choose appropriate smart pointer (unique_ptr vs shared_ptr)",
    Severity::Info
);

impl_cpp_pattern!(
    VirtualDestructorPattern,
    "virtual_destructor",
    "Base class needs virtual destructor",
    Severity::Error
);

impl_cpp_pattern!(
    CopyConstructorPattern,
    "rule_of_five",
    "Implement rule of five (copy/move constructor/assignment, destructor)",
    Severity::Warning
);

impl_cpp_pattern!(
    AssignmentOperatorPattern,
    "assignment_operator",
    "Assignment operator should return reference to *this",
    Severity::Warning
);

impl_cpp_pattern!(
    ResourceOwnershipPattern,
    "resource_ownership",
    "Unclear resource ownership semantics",
    Severity::Warning
);

impl_cpp_pattern!(
    ExceptionSafetyPattern,
    "exception_safety",
    "Code may not be exception-safe",
    Severity::Warning
);

impl_cpp_pattern!(
    SlicingPattern,
    "object_slicing",
    "Potential object slicing when passing by value",
    Severity::Warning
);

impl_cpp_pattern!(
    ConstCorrectnessPattern,
    "const_correctness",
    "Add const qualifier for const correctness",
    Severity::Info
);

impl_cpp_pattern!(
    IncludeGuardPattern,
    "include_guard",
    "Use #pragma once or include guards",
    Severity::Info
);

impl_cpp_pattern!(
    HeaderOnlyPattern,
    "header_only",
    "Function defined in header without inline",
    Severity::Warning
);

impl_cpp_pattern!(
    NamespacePattern,
    "namespace_usage",
    "Avoid 'using namespace' in headers",
    Severity::Warning
);

impl_cpp_pattern!(
    AutoKeywordPattern,
    "auto_keyword",
    "Consider using auto for type deduction",
    Severity::Info
);

impl_cpp_pattern!(
    RangeBasedForPattern,
    "range_based_for",
    "Use range-based for loop",
    Severity::Info
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpp_patterns_creation() {
        let patterns = CppPatterns::new();
        assert!(patterns.all_detectors().len() >= 15);
    }
}
