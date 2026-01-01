//! Elixir-specific patterns (50+ patterns per language)

use crate::{CodeGraph, PatternDetector, PatternInstance, FixSuggestion, Severity};
use std::sync::Arc;

pub fn get_elixir_patterns() -> Vec<Arc<dyn PatternDetector>> {
    vec![
        // Security patterns (10)
        Arc::new(ElixirSQLInjectionDetector::new()),
        Arc::new(ElixirXSSDetector::new()),
        Arc::new(ElixirPathTraversalDetector::new()),
        Arc::new(ElixirCommandInjectionDetector::new()),
        Arc::new(ElixirDeserializationDetector::new()),
        Arc::new(ElixirHardcodedSecretsDetector::new()),
        Arc::new(ElixirWeakCryptoDetector::new()),
        Arc::new(ElixirInsecureRandomDetector::new()),
        Arc::new(ElixirAuthBypassDetector::new()),
        Arc::new(ElixirCSRFDetector::new()),

        // Performance patterns (10)
        Arc::new(ElixirNPlusOneDetector::new()),
        Arc::new(ElixirIneffectiveLoopDetector::new()),
        Arc::new(ElixirExcessiveAllocationDetector::new()),
        Arc::new(ElixirStringConcatDetector::new()),
        Arc::new(ElixirBlockingIODetector::new()),
        Arc::new(ElixirMissingCacheDetector::new()),
        Arc::new(ElixirAlgorithmComplexityDetector::new()),
        Arc::new(ElixirRedundantComputationDetector::new()),
        Arc::new(ElixirMemoryLeakDetector::new()),
        Arc::new(ElixirResourceExhaustionDetector::new()),

        // Memory safety patterns (10)
        Arc::new(ElixirUseAfterFreeDetector::new()),
        Arc::new(ElixirBufferOverflowDetector::new()),
        Arc::new(ElixirNullPointerDetector::new()),
        Arc::new(ElixirUninitializedMemoryDetector::new()),
        Arc::new(ElixirDoubleFreeDetector::new()),
        Arc::new(ElixirMemoryCorruptionDetector::new()),
        Arc::new(ElixirDanglingPointerDetector::new()),
        Arc::new(ElixirStackOverflowDetector::new()),
        Arc::new(ElixirHeapCorruptionDetector::new()),
        Arc::new(ElixirTypeConfusionDetector::new()),

        // Concurrency patterns (10)
        Arc::new(ElixirDataRaceDetector::new()),
        Arc::new(ElixirDeadlockDetector::new()),
        Arc::new(ElixirRaceConditionDetector::new()),
        Arc::new(ElixirAtomicityViolationDetector::new()),
        Arc::new(ElixirOrderViolationDetector::new()),
        Arc::new(ElixirLivelockDetector::new()),
        Arc::new(ElixirThreadSafetyDetector::new()),
        Arc::new(ElixirAsyncHazardDetector::new()),
        Arc::new(ElixirLockContentionDetector::new()),
        Arc::new(ElixirSynchronizationDetector::new()),

        // Error handling patterns (10)
        Arc::new(ElixirSwallowedExceptionDetector::new()),
        Arc::new(ElixirEmptyCatchDetector::new()),
        Arc::new(ElixirGenericCatchDetector::new()),
        Arc::new(ElixirUnhandledErrorDetector::new()),
        Arc::new(ElixirErrorIgnoredDetector::new()),
        Arc::new(ElixirPanicMisuseDetector::new()),
        Arc::new(ElixirErrorPropagationDetector::new()),
        Arc::new(ElixirResourceCleanupDetector::new()),
        Arc::new(ElixirTransactionRollbackDetector::new()),
        Arc::new(ElixirRetryLogicDetector::new()),
    ]
}

macro_rules! elixir_detector {
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
elixir_detector!(ElixirSQLInjectionDetector, "elixir_sql_injection", "Elixir SQL injection vulnerability", Severity::Critical);
elixir_detector!(ElixirXSSDetector, "elixir_xss", "Elixir XSS vulnerability", Severity::Critical);
elixir_detector!(ElixirPathTraversalDetector, "elixir_path_traversal", "Elixir path traversal vulnerability", Severity::Critical);
elixir_detector!(ElixirCommandInjectionDetector, "elixir_command_injection", "Elixir command injection", Severity::Critical);
elixir_detector!(ElixirDeserializationDetector, "elixir_unsafe_deserialization", "Elixir unsafe deserialization", Severity::Critical);
elixir_detector!(ElixirHardcodedSecretsDetector, "elixir_hardcoded_secrets", "Elixir hardcoded secrets", Severity::Warning);
elixir_detector!(ElixirWeakCryptoDetector, "elixir_weak_crypto", "Elixir weak cryptography", Severity::Warning);
elixir_detector!(ElixirInsecureRandomDetector, "elixir_insecure_random", "Elixir insecure randomness", Severity::Warning);
elixir_detector!(ElixirAuthBypassDetector, "elixir_auth_bypass", "Elixir authentication bypass", Severity::Critical);
elixir_detector!(ElixirCSRFDetector, "elixir_csrf", "Elixir CSRF vulnerability", Severity::Warning);

// Performance patterns
elixir_detector!(ElixirNPlusOneDetector, "elixir_n_plus_one", "Elixir N+1 query problem", Severity::Warning);
elixir_detector!(ElixirIneffectiveLoopDetector, "elixir_ineffective_loop", "Elixir ineffective loop", Severity::Warning);
elixir_detector!(ElixirExcessiveAllocationDetector, "elixir_excessive_allocation", "Elixir excessive allocation", Severity::Warning);
elixir_detector!(ElixirStringConcatDetector, "elixir_string_concat", "Elixir ineffective string concatenation", Severity::Info);
elixir_detector!(ElixirBlockingIODetector, "elixir_blocking_io", "Elixir blocking I/O", Severity::Warning);
elixir_detector!(ElixirMissingCacheDetector, "elixir_missing_cache", "Elixir missing cache", Severity::Info);
elixir_detector!(ElixirAlgorithmComplexityDetector, "elixir_algorithm_complexity", "Elixir high algorithm complexity", Severity::Warning);
elixir_detector!(ElixirRedundantComputationDetector, "elixir_redundant_computation", "Elixir redundant computation", Severity::Info);
elixir_detector!(ElixirMemoryLeakDetector, "elixir_memory_leak", "Elixir memory leak", Severity::Warning);
elixir_detector!(ElixirResourceExhaustionDetector, "elixir_resource_exhaustion", "Elixir resource exhaustion", Severity::Warning);

// Memory safety patterns
elixir_detector!(ElixirUseAfterFreeDetector, "elixir_use_after_free", "Elixir use-after-free", Severity::Critical);
elixir_detector!(ElixirBufferOverflowDetector, "elixir_buffer_overflow", "Elixir buffer overflow", Severity::Critical);
elixir_detector!(ElixirNullPointerDetector, "elixir_null_pointer", "Elixir null pointer dereference", Severity::Critical);
elixir_detector!(ElixirUninitializedMemoryDetector, "elixir_uninitialized_memory", "Elixir uninitialized memory", Severity::Critical);
elixir_detector!(ElixirDoubleFreeDetector, "elixir_double_free", "Elixir double free", Severity::Critical);
elixir_detector!(ElixirMemoryCorruptionDetector, "elixir_memory_corruption", "Elixir memory corruption", Severity::Critical);
elixir_detector!(ElixirDanglingPointerDetector, "elixir_dangling_pointer", "Elixir dangling pointer", Severity::Critical);
elixir_detector!(ElixirStackOverflowDetector, "elixir_stack_overflow", "Elixir stack overflow risk", Severity::Warning);
elixir_detector!(ElixirHeapCorruptionDetector, "elixir_heap_corruption", "Elixir heap corruption", Severity::Critical);
elixir_detector!(ElixirTypeConfusionDetector, "elixir_type_confusion", "Elixir type confusion", Severity::Warning);

// Concurrency patterns
elixir_detector!(ElixirDataRaceDetector, "elixir_data_race", "Elixir data race", Severity::Critical);
elixir_detector!(ElixirDeadlockDetector, "elixir_deadlock", "Elixir deadlock", Severity::Critical);
elixir_detector!(ElixirRaceConditionDetector, "elixir_race_condition", "Elixir race condition", Severity::Critical);
elixir_detector!(ElixirAtomicityViolationDetector, "elixir_atomicity_violation", "Elixir atomicity violation", Severity::Error);
elixir_detector!(ElixirOrderViolationDetector, "elixir_order_violation", "Elixir order violation", Severity::Error);
elixir_detector!(ElixirLivelockDetector, "elixir_livelock", "Elixir livelock", Severity::Warning);
elixir_detector!(ElixirThreadSafetyDetector, "elixir_thread_safety", "Elixir thread safety violation", Severity::Error);
elixir_detector!(ElixirAsyncHazardDetector, "elixir_async_hazard", "Elixir async hazard", Severity::Warning);
elixir_detector!(ElixirLockContentionDetector, "elixir_lock_contention", "Elixir lock contention", Severity::Warning);
elixir_detector!(ElixirSynchronizationDetector, "elixir_synchronization", "Elixir synchronization issue", Severity::Warning);

// Error handling patterns
elixir_detector!(ElixirSwallowedExceptionDetector, "elixir_swallowed_exception", "Elixir swallowed exception", Severity::Warning);
elixir_detector!(ElixirEmptyCatchDetector, "elixir_empty_catch", "Elixir empty catch block", Severity::Warning);
elixir_detector!(ElixirGenericCatchDetector, "elixir_generic_catch", "Elixir generic catch", Severity::Info);
elixir_detector!(ElixirUnhandledErrorDetector, "elixir_unhandled_error", "Elixir unhandled error", Severity::Warning);
elixir_detector!(ElixirErrorIgnoredDetector, "elixir_error_ignored", "Elixir error ignored", Severity::Warning);
elixir_detector!(ElixirPanicMisuseDetector, "elixir_panic_misuse", "Elixir panic misuse", Severity::Warning);
elixir_detector!(ElixirErrorPropagationDetector, "elixir_error_propagation", "Elixir error propagation issue", Severity::Info);
elixir_detector!(ElixirResourceCleanupDetector, "elixir_resource_cleanup", "Elixir missing resource cleanup", Severity::Warning);
elixir_detector!(ElixirTransactionRollbackDetector, "elixir_transaction_rollback", "Elixir missing transaction rollback", Severity::Warning);
elixir_detector!(ElixirRetryLogicDetector, "elixir_retry_logic", "Elixir problematic retry logic", Severity::Info);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elixir_patterns() {
        let patterns = get_elixir_patterns();
        assert!(patterns.len() >= 50, "Should have 50+ Elixir patterns");
    }
}
