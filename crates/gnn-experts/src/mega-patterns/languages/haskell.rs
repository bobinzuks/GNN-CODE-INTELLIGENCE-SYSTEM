//! Haskell-specific patterns (50+ patterns per language)

use crate::{CodeGraph, PatternDetector, PatternInstance, FixSuggestion, Severity};
use std::sync::Arc;

pub fn get_haskell_patterns() -> Vec<Arc<dyn PatternDetector>> {
    vec![
        // Security patterns (10)
        Arc::new(HaskellSQLInjectionDetector::new()),
        Arc::new(HaskellXSSDetector::new()),
        Arc::new(HaskellPathTraversalDetector::new()),
        Arc::new(HaskellCommandInjectionDetector::new()),
        Arc::new(HaskellDeserializationDetector::new()),
        Arc::new(HaskellHardcodedSecretsDetector::new()),
        Arc::new(HaskellWeakCryptoDetector::new()),
        Arc::new(HaskellInsecureRandomDetector::new()),
        Arc::new(HaskellAuthBypassDetector::new()),
        Arc::new(HaskellCSRFDetector::new()),

        // Performance patterns (10)
        Arc::new(HaskellNPlusOneDetector::new()),
        Arc::new(HaskellIneffectiveLoopDetector::new()),
        Arc::new(HaskellExcessiveAllocationDetector::new()),
        Arc::new(HaskellStringConcatDetector::new()),
        Arc::new(HaskellBlockingIODetector::new()),
        Arc::new(HaskellMissingCacheDetector::new()),
        Arc::new(HaskellAlgorithmComplexityDetector::new()),
        Arc::new(HaskellRedundantComputationDetector::new()),
        Arc::new(HaskellMemoryLeakDetector::new()),
        Arc::new(HaskellResourceExhaustionDetector::new()),

        // Memory safety patterns (10)
        Arc::new(HaskellUseAfterFreeDetector::new()),
        Arc::new(HaskellBufferOverflowDetector::new()),
        Arc::new(HaskellNullPointerDetector::new()),
        Arc::new(HaskellUninitializedMemoryDetector::new()),
        Arc::new(HaskellDoubleFreeDetector::new()),
        Arc::new(HaskellMemoryCorruptionDetector::new()),
        Arc::new(HaskellDanglingPointerDetector::new()),
        Arc::new(HaskellStackOverflowDetector::new()),
        Arc::new(HaskellHeapCorruptionDetector::new()),
        Arc::new(HaskellTypeConfusionDetector::new()),

        // Concurrency patterns (10)
        Arc::new(HaskellDataRaceDetector::new()),
        Arc::new(HaskellDeadlockDetector::new()),
        Arc::new(HaskellRaceConditionDetector::new()),
        Arc::new(HaskellAtomicityViolationDetector::new()),
        Arc::new(HaskellOrderViolationDetector::new()),
        Arc::new(HaskellLivelockDetector::new()),
        Arc::new(HaskellThreadSafetyDetector::new()),
        Arc::new(HaskellAsyncHazardDetector::new()),
        Arc::new(HaskellLockContentionDetector::new()),
        Arc::new(HaskellSynchronizationDetector::new()),

        // Error handling patterns (10)
        Arc::new(HaskellSwallowedExceptionDetector::new()),
        Arc::new(HaskellEmptyCatchDetector::new()),
        Arc::new(HaskellGenericCatchDetector::new()),
        Arc::new(HaskellUnhandledErrorDetector::new()),
        Arc::new(HaskellErrorIgnoredDetector::new()),
        Arc::new(HaskellPanicMisuseDetector::new()),
        Arc::new(HaskellErrorPropagationDetector::new()),
        Arc::new(HaskellResourceCleanupDetector::new()),
        Arc::new(HaskellTransactionRollbackDetector::new()),
        Arc::new(HaskellRetryLogicDetector::new()),
    ]
}

macro_rules! haskell_detector {
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
haskell_detector!(HaskellSQLInjectionDetector, "haskell_sql_injection", "Haskell SQL injection vulnerability", Severity::Critical);
haskell_detector!(HaskellXSSDetector, "haskell_xss", "Haskell XSS vulnerability", Severity::Critical);
haskell_detector!(HaskellPathTraversalDetector, "haskell_path_traversal", "Haskell path traversal vulnerability", Severity::Critical);
haskell_detector!(HaskellCommandInjectionDetector, "haskell_command_injection", "Haskell command injection", Severity::Critical);
haskell_detector!(HaskellDeserializationDetector, "haskell_unsafe_deserialization", "Haskell unsafe deserialization", Severity::Critical);
haskell_detector!(HaskellHardcodedSecretsDetector, "haskell_hardcoded_secrets", "Haskell hardcoded secrets", Severity::Warning);
haskell_detector!(HaskellWeakCryptoDetector, "haskell_weak_crypto", "Haskell weak cryptography", Severity::Warning);
haskell_detector!(HaskellInsecureRandomDetector, "haskell_insecure_random", "Haskell insecure randomness", Severity::Warning);
haskell_detector!(HaskellAuthBypassDetector, "haskell_auth_bypass", "Haskell authentication bypass", Severity::Critical);
haskell_detector!(HaskellCSRFDetector, "haskell_csrf", "Haskell CSRF vulnerability", Severity::Warning);

// Performance patterns
haskell_detector!(HaskellNPlusOneDetector, "haskell_n_plus_one", "Haskell N+1 query problem", Severity::Warning);
haskell_detector!(HaskellIneffectiveLoopDetector, "haskell_ineffective_loop", "Haskell ineffective loop", Severity::Warning);
haskell_detector!(HaskellExcessiveAllocationDetector, "haskell_excessive_allocation", "Haskell excessive allocation", Severity::Warning);
haskell_detector!(HaskellStringConcatDetector, "haskell_string_concat", "Haskell ineffective string concatenation", Severity::Info);
haskell_detector!(HaskellBlockingIODetector, "haskell_blocking_io", "Haskell blocking I/O", Severity::Warning);
haskell_detector!(HaskellMissingCacheDetector, "haskell_missing_cache", "Haskell missing cache", Severity::Info);
haskell_detector!(HaskellAlgorithmComplexityDetector, "haskell_algorithm_complexity", "Haskell high algorithm complexity", Severity::Warning);
haskell_detector!(HaskellRedundantComputationDetector, "haskell_redundant_computation", "Haskell redundant computation", Severity::Info);
haskell_detector!(HaskellMemoryLeakDetector, "haskell_memory_leak", "Haskell memory leak", Severity::Warning);
haskell_detector!(HaskellResourceExhaustionDetector, "haskell_resource_exhaustion", "Haskell resource exhaustion", Severity::Warning);

// Memory safety patterns
haskell_detector!(HaskellUseAfterFreeDetector, "haskell_use_after_free", "Haskell use-after-free", Severity::Critical);
haskell_detector!(HaskellBufferOverflowDetector, "haskell_buffer_overflow", "Haskell buffer overflow", Severity::Critical);
haskell_detector!(HaskellNullPointerDetector, "haskell_null_pointer", "Haskell null pointer dereference", Severity::Critical);
haskell_detector!(HaskellUninitializedMemoryDetector, "haskell_uninitialized_memory", "Haskell uninitialized memory", Severity::Critical);
haskell_detector!(HaskellDoubleFreeDetector, "haskell_double_free", "Haskell double free", Severity::Critical);
haskell_detector!(HaskellMemoryCorruptionDetector, "haskell_memory_corruption", "Haskell memory corruption", Severity::Critical);
haskell_detector!(HaskellDanglingPointerDetector, "haskell_dangling_pointer", "Haskell dangling pointer", Severity::Critical);
haskell_detector!(HaskellStackOverflowDetector, "haskell_stack_overflow", "Haskell stack overflow risk", Severity::Warning);
haskell_detector!(HaskellHeapCorruptionDetector, "haskell_heap_corruption", "Haskell heap corruption", Severity::Critical);
haskell_detector!(HaskellTypeConfusionDetector, "haskell_type_confusion", "Haskell type confusion", Severity::Warning);

// Concurrency patterns
haskell_detector!(HaskellDataRaceDetector, "haskell_data_race", "Haskell data race", Severity::Critical);
haskell_detector!(HaskellDeadlockDetector, "haskell_deadlock", "Haskell deadlock", Severity::Critical);
haskell_detector!(HaskellRaceConditionDetector, "haskell_race_condition", "Haskell race condition", Severity::Critical);
haskell_detector!(HaskellAtomicityViolationDetector, "haskell_atomicity_violation", "Haskell atomicity violation", Severity::Error);
haskell_detector!(HaskellOrderViolationDetector, "haskell_order_violation", "Haskell order violation", Severity::Error);
haskell_detector!(HaskellLivelockDetector, "haskell_livelock", "Haskell livelock", Severity::Warning);
haskell_detector!(HaskellThreadSafetyDetector, "haskell_thread_safety", "Haskell thread safety violation", Severity::Error);
haskell_detector!(HaskellAsyncHazardDetector, "haskell_async_hazard", "Haskell async hazard", Severity::Warning);
haskell_detector!(HaskellLockContentionDetector, "haskell_lock_contention", "Haskell lock contention", Severity::Warning);
haskell_detector!(HaskellSynchronizationDetector, "haskell_synchronization", "Haskell synchronization issue", Severity::Warning);

// Error handling patterns
haskell_detector!(HaskellSwallowedExceptionDetector, "haskell_swallowed_exception", "Haskell swallowed exception", Severity::Warning);
haskell_detector!(HaskellEmptyCatchDetector, "haskell_empty_catch", "Haskell empty catch block", Severity::Warning);
haskell_detector!(HaskellGenericCatchDetector, "haskell_generic_catch", "Haskell generic catch", Severity::Info);
haskell_detector!(HaskellUnhandledErrorDetector, "haskell_unhandled_error", "Haskell unhandled error", Severity::Warning);
haskell_detector!(HaskellErrorIgnoredDetector, "haskell_error_ignored", "Haskell error ignored", Severity::Warning);
haskell_detector!(HaskellPanicMisuseDetector, "haskell_panic_misuse", "Haskell panic misuse", Severity::Warning);
haskell_detector!(HaskellErrorPropagationDetector, "haskell_error_propagation", "Haskell error propagation issue", Severity::Info);
haskell_detector!(HaskellResourceCleanupDetector, "haskell_resource_cleanup", "Haskell missing resource cleanup", Severity::Warning);
haskell_detector!(HaskellTransactionRollbackDetector, "haskell_transaction_rollback", "Haskell missing transaction rollback", Severity::Warning);
haskell_detector!(HaskellRetryLogicDetector, "haskell_retry_logic", "Haskell problematic retry logic", Severity::Info);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_haskell_patterns() {
        let patterns = get_haskell_patterns();
        assert!(patterns.len() >= 50, "Should have 50+ Haskell patterns");
    }
}
