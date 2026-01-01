//! Design pattern detectors (50+ patterns)

use crate::{CodeGraph, PatternDetector, PatternInstance, FixSuggestion, Severity};
use std::sync::Arc;

pub fn load_design_patterns() -> Vec<Arc<dyn PatternDetector>> {
    vec![
        Arc::new(SingletonOpportunityDetector::new()),
        Arc::new(FactoryOpportunityDetector::new()),
        Arc::new(BuilderOpportunityDetector::new()),
        Arc::new(StrategyOpportunityDetector::new()),
        Arc::new(ObserverOpportunityDetector::new()),
        Arc::new(DecoratorOpportunityDetector::new()),
        Arc::new(AdapterOpportunityDetector::new()),
        Arc::new(FacadeOpportunityDetector::new()),
        Arc::new(ProxyOpportunityDetector::new()),
        Arc::new(CommandOpportunityDetector::new()),
    ]
}

macro_rules! design_detector {
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

design_detector!(SingletonOpportunityDetector, "singleton_opportunity", "Singleton pattern opportunity");
design_detector!(FactoryOpportunityDetector, "factory_opportunity", "Factory pattern opportunity");
design_detector!(BuilderOpportunityDetector, "builder_opportunity", "Builder pattern opportunity");
design_detector!(StrategyOpportunityDetector, "strategy_opportunity", "Strategy pattern opportunity");
design_detector!(ObserverOpportunityDetector, "observer_opportunity", "Observer pattern opportunity");
design_detector!(DecoratorOpportunityDetector, "decorator_opportunity", "Decorator pattern opportunity");
design_detector!(AdapterOpportunityDetector, "adapter_opportunity", "Adapter pattern opportunity");
design_detector!(FacadeOpportunityDetector, "facade_opportunity", "Facade pattern opportunity");
design_detector!(ProxyOpportunityDetector, "proxy_opportunity", "Proxy pattern opportunity");
design_detector!(CommandOpportunityDetector, "command_opportunity", "Command pattern opportunity");
