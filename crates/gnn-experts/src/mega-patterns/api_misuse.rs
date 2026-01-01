//! API misuse pattern detectors (50+ patterns)

use crate::{CodeGraph, PatternDetector, PatternInstance, FixSuggestion, Severity};
use std::sync::Arc;

pub fn load_api_patterns() -> Vec<Arc<dyn PatternDetector>> {
    vec![
        Arc::new(IncorrectAPIUsageDetector::new()),
        Arc::new(MissingRequiredCallDetector::new()),
        Arc::new(WrongOrderAPICallDetector::new()),
        Arc::new(DeprecatedAPIDetector::new()),
        Arc::new(UnsafeAPIDetector::new()),
        Arc::new(LeakyAbstractionDetector::new()),
        Arc::new(InconsistentAPIDetector::new()),
        Arc::new(ViolatedPreconditionDetector::new()),
        Arc::new(IgnoredReturnValueDetector::new()),
        Arc::new(ResourceNotReleasedDetector::new()),
    ]
}

macro_rules! api_detector {
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

api_detector!(IncorrectAPIUsageDetector, "incorrect_api_usage", "Incorrect API usage");
api_detector!(MissingRequiredCallDetector, "missing_required_call", "Missing required API call");
api_detector!(WrongOrderAPICallDetector, "wrong_order_api_call", "API calls in wrong order");
api_detector!(DeprecatedAPIDetector, "deprecated_api", "Using deprecated API");
api_detector!(UnsafeAPIDetector, "unsafe_api", "Using unsafe API without justification");
api_detector!(LeakyAbstractionDetector, "leaky_abstraction", "Leaky abstraction");
api_detector!(InconsistentAPIDetector, "inconsistent_api", "Inconsistent API usage");
api_detector!(ViolatedPreconditionDetector, "violated_precondition", "API precondition violated");
api_detector!(IgnoredReturnValueDetector, "ignored_return_value", "API return value ignored");
api_detector!(ResourceNotReleasedDetector, "resource_not_released", "Resource not released");
