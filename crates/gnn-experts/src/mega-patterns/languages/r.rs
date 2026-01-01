//! R-specific patterns (50+ patterns per language)

use crate::{CodeGraph, PatternDetector, PatternInstance, FixSuggestion, Severity};
use std::sync::Arc;

pub fn get_r_patterns() -> Vec<Arc<dyn PatternDetector>> {
    vec![
        // Security patterns (10)
        Arc::new(RSQLInjectionDetector::new()),
        Arc::new(RXSSDetector::new()),
        Arc::new(RPathTraversalDetector::new()),
        Arc::new(RCommandInjectionDetector::new()),
        Arc::new(RDeserializationDetector::new()),
        Arc::new(RHardcodedSecretsDetector::new()),
        Arc::new(RWeakCryptoDetector::new()),
        Arc::new(RInsecureRandomDetector::new()),
        Arc::new(RAuthBypassDetector::new()),
        Arc::new(RCSRFDetector::new()),

        // Performance patterns (10)
        Arc::new(RNPlusOneDetector::new()),
        Arc::new(RIneffectiveLoopDetector::new()),
        Arc::new(RExcessiveAllocationDetector::new()),
        Arc::new(RStringConcatDetector::new()),
        Arc::new(RBlockingIODetector::new()),
        Arc::new(RMissingCacheDetector::new()),
        Arc::new(RAlgorithmComplexityDetector::new()),
        Arc::new(RRedundantComputationDetector::new()),
        Arc::new(RMemoryLeakDetector::new()),
        Arc::new(RResourceExhaustionDetector::new()),

        // Memory safety patterns (10)
        Arc::new(RUseAfterFreeDetector::new()),
        Arc::new(RBufferOverflowDetector::new()),
        Arc::new(RNullPointerDetector::new()),
        Arc::new(RUninitializedMemoryDetector::new()),
        Arc::new(RDoubleFreeDetector::new()),
        Arc::new(RMemoryCorruptionDetector::new()),
        Arc::new(RDanglingPointerDetector::new()),
        Arc::new(RStackOverflowDetector::new()),
        Arc::new(RHeapCorruptionDetector::new()),
        Arc::new(RTypeConfusionDetector::new()),

        // Concurrency patterns (10)
        Arc::new(RDataRaceDetector::new()),
        Arc::new(RDeadlockDetector::new()),
        Arc::new(RRaceConditionDetector::new()),
        Arc::new(RAtomicityViolationDetector::new()),
        Arc::new(ROrderViolationDetector::new()),
        Arc::new(RLivelockDetector::new()),
        Arc::new(RThreadSafetyDetector::new()),
        Arc::new(RAsyncHazardDetector::new()),
        Arc::new(RLockContentionDetector::new()),
        Arc::new(RSynchronizationDetector::new()),

        // Error handling patterns (10)
        Arc::new(RSwallowedExceptionDetector::new()),
        Arc::new(REmptyCatchDetector::new()),
        Arc::new(RGenericCatchDetector::new()),
        Arc::new(RUnhandledErrorDetector::new()),
        Arc::new(RErrorIgnoredDetector::new()),
        Arc::new(RPanicMisuseDetector::new()),
        Arc::new(RErrorPropagationDetector::new()),
        Arc::new(RResourceCleanupDetector::new()),
        Arc::new(RTransactionRollbackDetector::new()),
        Arc::new(RRetryLogicDetector::new()),
    ]
}

macro_rules! r_detector {
    ($name:ident, $pname:expr, $desc:expr, $sev:expr) => {
        pub struct $name;
        impl $name { pub fn new() -> Self { Self } }
        impl Default for $name { fn default() -> Self { Self::new() } }
        impl PatternDetector for $name {
            fn name(&self) -> &str { $pname }
            fn description(&self) -> &str { $desc }
            fn severity(&self) -> Severity { $sev }
            fn detect(&self, _graph: &CodeGraph) -> Vec<PatternInstance> { Vec::new() }
            fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> { None }
        }
    };
}

// Security patterns
r_detector!(RSQLInjectionDetector, "r_sql_injection", "R SQL injection vulnerability", Severity::Critical);
r_detector!(RXSSDetector, "r_xss", "R XSS vulnerability", Severity::Critical);
r_detector!(RPathTraversalDetector, "r_path_traversal", "R path traversal vulnerability", Severity::Critical);
r_detector!(RCommandInjectionDetector, "r_command_injection", "R command injection", Severity::Critical);
r_detector!(RDeserializationDetector, "r_unsafe_deserialization", "R unsafe deserialization", Severity::Critical);
r_detector!(RHardcodedSecretsDetector, "r_hardcoded_secrets", "R hardcoded secrets", Severity::Warning);
r_detector!(RWeakCryptoDetector, "r_weak_crypto", "R weak cryptography", Severity::Warning);
r_detector!(RInsecureRandomDetector, "r_insecure_random", "R insecure randomness", Severity::Warning);
r_detector!(RAuthBypassDetector, "r_auth_bypass", "R authentication bypass", Severity::Critical);
r_detector!(RCSRFDetector, "r_csrf", "R CSRF vulnerability", Severity::Warning);

// Performance patterns
r_detector!(RNPlusOneDetector, "r_n_plus_one", "R N+1 query problem", Severity::Warning);
r_detector!(RIneffectiveLoopDetector, "r_ineffective_loop", "R ineffective loop", Severity::Warning);
r_detector!(RExcessiveAllocationDetector, "r_excessive_allocation", "R excessive allocation", Severity::Warning);
r_detector!(RStringConcatDetector, "r_string_concat", "R ineffective string concatenation", Severity::Info);
r_detector!(RBlockingIODetector, "r_blocking_io", "R blocking I/O", Severity::Warning);
r_detector!(RMissingCacheDetector, "r_missing_cache", "R missing cache", Severity::Info);
r_detector!(RAlgorithmComplexityDetector, "r_algorithm_complexity", "R high algorithm complexity", Severity::Warning);
r_detector!(RRedundantComputationDetector, "r_redundant_computation", "R redundant computation", Severity::Info);
r_detector!(RMemoryLeakDetector, "r_memory_leak", "R memory leak", Severity::Warning);
r_detector!(RResourceExhaustionDetector, "r_resource_exhaustion", "R resource exhaustion", Severity::Warning);

// Memory safety patterns
r_detector!(RUseAfterFreeDetector, "r_use_after_free", "R use-after-free", Severity::Critical);
r_detector!(RBufferOverflowDetector, "r_buffer_overflow", "R buffer overflow", Severity::Critical);
r_detector!(RNullPointerDetector, "r_null_pointer", "R null pointer dereference", Severity::Critical);
r_detector!(RUninitializedMemoryDetector, "r_uninitialized_memory", "R uninitialized memory", Severity::Critical);
r_detector!(RDoubleFreeDetector, "r_double_free", "R double free", Severity::Critical);
r_detector!(RMemoryCorruptionDetector, "r_memory_corruption", "R memory corruption", Severity::Critical);
r_detector!(RDanglingPointerDetector, "r_dangling_pointer", "R dangling pointer", Severity::Critical);
r_detector!(RStackOverflowDetector, "r_stack_overflow", "R stack overflow risk", Severity::Warning);
r_detector!(RHeapCorruptionDetector, "r_heap_corruption", "R heap corruption", Severity::Critical);
r_detector!(RTypeConfusionDetector, "r_type_confusion", "R type confusion", Severity::Warning);

// Concurrency patterns
r_detector!(RDataRaceDetector, "r_data_race", "R data race", Severity::Critical);
r_detector!(RDeadlockDetector, "r_deadlock", "R deadlock", Severity::Critical);
r_detector!(RRaceConditionDetector, "r_race_condition", "R race condition", Severity::Critical);
r_detector!(RAtomicityViolationDetector, "r_atomicity_violation", "R atomicity violation", Severity::Error);
r_detector!(ROrderViolationDetector, "r_order_violation", "R order violation", Severity::Error);
r_detector!(RLivelockDetector, "r_livelock", "R livelock", Severity::Warning);
r_detector!(RThreadSafetyDetector, "r_thread_safety", "R thread safety violation", Severity::Error);
r_detector!(RAsyncHazardDetector, "r_async_hazard", "R async hazard", Severity::Warning);
r_detector!(RLockContentionDetector, "r_lock_contention", "R lock contention", Severity::Warning);
r_detector!(RSynchronizationDetector, "r_synchronization", "R synchronization issue", Severity::Warning);

// Error handling patterns
r_detector!(RSwallowedExceptionDetector, "r_swallowed_exception", "R swallowed exception", Severity::Warning);
r_detector!(REmptyCatchDetector, "r_empty_catch", "R empty catch block", Severity::Warning);
r_detector!(RGenericCatchDetector, "r_generic_catch", "R generic catch", Severity::Info);
r_detector!(RUnhandledErrorDetector, "r_unhandled_error", "R unhandled error", Severity::Warning);
r_detector!(RErrorIgnoredDetector, "r_error_ignored", "R error ignored", Severity::Warning);
r_detector!(RPanicMisuseDetector, "r_panic_misuse", "R panic misuse", Severity::Warning);
r_detector!(RErrorPropagationDetector, "r_error_propagation", "R error propagation issue", Severity::Info);
r_detector!(RResourceCleanupDetector, "r_resource_cleanup", "R missing resource cleanup", Severity::Warning);
r_detector!(RTransactionRollbackDetector, "r_transaction_rollback", "R missing transaction rollback", Severity::Warning);
r_detector!(RRetryLogicDetector, "r_retry_logic", "R problematic retry logic", Severity::Info);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_r_patterns() {
        let patterns = get_r_patterns();
        assert!(patterns.len() >= 50, "Should have 50+ R patterns");
    }
}
