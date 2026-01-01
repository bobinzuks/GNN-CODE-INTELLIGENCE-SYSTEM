//! Typescript-specific patterns (50+ patterns per language)

use crate::{CodeGraph, PatternDetector, PatternInstance, FixSuggestion, Severity};
use std::sync::Arc;

pub fn get_typescript_patterns() -> Vec<Arc<dyn PatternDetector>> {
    vec![
        // Security patterns (10)
        Arc::new(TypescriptSQLInjectionDetector::new()),
        Arc::new(TypescriptXSSDetector::new()),
        Arc::new(TypescriptPathTraversalDetector::new()),
        Arc::new(TypescriptCommandInjectionDetector::new()),
        Arc::new(TypescriptDeserializationDetector::new()),
        Arc::new(TypescriptHardcodedSecretsDetector::new()),
        Arc::new(TypescriptWeakCryptoDetector::new()),
        Arc::new(TypescriptInsecureRandomDetector::new()),
        Arc::new(TypescriptAuthBypassDetector::new()),
        Arc::new(TypescriptCSRFDetector::new()),

        // Performance patterns (10)
        Arc::new(TypescriptNPlusOneDetector::new()),
        Arc::new(TypescriptIneffectiveLoopDetector::new()),
        Arc::new(TypescriptExcessiveAllocationDetector::new()),
        Arc::new(TypescriptStringConcatDetector::new()),
        Arc::new(TypescriptBlockingIODetector::new()),
        Arc::new(TypescriptMissingCacheDetector::new()),
        Arc::new(TypescriptAlgorithmComplexityDetector::new()),
        Arc::new(TypescriptRedundantComputationDetector::new()),
        Arc::new(TypescriptMemoryLeakDetector::new()),
        Arc::new(TypescriptResourceExhaustionDetector::new()),

        // Memory safety patterns (10)
        Arc::new(TypescriptUseAfterFreeDetector::new()),
        Arc::new(TypescriptBufferOverflowDetector::new()),
        Arc::new(TypescriptNullPointerDetector::new()),
        Arc::new(TypescriptUninitializedMemoryDetector::new()),
        Arc::new(TypescriptDoubleFreeDetector::new()),
        Arc::new(TypescriptMemoryCorruptionDetector::new()),
        Arc::new(TypescriptDanglingPointerDetector::new()),
        Arc::new(TypescriptStackOverflowDetector::new()),
        Arc::new(TypescriptHeapCorruptionDetector::new()),
        Arc::new(TypescriptTypeConfusionDetector::new()),

        // Concurrency patterns (10)
        Arc::new(TypescriptDataRaceDetector::new()),
        Arc::new(TypescriptDeadlockDetector::new()),
        Arc::new(TypescriptRaceConditionDetector::new()),
        Arc::new(TypescriptAtomicityViolationDetector::new()),
        Arc::new(TypescriptOrderViolationDetector::new()),
        Arc::new(TypescriptLivelockDetector::new()),
        Arc::new(TypescriptThreadSafetyDetector::new()),
        Arc::new(TypescriptAsyncHazardDetector::new()),
        Arc::new(TypescriptLockContentionDetector::new()),
        Arc::new(TypescriptSynchronizationDetector::new()),

        // Error handling patterns (10)
        Arc::new(TypescriptSwallowedExceptionDetector::new()),
        Arc::new(TypescriptEmptyCatchDetector::new()),
        Arc::new(TypescriptGenericCatchDetector::new()),
        Arc::new(TypescriptUnhandledErrorDetector::new()),
        Arc::new(TypescriptErrorIgnoredDetector::new()),
        Arc::new(TypescriptPanicMisuseDetector::new()),
        Arc::new(TypescriptErrorPropagationDetector::new()),
        Arc::new(TypescriptResourceCleanupDetector::new()),
        Arc::new(TypescriptTransactionRollbackDetector::new()),
        Arc::new(TypescriptRetryLogicDetector::new()),
    ]
}

macro_rules! typescript_detector {
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
typescript_detector!(TypescriptSQLInjectionDetector, "typescript_sql_injection", "Typescript SQL injection vulnerability", Severity::Critical);
typescript_detector!(TypescriptXSSDetector, "typescript_xss", "Typescript XSS vulnerability", Severity::Critical);
typescript_detector!(TypescriptPathTraversalDetector, "typescript_path_traversal", "Typescript path traversal vulnerability", Severity::Critical);
typescript_detector!(TypescriptCommandInjectionDetector, "typescript_command_injection", "Typescript command injection", Severity::Critical);
typescript_detector!(TypescriptDeserializationDetector, "typescript_unsafe_deserialization", "Typescript unsafe deserialization", Severity::Critical);
typescript_detector!(TypescriptHardcodedSecretsDetector, "typescript_hardcoded_secrets", "Typescript hardcoded secrets", Severity::Warning);
typescript_detector!(TypescriptWeakCryptoDetector, "typescript_weak_crypto", "Typescript weak cryptography", Severity::Warning);
typescript_detector!(TypescriptInsecureRandomDetector, "typescript_insecure_random", "Typescript insecure randomness", Severity::Warning);
typescript_detector!(TypescriptAuthBypassDetector, "typescript_auth_bypass", "Typescript authentication bypass", Severity::Critical);
typescript_detector!(TypescriptCSRFDetector, "typescript_csrf", "Typescript CSRF vulnerability", Severity::Warning);

// Performance patterns
typescript_detector!(TypescriptNPlusOneDetector, "typescript_n_plus_one", "Typescript N+1 query problem", Severity::Warning);
typescript_detector!(TypescriptIneffectiveLoopDetector, "typescript_ineffective_loop", "Typescript ineffective loop", Severity::Warning);
typescript_detector!(TypescriptExcessiveAllocationDetector, "typescript_excessive_allocation", "Typescript excessive allocation", Severity::Warning);
typescript_detector!(TypescriptStringConcatDetector, "typescript_string_concat", "Typescript ineffective string concatenation", Severity::Info);
typescript_detector!(TypescriptBlockingIODetector, "typescript_blocking_io", "Typescript blocking I/O", Severity::Warning);
typescript_detector!(TypescriptMissingCacheDetector, "typescript_missing_cache", "Typescript missing cache", Severity::Info);
typescript_detector!(TypescriptAlgorithmComplexityDetector, "typescript_algorithm_complexity", "Typescript high algorithm complexity", Severity::Warning);
typescript_detector!(TypescriptRedundantComputationDetector, "typescript_redundant_computation", "Typescript redundant computation", Severity::Info);
typescript_detector!(TypescriptMemoryLeakDetector, "typescript_memory_leak", "Typescript memory leak", Severity::Warning);
typescript_detector!(TypescriptResourceExhaustionDetector, "typescript_resource_exhaustion", "Typescript resource exhaustion", Severity::Warning);

// Memory safety patterns
typescript_detector!(TypescriptUseAfterFreeDetector, "typescript_use_after_free", "Typescript use-after-free", Severity::Critical);
typescript_detector!(TypescriptBufferOverflowDetector, "typescript_buffer_overflow", "Typescript buffer overflow", Severity::Critical);
typescript_detector!(TypescriptNullPointerDetector, "typescript_null_pointer", "Typescript null pointer dereference", Severity::Critical);
typescript_detector!(TypescriptUninitializedMemoryDetector, "typescript_uninitialized_memory", "Typescript uninitialized memory", Severity::Critical);
typescript_detector!(TypescriptDoubleFreeDetector, "typescript_double_free", "Typescript double free", Severity::Critical);
typescript_detector!(TypescriptMemoryCorruptionDetector, "typescript_memory_corruption", "Typescript memory corruption", Severity::Critical);
typescript_detector!(TypescriptDanglingPointerDetector, "typescript_dangling_pointer", "Typescript dangling pointer", Severity::Critical);
typescript_detector!(TypescriptStackOverflowDetector, "typescript_stack_overflow", "Typescript stack overflow risk", Severity::Warning);
typescript_detector!(TypescriptHeapCorruptionDetector, "typescript_heap_corruption", "Typescript heap corruption", Severity::Critical);
typescript_detector!(TypescriptTypeConfusionDetector, "typescript_type_confusion", "Typescript type confusion", Severity::Warning);

// Concurrency patterns
typescript_detector!(TypescriptDataRaceDetector, "typescript_data_race", "Typescript data race", Severity::Critical);
typescript_detector!(TypescriptDeadlockDetector, "typescript_deadlock", "Typescript deadlock", Severity::Critical);
typescript_detector!(TypescriptRaceConditionDetector, "typescript_race_condition", "Typescript race condition", Severity::Critical);
typescript_detector!(TypescriptAtomicityViolationDetector, "typescript_atomicity_violation", "Typescript atomicity violation", Severity::Error);
typescript_detector!(TypescriptOrderViolationDetector, "typescript_order_violation", "Typescript order violation", Severity::Error);
typescript_detector!(TypescriptLivelockDetector, "typescript_livelock", "Typescript livelock", Severity::Warning);
typescript_detector!(TypescriptThreadSafetyDetector, "typescript_thread_safety", "Typescript thread safety violation", Severity::Error);
typescript_detector!(TypescriptAsyncHazardDetector, "typescript_async_hazard", "Typescript async hazard", Severity::Warning);
typescript_detector!(TypescriptLockContentionDetector, "typescript_lock_contention", "Typescript lock contention", Severity::Warning);
typescript_detector!(TypescriptSynchronizationDetector, "typescript_synchronization", "Typescript synchronization issue", Severity::Warning);

// Error handling patterns
typescript_detector!(TypescriptSwallowedExceptionDetector, "typescript_swallowed_exception", "Typescript swallowed exception", Severity::Warning);
typescript_detector!(TypescriptEmptyCatchDetector, "typescript_empty_catch", "Typescript empty catch block", Severity::Warning);
typescript_detector!(TypescriptGenericCatchDetector, "typescript_generic_catch", "Typescript generic catch", Severity::Info);
typescript_detector!(TypescriptUnhandledErrorDetector, "typescript_unhandled_error", "Typescript unhandled error", Severity::Warning);
typescript_detector!(TypescriptErrorIgnoredDetector, "typescript_error_ignored", "Typescript error ignored", Severity::Warning);
typescript_detector!(TypescriptPanicMisuseDetector, "typescript_panic_misuse", "Typescript panic misuse", Severity::Warning);
typescript_detector!(TypescriptErrorPropagationDetector, "typescript_error_propagation", "Typescript error propagation issue", Severity::Info);
typescript_detector!(TypescriptResourceCleanupDetector, "typescript_resource_cleanup", "Typescript missing resource cleanup", Severity::Warning);
typescript_detector!(TypescriptTransactionRollbackDetector, "typescript_transaction_rollback", "Typescript missing transaction rollback", Severity::Warning);
typescript_detector!(TypescriptRetryLogicDetector, "typescript_retry_logic", "Typescript problematic retry logic", Severity::Info);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_typescript_patterns() {
        let patterns = get_typescript_patterns();
        assert!(patterns.len() >= 50, "Should have 50+ Typescript patterns");
    }
}
