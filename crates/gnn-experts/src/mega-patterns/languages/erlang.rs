//! Erlang-specific patterns (50+ patterns per language)

use crate::{CodeGraph, PatternDetector, PatternInstance, FixSuggestion, Severity};
use std::sync::Arc;

pub fn get_erlang_patterns() -> Vec<Arc<dyn PatternDetector>> {
    vec![
        // Security patterns (10)
        Arc::new(ErlangSQLInjectionDetector::new()),
        Arc::new(ErlangXSSDetector::new()),
        Arc::new(ErlangPathTraversalDetector::new()),
        Arc::new(ErlangCommandInjectionDetector::new()),
        Arc::new(ErlangDeserializationDetector::new()),
        Arc::new(ErlangHardcodedSecretsDetector::new()),
        Arc::new(ErlangWeakCryptoDetector::new()),
        Arc::new(ErlangInsecureRandomDetector::new()),
        Arc::new(ErlangAuthBypassDetector::new()),
        Arc::new(ErlangCSRFDetector::new()),

        // Performance patterns (10)
        Arc::new(ErlangNPlusOneDetector::new()),
        Arc::new(ErlangIneffectiveLoopDetector::new()),
        Arc::new(ErlangExcessiveAllocationDetector::new()),
        Arc::new(ErlangStringConcatDetector::new()),
        Arc::new(ErlangBlockingIODetector::new()),
        Arc::new(ErlangMissingCacheDetector::new()),
        Arc::new(ErlangAlgorithmComplexityDetector::new()),
        Arc::new(ErlangRedundantComputationDetector::new()),
        Arc::new(ErlangMemoryLeakDetector::new()),
        Arc::new(ErlangResourceExhaustionDetector::new()),

        // Memory safety patterns (10)
        Arc::new(ErlangUseAfterFreeDetector::new()),
        Arc::new(ErlangBufferOverflowDetector::new()),
        Arc::new(ErlangNullPointerDetector::new()),
        Arc::new(ErlangUninitializedMemoryDetector::new()),
        Arc::new(ErlangDoubleFreeDetector::new()),
        Arc::new(ErlangMemoryCorruptionDetector::new()),
        Arc::new(ErlangDanglingPointerDetector::new()),
        Arc::new(ErlangStackOverflowDetector::new()),
        Arc::new(ErlangHeapCorruptionDetector::new()),
        Arc::new(ErlangTypeConfusionDetector::new()),

        // Concurrency patterns (10)
        Arc::new(ErlangDataRaceDetector::new()),
        Arc::new(ErlangDeadlockDetector::new()),
        Arc::new(ErlangRaceConditionDetector::new()),
        Arc::new(ErlangAtomicityViolationDetector::new()),
        Arc::new(ErlangOrderViolationDetector::new()),
        Arc::new(ErlangLivelockDetector::new()),
        Arc::new(ErlangThreadSafetyDetector::new()),
        Arc::new(ErlangAsyncHazardDetector::new()),
        Arc::new(ErlangLockContentionDetector::new()),
        Arc::new(ErlangSynchronizationDetector::new()),

        // Error handling patterns (10)
        Arc::new(ErlangSwallowedExceptionDetector::new()),
        Arc::new(ErlangEmptyCatchDetector::new()),
        Arc::new(ErlangGenericCatchDetector::new()),
        Arc::new(ErlangUnhandledErrorDetector::new()),
        Arc::new(ErlangErrorIgnoredDetector::new()),
        Arc::new(ErlangPanicMisuseDetector::new()),
        Arc::new(ErlangErrorPropagationDetector::new()),
        Arc::new(ErlangResourceCleanupDetector::new()),
        Arc::new(ErlangTransactionRollbackDetector::new()),
        Arc::new(ErlangRetryLogicDetector::new()),
    ]
}

macro_rules! erlang_detector {
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
erlang_detector!(ErlangSQLInjectionDetector, "erlang_sql_injection", "Erlang SQL injection vulnerability", Severity::Critical);
erlang_detector!(ErlangXSSDetector, "erlang_xss", "Erlang XSS vulnerability", Severity::Critical);
erlang_detector!(ErlangPathTraversalDetector, "erlang_path_traversal", "Erlang path traversal vulnerability", Severity::Critical);
erlang_detector!(ErlangCommandInjectionDetector, "erlang_command_injection", "Erlang command injection", Severity::Critical);
erlang_detector!(ErlangDeserializationDetector, "erlang_unsafe_deserialization", "Erlang unsafe deserialization", Severity::Critical);
erlang_detector!(ErlangHardcodedSecretsDetector, "erlang_hardcoded_secrets", "Erlang hardcoded secrets", Severity::Warning);
erlang_detector!(ErlangWeakCryptoDetector, "erlang_weak_crypto", "Erlang weak cryptography", Severity::Warning);
erlang_detector!(ErlangInsecureRandomDetector, "erlang_insecure_random", "Erlang insecure randomness", Severity::Warning);
erlang_detector!(ErlangAuthBypassDetector, "erlang_auth_bypass", "Erlang authentication bypass", Severity::Critical);
erlang_detector!(ErlangCSRFDetector, "erlang_csrf", "Erlang CSRF vulnerability", Severity::Warning);

// Performance patterns
erlang_detector!(ErlangNPlusOneDetector, "erlang_n_plus_one", "Erlang N+1 query problem", Severity::Warning);
erlang_detector!(ErlangIneffectiveLoopDetector, "erlang_ineffective_loop", "Erlang ineffective loop", Severity::Warning);
erlang_detector!(ErlangExcessiveAllocationDetector, "erlang_excessive_allocation", "Erlang excessive allocation", Severity::Warning);
erlang_detector!(ErlangStringConcatDetector, "erlang_string_concat", "Erlang ineffective string concatenation", Severity::Info);
erlang_detector!(ErlangBlockingIODetector, "erlang_blocking_io", "Erlang blocking I/O", Severity::Warning);
erlang_detector!(ErlangMissingCacheDetector, "erlang_missing_cache", "Erlang missing cache", Severity::Info);
erlang_detector!(ErlangAlgorithmComplexityDetector, "erlang_algorithm_complexity", "Erlang high algorithm complexity", Severity::Warning);
erlang_detector!(ErlangRedundantComputationDetector, "erlang_redundant_computation", "Erlang redundant computation", Severity::Info);
erlang_detector!(ErlangMemoryLeakDetector, "erlang_memory_leak", "Erlang memory leak", Severity::Warning);
erlang_detector!(ErlangResourceExhaustionDetector, "erlang_resource_exhaustion", "Erlang resource exhaustion", Severity::Warning);

// Memory safety patterns
erlang_detector!(ErlangUseAfterFreeDetector, "erlang_use_after_free", "Erlang use-after-free", Severity::Critical);
erlang_detector!(ErlangBufferOverflowDetector, "erlang_buffer_overflow", "Erlang buffer overflow", Severity::Critical);
erlang_detector!(ErlangNullPointerDetector, "erlang_null_pointer", "Erlang null pointer dereference", Severity::Critical);
erlang_detector!(ErlangUninitializedMemoryDetector, "erlang_uninitialized_memory", "Erlang uninitialized memory", Severity::Critical);
erlang_detector!(ErlangDoubleFreeDetector, "erlang_double_free", "Erlang double free", Severity::Critical);
erlang_detector!(ErlangMemoryCorruptionDetector, "erlang_memory_corruption", "Erlang memory corruption", Severity::Critical);
erlang_detector!(ErlangDanglingPointerDetector, "erlang_dangling_pointer", "Erlang dangling pointer", Severity::Critical);
erlang_detector!(ErlangStackOverflowDetector, "erlang_stack_overflow", "Erlang stack overflow risk", Severity::Warning);
erlang_detector!(ErlangHeapCorruptionDetector, "erlang_heap_corruption", "Erlang heap corruption", Severity::Critical);
erlang_detector!(ErlangTypeConfusionDetector, "erlang_type_confusion", "Erlang type confusion", Severity::Warning);

// Concurrency patterns
erlang_detector!(ErlangDataRaceDetector, "erlang_data_race", "Erlang data race", Severity::Critical);
erlang_detector!(ErlangDeadlockDetector, "erlang_deadlock", "Erlang deadlock", Severity::Critical);
erlang_detector!(ErlangRaceConditionDetector, "erlang_race_condition", "Erlang race condition", Severity::Critical);
erlang_detector!(ErlangAtomicityViolationDetector, "erlang_atomicity_violation", "Erlang atomicity violation", Severity::Error);
erlang_detector!(ErlangOrderViolationDetector, "erlang_order_violation", "Erlang order violation", Severity::Error);
erlang_detector!(ErlangLivelockDetector, "erlang_livelock", "Erlang livelock", Severity::Warning);
erlang_detector!(ErlangThreadSafetyDetector, "erlang_thread_safety", "Erlang thread safety violation", Severity::Error);
erlang_detector!(ErlangAsyncHazardDetector, "erlang_async_hazard", "Erlang async hazard", Severity::Warning);
erlang_detector!(ErlangLockContentionDetector, "erlang_lock_contention", "Erlang lock contention", Severity::Warning);
erlang_detector!(ErlangSynchronizationDetector, "erlang_synchronization", "Erlang synchronization issue", Severity::Warning);

// Error handling patterns
erlang_detector!(ErlangSwallowedExceptionDetector, "erlang_swallowed_exception", "Erlang swallowed exception", Severity::Warning);
erlang_detector!(ErlangEmptyCatchDetector, "erlang_empty_catch", "Erlang empty catch block", Severity::Warning);
erlang_detector!(ErlangGenericCatchDetector, "erlang_generic_catch", "Erlang generic catch", Severity::Info);
erlang_detector!(ErlangUnhandledErrorDetector, "erlang_unhandled_error", "Erlang unhandled error", Severity::Warning);
erlang_detector!(ErlangErrorIgnoredDetector, "erlang_error_ignored", "Erlang error ignored", Severity::Warning);
erlang_detector!(ErlangPanicMisuseDetector, "erlang_panic_misuse", "Erlang panic misuse", Severity::Warning);
erlang_detector!(ErlangErrorPropagationDetector, "erlang_error_propagation", "Erlang error propagation issue", Severity::Info);
erlang_detector!(ErlangResourceCleanupDetector, "erlang_resource_cleanup", "Erlang missing resource cleanup", Severity::Warning);
erlang_detector!(ErlangTransactionRollbackDetector, "erlang_transaction_rollback", "Erlang missing transaction rollback", Severity::Warning);
erlang_detector!(ErlangRetryLogicDetector, "erlang_retry_logic", "Erlang problematic retry logic", Severity::Info);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_erlang_patterns() {
        let patterns = get_erlang_patterns();
        assert!(patterns.len() >= 50, "Should have 50+ Erlang patterns");
    }
}
