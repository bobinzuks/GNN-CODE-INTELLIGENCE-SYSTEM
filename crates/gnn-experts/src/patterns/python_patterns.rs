//! Python-specific pattern detectors
//!
//! Comprehensive set of pattern detectors for Python code including:
//! - Exception handling patterns
//! - Import patterns
//! - Type hint patterns
//! - Async/await patterns
//! - Global variable patterns
//! - Mutable default arguments

use super::{PatternDetector, PatternInstance, FixSuggestion};
use crate::{CodeGraph, CodeNode, NodeKind, Location, Severity};
use std::sync::Arc;

pub struct PythonPatterns {
    detectors: Vec<Arc<dyn PatternDetector>>,
}

impl PythonPatterns {
    pub fn new() -> Self {
        let detectors: Vec<Arc<dyn PatternDetector>> = vec![
            Arc::new(BareExceptPattern::new()),
            Arc::new(ExceptPassPattern::new()),
            Arc::new(BroadExceptionPattern::new()),
            Arc::new(GlobalVariablePattern::new()),
            Arc::new(MutableDefaultArgumentPattern::new()),
            Arc::new(MissingTypeHintsPattern::new()),
            Arc::new(IncompleteTypeHintsPattern::new()),
            Arc::new(StarImportPattern::new()),
            Arc::new(UnusedImportPattern::new()),
            Arc::new(ImportOrderPattern::new()),
            Arc::new(AsyncWithoutAwaitPattern::new()),
            Arc::new(BlockingInAsyncPattern::new()),
            Arc::new(AsyncGeneratorPattern::new()),
            Arc::new(ListComprehensionPattern::new()),
            Arc::new(DictGetPattern::new()),
            Arc::new(StringConcatenationPattern::new()),
            Arc::new(PlusEqualListPattern::new()),
            Arc::new(IsNonePattern::new()),
            Arc::new(LenCheckPattern::new()),
            Arc::new(RangeLoopPattern::new()),
            Arc::new(OpenWithoutContextPattern::new()),
            Arc::new(AssertInProductionPattern::new()),
            Arc::new(PrintStatementPattern::new()),
            Arc::new(ClassMethodDecoratorPattern::new()),
            Arc::new(PrivateMethodPattern::new()),
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

impl Default for PythonPatterns {
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

// Pattern 1: Bare Except
struct BareExceptPattern;
impl BareExceptPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for BareExceptPattern {
    fn name(&self) -> &str {
        "bare_except"
    }
    fn description(&self) -> &str {
        "Bare except clause catches all exceptions including system exits"
    }
    fn severity(&self) -> Severity {
        Severity::Error
    }
    fn detect(&self, graph: &CodeGraph) -> Vec<PatternInstance> {
        let mut instances = Vec::new();
        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];
            if let Some(sig) = &node.signature {
                if sig.contains("except:") {
                    instances.push(
                        PatternInstance::new(
                            self.name(),
                            node_location(node),
                            self.severity(),
                            format!("Bare except clause in '{}'. Specify exception type.", node.name),
                        )
                        .with_context(sig.clone())
                        .with_confidence(0.95),
                    );
                }
            }
        }
        instances
    }
    fn confidence(&self) -> f32 {
        0.95
    }
    fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
        Some(
            FixSuggestion::new(
                self.name(),
                "Specify exception type or use Exception",
                "except:",
                "except Exception as e:",
            )
            .with_confidence(0.9)
            .automated(),
        )
    }
}

// Pattern 2: Except Pass
struct ExceptPassPattern;
impl ExceptPassPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for ExceptPassPattern {
    fn name(&self) -> &str {
        "except_pass"
    }
    fn description(&self) -> &str {
        "Exception silently swallowed with pass"
    }
    fn severity(&self) -> Severity {
        Severity::Warning
    }
    fn detect(&self, graph: &CodeGraph) -> Vec<PatternInstance> {
        let mut instances = Vec::new();
        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];
            if let Some(sig) = &node.signature {
                if (sig.contains("except") && sig.contains("pass")) || sig.contains("except:\n    pass") {
                    instances.push(
                        PatternInstance::new(
                            self.name(),
                            node_location(node),
                            self.severity(),
                            format!("Exception silently caught in '{}'. Log or handle properly.", node.name),
                        )
                        .with_context(sig.clone())
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
                "Log the exception or handle it appropriately",
                "except Exception:\n    pass",
                "except Exception as e:\n    logger.error(f\"Error: {e}\")",
            )
            .with_confidence(0.8),
        )
    }
}

// Pattern 3: Broad Exception
struct BroadExceptionPattern;
impl BroadExceptionPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for BroadExceptionPattern {
    fn name(&self) -> &str {
        "broad_exception"
    }
    fn description(&self) -> &str {
        "Catching Exception or BaseException too broadly"
    }
    fn severity(&self) -> Severity {
        Severity::Warning
    }
    fn detect(&self, graph: &CodeGraph) -> Vec<PatternInstance> {
        let mut instances = Vec::new();
        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];
            if let Some(sig) = &node.signature {
                if sig.contains("except Exception:") || sig.contains("except BaseException:") {
                    instances.push(
                        PatternInstance::new(
                            self.name(),
                            node_location(node),
                            self.severity(),
                            format!("Broad exception catch in '{}'. Use specific exception types.", node.name),
                        )
                        .with_context(sig.clone())
                        .with_confidence(0.8),
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
                "except Exception:",
                "except (ValueError, KeyError) as e:",
            )
            .with_confidence(0.7),
        )
    }
}

// Pattern 4: Global Variable
struct GlobalVariablePattern;
impl GlobalVariablePattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for GlobalVariablePattern {
    fn name(&self) -> &str {
        "global_variable_usage"
    }
    fn description(&self) -> &str {
        "Global variable usage makes code hard to test and maintain"
    }
    fn severity(&self) -> Severity {
        Severity::Warning
    }
    fn detect(&self, graph: &CodeGraph) -> Vec<PatternInstance> {
        let mut instances = Vec::new();
        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];
            if let Some(sig) = &node.signature {
                if sig.contains("global ") {
                    instances.push(
                        PatternInstance::new(
                            self.name(),
                            node_location(node),
                            self.severity(),
                            format!("Global variable usage in '{}'. Consider using class attributes or parameters.", node.name),
                        )
                        .with_context(sig.clone())
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
                "Use class attributes or pass as parameters",
                "global counter\ncounter += 1",
                "class State:\n    counter: int = 0\n\nstate.counter += 1",
            )
            .with_confidence(0.7),
        )
    }
}

// Pattern 5: Mutable Default Argument
struct MutableDefaultArgumentPattern;
impl MutableDefaultArgumentPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for MutableDefaultArgumentPattern {
    fn name(&self) -> &str {
        "mutable_default_argument"
    }
    fn description(&self) -> &str {
        "Mutable default argument (list/dict) is shared across calls"
    }
    fn severity(&self) -> Severity {
        Severity::Error
    }
    fn detect(&self, graph: &CodeGraph) -> Vec<PatternInstance> {
        let mut instances = Vec::new();
        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];
            if matches!(node.kind, NodeKind::Function | NodeKind::Method) {
                if let Some(sig) = &node.signature {
                    if sig.contains("=[]") || sig.contains("={}") || sig.contains("= []") || sig.contains("= {}") {
                        instances.push(
                            PatternInstance::new(
                                self.name(),
                                node_location(node),
                                self.severity(),
                                format!("Mutable default argument in '{}'. Use None and initialize inside function.", node.name),
                            )
                            .with_context(sig.clone())
                            .with_confidence(0.95),
                        );
                    }
                }
            }
        }
        instances
    }
    fn confidence(&self) -> f32 {
        0.95
    }
    fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
        Some(
            FixSuggestion::new(
                self.name(),
                "Use None as default and initialize inside function",
                "def foo(items=[]):",
                "def foo(items=None):\n    if items is None:\n        items = []",
            )
            .with_confidence(0.95)
            .automated(),
        )
    }
}

// Pattern 6: Missing Type Hints
struct MissingTypeHintsPattern;
impl MissingTypeHintsPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for MissingTypeHintsPattern {
    fn name(&self) -> &str {
        "missing_type_hints"
    }
    fn description(&self) -> &str {
        "Function missing type hints"
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
                    if !sig.contains("->") && !sig.contains(":") {
                        instances.push(
                            PatternInstance::new(
                                self.name(),
                                node_location(node),
                                self.severity(),
                                format!("Function '{}' lacks type hints. Add type annotations.", node.name),
                            )
                            .with_context(sig.clone())
                            .with_confidence(0.7),
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
                "Add type hints for parameters and return value",
                "def process(data):",
                "def process(data: list[str]) -> bool:",
            )
            .with_confidence(0.6),
        )
    }
}

// Pattern 7: Incomplete Type Hints
struct IncompleteTypeHintsPattern;
impl IncompleteTypeHintsPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for IncompleteTypeHintsPattern {
    fn name(&self) -> &str {
        "incomplete_type_hints"
    }
    fn description(&self) -> &str {
        "Using generic types without specifics (list instead of list[str])"
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
                "Use specific type parameters",
                "def foo(items: list):",
                "def foo(items: list[str]):",
            )
            .with_confidence(0.7),
        )
    }
}

// Pattern 8: Star Import
struct StarImportPattern;
impl StarImportPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for StarImportPattern {
    fn name(&self) -> &str {
        "star_import"
    }
    fn description(&self) -> &str {
        "Star imports pollute namespace and hide dependencies"
    }
    fn severity(&self) -> Severity {
        Severity::Warning
    }
    fn detect(&self, graph: &CodeGraph) -> Vec<PatternInstance> {
        let mut instances = Vec::new();
        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];
            if node.kind == NodeKind::Import {
                if let Some(sig) = &node.signature {
                    if sig.contains("import *") {
                        instances.push(
                            PatternInstance::new(
                                self.name(),
                                node_location(node),
                                self.severity(),
                                "Star import found. Import specific names instead.".to_string(),
                            )
                            .with_context(sig.clone())
                            .with_confidence(0.95),
                        );
                    }
                }
            }
        }
        instances
    }
    fn confidence(&self) -> f32 {
        0.95
    }
    fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> {
        Some(
            FixSuggestion::new(
                self.name(),
                "Import specific names",
                "from module import *",
                "from module import name1, name2",
            )
            .with_confidence(0.9),
        )
    }
}

// Pattern 9: Unused Import
struct UnusedImportPattern;
impl UnusedImportPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for UnusedImportPattern {
    fn name(&self) -> &str {
        "unused_import"
    }
    fn description(&self) -> &str {
        "Import statement not used in code"
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
                "Remove unused import",
                "import unused_module",
                "# Removed unused import",
            )
            .with_confidence(0.95)
            .automated(),
        )
    }
}

// Pattern 10: Import Order
struct ImportOrderPattern;
impl ImportOrderPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for ImportOrderPattern {
    fn name(&self) -> &str {
        "import_order"
    }
    fn description(&self) -> &str {
        "Imports not ordered (stdlib, third-party, local)"
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
                "Order imports: stdlib, third-party, local",
                "import local\nimport os\nimport requests",
                "import os\n\nimport requests\n\nimport local",
            )
            .with_confidence(0.85),
        )
    }
}

// Pattern 11-25: Additional Python-specific patterns

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
        "Async function without await calls"
    }
    fn severity(&self) -> Severity {
        Severity::Warning
    }
    fn detect(&self, graph: &CodeGraph) -> Vec<PatternInstance> {
        let mut instances = Vec::new();
        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];
            if node.is_async {
                if let Some(sig) = &node.signature {
                    if !sig.contains("await") {
                        instances.push(
                            PatternInstance::new(
                                self.name(),
                                node_location(node),
                                self.severity(),
                                format!("Async function '{}' doesn't await. Remove async or add await.", node.name),
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
                "Remove async if not needed, or add await calls",
                "async def foo():\n    return bar()",
                "def foo():\n    return bar()\n# OR\nasync def foo():\n    return await async_bar()",
            )
            .with_confidence(0.75),
        )
    }
}

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
        "Blocking I/O in async function"
    }
    fn severity(&self) -> Severity {
        Severity::Warning
    }
    fn detect(&self, graph: &CodeGraph) -> Vec<PatternInstance> {
        let mut instances = Vec::new();
        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];
            if node.is_async {
                if let Some(sig) = &node.signature {
                    let blocking_patterns = ["time.sleep", "requests.", "open("];
                    for pattern in &blocking_patterns {
                        if sig.contains(pattern) {
                            instances.push(
                                PatternInstance::new(
                                    self.name(),
                                    node_location(node),
                                    self.severity(),
                                    format!("Blocking call '{}' in async function '{}'. Use async alternative.", pattern, node.name),
                                )
                                .with_confidence(0.85),
                            );
                        }
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
                "Use async alternatives",
                "time.sleep(1)\nrequests.get(url)",
                "await asyncio.sleep(1)\nawait aiohttp.get(url)",
            )
            .with_confidence(0.8),
        )
    }
}

struct AsyncGeneratorPattern;
impl AsyncGeneratorPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for AsyncGeneratorPattern {
    fn name(&self) -> &str {
        "async_generator_usage"
    }
    fn description(&self) -> &str {
        "Consider using async generator for large datasets"
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

struct ListComprehensionPattern;
impl ListComprehensionPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for ListComprehensionPattern {
    fn name(&self) -> &str {
        "inefficient_loop"
    }
    fn description(&self) -> &str {
        "Loop can be replaced with list comprehension"
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
                "Use list comprehension for better performance",
                "result = []\nfor x in items:\n    result.append(x * 2)",
                "result = [x * 2 for x in items]",
            )
            .with_confidence(0.8),
        )
    }
}

struct DictGetPattern;
impl DictGetPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for DictGetPattern {
    fn name(&self) -> &str {
        "dict_get_usage"
    }
    fn description(&self) -> &str {
        "Use dict.get() instead of checking key existence"
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
                "Use dict.get() for cleaner code",
                "if key in dict:\n    value = dict[key]\nelse:\n    value = default",
                "value = dict.get(key, default)",
            )
            .with_confidence(0.85),
        )
    }
}

struct StringConcatenationPattern;
impl StringConcatenationPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for StringConcatenationPattern {
    fn name(&self) -> &str {
        "string_concatenation_loop"
    }
    fn description(&self) -> &str {
        "String concatenation in loop is inefficient"
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
                "Use join() for string concatenation in loops",
                "result = ''\nfor s in strings:\n    result += s",
                "result = ''.join(strings)",
            )
            .with_confidence(0.9),
        )
    }
}

struct PlusEqualListPattern;
impl PlusEqualListPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for PlusEqualListPattern {
    fn name(&self) -> &str {
        "list_concatenation"
    }
    fn description(&self) -> &str {
        "Use extend() instead of += for lists"
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
                "Use extend() for better performance",
                "list1 += list2",
                "list1.extend(list2)",
            )
            .with_confidence(0.75),
        )
    }
}

struct IsNonePattern;
impl IsNonePattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for IsNonePattern {
    fn name(&self) -> &str {
        "is_none_comparison"
    }
    fn description(&self) -> &str {
        "Use 'is None' instead of '== None'"
    }
    fn severity(&self) -> Severity {
        Severity::Warning
    }
    fn detect(&self, graph: &CodeGraph) -> Vec<PatternInstance> {
        let mut instances = Vec::new();
        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];
            if let Some(sig) = &node.signature {
                if sig.contains("== None") || sig.contains("!= None") {
                    instances.push(
                        PatternInstance::new(
                            self.name(),
                            node_location(node),
                            self.severity(),
                            "Use 'is None' or 'is not None' for None comparison".to_string(),
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
                "Use identity comparison for None",
                "if x == None:\nif y != None:",
                "if x is None:\nif y is not None:",
            )
            .with_confidence(0.95)
            .automated(),
        )
    }
}

struct LenCheckPattern;
impl LenCheckPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for LenCheckPattern {
    fn name(&self) -> &str {
        "len_check"
    }
    fn description(&self) -> &str {
        "Use truthiness instead of len() for emptiness check"
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
                "Use truthiness for cleaner code",
                "if len(items) > 0:\nif len(items) == 0:",
                "if items:\nif not items:",
            )
            .with_confidence(0.85),
        )
    }
}

struct RangeLoopPattern;
impl RangeLoopPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for RangeLoopPattern {
    fn name(&self) -> &str {
        "range_len_loop"
    }
    fn description(&self) -> &str {
        "Use enumerate() instead of range(len())"
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
                "Use enumerate() for cleaner iteration",
                "for i in range(len(items)):\n    item = items[i]",
                "for i, item in enumerate(items):",
            )
            .with_confidence(0.9),
        )
    }
}

struct OpenWithoutContextPattern;
impl OpenWithoutContextPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for OpenWithoutContextPattern {
    fn name(&self) -> &str {
        "open_without_context"
    }
    fn description(&self) -> &str {
        "File opened without context manager (with statement)"
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
                "Use with statement for automatic resource cleanup",
                "f = open('file.txt')\ndata = f.read()\nf.close()",
                "with open('file.txt') as f:\n    data = f.read()",
            )
            .with_confidence(0.9),
        )
    }
}

struct AssertInProductionPattern;
impl AssertInProductionPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for AssertInProductionPattern {
    fn name(&self) -> &str {
        "assert_in_production"
    }
    fn description(&self) -> &str {
        "Assert statements disabled in optimized mode"
    }
    fn severity(&self) -> Severity {
        Severity::Warning
    }
    fn detect(&self, graph: &CodeGraph) -> Vec<PatternInstance> {
        let mut instances = Vec::new();
        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];
            if let Some(sig) = &node.signature {
                if sig.contains("assert ") {
                    instances.push(
                        PatternInstance::new(
                            self.name(),
                            node_location(node),
                            self.severity(),
                            "Assert statement found. Use proper validation in production.".to_string(),
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
                "Use explicit checks and raise exceptions",
                "assert value > 0",
                "if not value > 0:\n    raise ValueError('Value must be positive')",
            )
            .with_confidence(0.8),
        )
    }
}

struct PrintStatementPattern;
impl PrintStatementPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for PrintStatementPattern {
    fn name(&self) -> &str {
        "print_statement"
    }
    fn description(&self) -> &str {
        "Print statements should use logging in production"
    }
    fn severity(&self) -> Severity {
        Severity::Info
    }
    fn detect(&self, graph: &CodeGraph) -> Vec<PatternInstance> {
        let mut instances = Vec::new();
        for node_idx in graph.node_indices() {
            let node = &graph[node_idx];
            if let Some(sig) = &node.signature {
                if sig.contains("print(") {
                    instances.push(
                        PatternInstance::new(
                            self.name(),
                            node_location(node),
                            self.severity(),
                            "Print statement found. Consider using logging module.".to_string(),
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
                "Use logging instead of print",
                "print('Debug info:', value)",
                "logger.debug('Debug info: %s', value)",
            )
            .with_confidence(0.7),
        )
    }
}

struct ClassMethodDecoratorPattern;
impl ClassMethodDecoratorPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for ClassMethodDecoratorPattern {
    fn name(&self) -> &str {
        "classmethod_vs_staticmethod"
    }
    fn description(&self) -> &str {
        "Consider if @staticmethod is more appropriate"
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

struct PrivateMethodPattern;
impl PrivateMethodPattern {
    fn new() -> Self {
        Self
    }
}
impl PatternDetector for PrivateMethodPattern {
    fn name(&self) -> &str {
        "private_method_naming"
    }
    fn description(&self) -> &str {
        "Private methods should start with underscore"
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_python_patterns_creation() {
        let patterns = PythonPatterns::new();
        assert!(!patterns.all_detectors().is_empty());
        assert!(patterns.all_detectors().len() >= 20);
    }

    #[test]
    fn test_bare_except_pattern() {
        let detector = BareExceptPattern::new();
        assert_eq!(detector.name(), "bare_except");
        assert_eq!(detector.severity(), Severity::Error);
    }
}
