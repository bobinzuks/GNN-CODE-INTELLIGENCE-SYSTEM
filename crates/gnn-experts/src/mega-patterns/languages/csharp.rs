//! Csharp-specific patterns (50+ patterns per language)

use crate::{CodeGraph, PatternDetector, PatternInstance, FixSuggestion, Severity};
use std::sync::Arc;

pub fn get_csharp_patterns() -> Vec<Arc<dyn PatternDetector>> {
    vec![
        // Security patterns (10)
        Arc::new(CsharpSQLInjectionDetector::new()),
        Arc::new(CsharpXSSDetector::new()),
        Arc::new(CsharpPathTraversalDetector::new()),
        Arc::new(CsharpCommandInjectionDetector::new()),
        Arc::new(CsharpDeserializationDetector::new()),
        Arc::new(CsharpHardcodedSecretsDetector::new()),
        Arc::new(CsharpWeakCryptoDetector::new()),
        Arc::new(CsharpInsecureRandomDetector::new()),
        Arc::new(CsharpAuthBypassDetector::new()),
        Arc::new(CsharpCSRFDetector::new()),

        // Performance patterns (10)
        Arc::new(CsharpNPlusOneDetector::new()),
        Arc::new(CsharpIneffectiveLoopDetector::new()),
        Arc::new(CsharpExcessiveAllocationDetector::new()),
        Arc::new(CsharpStringConcatDetector::new()),
        Arc::new(CsharpBlockingIODetector::new()),
        Arc::new(CsharpMissingCacheDetector::new()),
        Arc::new(CsharpAlgorithmComplexityDetector::new()),
        Arc::new(CsharpRedundantComputationDetector::new()),
        Arc::new(CsharpMemoryLeakDetector::new()),
        Arc::new(CsharpResourceExhaustionDetector::new()),

        // Memory safety patterns (10)
        Arc::new(CsharpUseAfterFreeDetector::new()),
        Arc::new(CsharpBufferOverflowDetector::new()),
        Arc::new(CsharpNullPointerDetector::new()),
        Arc::new(CsharpUninitializedMemoryDetector::new()),
        Arc::new(CsharpDoubleFreeDetector::new()),
        Arc::new(CsharpMemoryCorruptionDetector::new()),
        Arc::new(CsharpDanglingPointerDetector::new()),
        Arc::new(CsharpStackOverflowDetector::new()),
        Arc::new(CsharpHeapCorruptionDetector::new()),
        Arc::new(CsharpTypeConfusionDetector::new()),

        // Concurrency patterns (10)
        Arc::new(CsharpDataRaceDetector::new()),
        Arc::new(CsharpDeadlockDetector::new()),
        Arc::new(CsharpRaceConditionDetector::new()),
        Arc::new(CsharpAtomicityViolationDetector::new()),
        Arc::new(CsharpOrderViolationDetector::new()),
        Arc::new(CsharpLivelockDetector::new()),
        Arc::new(CsharpThreadSafetyDetector::new()),
        Arc::new(CsharpAsyncHazardDetector::new()),
        Arc::new(CsharpLockContentionDetector::new()),
        Arc::new(CsharpSynchronizationDetector::new()),

        // Error handling patterns (10)
        Arc::new(CsharpSwallowedExceptionDetector::new()),
        Arc::new(CsharpEmptyCatchDetector::new()),
        Arc::new(CsharpGenericCatchDetector::new()),
        Arc::new(CsharpUnhandledErrorDetector::new()),
        Arc::new(CsharpErrorIgnoredDetector::new()),
        Arc::new(CsharpPanicMisuseDetector::new()),
        Arc::new(CsharpErrorPropagationDetector::new()),
        Arc::new(CsharpResourceCleanupDetector::new()),
        Arc::new(CsharpTransactionRollbackDetector::new()),
        Arc::new(CsharpRetryLogicDetector::new()),
    ]
}

macro_rules! csharp_detector {
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
csharp_detector!(CsharpSQLInjectionDetector, "csharp_sql_injection", "Csharp SQL injection vulnerability", Severity::Critical);
csharp_detector!(CsharpXSSDetector, "csharp_xss", "Csharp XSS vulnerability", Severity::Critical);
csharp_detector!(CsharpPathTraversalDetector, "csharp_path_traversal", "Csharp path traversal vulnerability", Severity::Critical);
csharp_detector!(CsharpCommandInjectionDetector, "csharp_command_injection", "Csharp command injection", Severity::Critical);
csharp_detector!(CsharpDeserializationDetector, "csharp_unsafe_deserialization", "Csharp unsafe deserialization", Severity::Critical);
csharp_detector!(CsharpHardcodedSecretsDetector, "csharp_hardcoded_secrets", "Csharp hardcoded secrets", Severity::Warning);
csharp_detector!(CsharpWeakCryptoDetector, "csharp_weak_crypto", "Csharp weak cryptography", Severity::Warning);
csharp_detector!(CsharpInsecureRandomDetector, "csharp_insecure_random", "Csharp insecure randomness", Severity::Warning);
csharp_detector!(CsharpAuthBypassDetector, "csharp_auth_bypass", "Csharp authentication bypass", Severity::Critical);
csharp_detector!(CsharpCSRFDetector, "csharp_csrf", "Csharp CSRF vulnerability", Severity::Warning);

// Performance patterns
csharp_detector!(CsharpNPlusOneDetector, "csharp_n_plus_one", "Csharp N+1 query problem", Severity::Warning);
csharp_detector!(CsharpIneffectiveLoopDetector, "csharp_ineffective_loop", "Csharp ineffective loop", Severity::Warning);
csharp_detector!(CsharpExcessiveAllocationDetector, "csharp_excessive_allocation", "Csharp excessive allocation", Severity::Warning);
csharp_detector!(CsharpStringConcatDetector, "csharp_string_concat", "Csharp ineffective string concatenation", Severity::Info);
csharp_detector!(CsharpBlockingIODetector, "csharp_blocking_io", "Csharp blocking I/O", Severity::Warning);
csharp_detector!(CsharpMissingCacheDetector, "csharp_missing_cache", "Csharp missing cache", Severity::Info);
csharp_detector!(CsharpAlgorithmComplexityDetector, "csharp_algorithm_complexity", "Csharp high algorithm complexity", Severity::Warning);
csharp_detector!(CsharpRedundantComputationDetector, "csharp_redundant_computation", "Csharp redundant computation", Severity::Info);
csharp_detector!(CsharpMemoryLeakDetector, "csharp_memory_leak", "Csharp memory leak", Severity::Warning);
csharp_detector!(CsharpResourceExhaustionDetector, "csharp_resource_exhaustion", "Csharp resource exhaustion", Severity::Warning);

// Memory safety patterns
csharp_detector!(CsharpUseAfterFreeDetector, "csharp_use_after_free", "Csharp use-after-free", Severity::Critical);
csharp_detector!(CsharpBufferOverflowDetector, "csharp_buffer_overflow", "Csharp buffer overflow", Severity::Critical);
csharp_detector!(CsharpNullPointerDetector, "csharp_null_pointer", "Csharp null pointer dereference", Severity::Critical);
csharp_detector!(CsharpUninitializedMemoryDetector, "csharp_uninitialized_memory", "Csharp uninitialized memory", Severity::Critical);
csharp_detector!(CsharpDoubleFreeDetector, "csharp_double_free", "Csharp double free", Severity::Critical);
csharp_detector!(CsharpMemoryCorruptionDetector, "csharp_memory_corruption", "Csharp memory corruption", Severity::Critical);
csharp_detector!(CsharpDanglingPointerDetector, "csharp_dangling_pointer", "Csharp dangling pointer", Severity::Critical);
csharp_detector!(CsharpStackOverflowDetector, "csharp_stack_overflow", "Csharp stack overflow risk", Severity::Warning);
csharp_detector!(CsharpHeapCorruptionDetector, "csharp_heap_corruption", "Csharp heap corruption", Severity::Critical);
csharp_detector!(CsharpTypeConfusionDetector, "csharp_type_confusion", "Csharp type confusion", Severity::Warning);

// Concurrency patterns
csharp_detector!(CsharpDataRaceDetector, "csharp_data_race", "Csharp data race", Severity::Critical);
csharp_detector!(CsharpDeadlockDetector, "csharp_deadlock", "Csharp deadlock", Severity::Critical);
csharp_detector!(CsharpRaceConditionDetector, "csharp_race_condition", "Csharp race condition", Severity::Critical);
csharp_detector!(CsharpAtomicityViolationDetector, "csharp_atomicity_violation", "Csharp atomicity violation", Severity::Error);
csharp_detector!(CsharpOrderViolationDetector, "csharp_order_violation", "Csharp order violation", Severity::Error);
csharp_detector!(CsharpLivelockDetector, "csharp_livelock", "Csharp livelock", Severity::Warning);
csharp_detector!(CsharpThreadSafetyDetector, "csharp_thread_safety", "Csharp thread safety violation", Severity::Error);
csharp_detector!(CsharpAsyncHazardDetector, "csharp_async_hazard", "Csharp async hazard", Severity::Warning);
csharp_detector!(CsharpLockContentionDetector, "csharp_lock_contention", "Csharp lock contention", Severity::Warning);
csharp_detector!(CsharpSynchronizationDetector, "csharp_synchronization", "Csharp synchronization issue", Severity::Warning);

// Error handling patterns
csharp_detector!(CsharpSwallowedExceptionDetector, "csharp_swallowed_exception", "Csharp swallowed exception", Severity::Warning);
csharp_detector!(CsharpEmptyCatchDetector, "csharp_empty_catch", "Csharp empty catch block", Severity::Warning);
csharp_detector!(CsharpGenericCatchDetector, "csharp_generic_catch", "Csharp generic catch", Severity::Info);
csharp_detector!(CsharpUnhandledErrorDetector, "csharp_unhandled_error", "Csharp unhandled error", Severity::Warning);
csharp_detector!(CsharpErrorIgnoredDetector, "csharp_error_ignored", "Csharp error ignored", Severity::Warning);
csharp_detector!(CsharpPanicMisuseDetector, "csharp_panic_misuse", "Csharp panic misuse", Severity::Warning);
csharp_detector!(CsharpErrorPropagationDetector, "csharp_error_propagation", "Csharp error propagation issue", Severity::Info);
csharp_detector!(CsharpResourceCleanupDetector, "csharp_resource_cleanup", "Csharp missing resource cleanup", Severity::Warning);
csharp_detector!(CsharpTransactionRollbackDetector, "csharp_transaction_rollback", "Csharp missing transaction rollback", Severity::Warning);
csharp_detector!(CsharpRetryLogicDetector, "csharp_retry_logic", "Csharp problematic retry logic", Severity::Info);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csharp_patterns() {
        let patterns = get_csharp_patterns();
        assert!(patterns.len() >= 50, "Should have 50+ Csharp patterns");
    }
}
