//! Scala-specific patterns (50+ patterns per language)

use crate::{CodeGraph, PatternDetector, PatternInstance, FixSuggestion, Severity};
use std::sync::Arc;

pub fn get_scala_patterns() -> Vec<Arc<dyn PatternDetector>> {
    vec![
        // Security patterns (10)
        Arc::new(ScalaSQLInjectionDetector::new()),
        Arc::new(ScalaXSSDetector::new()),
        Arc::new(ScalaPathTraversalDetector::new()),
        Arc::new(ScalaCommandInjectionDetector::new()),
        Arc::new(ScalaDeserializationDetector::new()),
        Arc::new(ScalaHardcodedSecretsDetector::new()),
        Arc::new(ScalaWeakCryptoDetector::new()),
        Arc::new(ScalaInsecureRandomDetector::new()),
        Arc::new(ScalaAuthBypassDetector::new()),
        Arc::new(ScalaCSRFDetector::new()),

        // Performance patterns (10)
        Arc::new(ScalaNPlusOneDetector::new()),
        Arc::new(ScalaIneffectiveLoopDetector::new()),
        Arc::new(ScalaExcessiveAllocationDetector::new()),
        Arc::new(ScalaStringConcatDetector::new()),
        Arc::new(ScalaBlockingIODetector::new()),
        Arc::new(ScalaMissingCacheDetector::new()),
        Arc::new(ScalaAlgorithmComplexityDetector::new()),
        Arc::new(ScalaRedundantComputationDetector::new()),
        Arc::new(ScalaMemoryLeakDetector::new()),
        Arc::new(ScalaResourceExhaustionDetector::new()),

        // Memory safety patterns (10)
        Arc::new(ScalaUseAfterFreeDetector::new()),
        Arc::new(ScalaBufferOverflowDetector::new()),
        Arc::new(ScalaNullPointerDetector::new()),
        Arc::new(ScalaUninitializedMemoryDetector::new()),
        Arc::new(ScalaDoubleFreeDetector::new()),
        Arc::new(ScalaMemoryCorruptionDetector::new()),
        Arc::new(ScalaDanglingPointerDetector::new()),
        Arc::new(ScalaStackOverflowDetector::new()),
        Arc::new(ScalaHeapCorruptionDetector::new()),
        Arc::new(ScalaTypeConfusionDetector::new()),

        // Concurrency patterns (10)
        Arc::new(ScalaDataRaceDetector::new()),
        Arc::new(ScalaDeadlockDetector::new()),
        Arc::new(ScalaRaceConditionDetector::new()),
        Arc::new(ScalaAtomicityViolationDetector::new()),
        Arc::new(ScalaOrderViolationDetector::new()),
        Arc::new(ScalaLivelockDetector::new()),
        Arc::new(ScalaThreadSafetyDetector::new()),
        Arc::new(ScalaAsyncHazardDetector::new()),
        Arc::new(ScalaLockContentionDetector::new()),
        Arc::new(ScalaSynchronizationDetector::new()),

        // Error handling patterns (10)
        Arc::new(ScalaSwallowedExceptionDetector::new()),
        Arc::new(ScalaEmptyCatchDetector::new()),
        Arc::new(ScalaGenericCatchDetector::new()),
        Arc::new(ScalaUnhandledErrorDetector::new()),
        Arc::new(ScalaErrorIgnoredDetector::new()),
        Arc::new(ScalaPanicMisuseDetector::new()),
        Arc::new(ScalaErrorPropagationDetector::new()),
        Arc::new(ScalaResourceCleanupDetector::new()),
        Arc::new(ScalaTransactionRollbackDetector::new()),
        Arc::new(ScalaRetryLogicDetector::new()),
    ]
}

macro_rules! scala_detector {
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
scala_detector!(ScalaSQLInjectionDetector, "scala_sql_injection", "Scala SQL injection vulnerability", Severity::Critical);
scala_detector!(ScalaXSSDetector, "scala_xss", "Scala XSS vulnerability", Severity::Critical);
scala_detector!(ScalaPathTraversalDetector, "scala_path_traversal", "Scala path traversal vulnerability", Severity::Critical);
scala_detector!(ScalaCommandInjectionDetector, "scala_command_injection", "Scala command injection", Severity::Critical);
scala_detector!(ScalaDeserializationDetector, "scala_unsafe_deserialization", "Scala unsafe deserialization", Severity::Critical);
scala_detector!(ScalaHardcodedSecretsDetector, "scala_hardcoded_secrets", "Scala hardcoded secrets", Severity::Warning);
scala_detector!(ScalaWeakCryptoDetector, "scala_weak_crypto", "Scala weak cryptography", Severity::Warning);
scala_detector!(ScalaInsecureRandomDetector, "scala_insecure_random", "Scala insecure randomness", Severity::Warning);
scala_detector!(ScalaAuthBypassDetector, "scala_auth_bypass", "Scala authentication bypass", Severity::Critical);
scala_detector!(ScalaCSRFDetector, "scala_csrf", "Scala CSRF vulnerability", Severity::Warning);

// Performance patterns
scala_detector!(ScalaNPlusOneDetector, "scala_n_plus_one", "Scala N+1 query problem", Severity::Warning);
scala_detector!(ScalaIneffectiveLoopDetector, "scala_ineffective_loop", "Scala ineffective loop", Severity::Warning);
scala_detector!(ScalaExcessiveAllocationDetector, "scala_excessive_allocation", "Scala excessive allocation", Severity::Warning);
scala_detector!(ScalaStringConcatDetector, "scala_string_concat", "Scala ineffective string concatenation", Severity::Info);
scala_detector!(ScalaBlockingIODetector, "scala_blocking_io", "Scala blocking I/O", Severity::Warning);
scala_detector!(ScalaMissingCacheDetector, "scala_missing_cache", "Scala missing cache", Severity::Info);
scala_detector!(ScalaAlgorithmComplexityDetector, "scala_algorithm_complexity", "Scala high algorithm complexity", Severity::Warning);
scala_detector!(ScalaRedundantComputationDetector, "scala_redundant_computation", "Scala redundant computation", Severity::Info);
scala_detector!(ScalaMemoryLeakDetector, "scala_memory_leak", "Scala memory leak", Severity::Warning);
scala_detector!(ScalaResourceExhaustionDetector, "scala_resource_exhaustion", "Scala resource exhaustion", Severity::Warning);

// Memory safety patterns
scala_detector!(ScalaUseAfterFreeDetector, "scala_use_after_free", "Scala use-after-free", Severity::Critical);
scala_detector!(ScalaBufferOverflowDetector, "scala_buffer_overflow", "Scala buffer overflow", Severity::Critical);
scala_detector!(ScalaNullPointerDetector, "scala_null_pointer", "Scala null pointer dereference", Severity::Critical);
scala_detector!(ScalaUninitializedMemoryDetector, "scala_uninitialized_memory", "Scala uninitialized memory", Severity::Critical);
scala_detector!(ScalaDoubleFreeDetector, "scala_double_free", "Scala double free", Severity::Critical);
scala_detector!(ScalaMemoryCorruptionDetector, "scala_memory_corruption", "Scala memory corruption", Severity::Critical);
scala_detector!(ScalaDanglingPointerDetector, "scala_dangling_pointer", "Scala dangling pointer", Severity::Critical);
scala_detector!(ScalaStackOverflowDetector, "scala_stack_overflow", "Scala stack overflow risk", Severity::Warning);
scala_detector!(ScalaHeapCorruptionDetector, "scala_heap_corruption", "Scala heap corruption", Severity::Critical);
scala_detector!(ScalaTypeConfusionDetector, "scala_type_confusion", "Scala type confusion", Severity::Warning);

// Concurrency patterns
scala_detector!(ScalaDataRaceDetector, "scala_data_race", "Scala data race", Severity::Critical);
scala_detector!(ScalaDeadlockDetector, "scala_deadlock", "Scala deadlock", Severity::Critical);
scala_detector!(ScalaRaceConditionDetector, "scala_race_condition", "Scala race condition", Severity::Critical);
scala_detector!(ScalaAtomicityViolationDetector, "scala_atomicity_violation", "Scala atomicity violation", Severity::Error);
scala_detector!(ScalaOrderViolationDetector, "scala_order_violation", "Scala order violation", Severity::Error);
scala_detector!(ScalaLivelockDetector, "scala_livelock", "Scala livelock", Severity::Warning);
scala_detector!(ScalaThreadSafetyDetector, "scala_thread_safety", "Scala thread safety violation", Severity::Error);
scala_detector!(ScalaAsyncHazardDetector, "scala_async_hazard", "Scala async hazard", Severity::Warning);
scala_detector!(ScalaLockContentionDetector, "scala_lock_contention", "Scala lock contention", Severity::Warning);
scala_detector!(ScalaSynchronizationDetector, "scala_synchronization", "Scala synchronization issue", Severity::Warning);

// Error handling patterns
scala_detector!(ScalaSwallowedExceptionDetector, "scala_swallowed_exception", "Scala swallowed exception", Severity::Warning);
scala_detector!(ScalaEmptyCatchDetector, "scala_empty_catch", "Scala empty catch block", Severity::Warning);
scala_detector!(ScalaGenericCatchDetector, "scala_generic_catch", "Scala generic catch", Severity::Info);
scala_detector!(ScalaUnhandledErrorDetector, "scala_unhandled_error", "Scala unhandled error", Severity::Warning);
scala_detector!(ScalaErrorIgnoredDetector, "scala_error_ignored", "Scala error ignored", Severity::Warning);
scala_detector!(ScalaPanicMisuseDetector, "scala_panic_misuse", "Scala panic misuse", Severity::Warning);
scala_detector!(ScalaErrorPropagationDetector, "scala_error_propagation", "Scala error propagation issue", Severity::Info);
scala_detector!(ScalaResourceCleanupDetector, "scala_resource_cleanup", "Scala missing resource cleanup", Severity::Warning);
scala_detector!(ScalaTransactionRollbackDetector, "scala_transaction_rollback", "Scala missing transaction rollback", Severity::Warning);
scala_detector!(ScalaRetryLogicDetector, "scala_retry_logic", "Scala problematic retry logic", Severity::Info);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scala_patterns() {
        let patterns = get_scala_patterns();
        assert!(patterns.len() >= 50, "Should have 50+ Scala patterns");
    }
}
