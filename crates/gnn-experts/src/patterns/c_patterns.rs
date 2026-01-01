//! C-specific pattern detectors

use super::{PatternDetector, PatternInstance, FixSuggestion};
use crate::{CodeGraph, CodeNode, Location, Severity};
use std::sync::Arc;

pub struct CPatterns {
    detectors: Vec<Arc<dyn PatternDetector>>,
}

impl CPatterns {
    pub fn new() -> Self {
        let detectors: Vec<Arc<dyn PatternDetector>> = vec![
            Arc::new(BufferOverflowPattern::new()),
            Arc::new(NullPointerDereferencePattern::new()),
            Arc::new(MemoryLeakPattern::new()),
            Arc::new(DoubleFreePattern::new()),
            Arc::new(UseAfterFreePattern::new()),
            Arc::new(UninitializedVariablePattern::new()),
            Arc::new(DanglingPointerPattern::new()),
            Arc::new(IntegerOverflowPattern::new()),
            Arc::new(SignedUnsignedMismatchPattern::new()),
            Arc::new(FormatStringVulnPattern::new()),
            Arc::new(UncheckedReturnPattern::new()),
            Arc::new(GetsUsagePattern::new()),
            Arc::new(StrcpyUsagePattern::new()),
            Arc::new(SprintfUsagePattern::new()),
            Arc::new(MallocSizeofPattern::new()),
            Arc::new(FreeNullCheckPattern::new()),
            Arc::new(PointerArithmeticPattern::new()),
            Arc::new(CastingPattern::new()),
            Arc::new(MacroSideEffectPattern::new()),
            Arc::new(IncludeGuardPattern::new()),
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

impl Default for CPatterns {
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

struct BufferOverflowPattern;
impl BufferOverflowPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for BufferOverflowPattern {
    fn name(&self) -> &str {
        "buffer_overflow"
    }
    fn description(&self) -> &str {
        "Potential buffer overflow - bounds not checked"
    }
    fn severity(&self) -> Severity {
        Severity::Error
    }
    fn detect(&self, graph: &CodeGraph) -> Vec<PatternInstance> {
        let mut instances = Vec::new();
        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];
            if let Some(sig) = &node.signature {
                let unsafe_funcs = ["strcpy", "strcat", "sprintf", "gets"];
                for func in &unsafe_funcs {
                    if sig.contains(func) {
                        instances.push(
                            PatternInstance::new(
                                self.name(),
                                node_location(node),
                                self.severity(),
                                format!("Unsafe function '{}' can cause buffer overflow. Use safe alternative.", func),
                            )
                            .with_confidence(0.95)
                            .with_metadata("unsafe_function", func.to_string()),
                        );
                    }
                }
            }
        }
        instances
    }
    fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
        Some(
            FixSuggestion::new(
                self.name(),
                "Use safe alternatives",
                "strcpy(dest, src);\nstrcat(dest, src);\nsprintf(buf, fmt, ...);\ngets(buf);",
                "strncpy(dest, src, sizeof(dest));\nstrncat(dest, src, sizeof(dest)-strlen(dest)-1);\nsnprintf(buf, sizeof(buf), fmt, ...);\nfgets(buf, sizeof(buf), stdin);",
            )
            .with_confidence(0.95)
            .automated(),
        )
    }
}

struct NullPointerDereferencePattern;
impl NullPointerDereferencePattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for NullPointerDereferencePattern {
    fn name(&self) -> &str {
        "null_pointer_dereference"
    }
    fn description(&self) -> &str {
        "Pointer dereferenced without NULL check"
    }
    fn severity(&self) -> Severity {
        Severity::Error
    }
    fn detect(&self, graph: &CodeGraph) -> Vec<PatternInstance> {
        let mut instances = Vec::new();
        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];
            if let Some(sig) = &node.signature {
                if (sig.contains("->") || sig.contains("*ptr")) && !sig.contains("!= NULL") && !sig.contains("== NULL") {
                    instances.push(
                        PatternInstance::new(
                            self.name(),
                            node_location(node),
                            self.severity(),
                            "Pointer dereference without NULL check. Add validation.".to_string(),
                        )
                        .with_confidence(0.7),
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
                "Check for NULL before dereferencing",
                "ptr->field = value;",
                "if (ptr != NULL) {\n    ptr->field = value;\n}",
            )
            .with_confidence(0.85),
        )
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
        "malloc/calloc without corresponding free"
    }
    fn severity(&self) -> Severity {
        Severity::Error
    }
    fn detect(&self, graph: &CodeGraph) -> Vec<PatternInstance> {
        let mut instances = Vec::new();
        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];
            if let Some(sig) = &node.signature {
                if (sig.contains("malloc(") || sig.contains("calloc(")) && !sig.contains("free(") {
                    instances.push(
                        PatternInstance::new(
                            self.name(),
                            node_location(node),
                            self.severity(),
                            "Memory allocated but no corresponding free() found. Potential leak.".to_string(),
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
                "Ensure every malloc has a corresponding free",
                "int* ptr = malloc(sizeof(int) * 10);",
                "int* ptr = malloc(sizeof(int) * 10);\nif (ptr == NULL) return ERROR;\n// use ptr\nfree(ptr);",
            )
            .with_confidence(0.85),
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
        "Pointer potentially freed twice"
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
                "Set pointer to NULL after free",
                "free(ptr);",
                "free(ptr);\nptr = NULL;",
            )
            .with_confidence(0.9),
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
        "Pointer used after being freed"
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

struct UninitializedVariablePattern;
impl UninitializedVariablePattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for UninitializedVariablePattern {
    fn name(&self) -> &str {
        "uninitialized_variable"
    }
    fn description(&self) -> &str {
        "Variable used before initialization"
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
                "Initialize variables at declaration",
                "int x;\nprintf(\"%d\", x);",
                "int x = 0;\nprintf(\"%d\", x);",
            )
            .with_confidence(0.9),
        )
    }
}

macro_rules! impl_c_pattern {
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

impl_c_pattern!(
    DanglingPointerPattern,
    "dangling_pointer",
    "Pointer to local variable returned",
    Severity::Error
);

impl_c_pattern!(
    IntegerOverflowPattern,
    "integer_overflow",
    "Potential integer overflow",
    Severity::Warning
);

impl_c_pattern!(
    SignedUnsignedMismatchPattern,
    "signed_unsigned_mismatch",
    "Mixing signed and unsigned types",
    Severity::Warning
);

impl_c_pattern!(
    FormatStringVulnPattern,
    "format_string_vuln",
    "Format string vulnerability",
    Severity::Error
);

impl_c_pattern!(
    UncheckedReturnPattern,
    "unchecked_return",
    "Function return value not checked",
    Severity::Warning
);

impl_c_pattern!(
    GetsUsagePattern,
    "gets_usage",
    "gets() is unsafe - use fgets()",
    Severity::Error
);

impl_c_pattern!(
    StrcpyUsagePattern,
    "strcpy_usage",
    "strcpy() is unsafe - use strncpy()",
    Severity::Error
);

impl_c_pattern!(
    SprintfUsagePattern,
    "sprintf_usage",
    "sprintf() is unsafe - use snprintf()",
    Severity::Error
);

impl_c_pattern!(
    MallocSizeofPattern,
    "malloc_sizeof",
    "Ensure correct sizeof in malloc",
    Severity::Warning
);

impl_c_pattern!(
    FreeNullCheckPattern,
    "free_null_check",
    "Check for NULL before free (though free(NULL) is safe)",
    Severity::Info
);

impl_c_pattern!(
    PointerArithmeticPattern,
    "pointer_arithmetic",
    "Pointer arithmetic may cause out-of-bounds access",
    Severity::Warning
);

impl_c_pattern!(
    CastingPattern,
    "unsafe_casting",
    "Unsafe type casting",
    Severity::Warning
);

impl_c_pattern!(
    MacroSideEffectPattern,
    "macro_side_effect",
    "Macro argument evaluated multiple times",
    Severity::Warning
);

impl_c_pattern!(
    IncludeGuardPattern,
    "include_guard",
    "Header file needs include guards",
    Severity::Info
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c_patterns_creation() {
        let patterns = CPatterns::new();
        assert!(patterns.all_detectors().len() >= 15);
    }
}
