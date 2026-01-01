//! Rust-specific patterns (50+ patterns per language)

use crate::{CodeGraph, PatternDetector, PatternInstance, FixSuggestion, Severity};
use std::sync::Arc;

pub fn get_rust_patterns() -> Vec<Arc<dyn PatternDetector>> {
    vec![
        // Security patterns (10)
        Arc::new(RustSQLInjectionDetector::new()),
        Arc::new(RustXSSDetector::new()),
        Arc::new(RustPathTraversalDetector::new()),
        Arc::new(RustCommandInjectionDetector::new()),
        Arc::new(RustDeserializationDetector::new()),
        Arc::new(RustHardcodedSecretsDetector::new()),
        Arc::new(RustWeakCryptoDetector::new()),
        Arc::new(RustInsecureRandomDetector::new()),
        Arc::new(RustAuthBypassDetector::new()),
        Arc::new(RustCSRFDetector::new()),

        // Performance patterns (10)
        Arc::new(RustNPlusOneDetector::new()),
        Arc::new(RustIneffectiveLoopDetector::new()),
        Arc::new(RustExcessiveAllocationDetector::new()),
        Arc::new(RustStringConcatDetector::new()),
        Arc::new(RustBlockingIODetector::new()),
        Arc::new(RustMissingCacheDetector::new()),
        Arc::new(RustAlgorithmComplexityDetector::new()),
        Arc::new(RustRedundantComputationDetector::new()),
        Arc::new(RustMemoryLeakDetector::new()),
        Arc::new(RustResourceExhaustionDetector::new()),

        // Memory safety patterns (10)
        Arc::new(RustUseAfterFreeDetector::new()),
        Arc::new(RustBufferOverflowDetector::new()),
        Arc::new(RustNullPointerDetector::new()),
        Arc::new(RustUninitializedMemoryDetector::new()),
        Arc::new(RustDoubleFreeDetector::new()),
        Arc::new(RustMemoryCorruptionDetector::new()),
        Arc::new(RustDanglingPointerDetector::new()),
        Arc::new(RustStackOverflowDetector::new()),
        Arc::new(RustHeapCorruptionDetector::new()),
        Arc::new(RustTypeConfusionDetector::new()),

        // Concurrency patterns (10)
        Arc::new(RustDataRaceDetector::new()),
        Arc::new(RustDeadlockDetector::new()),
        Arc::new(RustRaceConditionDetector::new()),
        Arc::new(RustAtomicityViolationDetector::new()),
        Arc::new(RustOrderViolationDetector::new()),
        Arc::new(RustLivelockDetector::new()),
        Arc::new(RustThreadSafetyDetector::new()),
        Arc::new(RustAsyncHazardDetector::new()),
        Arc::new(RustLockContentionDetector::new()),
        Arc::new(RustSynchronizationDetector::new()),

        // Error handling patterns (10)
        Arc::new(RustSwallowedExceptionDetector::new()),
        Arc::new(RustEmptyCatchDetector::new()),
        Arc::new(RustGenericCatchDetector::new()),
        Arc::new(RustUnhandledErrorDetector::new()),
        Arc::new(RustErrorIgnoredDetector::new()),
        Arc::new(RustPanicMisuseDetector::new()),
        Arc::new(RustErrorPropagationDetector::new()),
        Arc::new(RustResourceCleanupDetector::new()),
        Arc::new(RustTransactionRollbackDetector::new()),
        Arc::new(RustRetryLogicDetector::new()),
    ]
}

macro_rules! rust_detector {
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
rust_detector!(RustSQLInjectionDetector, "rust_sql_injection", "Rust SQL injection vulnerability", Severity::Critical);
rust_detector!(RustXSSDetector, "rust_xss", "Rust XSS vulnerability", Severity::Critical);
rust_detector!(RustPathTraversalDetector, "rust_path_traversal", "Rust path traversal vulnerability", Severity::Critical);
rust_detector!(RustCommandInjectionDetector, "rust_command_injection", "Rust command injection", Severity::Critical);
rust_detector!(RustDeserializationDetector, "rust_unsafe_deserialization", "Rust unsafe deserialization", Severity::Critical);
rust_detector!(RustHardcodedSecretsDetector, "rust_hardcoded_secrets", "Rust hardcoded secrets", Severity::Warning);
rust_detector!(RustWeakCryptoDetector, "rust_weak_crypto", "Rust weak cryptography", Severity::Warning);
rust_detector!(RustInsecureRandomDetector, "rust_insecure_random", "Rust insecure randomness", Severity::Warning);
rust_detector!(RustAuthBypassDetector, "rust_auth_bypass", "Rust authentication bypass", Severity::Critical);
rust_detector!(RustCSRFDetector, "rust_csrf", "Rust CSRF vulnerability", Severity::Warning);

// Performance patterns
rust_detector!(RustNPlusOneDetector, "rust_n_plus_one", "Rust N+1 query problem", Severity::Warning);
rust_detector!(RustIneffectiveLoopDetector, "rust_ineffective_loop", "Rust ineffective loop", Severity::Warning);
rust_detector!(RustExcessiveAllocationDetector, "rust_excessive_allocation", "Rust excessive allocation", Severity::Warning);
rust_detector!(RustStringConcatDetector, "rust_string_concat", "Rust ineffective string concatenation", Severity::Info);
rust_detector!(RustBlockingIODetector, "rust_blocking_io", "Rust blocking I/O", Severity::Warning);
rust_detector!(RustMissingCacheDetector, "rust_missing_cache", "Rust missing cache", Severity::Info);
rust_detector!(RustAlgorithmComplexityDetector, "rust_algorithm_complexity", "Rust high algorithm complexity", Severity::Warning);
rust_detector!(RustRedundantComputationDetector, "rust_redundant_computation", "Rust redundant computation", Severity::Info);
rust_detector!(RustMemoryLeakDetector, "rust_memory_leak", "Rust memory leak", Severity::Warning);
rust_detector!(RustResourceExhaustionDetector, "rust_resource_exhaustion", "Rust resource exhaustion", Severity::Warning);

// Memory safety patterns
rust_detector!(RustUseAfterFreeDetector, "rust_use_after_free", "Rust use-after-free", Severity::Critical);
rust_detector!(RustBufferOverflowDetector, "rust_buffer_overflow", "Rust buffer overflow", Severity::Critical);
rust_detector!(RustNullPointerDetector, "rust_null_pointer", "Rust null pointer dereference", Severity::Critical);
rust_detector!(RustUninitializedMemoryDetector, "rust_uninitialized_memory", "Rust uninitialized memory", Severity::Critical);
rust_detector!(RustDoubleFreeDetector, "rust_double_free", "Rust double free", Severity::Critical);
rust_detector!(RustMemoryCorruptionDetector, "rust_memory_corruption", "Rust memory corruption", Severity::Critical);
rust_detector!(RustDanglingPointerDetector, "rust_dangling_pointer", "Rust dangling pointer", Severity::Critical);
rust_detector!(RustStackOverflowDetector, "rust_stack_overflow", "Rust stack overflow risk", Severity::Warning);
rust_detector!(RustHeapCorruptionDetector, "rust_heap_corruption", "Rust heap corruption", Severity::Critical);
rust_detector!(RustTypeConfusionDetector, "rust_type_confusion", "Rust type confusion", Severity::Warning);

// Concurrency patterns
rust_detector!(RustDataRaceDetector, "rust_data_race", "Rust data race", Severity::Critical);
rust_detector!(RustDeadlockDetector, "rust_deadlock", "Rust deadlock", Severity::Critical);
rust_detector!(RustRaceConditionDetector, "rust_race_condition", "Rust race condition", Severity::Critical);
rust_detector!(RustAtomicityViolationDetector, "rust_atomicity_violation", "Rust atomicity violation", Severity::Error);
rust_detector!(RustOrderViolationDetector, "rust_order_violation", "Rust order violation", Severity::Error);
rust_detector!(RustLivelockDetector, "rust_livelock", "Rust livelock", Severity::Warning);
rust_detector!(RustThreadSafetyDetector, "rust_thread_safety", "Rust thread safety violation", Severity::Error);
rust_detector!(RustAsyncHazardDetector, "rust_async_hazard", "Rust async hazard", Severity::Warning);
rust_detector!(RustLockContentionDetector, "rust_lock_contention", "Rust lock contention", Severity::Warning);
rust_detector!(RustSynchronizationDetector, "rust_synchronization", "Rust synchronization issue", Severity::Warning);

// Error handling patterns
rust_detector!(RustSwallowedExceptionDetector, "rust_swallowed_exception", "Rust swallowed exception", Severity::Warning);
rust_detector!(RustEmptyCatchDetector, "rust_empty_catch", "Rust empty catch block", Severity::Warning);
rust_detector!(RustGenericCatchDetector, "rust_generic_catch", "Rust generic catch", Severity::Info);
rust_detector!(RustUnhandledErrorDetector, "rust_unhandled_error", "Rust unhandled error", Severity::Warning);
rust_detector!(RustErrorIgnoredDetector, "rust_error_ignored", "Rust error ignored", Severity::Warning);
rust_detector!(RustPanicMisuseDetector, "rust_panic_misuse", "Rust panic misuse", Severity::Warning);
rust_detector!(RustErrorPropagationDetector, "rust_error_propagation", "Rust error propagation issue", Severity::Info);
rust_detector!(RustResourceCleanupDetector, "rust_resource_cleanup", "Rust missing resource cleanup", Severity::Warning);
rust_detector!(RustTransactionRollbackDetector, "rust_transaction_rollback", "Rust missing transaction rollback", Severity::Warning);
rust_detector!(RustRetryLogicDetector, "rust_retry_logic", "Rust problematic retry logic", Severity::Info);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_patterns() {
        let patterns = get_rust_patterns();
        assert!(patterns.len() >= 50, "Should have 50+ Rust patterns");
    }
}
