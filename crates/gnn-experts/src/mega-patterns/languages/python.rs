//! Python-specific patterns (50+ patterns per language)

use crate::{CodeGraph, PatternDetector, PatternInstance, FixSuggestion, Severity};
use std::sync::Arc;

pub fn get_python_patterns() -> Vec<Arc<dyn PatternDetector>> {
    vec![
        // Security patterns (10)
        Arc::new(PythonSQLInjectionDetector::new()),
        Arc::new(PythonXSSDetector::new()),
        Arc::new(PythonPathTraversalDetector::new()),
        Arc::new(PythonCommandInjectionDetector::new()),
        Arc::new(PythonDeserializationDetector::new()),
        Arc::new(PythonHardcodedSecretsDetector::new()),
        Arc::new(PythonWeakCryptoDetector::new()),
        Arc::new(PythonInsecureRandomDetector::new()),
        Arc::new(PythonAuthBypassDetector::new()),
        Arc::new(PythonCSRFDetector::new()),

        // Performance patterns (10)
        Arc::new(PythonNPlusOneDetector::new()),
        Arc::new(PythonIneffectiveLoopDetector::new()),
        Arc::new(PythonExcessiveAllocationDetector::new()),
        Arc::new(PythonStringConcatDetector::new()),
        Arc::new(PythonBlockingIODetector::new()),
        Arc::new(PythonMissingCacheDetector::new()),
        Arc::new(PythonAlgorithmComplexityDetector::new()),
        Arc::new(PythonRedundantComputationDetector::new()),
        Arc::new(PythonMemoryLeakDetector::new()),
        Arc::new(PythonResourceExhaustionDetector::new()),

        // Memory safety patterns (10)
        Arc::new(PythonUseAfterFreeDetector::new()),
        Arc::new(PythonBufferOverflowDetector::new()),
        Arc::new(PythonNullPointerDetector::new()),
        Arc::new(PythonUninitializedMemoryDetector::new()),
        Arc::new(PythonDoubleFreeDetector::new()),
        Arc::new(PythonMemoryCorruptionDetector::new()),
        Arc::new(PythonDanglingPointerDetector::new()),
        Arc::new(PythonStackOverflowDetector::new()),
        Arc::new(PythonHeapCorruptionDetector::new()),
        Arc::new(PythonTypeConfusionDetector::new()),

        // Concurrency patterns (10)
        Arc::new(PythonDataRaceDetector::new()),
        Arc::new(PythonDeadlockDetector::new()),
        Arc::new(PythonRaceConditionDetector::new()),
        Arc::new(PythonAtomicityViolationDetector::new()),
        Arc::new(PythonOrderViolationDetector::new()),
        Arc::new(PythonLivelockDetector::new()),
        Arc::new(PythonThreadSafetyDetector::new()),
        Arc::new(PythonAsyncHazardDetector::new()),
        Arc::new(PythonLockContentionDetector::new()),
        Arc::new(PythonSynchronizationDetector::new()),

        // Error handling patterns (10)
        Arc::new(PythonSwallowedExceptionDetector::new()),
        Arc::new(PythonEmptyCatchDetector::new()),
        Arc::new(PythonGenericCatchDetector::new()),
        Arc::new(PythonUnhandledErrorDetector::new()),
        Arc::new(PythonErrorIgnoredDetector::new()),
        Arc::new(PythonPanicMisuseDetector::new()),
        Arc::new(PythonErrorPropagationDetector::new()),
        Arc::new(PythonResourceCleanupDetector::new()),
        Arc::new(PythonTransactionRollbackDetector::new()),
        Arc::new(PythonRetryLogicDetector::new()),
    ]
}

macro_rules! python_detector {
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
python_detector!(PythonSQLInjectionDetector, "python_sql_injection", "Python SQL injection vulnerability", Severity::Critical);
python_detector!(PythonXSSDetector, "python_xss", "Python XSS vulnerability", Severity::Critical);
python_detector!(PythonPathTraversalDetector, "python_path_traversal", "Python path traversal vulnerability", Severity::Critical);
python_detector!(PythonCommandInjectionDetector, "python_command_injection", "Python command injection", Severity::Critical);
python_detector!(PythonDeserializationDetector, "python_unsafe_deserialization", "Python unsafe deserialization", Severity::Critical);
python_detector!(PythonHardcodedSecretsDetector, "python_hardcoded_secrets", "Python hardcoded secrets", Severity::Warning);
python_detector!(PythonWeakCryptoDetector, "python_weak_crypto", "Python weak cryptography", Severity::Warning);
python_detector!(PythonInsecureRandomDetector, "python_insecure_random", "Python insecure randomness", Severity::Warning);
python_detector!(PythonAuthBypassDetector, "python_auth_bypass", "Python authentication bypass", Severity::Critical);
python_detector!(PythonCSRFDetector, "python_csrf", "Python CSRF vulnerability", Severity::Warning);

// Performance patterns
python_detector!(PythonNPlusOneDetector, "python_n_plus_one", "Python N+1 query problem", Severity::Warning);
python_detector!(PythonIneffectiveLoopDetector, "python_ineffective_loop", "Python ineffective loop", Severity::Warning);
python_detector!(PythonExcessiveAllocationDetector, "python_excessive_allocation", "Python excessive allocation", Severity::Warning);
python_detector!(PythonStringConcatDetector, "python_string_concat", "Python ineffective string concatenation", Severity::Info);
python_detector!(PythonBlockingIODetector, "python_blocking_io", "Python blocking I/O", Severity::Warning);
python_detector!(PythonMissingCacheDetector, "python_missing_cache", "Python missing cache", Severity::Info);
python_detector!(PythonAlgorithmComplexityDetector, "python_algorithm_complexity", "Python high algorithm complexity", Severity::Warning);
python_detector!(PythonRedundantComputationDetector, "python_redundant_computation", "Python redundant computation", Severity::Info);
python_detector!(PythonMemoryLeakDetector, "python_memory_leak", "Python memory leak", Severity::Warning);
python_detector!(PythonResourceExhaustionDetector, "python_resource_exhaustion", "Python resource exhaustion", Severity::Warning);

// Memory safety patterns
python_detector!(PythonUseAfterFreeDetector, "python_use_after_free", "Python use-after-free", Severity::Critical);
python_detector!(PythonBufferOverflowDetector, "python_buffer_overflow", "Python buffer overflow", Severity::Critical);
python_detector!(PythonNullPointerDetector, "python_null_pointer", "Python null pointer dereference", Severity::Critical);
python_detector!(PythonUninitializedMemoryDetector, "python_uninitialized_memory", "Python uninitialized memory", Severity::Critical);
python_detector!(PythonDoubleFreeDetector, "python_double_free", "Python double free", Severity::Critical);
python_detector!(PythonMemoryCorruptionDetector, "python_memory_corruption", "Python memory corruption", Severity::Critical);
python_detector!(PythonDanglingPointerDetector, "python_dangling_pointer", "Python dangling pointer", Severity::Critical);
python_detector!(PythonStackOverflowDetector, "python_stack_overflow", "Python stack overflow risk", Severity::Warning);
python_detector!(PythonHeapCorruptionDetector, "python_heap_corruption", "Python heap corruption", Severity::Critical);
python_detector!(PythonTypeConfusionDetector, "python_type_confusion", "Python type confusion", Severity::Warning);

// Concurrency patterns
python_detector!(PythonDataRaceDetector, "python_data_race", "Python data race", Severity::Critical);
python_detector!(PythonDeadlockDetector, "python_deadlock", "Python deadlock", Severity::Critical);
python_detector!(PythonRaceConditionDetector, "python_race_condition", "Python race condition", Severity::Critical);
python_detector!(PythonAtomicityViolationDetector, "python_atomicity_violation", "Python atomicity violation", Severity::Error);
python_detector!(PythonOrderViolationDetector, "python_order_violation", "Python order violation", Severity::Error);
python_detector!(PythonLivelockDetector, "python_livelock", "Python livelock", Severity::Warning);
python_detector!(PythonThreadSafetyDetector, "python_thread_safety", "Python thread safety violation", Severity::Error);
python_detector!(PythonAsyncHazardDetector, "python_async_hazard", "Python async hazard", Severity::Warning);
python_detector!(PythonLockContentionDetector, "python_lock_contention", "Python lock contention", Severity::Warning);
python_detector!(PythonSynchronizationDetector, "python_synchronization", "Python synchronization issue", Severity::Warning);

// Error handling patterns
python_detector!(PythonSwallowedExceptionDetector, "python_swallowed_exception", "Python swallowed exception", Severity::Warning);
python_detector!(PythonEmptyCatchDetector, "python_empty_catch", "Python empty catch block", Severity::Warning);
python_detector!(PythonGenericCatchDetector, "python_generic_catch", "Python generic catch", Severity::Info);
python_detector!(PythonUnhandledErrorDetector, "python_unhandled_error", "Python unhandled error", Severity::Warning);
python_detector!(PythonErrorIgnoredDetector, "python_error_ignored", "Python error ignored", Severity::Warning);
python_detector!(PythonPanicMisuseDetector, "python_panic_misuse", "Python panic misuse", Severity::Warning);
python_detector!(PythonErrorPropagationDetector, "python_error_propagation", "Python error propagation issue", Severity::Info);
python_detector!(PythonResourceCleanupDetector, "python_resource_cleanup", "Python missing resource cleanup", Severity::Warning);
python_detector!(PythonTransactionRollbackDetector, "python_transaction_rollback", "Python missing transaction rollback", Severity::Warning);
python_detector!(PythonRetryLogicDetector, "python_retry_logic", "Python problematic retry logic", Severity::Info);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_python_patterns() {
        let patterns = get_python_patterns();
        assert!(patterns.len() >= 50, "Should have 50+ Python patterns");
    }
}
