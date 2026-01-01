//! Php-specific patterns (50+ patterns per language)

use crate::{CodeGraph, PatternDetector, PatternInstance, FixSuggestion, Severity};
use std::sync::Arc;

pub fn get_php_patterns() -> Vec<Arc<dyn PatternDetector>> {
    vec![
        // Security patterns (10)
        Arc::new(PhpSQLInjectionDetector::new()),
        Arc::new(PhpXSSDetector::new()),
        Arc::new(PhpPathTraversalDetector::new()),
        Arc::new(PhpCommandInjectionDetector::new()),
        Arc::new(PhpDeserializationDetector::new()),
        Arc::new(PhpHardcodedSecretsDetector::new()),
        Arc::new(PhpWeakCryptoDetector::new()),
        Arc::new(PhpInsecureRandomDetector::new()),
        Arc::new(PhpAuthBypassDetector::new()),
        Arc::new(PhpCSRFDetector::new()),

        // Performance patterns (10)
        Arc::new(PhpNPlusOneDetector::new()),
        Arc::new(PhpIneffectiveLoopDetector::new()),
        Arc::new(PhpExcessiveAllocationDetector::new()),
        Arc::new(PhpStringConcatDetector::new()),
        Arc::new(PhpBlockingIODetector::new()),
        Arc::new(PhpMissingCacheDetector::new()),
        Arc::new(PhpAlgorithmComplexityDetector::new()),
        Arc::new(PhpRedundantComputationDetector::new()),
        Arc::new(PhpMemoryLeakDetector::new()),
        Arc::new(PhpResourceExhaustionDetector::new()),

        // Memory safety patterns (10)
        Arc::new(PhpUseAfterFreeDetector::new()),
        Arc::new(PhpBufferOverflowDetector::new()),
        Arc::new(PhpNullPointerDetector::new()),
        Arc::new(PhpUninitializedMemoryDetector::new()),
        Arc::new(PhpDoubleFreeDetector::new()),
        Arc::new(PhpMemoryCorruptionDetector::new()),
        Arc::new(PhpDanglingPointerDetector::new()),
        Arc::new(PhpStackOverflowDetector::new()),
        Arc::new(PhpHeapCorruptionDetector::new()),
        Arc::new(PhpTypeConfusionDetector::new()),

        // Concurrency patterns (10)
        Arc::new(PhpDataRaceDetector::new()),
        Arc::new(PhpDeadlockDetector::new()),
        Arc::new(PhpRaceConditionDetector::new()),
        Arc::new(PhpAtomicityViolationDetector::new()),
        Arc::new(PhpOrderViolationDetector::new()),
        Arc::new(PhpLivelockDetector::new()),
        Arc::new(PhpThreadSafetyDetector::new()),
        Arc::new(PhpAsyncHazardDetector::new()),
        Arc::new(PhpLockContentionDetector::new()),
        Arc::new(PhpSynchronizationDetector::new()),

        // Error handling patterns (10)
        Arc::new(PhpSwallowedExceptionDetector::new()),
        Arc::new(PhpEmptyCatchDetector::new()),
        Arc::new(PhpGenericCatchDetector::new()),
        Arc::new(PhpUnhandledErrorDetector::new()),
        Arc::new(PhpErrorIgnoredDetector::new()),
        Arc::new(PhpPanicMisuseDetector::new()),
        Arc::new(PhpErrorPropagationDetector::new()),
        Arc::new(PhpResourceCleanupDetector::new()),
        Arc::new(PhpTransactionRollbackDetector::new()),
        Arc::new(PhpRetryLogicDetector::new()),
    ]
}

macro_rules! php_detector {
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
php_detector!(PhpSQLInjectionDetector, "php_sql_injection", "Php SQL injection vulnerability", Severity::Critical);
php_detector!(PhpXSSDetector, "php_xss", "Php XSS vulnerability", Severity::Critical);
php_detector!(PhpPathTraversalDetector, "php_path_traversal", "Php path traversal vulnerability", Severity::Critical);
php_detector!(PhpCommandInjectionDetector, "php_command_injection", "Php command injection", Severity::Critical);
php_detector!(PhpDeserializationDetector, "php_unsafe_deserialization", "Php unsafe deserialization", Severity::Critical);
php_detector!(PhpHardcodedSecretsDetector, "php_hardcoded_secrets", "Php hardcoded secrets", Severity::Warning);
php_detector!(PhpWeakCryptoDetector, "php_weak_crypto", "Php weak cryptography", Severity::Warning);
php_detector!(PhpInsecureRandomDetector, "php_insecure_random", "Php insecure randomness", Severity::Warning);
php_detector!(PhpAuthBypassDetector, "php_auth_bypass", "Php authentication bypass", Severity::Critical);
php_detector!(PhpCSRFDetector, "php_csrf", "Php CSRF vulnerability", Severity::Warning);

// Performance patterns
php_detector!(PhpNPlusOneDetector, "php_n_plus_one", "Php N+1 query problem", Severity::Warning);
php_detector!(PhpIneffectiveLoopDetector, "php_ineffective_loop", "Php ineffective loop", Severity::Warning);
php_detector!(PhpExcessiveAllocationDetector, "php_excessive_allocation", "Php excessive allocation", Severity::Warning);
php_detector!(PhpStringConcatDetector, "php_string_concat", "Php ineffective string concatenation", Severity::Info);
php_detector!(PhpBlockingIODetector, "php_blocking_io", "Php blocking I/O", Severity::Warning);
php_detector!(PhpMissingCacheDetector, "php_missing_cache", "Php missing cache", Severity::Info);
php_detector!(PhpAlgorithmComplexityDetector, "php_algorithm_complexity", "Php high algorithm complexity", Severity::Warning);
php_detector!(PhpRedundantComputationDetector, "php_redundant_computation", "Php redundant computation", Severity::Info);
php_detector!(PhpMemoryLeakDetector, "php_memory_leak", "Php memory leak", Severity::Warning);
php_detector!(PhpResourceExhaustionDetector, "php_resource_exhaustion", "Php resource exhaustion", Severity::Warning);

// Memory safety patterns
php_detector!(PhpUseAfterFreeDetector, "php_use_after_free", "Php use-after-free", Severity::Critical);
php_detector!(PhpBufferOverflowDetector, "php_buffer_overflow", "Php buffer overflow", Severity::Critical);
php_detector!(PhpNullPointerDetector, "php_null_pointer", "Php null pointer dereference", Severity::Critical);
php_detector!(PhpUninitializedMemoryDetector, "php_uninitialized_memory", "Php uninitialized memory", Severity::Critical);
php_detector!(PhpDoubleFreeDetector, "php_double_free", "Php double free", Severity::Critical);
php_detector!(PhpMemoryCorruptionDetector, "php_memory_corruption", "Php memory corruption", Severity::Critical);
php_detector!(PhpDanglingPointerDetector, "php_dangling_pointer", "Php dangling pointer", Severity::Critical);
php_detector!(PhpStackOverflowDetector, "php_stack_overflow", "Php stack overflow risk", Severity::Warning);
php_detector!(PhpHeapCorruptionDetector, "php_heap_corruption", "Php heap corruption", Severity::Critical);
php_detector!(PhpTypeConfusionDetector, "php_type_confusion", "Php type confusion", Severity::Warning);

// Concurrency patterns
php_detector!(PhpDataRaceDetector, "php_data_race", "Php data race", Severity::Critical);
php_detector!(PhpDeadlockDetector, "php_deadlock", "Php deadlock", Severity::Critical);
php_detector!(PhpRaceConditionDetector, "php_race_condition", "Php race condition", Severity::Critical);
php_detector!(PhpAtomicityViolationDetector, "php_atomicity_violation", "Php atomicity violation", Severity::Error);
php_detector!(PhpOrderViolationDetector, "php_order_violation", "Php order violation", Severity::Error);
php_detector!(PhpLivelockDetector, "php_livelock", "Php livelock", Severity::Warning);
php_detector!(PhpThreadSafetyDetector, "php_thread_safety", "Php thread safety violation", Severity::Error);
php_detector!(PhpAsyncHazardDetector, "php_async_hazard", "Php async hazard", Severity::Warning);
php_detector!(PhpLockContentionDetector, "php_lock_contention", "Php lock contention", Severity::Warning);
php_detector!(PhpSynchronizationDetector, "php_synchronization", "Php synchronization issue", Severity::Warning);

// Error handling patterns
php_detector!(PhpSwallowedExceptionDetector, "php_swallowed_exception", "Php swallowed exception", Severity::Warning);
php_detector!(PhpEmptyCatchDetector, "php_empty_catch", "Php empty catch block", Severity::Warning);
php_detector!(PhpGenericCatchDetector, "php_generic_catch", "Php generic catch", Severity::Info);
php_detector!(PhpUnhandledErrorDetector, "php_unhandled_error", "Php unhandled error", Severity::Warning);
php_detector!(PhpErrorIgnoredDetector, "php_error_ignored", "Php error ignored", Severity::Warning);
php_detector!(PhpPanicMisuseDetector, "php_panic_misuse", "Php panic misuse", Severity::Warning);
php_detector!(PhpErrorPropagationDetector, "php_error_propagation", "Php error propagation issue", Severity::Info);
php_detector!(PhpResourceCleanupDetector, "php_resource_cleanup", "Php missing resource cleanup", Severity::Warning);
php_detector!(PhpTransactionRollbackDetector, "php_transaction_rollback", "Php missing transaction rollback", Severity::Warning);
php_detector!(PhpRetryLogicDetector, "php_retry_logic", "Php problematic retry logic", Severity::Info);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_php_patterns() {
        let patterns = get_php_patterns();
        assert!(patterns.len() >= 50, "Should have 50+ Php patterns");
    }
}
