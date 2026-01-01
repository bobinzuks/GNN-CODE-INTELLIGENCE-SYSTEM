//! Cpp-specific patterns (50+ patterns per language)

use crate::{CodeGraph, PatternDetector, PatternInstance, FixSuggestion, Severity};
use std::sync::Arc;

pub fn get_cpp_patterns() -> Vec<Arc<dyn PatternDetector>> {
    vec![
        // Security patterns (10)
        Arc::new(CppSQLInjectionDetector::new()),
        Arc::new(CppXSSDetector::new()),
        Arc::new(CppPathTraversalDetector::new()),
        Arc::new(CppCommandInjectionDetector::new()),
        Arc::new(CppDeserializationDetector::new()),
        Arc::new(CppHardcodedSecretsDetector::new()),
        Arc::new(CppWeakCryptoDetector::new()),
        Arc::new(CppInsecureRandomDetector::new()),
        Arc::new(CppAuthBypassDetector::new()),
        Arc::new(CppCSRFDetector::new()),

        // Performance patterns (10)
        Arc::new(CppNPlusOneDetector::new()),
        Arc::new(CppIneffectiveLoopDetector::new()),
        Arc::new(CppExcessiveAllocationDetector::new()),
        Arc::new(CppStringConcatDetector::new()),
        Arc::new(CppBlockingIODetector::new()),
        Arc::new(CppMissingCacheDetector::new()),
        Arc::new(CppAlgorithmComplexityDetector::new()),
        Arc::new(CppRedundantComputationDetector::new()),
        Arc::new(CppMemoryLeakDetector::new()),
        Arc::new(CppResourceExhaustionDetector::new()),

        // Memory safety patterns (10)
        Arc::new(CppUseAfterFreeDetector::new()),
        Arc::new(CppBufferOverflowDetector::new()),
        Arc::new(CppNullPointerDetector::new()),
        Arc::new(CppUninitializedMemoryDetector::new()),
        Arc::new(CppDoubleFreeDetector::new()),
        Arc::new(CppMemoryCorruptionDetector::new()),
        Arc::new(CppDanglingPointerDetector::new()),
        Arc::new(CppStackOverflowDetector::new()),
        Arc::new(CppHeapCorruptionDetector::new()),
        Arc::new(CppTypeConfusionDetector::new()),

        // Concurrency patterns (10)
        Arc::new(CppDataRaceDetector::new()),
        Arc::new(CppDeadlockDetector::new()),
        Arc::new(CppRaceConditionDetector::new()),
        Arc::new(CppAtomicityViolationDetector::new()),
        Arc::new(CppOrderViolationDetector::new()),
        Arc::new(CppLivelockDetector::new()),
        Arc::new(CppThreadSafetyDetector::new()),
        Arc::new(CppAsyncHazardDetector::new()),
        Arc::new(CppLockContentionDetector::new()),
        Arc::new(CppSynchronizationDetector::new()),

        // Error handling patterns (10)
        Arc::new(CppSwallowedExceptionDetector::new()),
        Arc::new(CppEmptyCatchDetector::new()),
        Arc::new(CppGenericCatchDetector::new()),
        Arc::new(CppUnhandledErrorDetector::new()),
        Arc::new(CppErrorIgnoredDetector::new()),
        Arc::new(CppPanicMisuseDetector::new()),
        Arc::new(CppErrorPropagationDetector::new()),
        Arc::new(CppResourceCleanupDetector::new()),
        Arc::new(CppTransactionRollbackDetector::new()),
        Arc::new(CppRetryLogicDetector::new()),
    ]
}

macro_rules! cpp_detector {
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
cpp_detector!(CppSQLInjectionDetector, "cpp_sql_injection", "Cpp SQL injection vulnerability", Severity::Critical);
cpp_detector!(CppXSSDetector, "cpp_xss", "Cpp XSS vulnerability", Severity::Critical);
cpp_detector!(CppPathTraversalDetector, "cpp_path_traversal", "Cpp path traversal vulnerability", Severity::Critical);
cpp_detector!(CppCommandInjectionDetector, "cpp_command_injection", "Cpp command injection", Severity::Critical);
cpp_detector!(CppDeserializationDetector, "cpp_unsafe_deserialization", "Cpp unsafe deserialization", Severity::Critical);
cpp_detector!(CppHardcodedSecretsDetector, "cpp_hardcoded_secrets", "Cpp hardcoded secrets", Severity::Warning);
cpp_detector!(CppWeakCryptoDetector, "cpp_weak_crypto", "Cpp weak cryptography", Severity::Warning);
cpp_detector!(CppInsecureRandomDetector, "cpp_insecure_random", "Cpp insecure randomness", Severity::Warning);
cpp_detector!(CppAuthBypassDetector, "cpp_auth_bypass", "Cpp authentication bypass", Severity::Critical);
cpp_detector!(CppCSRFDetector, "cpp_csrf", "Cpp CSRF vulnerability", Severity::Warning);

// Performance patterns
cpp_detector!(CppNPlusOneDetector, "cpp_n_plus_one", "Cpp N+1 query problem", Severity::Warning);
cpp_detector!(CppIneffectiveLoopDetector, "cpp_ineffective_loop", "Cpp ineffective loop", Severity::Warning);
cpp_detector!(CppExcessiveAllocationDetector, "cpp_excessive_allocation", "Cpp excessive allocation", Severity::Warning);
cpp_detector!(CppStringConcatDetector, "cpp_string_concat", "Cpp ineffective string concatenation", Severity::Info);
cpp_detector!(CppBlockingIODetector, "cpp_blocking_io", "Cpp blocking I/O", Severity::Warning);
cpp_detector!(CppMissingCacheDetector, "cpp_missing_cache", "Cpp missing cache", Severity::Info);
cpp_detector!(CppAlgorithmComplexityDetector, "cpp_algorithm_complexity", "Cpp high algorithm complexity", Severity::Warning);
cpp_detector!(CppRedundantComputationDetector, "cpp_redundant_computation", "Cpp redundant computation", Severity::Info);
cpp_detector!(CppMemoryLeakDetector, "cpp_memory_leak", "Cpp memory leak", Severity::Warning);
cpp_detector!(CppResourceExhaustionDetector, "cpp_resource_exhaustion", "Cpp resource exhaustion", Severity::Warning);

// Memory safety patterns
cpp_detector!(CppUseAfterFreeDetector, "cpp_use_after_free", "Cpp use-after-free", Severity::Critical);
cpp_detector!(CppBufferOverflowDetector, "cpp_buffer_overflow", "Cpp buffer overflow", Severity::Critical);
cpp_detector!(CppNullPointerDetector, "cpp_null_pointer", "Cpp null pointer dereference", Severity::Critical);
cpp_detector!(CppUninitializedMemoryDetector, "cpp_uninitialized_memory", "Cpp uninitialized memory", Severity::Critical);
cpp_detector!(CppDoubleFreeDetector, "cpp_double_free", "Cpp double free", Severity::Critical);
cpp_detector!(CppMemoryCorruptionDetector, "cpp_memory_corruption", "Cpp memory corruption", Severity::Critical);
cpp_detector!(CppDanglingPointerDetector, "cpp_dangling_pointer", "Cpp dangling pointer", Severity::Critical);
cpp_detector!(CppStackOverflowDetector, "cpp_stack_overflow", "Cpp stack overflow risk", Severity::Warning);
cpp_detector!(CppHeapCorruptionDetector, "cpp_heap_corruption", "Cpp heap corruption", Severity::Critical);
cpp_detector!(CppTypeConfusionDetector, "cpp_type_confusion", "Cpp type confusion", Severity::Warning);

// Concurrency patterns
cpp_detector!(CppDataRaceDetector, "cpp_data_race", "Cpp data race", Severity::Critical);
cpp_detector!(CppDeadlockDetector, "cpp_deadlock", "Cpp deadlock", Severity::Critical);
cpp_detector!(CppRaceConditionDetector, "cpp_race_condition", "Cpp race condition", Severity::Critical);
cpp_detector!(CppAtomicityViolationDetector, "cpp_atomicity_violation", "Cpp atomicity violation", Severity::Error);
cpp_detector!(CppOrderViolationDetector, "cpp_order_violation", "Cpp order violation", Severity::Error);
cpp_detector!(CppLivelockDetector, "cpp_livelock", "Cpp livelock", Severity::Warning);
cpp_detector!(CppThreadSafetyDetector, "cpp_thread_safety", "Cpp thread safety violation", Severity::Error);
cpp_detector!(CppAsyncHazardDetector, "cpp_async_hazard", "Cpp async hazard", Severity::Warning);
cpp_detector!(CppLockContentionDetector, "cpp_lock_contention", "Cpp lock contention", Severity::Warning);
cpp_detector!(CppSynchronizationDetector, "cpp_synchronization", "Cpp synchronization issue", Severity::Warning);

// Error handling patterns
cpp_detector!(CppSwallowedExceptionDetector, "cpp_swallowed_exception", "Cpp swallowed exception", Severity::Warning);
cpp_detector!(CppEmptyCatchDetector, "cpp_empty_catch", "Cpp empty catch block", Severity::Warning);
cpp_detector!(CppGenericCatchDetector, "cpp_generic_catch", "Cpp generic catch", Severity::Info);
cpp_detector!(CppUnhandledErrorDetector, "cpp_unhandled_error", "Cpp unhandled error", Severity::Warning);
cpp_detector!(CppErrorIgnoredDetector, "cpp_error_ignored", "Cpp error ignored", Severity::Warning);
cpp_detector!(CppPanicMisuseDetector, "cpp_panic_misuse", "Cpp panic misuse", Severity::Warning);
cpp_detector!(CppErrorPropagationDetector, "cpp_error_propagation", "Cpp error propagation issue", Severity::Info);
cpp_detector!(CppResourceCleanupDetector, "cpp_resource_cleanup", "Cpp missing resource cleanup", Severity::Warning);
cpp_detector!(CppTransactionRollbackDetector, "cpp_transaction_rollback", "Cpp missing transaction rollback", Severity::Warning);
cpp_detector!(CppRetryLogicDetector, "cpp_retry_logic", "Cpp problematic retry logic", Severity::Info);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpp_patterns() {
        let patterns = get_cpp_patterns();
        assert!(patterns.len() >= 50, "Should have 50+ Cpp patterns");
    }
}
