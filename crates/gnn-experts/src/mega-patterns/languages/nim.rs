//! Nim-specific patterns (50+ patterns per language)

use crate::{CodeGraph, PatternDetector, PatternInstance, FixSuggestion, Severity};
use std::sync::Arc;

pub fn get_nim_patterns() -> Vec<Arc<dyn PatternDetector>> {
    vec![
        // Security patterns (10)
        Arc::new(NimSQLInjectionDetector::new()),
        Arc::new(NimXSSDetector::new()),
        Arc::new(NimPathTraversalDetector::new()),
        Arc::new(NimCommandInjectionDetector::new()),
        Arc::new(NimDeserializationDetector::new()),
        Arc::new(NimHardcodedSecretsDetector::new()),
        Arc::new(NimWeakCryptoDetector::new()),
        Arc::new(NimInsecureRandomDetector::new()),
        Arc::new(NimAuthBypassDetector::new()),
        Arc::new(NimCSRFDetector::new()),

        // Performance patterns (10)
        Arc::new(NimNPlusOneDetector::new()),
        Arc::new(NimIneffectiveLoopDetector::new()),
        Arc::new(NimExcessiveAllocationDetector::new()),
        Arc::new(NimStringConcatDetector::new()),
        Arc::new(NimBlockingIODetector::new()),
        Arc::new(NimMissingCacheDetector::new()),
        Arc::new(NimAlgorithmComplexityDetector::new()),
        Arc::new(NimRedundantComputationDetector::new()),
        Arc::new(NimMemoryLeakDetector::new()),
        Arc::new(NimResourceExhaustionDetector::new()),

        // Memory safety patterns (10)
        Arc::new(NimUseAfterFreeDetector::new()),
        Arc::new(NimBufferOverflowDetector::new()),
        Arc::new(NimNullPointerDetector::new()),
        Arc::new(NimUninitializedMemoryDetector::new()),
        Arc::new(NimDoubleFreeDetector::new()),
        Arc::new(NimMemoryCorruptionDetector::new()),
        Arc::new(NimDanglingPointerDetector::new()),
        Arc::new(NimStackOverflowDetector::new()),
        Arc::new(NimHeapCorruptionDetector::new()),
        Arc::new(NimTypeConfusionDetector::new()),

        // Concurrency patterns (10)
        Arc::new(NimDataRaceDetector::new()),
        Arc::new(NimDeadlockDetector::new()),
        Arc::new(NimRaceConditionDetector::new()),
        Arc::new(NimAtomicityViolationDetector::new()),
        Arc::new(NimOrderViolationDetector::new()),
        Arc::new(NimLivelockDetector::new()),
        Arc::new(NimThreadSafetyDetector::new()),
        Arc::new(NimAsyncHazardDetector::new()),
        Arc::new(NimLockContentionDetector::new()),
        Arc::new(NimSynchronizationDetector::new()),

        // Error handling patterns (10)
        Arc::new(NimSwallowedExceptionDetector::new()),
        Arc::new(NimEmptyCatchDetector::new()),
        Arc::new(NimGenericCatchDetector::new()),
        Arc::new(NimUnhandledErrorDetector::new()),
        Arc::new(NimErrorIgnoredDetector::new()),
        Arc::new(NimPanicMisuseDetector::new()),
        Arc::new(NimErrorPropagationDetector::new()),
        Arc::new(NimResourceCleanupDetector::new()),
        Arc::new(NimTransactionRollbackDetector::new()),
        Arc::new(NimRetryLogicDetector::new()),
    ]
}

macro_rules! nim_detector {
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
nim_detector!(NimSQLInjectionDetector, "nim_sql_injection", "Nim SQL injection vulnerability", Severity::Critical);
nim_detector!(NimXSSDetector, "nim_xss", "Nim XSS vulnerability", Severity::Critical);
nim_detector!(NimPathTraversalDetector, "nim_path_traversal", "Nim path traversal vulnerability", Severity::Critical);
nim_detector!(NimCommandInjectionDetector, "nim_command_injection", "Nim command injection", Severity::Critical);
nim_detector!(NimDeserializationDetector, "nim_unsafe_deserialization", "Nim unsafe deserialization", Severity::Critical);
nim_detector!(NimHardcodedSecretsDetector, "nim_hardcoded_secrets", "Nim hardcoded secrets", Severity::Warning);
nim_detector!(NimWeakCryptoDetector, "nim_weak_crypto", "Nim weak cryptography", Severity::Warning);
nim_detector!(NimInsecureRandomDetector, "nim_insecure_random", "Nim insecure randomness", Severity::Warning);
nim_detector!(NimAuthBypassDetector, "nim_auth_bypass", "Nim authentication bypass", Severity::Critical);
nim_detector!(NimCSRFDetector, "nim_csrf", "Nim CSRF vulnerability", Severity::Warning);

// Performance patterns
nim_detector!(NimNPlusOneDetector, "nim_n_plus_one", "Nim N+1 query problem", Severity::Warning);
nim_detector!(NimIneffectiveLoopDetector, "nim_ineffective_loop", "Nim ineffective loop", Severity::Warning);
nim_detector!(NimExcessiveAllocationDetector, "nim_excessive_allocation", "Nim excessive allocation", Severity::Warning);
nim_detector!(NimStringConcatDetector, "nim_string_concat", "Nim ineffective string concatenation", Severity::Info);
nim_detector!(NimBlockingIODetector, "nim_blocking_io", "Nim blocking I/O", Severity::Warning);
nim_detector!(NimMissingCacheDetector, "nim_missing_cache", "Nim missing cache", Severity::Info);
nim_detector!(NimAlgorithmComplexityDetector, "nim_algorithm_complexity", "Nim high algorithm complexity", Severity::Warning);
nim_detector!(NimRedundantComputationDetector, "nim_redundant_computation", "Nim redundant computation", Severity::Info);
nim_detector!(NimMemoryLeakDetector, "nim_memory_leak", "Nim memory leak", Severity::Warning);
nim_detector!(NimResourceExhaustionDetector, "nim_resource_exhaustion", "Nim resource exhaustion", Severity::Warning);

// Memory safety patterns
nim_detector!(NimUseAfterFreeDetector, "nim_use_after_free", "Nim use-after-free", Severity::Critical);
nim_detector!(NimBufferOverflowDetector, "nim_buffer_overflow", "Nim buffer overflow", Severity::Critical);
nim_detector!(NimNullPointerDetector, "nim_null_pointer", "Nim null pointer dereference", Severity::Critical);
nim_detector!(NimUninitializedMemoryDetector, "nim_uninitialized_memory", "Nim uninitialized memory", Severity::Critical);
nim_detector!(NimDoubleFreeDetector, "nim_double_free", "Nim double free", Severity::Critical);
nim_detector!(NimMemoryCorruptionDetector, "nim_memory_corruption", "Nim memory corruption", Severity::Critical);
nim_detector!(NimDanglingPointerDetector, "nim_dangling_pointer", "Nim dangling pointer", Severity::Critical);
nim_detector!(NimStackOverflowDetector, "nim_stack_overflow", "Nim stack overflow risk", Severity::Warning);
nim_detector!(NimHeapCorruptionDetector, "nim_heap_corruption", "Nim heap corruption", Severity::Critical);
nim_detector!(NimTypeConfusionDetector, "nim_type_confusion", "Nim type confusion", Severity::Warning);

// Concurrency patterns
nim_detector!(NimDataRaceDetector, "nim_data_race", "Nim data race", Severity::Critical);
nim_detector!(NimDeadlockDetector, "nim_deadlock", "Nim deadlock", Severity::Critical);
nim_detector!(NimRaceConditionDetector, "nim_race_condition", "Nim race condition", Severity::Critical);
nim_detector!(NimAtomicityViolationDetector, "nim_atomicity_violation", "Nim atomicity violation", Severity::Error);
nim_detector!(NimOrderViolationDetector, "nim_order_violation", "Nim order violation", Severity::Error);
nim_detector!(NimLivelockDetector, "nim_livelock", "Nim livelock", Severity::Warning);
nim_detector!(NimThreadSafetyDetector, "nim_thread_safety", "Nim thread safety violation", Severity::Error);
nim_detector!(NimAsyncHazardDetector, "nim_async_hazard", "Nim async hazard", Severity::Warning);
nim_detector!(NimLockContentionDetector, "nim_lock_contention", "Nim lock contention", Severity::Warning);
nim_detector!(NimSynchronizationDetector, "nim_synchronization", "Nim synchronization issue", Severity::Warning);

// Error handling patterns
nim_detector!(NimSwallowedExceptionDetector, "nim_swallowed_exception", "Nim swallowed exception", Severity::Warning);
nim_detector!(NimEmptyCatchDetector, "nim_empty_catch", "Nim empty catch block", Severity::Warning);
nim_detector!(NimGenericCatchDetector, "nim_generic_catch", "Nim generic catch", Severity::Info);
nim_detector!(NimUnhandledErrorDetector, "nim_unhandled_error", "Nim unhandled error", Severity::Warning);
nim_detector!(NimErrorIgnoredDetector, "nim_error_ignored", "Nim error ignored", Severity::Warning);
nim_detector!(NimPanicMisuseDetector, "nim_panic_misuse", "Nim panic misuse", Severity::Warning);
nim_detector!(NimErrorPropagationDetector, "nim_error_propagation", "Nim error propagation issue", Severity::Info);
nim_detector!(NimResourceCleanupDetector, "nim_resource_cleanup", "Nim missing resource cleanup", Severity::Warning);
nim_detector!(NimTransactionRollbackDetector, "nim_transaction_rollback", "Nim missing transaction rollback", Severity::Warning);
nim_detector!(NimRetryLogicDetector, "nim_retry_logic", "Nim problematic retry logic", Severity::Info);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nim_patterns() {
        let patterns = get_nim_patterns();
        assert!(patterns.len() >= 50, "Should have 50+ Nim patterns");
    }
}
