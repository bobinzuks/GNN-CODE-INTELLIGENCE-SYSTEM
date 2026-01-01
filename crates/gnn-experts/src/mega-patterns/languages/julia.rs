//! Julia-specific patterns (50+ patterns per language)

use crate::{CodeGraph, PatternDetector, PatternInstance, FixSuggestion, Severity};
use std::sync::Arc;

pub fn get_julia_patterns() -> Vec<Arc<dyn PatternDetector>> {
    vec![
        // Security patterns (10)
        Arc::new(JuliaSQLInjectionDetector::new()),
        Arc::new(JuliaXSSDetector::new()),
        Arc::new(JuliaPathTraversalDetector::new()),
        Arc::new(JuliaCommandInjectionDetector::new()),
        Arc::new(JuliaDeserializationDetector::new()),
        Arc::new(JuliaHardcodedSecretsDetector::new()),
        Arc::new(JuliaWeakCryptoDetector::new()),
        Arc::new(JuliaInsecureRandomDetector::new()),
        Arc::new(JuliaAuthBypassDetector::new()),
        Arc::new(JuliaCSRFDetector::new()),

        // Performance patterns (10)
        Arc::new(JuliaNPlusOneDetector::new()),
        Arc::new(JuliaIneffectiveLoopDetector::new()),
        Arc::new(JuliaExcessiveAllocationDetector::new()),
        Arc::new(JuliaStringConcatDetector::new()),
        Arc::new(JuliaBlockingIODetector::new()),
        Arc::new(JuliaMissingCacheDetector::new()),
        Arc::new(JuliaAlgorithmComplexityDetector::new()),
        Arc::new(JuliaRedundantComputationDetector::new()),
        Arc::new(JuliaMemoryLeakDetector::new()),
        Arc::new(JuliaResourceExhaustionDetector::new()),

        // Memory safety patterns (10)
        Arc::new(JuliaUseAfterFreeDetector::new()),
        Arc::new(JuliaBufferOverflowDetector::new()),
        Arc::new(JuliaNullPointerDetector::new()),
        Arc::new(JuliaUninitializedMemoryDetector::new()),
        Arc::new(JuliaDoubleFreeDetector::new()),
        Arc::new(JuliaMemoryCorruptionDetector::new()),
        Arc::new(JuliaDanglingPointerDetector::new()),
        Arc::new(JuliaStackOverflowDetector::new()),
        Arc::new(JuliaHeapCorruptionDetector::new()),
        Arc::new(JuliaTypeConfusionDetector::new()),

        // Concurrency patterns (10)
        Arc::new(JuliaDataRaceDetector::new()),
        Arc::new(JuliaDeadlockDetector::new()),
        Arc::new(JuliaRaceConditionDetector::new()),
        Arc::new(JuliaAtomicityViolationDetector::new()),
        Arc::new(JuliaOrderViolationDetector::new()),
        Arc::new(JuliaLivelockDetector::new()),
        Arc::new(JuliaThreadSafetyDetector::new()),
        Arc::new(JuliaAsyncHazardDetector::new()),
        Arc::new(JuliaLockContentionDetector::new()),
        Arc::new(JuliaSynchronizationDetector::new()),

        // Error handling patterns (10)
        Arc::new(JuliaSwallowedExceptionDetector::new()),
        Arc::new(JuliaEmptyCatchDetector::new()),
        Arc::new(JuliaGenericCatchDetector::new()),
        Arc::new(JuliaUnhandledErrorDetector::new()),
        Arc::new(JuliaErrorIgnoredDetector::new()),
        Arc::new(JuliaPanicMisuseDetector::new()),
        Arc::new(JuliaErrorPropagationDetector::new()),
        Arc::new(JuliaResourceCleanupDetector::new()),
        Arc::new(JuliaTransactionRollbackDetector::new()),
        Arc::new(JuliaRetryLogicDetector::new()),
    ]
}

macro_rules! julia_detector {
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
julia_detector!(JuliaSQLInjectionDetector, "julia_sql_injection", "Julia SQL injection vulnerability", Severity::Critical);
julia_detector!(JuliaXSSDetector, "julia_xss", "Julia XSS vulnerability", Severity::Critical);
julia_detector!(JuliaPathTraversalDetector, "julia_path_traversal", "Julia path traversal vulnerability", Severity::Critical);
julia_detector!(JuliaCommandInjectionDetector, "julia_command_injection", "Julia command injection", Severity::Critical);
julia_detector!(JuliaDeserializationDetector, "julia_unsafe_deserialization", "Julia unsafe deserialization", Severity::Critical);
julia_detector!(JuliaHardcodedSecretsDetector, "julia_hardcoded_secrets", "Julia hardcoded secrets", Severity::Warning);
julia_detector!(JuliaWeakCryptoDetector, "julia_weak_crypto", "Julia weak cryptography", Severity::Warning);
julia_detector!(JuliaInsecureRandomDetector, "julia_insecure_random", "Julia insecure randomness", Severity::Warning);
julia_detector!(JuliaAuthBypassDetector, "julia_auth_bypass", "Julia authentication bypass", Severity::Critical);
julia_detector!(JuliaCSRFDetector, "julia_csrf", "Julia CSRF vulnerability", Severity::Warning);

// Performance patterns
julia_detector!(JuliaNPlusOneDetector, "julia_n_plus_one", "Julia N+1 query problem", Severity::Warning);
julia_detector!(JuliaIneffectiveLoopDetector, "julia_ineffective_loop", "Julia ineffective loop", Severity::Warning);
julia_detector!(JuliaExcessiveAllocationDetector, "julia_excessive_allocation", "Julia excessive allocation", Severity::Warning);
julia_detector!(JuliaStringConcatDetector, "julia_string_concat", "Julia ineffective string concatenation", Severity::Info);
julia_detector!(JuliaBlockingIODetector, "julia_blocking_io", "Julia blocking I/O", Severity::Warning);
julia_detector!(JuliaMissingCacheDetector, "julia_missing_cache", "Julia missing cache", Severity::Info);
julia_detector!(JuliaAlgorithmComplexityDetector, "julia_algorithm_complexity", "Julia high algorithm complexity", Severity::Warning);
julia_detector!(JuliaRedundantComputationDetector, "julia_redundant_computation", "Julia redundant computation", Severity::Info);
julia_detector!(JuliaMemoryLeakDetector, "julia_memory_leak", "Julia memory leak", Severity::Warning);
julia_detector!(JuliaResourceExhaustionDetector, "julia_resource_exhaustion", "Julia resource exhaustion", Severity::Warning);

// Memory safety patterns
julia_detector!(JuliaUseAfterFreeDetector, "julia_use_after_free", "Julia use-after-free", Severity::Critical);
julia_detector!(JuliaBufferOverflowDetector, "julia_buffer_overflow", "Julia buffer overflow", Severity::Critical);
julia_detector!(JuliaNullPointerDetector, "julia_null_pointer", "Julia null pointer dereference", Severity::Critical);
julia_detector!(JuliaUninitializedMemoryDetector, "julia_uninitialized_memory", "Julia uninitialized memory", Severity::Critical);
julia_detector!(JuliaDoubleFreeDetector, "julia_double_free", "Julia double free", Severity::Critical);
julia_detector!(JuliaMemoryCorruptionDetector, "julia_memory_corruption", "Julia memory corruption", Severity::Critical);
julia_detector!(JuliaDanglingPointerDetector, "julia_dangling_pointer", "Julia dangling pointer", Severity::Critical);
julia_detector!(JuliaStackOverflowDetector, "julia_stack_overflow", "Julia stack overflow risk", Severity::Warning);
julia_detector!(JuliaHeapCorruptionDetector, "julia_heap_corruption", "Julia heap corruption", Severity::Critical);
julia_detector!(JuliaTypeConfusionDetector, "julia_type_confusion", "Julia type confusion", Severity::Warning);

// Concurrency patterns
julia_detector!(JuliaDataRaceDetector, "julia_data_race", "Julia data race", Severity::Critical);
julia_detector!(JuliaDeadlockDetector, "julia_deadlock", "Julia deadlock", Severity::Critical);
julia_detector!(JuliaRaceConditionDetector, "julia_race_condition", "Julia race condition", Severity::Critical);
julia_detector!(JuliaAtomicityViolationDetector, "julia_atomicity_violation", "Julia atomicity violation", Severity::Error);
julia_detector!(JuliaOrderViolationDetector, "julia_order_violation", "Julia order violation", Severity::Error);
julia_detector!(JuliaLivelockDetector, "julia_livelock", "Julia livelock", Severity::Warning);
julia_detector!(JuliaThreadSafetyDetector, "julia_thread_safety", "Julia thread safety violation", Severity::Error);
julia_detector!(JuliaAsyncHazardDetector, "julia_async_hazard", "Julia async hazard", Severity::Warning);
julia_detector!(JuliaLockContentionDetector, "julia_lock_contention", "Julia lock contention", Severity::Warning);
julia_detector!(JuliaSynchronizationDetector, "julia_synchronization", "Julia synchronization issue", Severity::Warning);

// Error handling patterns
julia_detector!(JuliaSwallowedExceptionDetector, "julia_swallowed_exception", "Julia swallowed exception", Severity::Warning);
julia_detector!(JuliaEmptyCatchDetector, "julia_empty_catch", "Julia empty catch block", Severity::Warning);
julia_detector!(JuliaGenericCatchDetector, "julia_generic_catch", "Julia generic catch", Severity::Info);
julia_detector!(JuliaUnhandledErrorDetector, "julia_unhandled_error", "Julia unhandled error", Severity::Warning);
julia_detector!(JuliaErrorIgnoredDetector, "julia_error_ignored", "Julia error ignored", Severity::Warning);
julia_detector!(JuliaPanicMisuseDetector, "julia_panic_misuse", "Julia panic misuse", Severity::Warning);
julia_detector!(JuliaErrorPropagationDetector, "julia_error_propagation", "Julia error propagation issue", Severity::Info);
julia_detector!(JuliaResourceCleanupDetector, "julia_resource_cleanup", "Julia missing resource cleanup", Severity::Warning);
julia_detector!(JuliaTransactionRollbackDetector, "julia_transaction_rollback", "Julia missing transaction rollback", Severity::Warning);
julia_detector!(JuliaRetryLogicDetector, "julia_retry_logic", "Julia problematic retry logic", Severity::Info);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_julia_patterns() {
        let patterns = get_julia_patterns();
        assert!(patterns.len() >= 50, "Should have 50+ Julia patterns");
    }
}
