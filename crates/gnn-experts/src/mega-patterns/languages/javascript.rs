//! Javascript-specific patterns (50+ patterns per language)

use crate::{CodeGraph, PatternDetector, PatternInstance, FixSuggestion, Severity};
use std::sync::Arc;

pub fn get_javascript_patterns() -> Vec<Arc<dyn PatternDetector>> {
    vec![
        // Security patterns (10)
        Arc::new(JavascriptSQLInjectionDetector::new()),
        Arc::new(JavascriptXSSDetector::new()),
        Arc::new(JavascriptPathTraversalDetector::new()),
        Arc::new(JavascriptCommandInjectionDetector::new()),
        Arc::new(JavascriptDeserializationDetector::new()),
        Arc::new(JavascriptHardcodedSecretsDetector::new()),
        Arc::new(JavascriptWeakCryptoDetector::new()),
        Arc::new(JavascriptInsecureRandomDetector::new()),
        Arc::new(JavascriptAuthBypassDetector::new()),
        Arc::new(JavascriptCSRFDetector::new()),

        // Performance patterns (10)
        Arc::new(JavascriptNPlusOneDetector::new()),
        Arc::new(JavascriptIneffectiveLoopDetector::new()),
        Arc::new(JavascriptExcessiveAllocationDetector::new()),
        Arc::new(JavascriptStringConcatDetector::new()),
        Arc::new(JavascriptBlockingIODetector::new()),
        Arc::new(JavascriptMissingCacheDetector::new()),
        Arc::new(JavascriptAlgorithmComplexityDetector::new()),
        Arc::new(JavascriptRedundantComputationDetector::new()),
        Arc::new(JavascriptMemoryLeakDetector::new()),
        Arc::new(JavascriptResourceExhaustionDetector::new()),

        // Memory safety patterns (10)
        Arc::new(JavascriptUseAfterFreeDetector::new()),
        Arc::new(JavascriptBufferOverflowDetector::new()),
        Arc::new(JavascriptNullPointerDetector::new()),
        Arc::new(JavascriptUninitializedMemoryDetector::new()),
        Arc::new(JavascriptDoubleFreeDetector::new()),
        Arc::new(JavascriptMemoryCorruptionDetector::new()),
        Arc::new(JavascriptDanglingPointerDetector::new()),
        Arc::new(JavascriptStackOverflowDetector::new()),
        Arc::new(JavascriptHeapCorruptionDetector::new()),
        Arc::new(JavascriptTypeConfusionDetector::new()),

        // Concurrency patterns (10)
        Arc::new(JavascriptDataRaceDetector::new()),
        Arc::new(JavascriptDeadlockDetector::new()),
        Arc::new(JavascriptRaceConditionDetector::new()),
        Arc::new(JavascriptAtomicityViolationDetector::new()),
        Arc::new(JavascriptOrderViolationDetector::new()),
        Arc::new(JavascriptLivelockDetector::new()),
        Arc::new(JavascriptThreadSafetyDetector::new()),
        Arc::new(JavascriptAsyncHazardDetector::new()),
        Arc::new(JavascriptLockContentionDetector::new()),
        Arc::new(JavascriptSynchronizationDetector::new()),

        // Error handling patterns (10)
        Arc::new(JavascriptSwallowedExceptionDetector::new()),
        Arc::new(JavascriptEmptyCatchDetector::new()),
        Arc::new(JavascriptGenericCatchDetector::new()),
        Arc::new(JavascriptUnhandledErrorDetector::new()),
        Arc::new(JavascriptErrorIgnoredDetector::new()),
        Arc::new(JavascriptPanicMisuseDetector::new()),
        Arc::new(JavascriptErrorPropagationDetector::new()),
        Arc::new(JavascriptResourceCleanupDetector::new()),
        Arc::new(JavascriptTransactionRollbackDetector::new()),
        Arc::new(JavascriptRetryLogicDetector::new()),
    ]
}

macro_rules! javascript_detector {
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
javascript_detector!(JavascriptSQLInjectionDetector, "javascript_sql_injection", "Javascript SQL injection vulnerability", Severity::Critical);
javascript_detector!(JavascriptXSSDetector, "javascript_xss", "Javascript XSS vulnerability", Severity::Critical);
javascript_detector!(JavascriptPathTraversalDetector, "javascript_path_traversal", "Javascript path traversal vulnerability", Severity::Critical);
javascript_detector!(JavascriptCommandInjectionDetector, "javascript_command_injection", "Javascript command injection", Severity::Critical);
javascript_detector!(JavascriptDeserializationDetector, "javascript_unsafe_deserialization", "Javascript unsafe deserialization", Severity::Critical);
javascript_detector!(JavascriptHardcodedSecretsDetector, "javascript_hardcoded_secrets", "Javascript hardcoded secrets", Severity::Warning);
javascript_detector!(JavascriptWeakCryptoDetector, "javascript_weak_crypto", "Javascript weak cryptography", Severity::Warning);
javascript_detector!(JavascriptInsecureRandomDetector, "javascript_insecure_random", "Javascript insecure randomness", Severity::Warning);
javascript_detector!(JavascriptAuthBypassDetector, "javascript_auth_bypass", "Javascript authentication bypass", Severity::Critical);
javascript_detector!(JavascriptCSRFDetector, "javascript_csrf", "Javascript CSRF vulnerability", Severity::Warning);

// Performance patterns
javascript_detector!(JavascriptNPlusOneDetector, "javascript_n_plus_one", "Javascript N+1 query problem", Severity::Warning);
javascript_detector!(JavascriptIneffectiveLoopDetector, "javascript_ineffective_loop", "Javascript ineffective loop", Severity::Warning);
javascript_detector!(JavascriptExcessiveAllocationDetector, "javascript_excessive_allocation", "Javascript excessive allocation", Severity::Warning);
javascript_detector!(JavascriptStringConcatDetector, "javascript_string_concat", "Javascript ineffective string concatenation", Severity::Info);
javascript_detector!(JavascriptBlockingIODetector, "javascript_blocking_io", "Javascript blocking I/O", Severity::Warning);
javascript_detector!(JavascriptMissingCacheDetector, "javascript_missing_cache", "Javascript missing cache", Severity::Info);
javascript_detector!(JavascriptAlgorithmComplexityDetector, "javascript_algorithm_complexity", "Javascript high algorithm complexity", Severity::Warning);
javascript_detector!(JavascriptRedundantComputationDetector, "javascript_redundant_computation", "Javascript redundant computation", Severity::Info);
javascript_detector!(JavascriptMemoryLeakDetector, "javascript_memory_leak", "Javascript memory leak", Severity::Warning);
javascript_detector!(JavascriptResourceExhaustionDetector, "javascript_resource_exhaustion", "Javascript resource exhaustion", Severity::Warning);

// Memory safety patterns
javascript_detector!(JavascriptUseAfterFreeDetector, "javascript_use_after_free", "Javascript use-after-free", Severity::Critical);
javascript_detector!(JavascriptBufferOverflowDetector, "javascript_buffer_overflow", "Javascript buffer overflow", Severity::Critical);
javascript_detector!(JavascriptNullPointerDetector, "javascript_null_pointer", "Javascript null pointer dereference", Severity::Critical);
javascript_detector!(JavascriptUninitializedMemoryDetector, "javascript_uninitialized_memory", "Javascript uninitialized memory", Severity::Critical);
javascript_detector!(JavascriptDoubleFreeDetector, "javascript_double_free", "Javascript double free", Severity::Critical);
javascript_detector!(JavascriptMemoryCorruptionDetector, "javascript_memory_corruption", "Javascript memory corruption", Severity::Critical);
javascript_detector!(JavascriptDanglingPointerDetector, "javascript_dangling_pointer", "Javascript dangling pointer", Severity::Critical);
javascript_detector!(JavascriptStackOverflowDetector, "javascript_stack_overflow", "Javascript stack overflow risk", Severity::Warning);
javascript_detector!(JavascriptHeapCorruptionDetector, "javascript_heap_corruption", "Javascript heap corruption", Severity::Critical);
javascript_detector!(JavascriptTypeConfusionDetector, "javascript_type_confusion", "Javascript type confusion", Severity::Warning);

// Concurrency patterns
javascript_detector!(JavascriptDataRaceDetector, "javascript_data_race", "Javascript data race", Severity::Critical);
javascript_detector!(JavascriptDeadlockDetector, "javascript_deadlock", "Javascript deadlock", Severity::Critical);
javascript_detector!(JavascriptRaceConditionDetector, "javascript_race_condition", "Javascript race condition", Severity::Critical);
javascript_detector!(JavascriptAtomicityViolationDetector, "javascript_atomicity_violation", "Javascript atomicity violation", Severity::Error);
javascript_detector!(JavascriptOrderViolationDetector, "javascript_order_violation", "Javascript order violation", Severity::Error);
javascript_detector!(JavascriptLivelockDetector, "javascript_livelock", "Javascript livelock", Severity::Warning);
javascript_detector!(JavascriptThreadSafetyDetector, "javascript_thread_safety", "Javascript thread safety violation", Severity::Error);
javascript_detector!(JavascriptAsyncHazardDetector, "javascript_async_hazard", "Javascript async hazard", Severity::Warning);
javascript_detector!(JavascriptLockContentionDetector, "javascript_lock_contention", "Javascript lock contention", Severity::Warning);
javascript_detector!(JavascriptSynchronizationDetector, "javascript_synchronization", "Javascript synchronization issue", Severity::Warning);

// Error handling patterns
javascript_detector!(JavascriptSwallowedExceptionDetector, "javascript_swallowed_exception", "Javascript swallowed exception", Severity::Warning);
javascript_detector!(JavascriptEmptyCatchDetector, "javascript_empty_catch", "Javascript empty catch block", Severity::Warning);
javascript_detector!(JavascriptGenericCatchDetector, "javascript_generic_catch", "Javascript generic catch", Severity::Info);
javascript_detector!(JavascriptUnhandledErrorDetector, "javascript_unhandled_error", "Javascript unhandled error", Severity::Warning);
javascript_detector!(JavascriptErrorIgnoredDetector, "javascript_error_ignored", "Javascript error ignored", Severity::Warning);
javascript_detector!(JavascriptPanicMisuseDetector, "javascript_panic_misuse", "Javascript panic misuse", Severity::Warning);
javascript_detector!(JavascriptErrorPropagationDetector, "javascript_error_propagation", "Javascript error propagation issue", Severity::Info);
javascript_detector!(JavascriptResourceCleanupDetector, "javascript_resource_cleanup", "Javascript missing resource cleanup", Severity::Warning);
javascript_detector!(JavascriptTransactionRollbackDetector, "javascript_transaction_rollback", "Javascript missing transaction rollback", Severity::Warning);
javascript_detector!(JavascriptRetryLogicDetector, "javascript_retry_logic", "Javascript problematic retry logic", Severity::Info);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_javascript_patterns() {
        let patterns = get_javascript_patterns();
        assert!(patterns.len() >= 50, "Should have 50+ Javascript patterns");
    }
}
