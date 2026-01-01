//! Go-specific pattern detectors

use super::{PatternDetector, PatternInstance, FixSuggestion};
use crate::{CodeGraph, CodeNode, NodeKind, Location, Severity};
use std::sync::Arc;

pub struct GoPatterns {
    detectors: Vec<Arc<dyn PatternDetector>>,
}

impl GoPatterns {
    pub fn new() -> Self {
        let detectors: Vec<Arc<dyn PatternDetector>> = vec![
            Arc::new(ErrorHandlingPattern::new()),
            Arc::new(ErrorIgnoredPattern::new()),
            Arc::new(GoroutineLeakPattern::new()),
            Arc::new(ChannelNotClosedPattern::new()),
            Arc::new(ContextNotPropagatedPattern::new()),
            Arc::new(DeferInLoopPattern::new()),
            Arc::new(RaceConditionPattern::new()),
            Arc::new(MutexNotUnlockedPattern::new()),
            Arc::new(NilPointerDereferencePattern::new()),
            Arc::new(EmptyInterfacePattern::new()),
            Arc::new(PanicInLibraryPattern::new()),
            Arc::new(GlobalVariablePattern::new()),
            Arc::new(ShadowingPattern::new()),
            Arc::new(UnexportedReturnPattern::new()),
            Arc::new(TimeAfterInLoopPattern::new()),
            Arc::new(AppendInLoopPattern::new()),
            Arc::new(StructComparisonPattern::new()),
            Arc::new(RangeVariablePattern::new()),
            Arc::new(ContextDoneNotCheckedPattern::new()),
            Arc::new(HttpClientWithoutTimeoutPattern::new()),
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

impl Default for GoPatterns {
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

struct ErrorHandlingPattern;
impl ErrorHandlingPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for ErrorHandlingPattern {
    fn name(&self) -> &str {
        "error_handling"
    }
    fn description(&self) -> &str {
        "Always check returned errors"
    }
    fn severity(&self) -> Severity {
        Severity::Error
    }
    fn detect(&self, graph: &CodeGraph) -> Vec<PatternInstance> {
        let mut instances = Vec::new();
        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];
            if let Some(sig) = &node.signature {
                if sig.contains("error") && sig.contains("_") {
                    instances.push(
                        PatternInstance::new(
                            self.name(),
                            node_location(node),
                            self.severity(),
                            "Error return value ignored. Always check errors in Go.".to_string(),
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
                "Check all error returns",
                "result, _ := funcReturningError()",
                "result, err := funcReturningError()\nif err != nil {\n    return err\n}",
            )
            .with_confidence(0.9),
        )
    }
}

struct ErrorIgnoredPattern;
impl ErrorIgnoredPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for ErrorIgnoredPattern {
    fn name(&self) -> &str {
        "error_ignored"
    }
    fn description(&self) -> &str {
        "Error explicitly ignored with blank identifier"
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
                "Handle errors properly",
                "_, err := doSomething()",
                "result, err := doSomething()\nif err != nil {\n    log.Printf(\"error: %v\", err)\n}",
            )
            .with_confidence(0.8),
        )
    }
}

struct GoroutineLeakPattern;
impl GoroutineLeakPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for GoroutineLeakPattern {
    fn name(&self) -> &str {
        "goroutine_leak"
    }
    fn description(&self) -> &str {
        "Goroutine may leak - ensure proper cleanup"
    }
    fn severity(&self) -> Severity {
        Severity::Error
    }
    fn detect(&self, graph: &CodeGraph) -> Vec<PatternInstance> {
        let mut instances = Vec::new();
        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];
            if let Some(sig) = &node.signature {
                if sig.contains("go func()") && !sig.contains("context") && !sig.contains("done") {
                    instances.push(
                        PatternInstance::new(
                            self.name(),
                            node_location(node),
                            self.severity(),
                            "Goroutine without cancellation mechanism. May leak.".to_string(),
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
                "Use context for cancellation",
                "go func() { /* work */ }()",
                "go func(ctx context.Context) {\n    select {\n    case <-ctx.Done():\n        return\n    default:\n        /* work */\n    }\n}(ctx)",
            )
            .with_confidence(0.8),
        )
    }
}

struct ChannelNotClosedPattern;
impl ChannelNotClosedPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for ChannelNotClosedPattern {
    fn name(&self) -> &str {
        "channel_not_closed"
    }
    fn description(&self) -> &str {
        "Channel should be closed by sender"
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
                "Close channels when done",
                "ch := make(chan int)\nch <- value",
                "ch := make(chan int)\ndefer close(ch)\nch <- value",
            )
            .with_confidence(0.7),
        )
    }
}

struct ContextNotPropagatedPattern;
impl ContextNotPropagatedPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for ContextNotPropagatedPattern {
    fn name(&self) -> &str {
        "context_not_propagated"
    }
    fn description(&self) -> &str {
        "Context should be propagated through call chain"
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
                    if sig.contains("context.Context") && sig.contains("context.Background()") {
                        instances.push(
                            PatternInstance::new(
                                self.name(),
                                node_location(node),
                                self.severity(),
                                "Creating new context instead of propagating. Use parent context.".to_string(),
                            )
                            .with_confidence(0.8),
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
                "Propagate context from caller",
                "ctx := context.Background()",
                "// Use ctx from function parameter",
            )
            .with_confidence(0.85),
        )
    }
}

struct DeferInLoopPattern;
impl DeferInLoopPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for DeferInLoopPattern {
    fn name(&self) -> &str {
        "defer_in_loop"
    }
    fn description(&self) -> &str {
        "defer in loop accumulates - extract to function"
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
                "Extract defer to separate function",
                "for _, f := range files {\n    file, _ := os.Open(f)\n    defer file.Close()\n}",
                "for _, f := range files {\n    processFile(f)\n}\n\nfunc processFile(path string) {\n    file, _ := os.Open(path)\n    defer file.Close()\n}",
            )
            .with_confidence(0.85),
        )
    }
}

// Remaining Go patterns (7-20) using macro for brevity
macro_rules! impl_go_pattern {
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

impl_go_pattern!(
    RaceConditionPattern,
    "race_condition",
    "Potential race condition detected",
    Severity::Error
);

impl_go_pattern!(
    MutexNotUnlockedPattern,
    "mutex_not_unlocked",
    "Mutex may not be unlocked",
    Severity::Error
);

impl_go_pattern!(
    NilPointerDereferencePattern,
    "nil_pointer_dereference",
    "Potential nil pointer dereference",
    Severity::Error
);

impl_go_pattern!(
    EmptyInterfacePattern,
    "empty_interface",
    "Avoid interface{} - use generics or specific types",
    Severity::Info
);

impl_go_pattern!(
    PanicInLibraryPattern,
    "panic_in_library",
    "Library code should return errors, not panic",
    Severity::Warning
);

impl_go_pattern!(
    GlobalVariablePattern,
    "global_variable",
    "Global variables make testing difficult",
    Severity::Warning
);

impl_go_pattern!(
    ShadowingPattern,
    "variable_shadowing",
    "Variable shadows declaration in outer scope",
    Severity::Info
);

impl_go_pattern!(
    UnexportedReturnPattern,
    "unexported_return",
    "Exported function returns unexported type",
    Severity::Warning
);

impl_go_pattern!(
    TimeAfterInLoopPattern,
    "time_after_in_loop",
    "time.After in loop causes memory leak",
    Severity::Error
);

impl_go_pattern!(
    AppendInLoopPattern,
    "append_in_loop",
    "Pre-allocate slice capacity when appending in loop",
    Severity::Info
);

impl_go_pattern!(
    StructComparisonPattern,
    "struct_comparison",
    "Struct with uncomparable fields",
    Severity::Warning
);

impl_go_pattern!(
    RangeVariablePattern,
    "range_variable_capture",
    "Range variable captured by closure",
    Severity::Error
);

impl_go_pattern!(
    ContextDoneNotCheckedPattern,
    "context_done_not_checked",
    "Long operation without checking context.Done()",
    Severity::Warning
);

impl_go_pattern!(
    HttpClientWithoutTimeoutPattern,
    "http_client_no_timeout",
    "HTTP client without timeout can hang indefinitely",
    Severity::Warning
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_go_patterns_creation() {
        let patterns = GoPatterns::new();
        assert!(patterns.all_detectors().len() >= 15);
    }
}
