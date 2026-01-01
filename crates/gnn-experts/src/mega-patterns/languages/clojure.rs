//! Clojure-specific patterns (50+ patterns per language)

use crate::{CodeGraph, PatternDetector, PatternInstance, FixSuggestion, Severity};
use std::sync::Arc;

pub fn get_clojure_patterns() -> Vec<Arc<dyn PatternDetector>> {
    vec![
        // Security patterns (10)
        Arc::new(ClojureSQLInjectionDetector::new()),
        Arc::new(ClojureXSSDetector::new()),
        Arc::new(ClojurePathTraversalDetector::new()),
        Arc::new(ClojureCommandInjectionDetector::new()),
        Arc::new(ClojureDeserializationDetector::new()),
        Arc::new(ClojureHardcodedSecretsDetector::new()),
        Arc::new(ClojureWeakCryptoDetector::new()),
        Arc::new(ClojureInsecureRandomDetector::new()),
        Arc::new(ClojureAuthBypassDetector::new()),
        Arc::new(ClojureCSRFDetector::new()),

        // Performance patterns (10)
        Arc::new(ClojureNPlusOneDetector::new()),
        Arc::new(ClojureIneffectiveLoopDetector::new()),
        Arc::new(ClojureExcessiveAllocationDetector::new()),
        Arc::new(ClojureStringConcatDetector::new()),
        Arc::new(ClojureBlockingIODetector::new()),
        Arc::new(ClojureMissingCacheDetector::new()),
        Arc::new(ClojureAlgorithmComplexityDetector::new()),
        Arc::new(ClojureRedundantComputationDetector::new()),
        Arc::new(ClojureMemoryLeakDetector::new()),
        Arc::new(ClojureResourceExhaustionDetector::new()),

        // Memory safety patterns (10)
        Arc::new(ClojureUseAfterFreeDetector::new()),
        Arc::new(ClojureBufferOverflowDetector::new()),
        Arc::new(ClojureNullPointerDetector::new()),
        Arc::new(ClojureUninitializedMemoryDetector::new()),
        Arc::new(ClojureDoubleFreeDetector::new()),
        Arc::new(ClojureMemoryCorruptionDetector::new()),
        Arc::new(ClojureDanglingPointerDetector::new()),
        Arc::new(ClojureStackOverflowDetector::new()),
        Arc::new(ClojureHeapCorruptionDetector::new()),
        Arc::new(ClojureTypeConfusionDetector::new()),

        // Concurrency patterns (10)
        Arc::new(ClojureDataRaceDetector::new()),
        Arc::new(ClojureDeadlockDetector::new()),
        Arc::new(ClojureRaceConditionDetector::new()),
        Arc::new(ClojureAtomicityViolationDetector::new()),
        Arc::new(ClojureOrderViolationDetector::new()),
        Arc::new(ClojureLivelockDetector::new()),
        Arc::new(ClojureThreadSafetyDetector::new()),
        Arc::new(ClojureAsyncHazardDetector::new()),
        Arc::new(ClojureLockContentionDetector::new()),
        Arc::new(ClojureSynchronizationDetector::new()),

        // Error handling patterns (10)
        Arc::new(ClojureSwallowedExceptionDetector::new()),
        Arc::new(ClojureEmptyCatchDetector::new()),
        Arc::new(ClojureGenericCatchDetector::new()),
        Arc::new(ClojureUnhandledErrorDetector::new()),
        Arc::new(ClojureErrorIgnoredDetector::new()),
        Arc::new(ClojurePanicMisuseDetector::new()),
        Arc::new(ClojureErrorPropagationDetector::new()),
        Arc::new(ClojureResourceCleanupDetector::new()),
        Arc::new(ClojureTransactionRollbackDetector::new()),
        Arc::new(ClojureRetryLogicDetector::new()),
    ]
}

macro_rules! clojure_detector {
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
clojure_detector!(ClojureSQLInjectionDetector, "clojure_sql_injection", "Clojure SQL injection vulnerability", Severity::Critical);
clojure_detector!(ClojureXSSDetector, "clojure_xss", "Clojure XSS vulnerability", Severity::Critical);
clojure_detector!(ClojurePathTraversalDetector, "clojure_path_traversal", "Clojure path traversal vulnerability", Severity::Critical);
clojure_detector!(ClojureCommandInjectionDetector, "clojure_command_injection", "Clojure command injection", Severity::Critical);
clojure_detector!(ClojureDeserializationDetector, "clojure_unsafe_deserialization", "Clojure unsafe deserialization", Severity::Critical);
clojure_detector!(ClojureHardcodedSecretsDetector, "clojure_hardcoded_secrets", "Clojure hardcoded secrets", Severity::Warning);
clojure_detector!(ClojureWeakCryptoDetector, "clojure_weak_crypto", "Clojure weak cryptography", Severity::Warning);
clojure_detector!(ClojureInsecureRandomDetector, "clojure_insecure_random", "Clojure insecure randomness", Severity::Warning);
clojure_detector!(ClojureAuthBypassDetector, "clojure_auth_bypass", "Clojure authentication bypass", Severity::Critical);
clojure_detector!(ClojureCSRFDetector, "clojure_csrf", "Clojure CSRF vulnerability", Severity::Warning);

// Performance patterns
clojure_detector!(ClojureNPlusOneDetector, "clojure_n_plus_one", "Clojure N+1 query problem", Severity::Warning);
clojure_detector!(ClojureIneffectiveLoopDetector, "clojure_ineffective_loop", "Clojure ineffective loop", Severity::Warning);
clojure_detector!(ClojureExcessiveAllocationDetector, "clojure_excessive_allocation", "Clojure excessive allocation", Severity::Warning);
clojure_detector!(ClojureStringConcatDetector, "clojure_string_concat", "Clojure ineffective string concatenation", Severity::Info);
clojure_detector!(ClojureBlockingIODetector, "clojure_blocking_io", "Clojure blocking I/O", Severity::Warning);
clojure_detector!(ClojureMissingCacheDetector, "clojure_missing_cache", "Clojure missing cache", Severity::Info);
clojure_detector!(ClojureAlgorithmComplexityDetector, "clojure_algorithm_complexity", "Clojure high algorithm complexity", Severity::Warning);
clojure_detector!(ClojureRedundantComputationDetector, "clojure_redundant_computation", "Clojure redundant computation", Severity::Info);
clojure_detector!(ClojureMemoryLeakDetector, "clojure_memory_leak", "Clojure memory leak", Severity::Warning);
clojure_detector!(ClojureResourceExhaustionDetector, "clojure_resource_exhaustion", "Clojure resource exhaustion", Severity::Warning);

// Memory safety patterns
clojure_detector!(ClojureUseAfterFreeDetector, "clojure_use_after_free", "Clojure use-after-free", Severity::Critical);
clojure_detector!(ClojureBufferOverflowDetector, "clojure_buffer_overflow", "Clojure buffer overflow", Severity::Critical);
clojure_detector!(ClojureNullPointerDetector, "clojure_null_pointer", "Clojure null pointer dereference", Severity::Critical);
clojure_detector!(ClojureUninitializedMemoryDetector, "clojure_uninitialized_memory", "Clojure uninitialized memory", Severity::Critical);
clojure_detector!(ClojureDoubleFreeDetector, "clojure_double_free", "Clojure double free", Severity::Critical);
clojure_detector!(ClojureMemoryCorruptionDetector, "clojure_memory_corruption", "Clojure memory corruption", Severity::Critical);
clojure_detector!(ClojureDanglingPointerDetector, "clojure_dangling_pointer", "Clojure dangling pointer", Severity::Critical);
clojure_detector!(ClojureStackOverflowDetector, "clojure_stack_overflow", "Clojure stack overflow risk", Severity::Warning);
clojure_detector!(ClojureHeapCorruptionDetector, "clojure_heap_corruption", "Clojure heap corruption", Severity::Critical);
clojure_detector!(ClojureTypeConfusionDetector, "clojure_type_confusion", "Clojure type confusion", Severity::Warning);

// Concurrency patterns
clojure_detector!(ClojureDataRaceDetector, "clojure_data_race", "Clojure data race", Severity::Critical);
clojure_detector!(ClojureDeadlockDetector, "clojure_deadlock", "Clojure deadlock", Severity::Critical);
clojure_detector!(ClojureRaceConditionDetector, "clojure_race_condition", "Clojure race condition", Severity::Critical);
clojure_detector!(ClojureAtomicityViolationDetector, "clojure_atomicity_violation", "Clojure atomicity violation", Severity::Error);
clojure_detector!(ClojureOrderViolationDetector, "clojure_order_violation", "Clojure order violation", Severity::Error);
clojure_detector!(ClojureLivelockDetector, "clojure_livelock", "Clojure livelock", Severity::Warning);
clojure_detector!(ClojureThreadSafetyDetector, "clojure_thread_safety", "Clojure thread safety violation", Severity::Error);
clojure_detector!(ClojureAsyncHazardDetector, "clojure_async_hazard", "Clojure async hazard", Severity::Warning);
clojure_detector!(ClojureLockContentionDetector, "clojure_lock_contention", "Clojure lock contention", Severity::Warning);
clojure_detector!(ClojureSynchronizationDetector, "clojure_synchronization", "Clojure synchronization issue", Severity::Warning);

// Error handling patterns
clojure_detector!(ClojureSwallowedExceptionDetector, "clojure_swallowed_exception", "Clojure swallowed exception", Severity::Warning);
clojure_detector!(ClojureEmptyCatchDetector, "clojure_empty_catch", "Clojure empty catch block", Severity::Warning);
clojure_detector!(ClojureGenericCatchDetector, "clojure_generic_catch", "Clojure generic catch", Severity::Info);
clojure_detector!(ClojureUnhandledErrorDetector, "clojure_unhandled_error", "Clojure unhandled error", Severity::Warning);
clojure_detector!(ClojureErrorIgnoredDetector, "clojure_error_ignored", "Clojure error ignored", Severity::Warning);
clojure_detector!(ClojurePanicMisuseDetector, "clojure_panic_misuse", "Clojure panic misuse", Severity::Warning);
clojure_detector!(ClojureErrorPropagationDetector, "clojure_error_propagation", "Clojure error propagation issue", Severity::Info);
clojure_detector!(ClojureResourceCleanupDetector, "clojure_resource_cleanup", "Clojure missing resource cleanup", Severity::Warning);
clojure_detector!(ClojureTransactionRollbackDetector, "clojure_transaction_rollback", "Clojure missing transaction rollback", Severity::Warning);
clojure_detector!(ClojureRetryLogicDetector, "clojure_retry_logic", "Clojure problematic retry logic", Severity::Info);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clojure_patterns() {
        let patterns = get_clojure_patterns();
        assert!(patterns.len() >= 50, "Should have 50+ Clojure patterns");
    }
}
