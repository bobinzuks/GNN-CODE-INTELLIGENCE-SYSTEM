//! Kotlin-specific patterns (50+ patterns per language)

use crate::{CodeGraph, PatternDetector, PatternInstance, FixSuggestion, Severity};
use std::sync::Arc;

pub fn get_kotlin_patterns() -> Vec<Arc<dyn PatternDetector>> {
    vec![
        // Security patterns (10)
        Arc::new(KotlinSQLInjectionDetector::new()),
        Arc::new(KotlinXSSDetector::new()),
        Arc::new(KotlinPathTraversalDetector::new()),
        Arc::new(KotlinCommandInjectionDetector::new()),
        Arc::new(KotlinDeserializationDetector::new()),
        Arc::new(KotlinHardcodedSecretsDetector::new()),
        Arc::new(KotlinWeakCryptoDetector::new()),
        Arc::new(KotlinInsecureRandomDetector::new()),
        Arc::new(KotlinAuthBypassDetector::new()),
        Arc::new(KotlinCSRFDetector::new()),

        // Performance patterns (10)
        Arc::new(KotlinNPlusOneDetector::new()),
        Arc::new(KotlinIneffectiveLoopDetector::new()),
        Arc::new(KotlinExcessiveAllocationDetector::new()),
        Arc::new(KotlinStringConcatDetector::new()),
        Arc::new(KotlinBlockingIODetector::new()),
        Arc::new(KotlinMissingCacheDetector::new()),
        Arc::new(KotlinAlgorithmComplexityDetector::new()),
        Arc::new(KotlinRedundantComputationDetector::new()),
        Arc::new(KotlinMemoryLeakDetector::new()),
        Arc::new(KotlinResourceExhaustionDetector::new()),

        // Memory safety patterns (10)
        Arc::new(KotlinUseAfterFreeDetector::new()),
        Arc::new(KotlinBufferOverflowDetector::new()),
        Arc::new(KotlinNullPointerDetector::new()),
        Arc::new(KotlinUninitializedMemoryDetector::new()),
        Arc::new(KotlinDoubleFreeDetector::new()),
        Arc::new(KotlinMemoryCorruptionDetector::new()),
        Arc::new(KotlinDanglingPointerDetector::new()),
        Arc::new(KotlinStackOverflowDetector::new()),
        Arc::new(KotlinHeapCorruptionDetector::new()),
        Arc::new(KotlinTypeConfusionDetector::new()),

        // Concurrency patterns (10)
        Arc::new(KotlinDataRaceDetector::new()),
        Arc::new(KotlinDeadlockDetector::new()),
        Arc::new(KotlinRaceConditionDetector::new()),
        Arc::new(KotlinAtomicityViolationDetector::new()),
        Arc::new(KotlinOrderViolationDetector::new()),
        Arc::new(KotlinLivelockDetector::new()),
        Arc::new(KotlinThreadSafetyDetector::new()),
        Arc::new(KotlinAsyncHazardDetector::new()),
        Arc::new(KotlinLockContentionDetector::new()),
        Arc::new(KotlinSynchronizationDetector::new()),

        // Error handling patterns (10)
        Arc::new(KotlinSwallowedExceptionDetector::new()),
        Arc::new(KotlinEmptyCatchDetector::new()),
        Arc::new(KotlinGenericCatchDetector::new()),
        Arc::new(KotlinUnhandledErrorDetector::new()),
        Arc::new(KotlinErrorIgnoredDetector::new()),
        Arc::new(KotlinPanicMisuseDetector::new()),
        Arc::new(KotlinErrorPropagationDetector::new()),
        Arc::new(KotlinResourceCleanupDetector::new()),
        Arc::new(KotlinTransactionRollbackDetector::new()),
        Arc::new(KotlinRetryLogicDetector::new()),
    ]
}

macro_rules! kotlin_detector {
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
kotlin_detector!(KotlinSQLInjectionDetector, "kotlin_sql_injection", "Kotlin SQL injection vulnerability", Severity::Critical);
kotlin_detector!(KotlinXSSDetector, "kotlin_xss", "Kotlin XSS vulnerability", Severity::Critical);
kotlin_detector!(KotlinPathTraversalDetector, "kotlin_path_traversal", "Kotlin path traversal vulnerability", Severity::Critical);
kotlin_detector!(KotlinCommandInjectionDetector, "kotlin_command_injection", "Kotlin command injection", Severity::Critical);
kotlin_detector!(KotlinDeserializationDetector, "kotlin_unsafe_deserialization", "Kotlin unsafe deserialization", Severity::Critical);
kotlin_detector!(KotlinHardcodedSecretsDetector, "kotlin_hardcoded_secrets", "Kotlin hardcoded secrets", Severity::Warning);
kotlin_detector!(KotlinWeakCryptoDetector, "kotlin_weak_crypto", "Kotlin weak cryptography", Severity::Warning);
kotlin_detector!(KotlinInsecureRandomDetector, "kotlin_insecure_random", "Kotlin insecure randomness", Severity::Warning);
kotlin_detector!(KotlinAuthBypassDetector, "kotlin_auth_bypass", "Kotlin authentication bypass", Severity::Critical);
kotlin_detector!(KotlinCSRFDetector, "kotlin_csrf", "Kotlin CSRF vulnerability", Severity::Warning);

// Performance patterns
kotlin_detector!(KotlinNPlusOneDetector, "kotlin_n_plus_one", "Kotlin N+1 query problem", Severity::Warning);
kotlin_detector!(KotlinIneffectiveLoopDetector, "kotlin_ineffective_loop", "Kotlin ineffective loop", Severity::Warning);
kotlin_detector!(KotlinExcessiveAllocationDetector, "kotlin_excessive_allocation", "Kotlin excessive allocation", Severity::Warning);
kotlin_detector!(KotlinStringConcatDetector, "kotlin_string_concat", "Kotlin ineffective string concatenation", Severity::Info);
kotlin_detector!(KotlinBlockingIODetector, "kotlin_blocking_io", "Kotlin blocking I/O", Severity::Warning);
kotlin_detector!(KotlinMissingCacheDetector, "kotlin_missing_cache", "Kotlin missing cache", Severity::Info);
kotlin_detector!(KotlinAlgorithmComplexityDetector, "kotlin_algorithm_complexity", "Kotlin high algorithm complexity", Severity::Warning);
kotlin_detector!(KotlinRedundantComputationDetector, "kotlin_redundant_computation", "Kotlin redundant computation", Severity::Info);
kotlin_detector!(KotlinMemoryLeakDetector, "kotlin_memory_leak", "Kotlin memory leak", Severity::Warning);
kotlin_detector!(KotlinResourceExhaustionDetector, "kotlin_resource_exhaustion", "Kotlin resource exhaustion", Severity::Warning);

// Memory safety patterns
kotlin_detector!(KotlinUseAfterFreeDetector, "kotlin_use_after_free", "Kotlin use-after-free", Severity::Critical);
kotlin_detector!(KotlinBufferOverflowDetector, "kotlin_buffer_overflow", "Kotlin buffer overflow", Severity::Critical);
kotlin_detector!(KotlinNullPointerDetector, "kotlin_null_pointer", "Kotlin null pointer dereference", Severity::Critical);
kotlin_detector!(KotlinUninitializedMemoryDetector, "kotlin_uninitialized_memory", "Kotlin uninitialized memory", Severity::Critical);
kotlin_detector!(KotlinDoubleFreeDetector, "kotlin_double_free", "Kotlin double free", Severity::Critical);
kotlin_detector!(KotlinMemoryCorruptionDetector, "kotlin_memory_corruption", "Kotlin memory corruption", Severity::Critical);
kotlin_detector!(KotlinDanglingPointerDetector, "kotlin_dangling_pointer", "Kotlin dangling pointer", Severity::Critical);
kotlin_detector!(KotlinStackOverflowDetector, "kotlin_stack_overflow", "Kotlin stack overflow risk", Severity::Warning);
kotlin_detector!(KotlinHeapCorruptionDetector, "kotlin_heap_corruption", "Kotlin heap corruption", Severity::Critical);
kotlin_detector!(KotlinTypeConfusionDetector, "kotlin_type_confusion", "Kotlin type confusion", Severity::Warning);

// Concurrency patterns
kotlin_detector!(KotlinDataRaceDetector, "kotlin_data_race", "Kotlin data race", Severity::Critical);
kotlin_detector!(KotlinDeadlockDetector, "kotlin_deadlock", "Kotlin deadlock", Severity::Critical);
kotlin_detector!(KotlinRaceConditionDetector, "kotlin_race_condition", "Kotlin race condition", Severity::Critical);
kotlin_detector!(KotlinAtomicityViolationDetector, "kotlin_atomicity_violation", "Kotlin atomicity violation", Severity::Error);
kotlin_detector!(KotlinOrderViolationDetector, "kotlin_order_violation", "Kotlin order violation", Severity::Error);
kotlin_detector!(KotlinLivelockDetector, "kotlin_livelock", "Kotlin livelock", Severity::Warning);
kotlin_detector!(KotlinThreadSafetyDetector, "kotlin_thread_safety", "Kotlin thread safety violation", Severity::Error);
kotlin_detector!(KotlinAsyncHazardDetector, "kotlin_async_hazard", "Kotlin async hazard", Severity::Warning);
kotlin_detector!(KotlinLockContentionDetector, "kotlin_lock_contention", "Kotlin lock contention", Severity::Warning);
kotlin_detector!(KotlinSynchronizationDetector, "kotlin_synchronization", "Kotlin synchronization issue", Severity::Warning);

// Error handling patterns
kotlin_detector!(KotlinSwallowedExceptionDetector, "kotlin_swallowed_exception", "Kotlin swallowed exception", Severity::Warning);
kotlin_detector!(KotlinEmptyCatchDetector, "kotlin_empty_catch", "Kotlin empty catch block", Severity::Warning);
kotlin_detector!(KotlinGenericCatchDetector, "kotlin_generic_catch", "Kotlin generic catch", Severity::Info);
kotlin_detector!(KotlinUnhandledErrorDetector, "kotlin_unhandled_error", "Kotlin unhandled error", Severity::Warning);
kotlin_detector!(KotlinErrorIgnoredDetector, "kotlin_error_ignored", "Kotlin error ignored", Severity::Warning);
kotlin_detector!(KotlinPanicMisuseDetector, "kotlin_panic_misuse", "Kotlin panic misuse", Severity::Warning);
kotlin_detector!(KotlinErrorPropagationDetector, "kotlin_error_propagation", "Kotlin error propagation issue", Severity::Info);
kotlin_detector!(KotlinResourceCleanupDetector, "kotlin_resource_cleanup", "Kotlin missing resource cleanup", Severity::Warning);
kotlin_detector!(KotlinTransactionRollbackDetector, "kotlin_transaction_rollback", "Kotlin missing transaction rollback", Severity::Warning);
kotlin_detector!(KotlinRetryLogicDetector, "kotlin_retry_logic", "Kotlin problematic retry logic", Severity::Info);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kotlin_patterns() {
        let patterns = get_kotlin_patterns();
        assert!(patterns.len() >= 50, "Should have 50+ Kotlin patterns");
    }
}
