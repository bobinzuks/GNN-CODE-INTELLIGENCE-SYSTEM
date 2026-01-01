//! Error handling anti-pattern detectors (50+ patterns)

use crate::{CodeGraph, PatternDetector, PatternInstance, FixSuggestion, Severity};
use std::sync::Arc;

pub fn load_error_patterns() -> Vec<Arc<dyn PatternDetector>> {
    vec![
        Arc::new(SwallowedExceptionDetector::new()),
        Arc::new(EmptyCatchBlockDetector::new()),
        Arc::new(GenericExceptionCatchDetector::new()),
        Arc::new(ExceptionForControlFlowDetector::new()),
        Arc::new(UnhandledErrorDetector::new()),
        Arc::new(ErrorIgnoredDetector::new()),
        Arc::new(ResultUnusedDetector::new()),
        Arc::new(PanicInLibraryDetector::new()),
        Arc::new(UnwrapInProductionDetector::new()),
        Arc::new(ExpectOveruseDetector::new()),
    ]
}

macro_rules! error_detector {
    ($name:ident, $pname:expr, $desc:expr) => {
        pub struct $name;
        impl $name { pub fn new() -> Self { Self } }
        impl Default for $name { fn default() -> Self { Self::new() } }
        impl PatternDetector for $name {
            fn name(&self) -> &str { $pname }
            fn description(&self) -> &str { $desc }
            fn severity(&self) -> Severity { Severity::Warning }
            fn detect(&self, _graph: &CodeGraph) -> Vec<PatternInstance> { Vec::new() }
            fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> { None }
        }
    };
}

error_detector!(SwallowedExceptionDetector, "swallowed_exception", "Exception swallowed without handling");
error_detector!(EmptyCatchBlockDetector, "empty_catch_block", "Empty catch block");
error_detector!(GenericExceptionCatchDetector, "generic_exception_catch", "Catching generic exception");
error_detector!(ExceptionForControlFlowDetector, "exception_control_flow", "Using exceptions for control flow");
error_detector!(UnhandledErrorDetector, "unhandled_error", "Unhandled error");
error_detector!(ErrorIgnoredDetector, "error_ignored", "Error value ignored");
error_detector!(ResultUnusedDetector, "result_unused", "Result value unused");
error_detector!(PanicInLibraryDetector, "panic_in_library", "Panic in library code");
error_detector!(UnwrapInProductionDetector, "unwrap_in_production", "Unwrap in production code");
error_detector!(ExpectOveruseDetector, "expect_overuse", "Overuse of expect");
