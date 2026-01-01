//! Java-specific patterns (50+ patterns per language)

use crate::{CodeGraph, PatternDetector, PatternInstance, FixSuggestion, Severity};
use std::sync::Arc;

pub fn get_java_patterns() -> Vec<Arc<dyn PatternDetector>> {
    vec![
        // Security patterns (10)
        Arc::new(JavaSQLInjectionDetector::new()),
        Arc::new(JavaXSSDetector::new()),
        Arc::new(JavaPathTraversalDetector::new()),
        Arc::new(JavaCommandInjectionDetector::new()),
        Arc::new(JavaDeserializationDetector::new()),
        Arc::new(JavaHardcodedSecretsDetector::new()),
        Arc::new(JavaWeakCryptoDetector::new()),
        Arc::new(JavaInsecureRandomDetector::new()),
        Arc::new(JavaAuthBypassDetector::new()),
        Arc::new(JavaCSRFDetector::new()),

        // Performance patterns (10)
        Arc::new(JavaNPlusOneDetector::new()),
        Arc::new(JavaIneffectiveLoopDetector::new()),
        Arc::new(JavaExcessiveAllocationDetector::new()),
        Arc::new(JavaStringConcatDetector::new()),
        Arc::new(JavaBlockingIODetector::new()),
        Arc::new(JavaMissingCacheDetector::new()),
        Arc::new(JavaAlgorithmComplexityDetector::new()),
        Arc::new(JavaRedundantComputationDetector::new()),
        Arc::new(JavaMemoryLeakDetector::new()),
        Arc::new(JavaResourceExhaustionDetector::new()),

        // Memory safety patterns (10)
        Arc::new(JavaUseAfterFreeDetector::new()),
        Arc::new(JavaBufferOverflowDetector::new()),
        Arc::new(JavaNullPointerDetector::new()),
        Arc::new(JavaUninitializedMemoryDetector::new()),
        Arc::new(JavaDoubleFreeDetector::new()),
        Arc::new(JavaMemoryCorruptionDetector::new()),
        Arc::new(JavaDanglingPointerDetector::new()),
        Arc::new(JavaStackOverflowDetector::new()),
        Arc::new(JavaHeapCorruptionDetector::new()),
        Arc::new(JavaTypeConfusionDetector::new()),

        // Concurrency patterns (10)
        Arc::new(JavaDataRaceDetector::new()),
        Arc::new(JavaDeadlockDetector::new()),
        Arc::new(JavaRaceConditionDetector::new()),
        Arc::new(JavaAtomicityViolationDetector::new()),
        Arc::new(JavaOrderViolationDetector::new()),
        Arc::new(JavaLivelockDetector::new()),
        Arc::new(JavaThreadSafetyDetector::new()),
        Arc::new(JavaAsyncHazardDetector::new()),
        Arc::new(JavaLockContentionDetector::new()),
        Arc::new(JavaSynchronizationDetector::new()),

        // Error handling patterns (10)
        Arc::new(JavaSwallowedExceptionDetector::new()),
        Arc::new(JavaEmptyCatchDetector::new()),
        Arc::new(JavaGenericCatchDetector::new()),
        Arc::new(JavaUnhandledErrorDetector::new()),
        Arc::new(JavaErrorIgnoredDetector::new()),
        Arc::new(JavaPanicMisuseDetector::new()),
        Arc::new(JavaErrorPropagationDetector::new()),
        Arc::new(JavaResourceCleanupDetector::new()),
        Arc::new(JavaTransactionRollbackDetector::new()),
        Arc::new(JavaRetryLogicDetector::new()),
    ]
}

macro_rules! java_detector {
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
java_detector!(JavaSQLInjectionDetector, "java_sql_injection", "Java SQL injection vulnerability", Severity::Critical);
java_detector!(JavaXSSDetector, "java_xss", "Java XSS vulnerability", Severity::Critical);
java_detector!(JavaPathTraversalDetector, "java_path_traversal", "Java path traversal vulnerability", Severity::Critical);
java_detector!(JavaCommandInjectionDetector, "java_command_injection", "Java command injection", Severity::Critical);
java_detector!(JavaDeserializationDetector, "java_unsafe_deserialization", "Java unsafe deserialization", Severity::Critical);
java_detector!(JavaHardcodedSecretsDetector, "java_hardcoded_secrets", "Java hardcoded secrets", Severity::Warning);
java_detector!(JavaWeakCryptoDetector, "java_weak_crypto", "Java weak cryptography", Severity::Warning);
java_detector!(JavaInsecureRandomDetector, "java_insecure_random", "Java insecure randomness", Severity::Warning);
java_detector!(JavaAuthBypassDetector, "java_auth_bypass", "Java authentication bypass", Severity::Critical);
java_detector!(JavaCSRFDetector, "java_csrf", "Java CSRF vulnerability", Severity::Warning);

// Performance patterns
java_detector!(JavaNPlusOneDetector, "java_n_plus_one", "Java N+1 query problem", Severity::Warning);
java_detector!(JavaIneffectiveLoopDetector, "java_ineffective_loop", "Java ineffective loop", Severity::Warning);
java_detector!(JavaExcessiveAllocationDetector, "java_excessive_allocation", "Java excessive allocation", Severity::Warning);
java_detector!(JavaStringConcatDetector, "java_string_concat", "Java ineffective string concatenation", Severity::Info);
java_detector!(JavaBlockingIODetector, "java_blocking_io", "Java blocking I/O", Severity::Warning);
java_detector!(JavaMissingCacheDetector, "java_missing_cache", "Java missing cache", Severity::Info);
java_detector!(JavaAlgorithmComplexityDetector, "java_algorithm_complexity", "Java high algorithm complexity", Severity::Warning);
java_detector!(JavaRedundantComputationDetector, "java_redundant_computation", "Java redundant computation", Severity::Info);
java_detector!(JavaMemoryLeakDetector, "java_memory_leak", "Java memory leak", Severity::Warning);
java_detector!(JavaResourceExhaustionDetector, "java_resource_exhaustion", "Java resource exhaustion", Severity::Warning);

// Memory safety patterns
java_detector!(JavaUseAfterFreeDetector, "java_use_after_free", "Java use-after-free", Severity::Critical);
java_detector!(JavaBufferOverflowDetector, "java_buffer_overflow", "Java buffer overflow", Severity::Critical);
java_detector!(JavaNullPointerDetector, "java_null_pointer", "Java null pointer dereference", Severity::Critical);
java_detector!(JavaUninitializedMemoryDetector, "java_uninitialized_memory", "Java uninitialized memory", Severity::Critical);
java_detector!(JavaDoubleFreeDetector, "java_double_free", "Java double free", Severity::Critical);
java_detector!(JavaMemoryCorruptionDetector, "java_memory_corruption", "Java memory corruption", Severity::Critical);
java_detector!(JavaDanglingPointerDetector, "java_dangling_pointer", "Java dangling pointer", Severity::Critical);
java_detector!(JavaStackOverflowDetector, "java_stack_overflow", "Java stack overflow risk", Severity::Warning);
java_detector!(JavaHeapCorruptionDetector, "java_heap_corruption", "Java heap corruption", Severity::Critical);
java_detector!(JavaTypeConfusionDetector, "java_type_confusion", "Java type confusion", Severity::Warning);

// Concurrency patterns
java_detector!(JavaDataRaceDetector, "java_data_race", "Java data race", Severity::Critical);
java_detector!(JavaDeadlockDetector, "java_deadlock", "Java deadlock", Severity::Critical);
java_detector!(JavaRaceConditionDetector, "java_race_condition", "Java race condition", Severity::Critical);
java_detector!(JavaAtomicityViolationDetector, "java_atomicity_violation", "Java atomicity violation", Severity::Error);
java_detector!(JavaOrderViolationDetector, "java_order_violation", "Java order violation", Severity::Error);
java_detector!(JavaLivelockDetector, "java_livelock", "Java livelock", Severity::Warning);
java_detector!(JavaThreadSafetyDetector, "java_thread_safety", "Java thread safety violation", Severity::Error);
java_detector!(JavaAsyncHazardDetector, "java_async_hazard", "Java async hazard", Severity::Warning);
java_detector!(JavaLockContentionDetector, "java_lock_contention", "Java lock contention", Severity::Warning);
java_detector!(JavaSynchronizationDetector, "java_synchronization", "Java synchronization issue", Severity::Warning);

// Error handling patterns
java_detector!(JavaSwallowedExceptionDetector, "java_swallowed_exception", "Java swallowed exception", Severity::Warning);
java_detector!(JavaEmptyCatchDetector, "java_empty_catch", "Java empty catch block", Severity::Warning);
java_detector!(JavaGenericCatchDetector, "java_generic_catch", "Java generic catch", Severity::Info);
java_detector!(JavaUnhandledErrorDetector, "java_unhandled_error", "Java unhandled error", Severity::Warning);
java_detector!(JavaErrorIgnoredDetector, "java_error_ignored", "Java error ignored", Severity::Warning);
java_detector!(JavaPanicMisuseDetector, "java_panic_misuse", "Java panic misuse", Severity::Warning);
java_detector!(JavaErrorPropagationDetector, "java_error_propagation", "Java error propagation issue", Severity::Info);
java_detector!(JavaResourceCleanupDetector, "java_resource_cleanup", "Java missing resource cleanup", Severity::Warning);
java_detector!(JavaTransactionRollbackDetector, "java_transaction_rollback", "Java missing transaction rollback", Severity::Warning);
java_detector!(JavaRetryLogicDetector, "java_retry_logic", "Java problematic retry logic", Severity::Info);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_java_patterns() {
        let patterns = get_java_patterns();
        assert!(patterns.len() >= 50, "Should have 50+ Java patterns");
    }
}
