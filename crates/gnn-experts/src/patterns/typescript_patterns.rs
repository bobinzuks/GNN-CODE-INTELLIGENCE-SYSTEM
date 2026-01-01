//! TypeScript-specific pattern detectors

use super::{PatternDetector, PatternInstance, FixSuggestion};
use crate::{CodeGraph, CodeNode, Location, Severity};
use std::sync::Arc;

pub struct TypeScriptPatterns {
    detectors: Vec<Arc<dyn PatternDetector>>,
}

impl TypeScriptPatterns {
    pub fn new() -> Self {
        let detectors: Vec<Arc<dyn PatternDetector>> = vec![
            Arc::new(AnyUsagePattern::new()),
            Arc::new(TypeAssertionPattern::new()),
            Arc::new(NonNullAssertionPattern::new()),
            Arc::new(ImplicitAnyPattern::new()),
            Arc::new(NullUndefinedCheckPattern::new()),
            Arc::new(InterfaceVsTypePattern::new()),
            Arc::new(EnumUsagePattern::new()),
            Arc::new(GenericConstraintPattern::new()),
            Arc::new(UnionTypePattern::new()),
            Arc::new(OptionalChainingPattern::new()),
            Arc::new(NullishCoalescingPattern::new()),
            Arc::new(ReadonlyPattern::new()),
            Arc::new(ConstAssertionPattern::new()),
            Arc::new(UtilityTypesPattern::new()),
            Arc::new(TypeGuardPattern::new()),
            Arc::new(DiscriminatedUnionPattern::new()),
            Arc::new(IndexSignaturePattern::new()),
            Arc::new(MappedTypePattern::new()),
            Arc::new(ConditionalTypePattern::new()),
            Arc::new(TemplateLiteralTypePattern::new()),
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

impl Default for TypeScriptPatterns {
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

macro_rules! impl_pattern {
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

struct AnyUsagePattern;
impl AnyUsagePattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for AnyUsagePattern {
    fn name(&self) -> &str {
        "any_usage"
    }
    fn description(&self) -> &str {
        "Avoid using 'any' type - defeats purpose of TypeScript"
    }
    fn severity(&self) -> Severity {
        Severity::Warning
    }
    fn detect(&self, graph: &CodeGraph) -> Vec<PatternInstance> {
        let mut instances = Vec::new();
        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];
            if let Some(sig) = &node.signature {
                if sig.contains(": any") || sig.contains("<any>") {
                    instances.push(
                        PatternInstance::new(
                            self.name(),
                            node_location(node),
                            self.severity(),
                            "'any' type found. Use specific types or 'unknown'".to_string(),
                        )
                        .with_confidence(0.9),
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
                "Use specific types or 'unknown'",
                "function foo(x: any)",
                "function foo(x: unknown) // or specific type",
            )
            .with_confidence(0.8),
        )
    }
}

impl_pattern!(
    TypeAssertionPattern,
    "type_assertion",
    "Excessive type assertions may indicate design issues",
    Severity::Info
);

impl_pattern!(
    NonNullAssertionPattern,
    "non_null_assertion",
    "Non-null assertion (!) bypasses type safety",
    Severity::Warning
);

impl_pattern!(
    ImplicitAnyPattern,
    "implicit_any",
    "Implicit any - enable noImplicitAny in tsconfig",
    Severity::Warning
);

impl_pattern!(
    NullUndefinedCheckPattern,
    "null_undefined_check",
    "Use strict null checks",
    Severity::Info
);

impl_pattern!(
    InterfaceVsTypePattern,
    "interface_vs_type",
    "Consider interface vs type alias",
    Severity::Info
);

impl_pattern!(
    EnumUsagePattern,
    "enum_usage",
    "Consider const enum or union type",
    Severity::Info
);

impl_pattern!(
    GenericConstraintPattern,
    "generic_constraint",
    "Add generic constraints for type safety",
    Severity::Info
);

impl_pattern!(
    UnionTypePattern,
    "union_type",
    "Complex union types - consider type guards",
    Severity::Info
);

impl_pattern!(
    OptionalChainingPattern,
    "optional_chaining",
    "Use optional chaining for safe property access",
    Severity::Info
);

impl_pattern!(
    NullishCoalescingPattern,
    "nullish_coalescing",
    "Use ?? instead of || for null/undefined",
    Severity::Info
);

impl_pattern!(
    ReadonlyPattern,
    "readonly_usage",
    "Consider readonly for immutable data",
    Severity::Info
);

impl_pattern!(
    ConstAssertionPattern,
    "const_assertion",
    "Use const assertion for literal types",
    Severity::Info
);

impl_pattern!(
    UtilityTypesPattern,
    "utility_types",
    "Use built-in utility types",
    Severity::Info
);

impl_pattern!(
    TypeGuardPattern,
    "type_guard",
    "Implement type guards for type narrowing",
    Severity::Info
);

impl_pattern!(
    DiscriminatedUnionPattern,
    "discriminated_union",
    "Use discriminated unions for type safety",
    Severity::Info
);

impl_pattern!(
    IndexSignaturePattern,
    "index_signature",
    "Avoid overly broad index signatures",
    Severity::Info
);

impl_pattern!(
    MappedTypePattern,
    "mapped_type",
    "Use mapped types for type transformations",
    Severity::Info
);

impl_pattern!(
    ConditionalTypePattern,
    "conditional_type",
    "Simplify complex conditional types",
    Severity::Info
);

impl_pattern!(
    TemplateLiteralTypePattern,
    "template_literal_type",
    "Use template literal types for string patterns",
    Severity::Info
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_typescript_patterns_creation() {
        let patterns = TypeScriptPatterns::new();
        assert!(patterns.all_detectors().len() >= 15);
    }
}
