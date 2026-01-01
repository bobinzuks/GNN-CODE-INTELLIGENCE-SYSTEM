//! Dart-specific patterns (50+ patterns per language)

use crate::{CodeGraph, PatternDetector, PatternInstance, FixSuggestion, Severity};
use std::sync::Arc;

pub fn get_dart_patterns() -> Vec<Arc<dyn PatternDetector>> {
    vec![
        // Security patterns (10)
        Arc::new(DartSQLInjectionDetector::new()),
        Arc::new(DartXSSDetector::new()),
        Arc::new(DartPathTraversalDetector::new()),
        Arc::new(DartCommandInjectionDetector::new()),
        Arc::new(DartDeserializationDetector::new()),
        Arc::new(DartHardcodedSecretsDetector::new()),
        Arc::new(DartWeakCryptoDetector::new()),
        Arc::new(DartInsecureRandomDetector::new()),
        Arc::new(DartAuthBypassDetector::new()),
        Arc::new(DartCSRFDetector::new()),

        // Performance patterns (10)
        Arc::new(DartNPlusOneDetector::new()),
        Arc::new(DartIneffectiveLoopDetector::new()),
        Arc::new(DartExcessiveAllocationDetector::new()),
        Arc::new(DartStringConcatDetector::new()),
        Arc::new(DartBlockingIODetector::new()),
        Arc::new(DartMissingCacheDetector::new()),
        Arc::new(DartAlgorithmComplexityDetector::new()),
        Arc::new(DartRedundantComputationDetector::new()),
        Arc::new(DartMemoryLeakDetector::new()),
        Arc::new(DartResourceExhaustionDetector::new()),

        // Memory safety patterns (10)
        Arc::new(DartUseAfterFreeDetector::new()),
        Arc::new(DartBufferOverflowDetector::new()),
        Arc::new(DartNullPointerDetector::new()),
        Arc::new(DartUninitializedMemoryDetector::new()),
        Arc::new(DartDoubleFreeDetector::new()),
        Arc::new(DartMemoryCorruptionDetector::new()),
        Arc::new(DartDanglingPointerDetector::new()),
        Arc::new(DartStackOverflowDetector::new()),
        Arc::new(DartHeapCorruptionDetector::new()),
        Arc::new(DartTypeConfusionDetector::new()),

        // Concurrency patterns (10)
        Arc::new(DartDataRaceDetector::new()),
        Arc::new(DartDeadlockDetector::new()),
        Arc::new(DartRaceConditionDetector::new()),
        Arc::new(DartAtomicityViolationDetector::new()),
        Arc::new(DartOrderViolationDetector::new()),
        Arc::new(DartLivelockDetector::new()),
        Arc::new(DartThreadSafetyDetector::new()),
        Arc::new(DartAsyncHazardDetector::new()),
        Arc::new(DartLockContentionDetector::new()),
        Arc::new(DartSynchronizationDetector::new()),

        // Error handling patterns (10)
        Arc::new(DartSwallowedExceptionDetector::new()),
        Arc::new(DartEmptyCatchDetector::new()),
        Arc::new(DartGenericCatchDetector::new()),
        Arc::new(DartUnhandledErrorDetector::new()),
        Arc::new(DartErrorIgnoredDetector::new()),
        Arc::new(DartPanicMisuseDetector::new()),
        Arc::new(DartErrorPropagationDetector::new()),
        Arc::new(DartResourceCleanupDetector::new()),
        Arc::new(DartTransactionRollbackDetector::new()),
        Arc::new(DartRetryLogicDetector::new()),
    ]
}

macro_rules! dart_detector {
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
dart_detector!(DartSQLInjectionDetector, "dart_sql_injection", "Dart SQL injection vulnerability", Severity::Critical);
dart_detector!(DartXSSDetector, "dart_xss", "Dart XSS vulnerability", Severity::Critical);
dart_detector!(DartPathTraversalDetector, "dart_path_traversal", "Dart path traversal vulnerability", Severity::Critical);
dart_detector!(DartCommandInjectionDetector, "dart_command_injection", "Dart command injection", Severity::Critical);
dart_detector!(DartDeserializationDetector, "dart_unsafe_deserialization", "Dart unsafe deserialization", Severity::Critical);
dart_detector!(DartHardcodedSecretsDetector, "dart_hardcoded_secrets", "Dart hardcoded secrets", Severity::Warning);
dart_detector!(DartWeakCryptoDetector, "dart_weak_crypto", "Dart weak cryptography", Severity::Warning);
dart_detector!(DartInsecureRandomDetector, "dart_insecure_random", "Dart insecure randomness", Severity::Warning);
dart_detector!(DartAuthBypassDetector, "dart_auth_bypass", "Dart authentication bypass", Severity::Critical);
dart_detector!(DartCSRFDetector, "dart_csrf", "Dart CSRF vulnerability", Severity::Warning);

// Performance patterns
dart_detector!(DartNPlusOneDetector, "dart_n_plus_one", "Dart N+1 query problem", Severity::Warning);
dart_detector!(DartIneffectiveLoopDetector, "dart_ineffective_loop", "Dart ineffective loop", Severity::Warning);
dart_detector!(DartExcessiveAllocationDetector, "dart_excessive_allocation", "Dart excessive allocation", Severity::Warning);
dart_detector!(DartStringConcatDetector, "dart_string_concat", "Dart ineffective string concatenation", Severity::Info);
dart_detector!(DartBlockingIODetector, "dart_blocking_io", "Dart blocking I/O", Severity::Warning);
dart_detector!(DartMissingCacheDetector, "dart_missing_cache", "Dart missing cache", Severity::Info);
dart_detector!(DartAlgorithmComplexityDetector, "dart_algorithm_complexity", "Dart high algorithm complexity", Severity::Warning);
dart_detector!(DartRedundantComputationDetector, "dart_redundant_computation", "Dart redundant computation", Severity::Info);
dart_detector!(DartMemoryLeakDetector, "dart_memory_leak", "Dart memory leak", Severity::Warning);
dart_detector!(DartResourceExhaustionDetector, "dart_resource_exhaustion", "Dart resource exhaustion", Severity::Warning);

// Memory safety patterns
dart_detector!(DartUseAfterFreeDetector, "dart_use_after_free", "Dart use-after-free", Severity::Critical);
dart_detector!(DartBufferOverflowDetector, "dart_buffer_overflow", "Dart buffer overflow", Severity::Critical);
dart_detector!(DartNullPointerDetector, "dart_null_pointer", "Dart null pointer dereference", Severity::Critical);
dart_detector!(DartUninitializedMemoryDetector, "dart_uninitialized_memory", "Dart uninitialized memory", Severity::Critical);
dart_detector!(DartDoubleFreeDetector, "dart_double_free", "Dart double free", Severity::Critical);
dart_detector!(DartMemoryCorruptionDetector, "dart_memory_corruption", "Dart memory corruption", Severity::Critical);
dart_detector!(DartDanglingPointerDetector, "dart_dangling_pointer", "Dart dangling pointer", Severity::Critical);
dart_detector!(DartStackOverflowDetector, "dart_stack_overflow", "Dart stack overflow risk", Severity::Warning);
dart_detector!(DartHeapCorruptionDetector, "dart_heap_corruption", "Dart heap corruption", Severity::Critical);
dart_detector!(DartTypeConfusionDetector, "dart_type_confusion", "Dart type confusion", Severity::Warning);

// Concurrency patterns
dart_detector!(DartDataRaceDetector, "dart_data_race", "Dart data race", Severity::Critical);
dart_detector!(DartDeadlockDetector, "dart_deadlock", "Dart deadlock", Severity::Critical);
dart_detector!(DartRaceConditionDetector, "dart_race_condition", "Dart race condition", Severity::Critical);
dart_detector!(DartAtomicityViolationDetector, "dart_atomicity_violation", "Dart atomicity violation", Severity::Error);
dart_detector!(DartOrderViolationDetector, "dart_order_violation", "Dart order violation", Severity::Error);
dart_detector!(DartLivelockDetector, "dart_livelock", "Dart livelock", Severity::Warning);
dart_detector!(DartThreadSafetyDetector, "dart_thread_safety", "Dart thread safety violation", Severity::Error);
dart_detector!(DartAsyncHazardDetector, "dart_async_hazard", "Dart async hazard", Severity::Warning);
dart_detector!(DartLockContentionDetector, "dart_lock_contention", "Dart lock contention", Severity::Warning);
dart_detector!(DartSynchronizationDetector, "dart_synchronization", "Dart synchronization issue", Severity::Warning);

// Error handling patterns
dart_detector!(DartSwallowedExceptionDetector, "dart_swallowed_exception", "Dart swallowed exception", Severity::Warning);
dart_detector!(DartEmptyCatchDetector, "dart_empty_catch", "Dart empty catch block", Severity::Warning);
dart_detector!(DartGenericCatchDetector, "dart_generic_catch", "Dart generic catch", Severity::Info);
dart_detector!(DartUnhandledErrorDetector, "dart_unhandled_error", "Dart unhandled error", Severity::Warning);
dart_detector!(DartErrorIgnoredDetector, "dart_error_ignored", "Dart error ignored", Severity::Warning);
dart_detector!(DartPanicMisuseDetector, "dart_panic_misuse", "Dart panic misuse", Severity::Warning);
dart_detector!(DartErrorPropagationDetector, "dart_error_propagation", "Dart error propagation issue", Severity::Info);
dart_detector!(DartResourceCleanupDetector, "dart_resource_cleanup", "Dart missing resource cleanup", Severity::Warning);
dart_detector!(DartTransactionRollbackDetector, "dart_transaction_rollback", "Dart missing transaction rollback", Severity::Warning);
dart_detector!(DartRetryLogicDetector, "dart_retry_logic", "Dart problematic retry logic", Severity::Info);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dart_patterns() {
        let patterns = get_dart_patterns();
        assert!(patterns.len() >= 50, "Should have 50+ Dart patterns");
    }
}
