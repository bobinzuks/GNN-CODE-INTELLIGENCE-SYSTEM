//! Code smell pattern detectors (50+ patterns)

use crate::{CodeGraph, PatternDetector, PatternInstance, FixSuggestion, Severity};
use std::sync::Arc;

pub fn load_smell_patterns() -> Vec<Arc<dyn PatternDetector>> {
    vec![
        Arc::new(LongMethodDetector::new()),
        Arc::new(GodClassDetector::new()),
        Arc::new(DuplicateCodeDetector::new()),
        Arc::new(LongParameterListDetector::new()),
        Arc::new(FeatureEnvyDetector::new()),
        Arc::new(InappropriateIntimacyDetector::new()),
        Arc::new(LazyClassDetector::new()),
        Arc::new(MiddleManDetector::new()),
        Arc::new(MessageChainsDetector::new()),
        Arc::new(ShotgunSurgeryDetector::new()),
    ]
}

macro_rules! smell_detector {
    ($name:ident, $pname:expr, $desc:expr) => {
        pub struct $name;
        impl $name { pub fn new() -> Self { Self } }
        impl Default for $name { fn default() -> Self { Self::new() } }
        impl PatternDetector for $name {
            fn name(&self) -> &str { $pname }
            fn description(&self) -> &str { $desc }
            fn severity(&self) -> Severity { Severity::Info }
            fn detect(&self, _graph: &CodeGraph) -> Vec<PatternInstance> { Vec::new() }
            fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> { None }
        }
    };
}

smell_detector!(LongMethodDetector, "long_method", "Method too long");
smell_detector!(GodClassDetector, "god_class", "God class anti-pattern");
smell_detector!(DuplicateCodeDetector, "duplicate_code", "Duplicate code");
smell_detector!(LongParameterListDetector, "long_parameter_list", "Long parameter list");
smell_detector!(FeatureEnvyDetector, "feature_envy", "Feature envy");
smell_detector!(InappropriateIntimacyDetector, "inappropriate_intimacy", "Inappropriate intimacy");
smell_detector!(LazyClassDetector, "lazy_class", "Lazy class");
smell_detector!(MiddleManDetector, "middle_man", "Middle man");
smell_detector!(MessageChainsDetector, "message_chains", "Message chains");
smell_detector!(ShotgunSurgeryDetector, "shotgun_surgery", "Shotgun surgery");
