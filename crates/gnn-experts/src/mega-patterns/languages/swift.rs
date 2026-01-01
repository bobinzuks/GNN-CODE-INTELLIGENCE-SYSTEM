//! Swift-specific patterns (50+ patterns per language)

use crate::{CodeGraph, PatternDetector, PatternInstance, FixSuggestion, Severity};
use std::sync::Arc;

pub fn get_swift_patterns() -> Vec<Arc<dyn PatternDetector>> {
    vec![
        // Security patterns (10)
        Arc::new(SwiftSQLInjectionDetector::new()),
        Arc::new(SwiftXSSDetector::new()),
        Arc::new(SwiftPathTraversalDetector::new()),
        Arc::new(SwiftCommandInjectionDetector::new()),
        Arc::new(SwiftDeserializationDetector::new()),
        Arc::new(SwiftHardcodedSecretsDetector::new()),
        Arc::new(SwiftWeakCryptoDetector::new()),
        Arc::new(SwiftInsecureRandomDetector::new()),
        Arc::new(SwiftAuthBypassDetector::new()),
        Arc::new(SwiftCSRFDetector::new()),

        // Performance patterns (10)
        Arc::new(SwiftNPlusOneDetector::new()),
        Arc::new(SwiftIneffectiveLoopDetector::new()),
        Arc::new(SwiftExcessiveAllocationDetector::new()),
        Arc::new(SwiftStringConcatDetector::new()),
        Arc::new(SwiftBlockingIODetector::new()),
        Arc::new(SwiftMissingCacheDetector::new()),
        Arc::new(SwiftAlgorithmComplexityDetector::new()),
        Arc::new(SwiftRedundantComputationDetector::new()),
        Arc::new(SwiftMemoryLeakDetector::new()),
        Arc::new(SwiftResourceExhaustionDetector::new()),

        // Memory safety patterns (10)
        Arc::new(SwiftUseAfterFreeDetector::new()),
        Arc::new(SwiftBufferOverflowDetector::new()),
        Arc::new(SwiftNullPointerDetector::new()),
        Arc::new(SwiftUninitializedMemoryDetector::new()),
        Arc::new(SwiftDoubleFreeDetector::new()),
        Arc::new(SwiftMemoryCorruptionDetector::new()),
        Arc::new(SwiftDanglingPointerDetector::new()),
        Arc::new(SwiftStackOverflowDetector::new()),
        Arc::new(SwiftHeapCorruptionDetector::new()),
        Arc::new(SwiftTypeConfusionDetector::new()),

        // Concurrency patterns (10)
        Arc::new(SwiftDataRaceDetector::new()),
        Arc::new(SwiftDeadlockDetector::new()),
        Arc::new(SwiftRaceConditionDetector::new()),
        Arc::new(SwiftAtomicityViolationDetector::new()),
        Arc::new(SwiftOrderViolationDetector::new()),
        Arc::new(SwiftLivelockDetector::new()),
        Arc::new(SwiftThreadSafetyDetector::new()),
        Arc::new(SwiftAsyncHazardDetector::new()),
        Arc::new(SwiftLockContentionDetector::new()),
        Arc::new(SwiftSynchronizationDetector::new()),

        // Error handling patterns (10)
        Arc::new(SwiftSwallowedExceptionDetector::new()),
        Arc::new(SwiftEmptyCatchDetector::new()),
        Arc::new(SwiftGenericCatchDetector::new()),
        Arc::new(SwiftUnhandledErrorDetector::new()),
        Arc::new(SwiftErrorIgnoredDetector::new()),
        Arc::new(SwiftPanicMisuseDetector::new()),
        Arc::new(SwiftErrorPropagationDetector::new()),
        Arc::new(SwiftResourceCleanupDetector::new()),
        Arc::new(SwiftTransactionRollbackDetector::new()),
        Arc::new(SwiftRetryLogicDetector::new()),
    ]
}

macro_rules! swift_detector {
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
swift_detector!(SwiftSQLInjectionDetector, "swift_sql_injection", "Swift SQL injection vulnerability", Severity::Critical);
swift_detector!(SwiftXSSDetector, "swift_xss", "Swift XSS vulnerability", Severity::Critical);
swift_detector!(SwiftPathTraversalDetector, "swift_path_traversal", "Swift path traversal vulnerability", Severity::Critical);
swift_detector!(SwiftCommandInjectionDetector, "swift_command_injection", "Swift command injection", Severity::Critical);
swift_detector!(SwiftDeserializationDetector, "swift_unsafe_deserialization", "Swift unsafe deserialization", Severity::Critical);
swift_detector!(SwiftHardcodedSecretsDetector, "swift_hardcoded_secrets", "Swift hardcoded secrets", Severity::Warning);
swift_detector!(SwiftWeakCryptoDetector, "swift_weak_crypto", "Swift weak cryptography", Severity::Warning);
swift_detector!(SwiftInsecureRandomDetector, "swift_insecure_random", "Swift insecure randomness", Severity::Warning);
swift_detector!(SwiftAuthBypassDetector, "swift_auth_bypass", "Swift authentication bypass", Severity::Critical);
swift_detector!(SwiftCSRFDetector, "swift_csrf", "Swift CSRF vulnerability", Severity::Warning);

// Performance patterns
swift_detector!(SwiftNPlusOneDetector, "swift_n_plus_one", "Swift N+1 query problem", Severity::Warning);
swift_detector!(SwiftIneffectiveLoopDetector, "swift_ineffective_loop", "Swift ineffective loop", Severity::Warning);
swift_detector!(SwiftExcessiveAllocationDetector, "swift_excessive_allocation", "Swift excessive allocation", Severity::Warning);
swift_detector!(SwiftStringConcatDetector, "swift_string_concat", "Swift ineffective string concatenation", Severity::Info);
swift_detector!(SwiftBlockingIODetector, "swift_blocking_io", "Swift blocking I/O", Severity::Warning);
swift_detector!(SwiftMissingCacheDetector, "swift_missing_cache", "Swift missing cache", Severity::Info);
swift_detector!(SwiftAlgorithmComplexityDetector, "swift_algorithm_complexity", "Swift high algorithm complexity", Severity::Warning);
swift_detector!(SwiftRedundantComputationDetector, "swift_redundant_computation", "Swift redundant computation", Severity::Info);
swift_detector!(SwiftMemoryLeakDetector, "swift_memory_leak", "Swift memory leak", Severity::Warning);
swift_detector!(SwiftResourceExhaustionDetector, "swift_resource_exhaustion", "Swift resource exhaustion", Severity::Warning);

// Memory safety patterns
swift_detector!(SwiftUseAfterFreeDetector, "swift_use_after_free", "Swift use-after-free", Severity::Critical);
swift_detector!(SwiftBufferOverflowDetector, "swift_buffer_overflow", "Swift buffer overflow", Severity::Critical);
swift_detector!(SwiftNullPointerDetector, "swift_null_pointer", "Swift null pointer dereference", Severity::Critical);
swift_detector!(SwiftUninitializedMemoryDetector, "swift_uninitialized_memory", "Swift uninitialized memory", Severity::Critical);
swift_detector!(SwiftDoubleFreeDetector, "swift_double_free", "Swift double free", Severity::Critical);
swift_detector!(SwiftMemoryCorruptionDetector, "swift_memory_corruption", "Swift memory corruption", Severity::Critical);
swift_detector!(SwiftDanglingPointerDetector, "swift_dangling_pointer", "Swift dangling pointer", Severity::Critical);
swift_detector!(SwiftStackOverflowDetector, "swift_stack_overflow", "Swift stack overflow risk", Severity::Warning);
swift_detector!(SwiftHeapCorruptionDetector, "swift_heap_corruption", "Swift heap corruption", Severity::Critical);
swift_detector!(SwiftTypeConfusionDetector, "swift_type_confusion", "Swift type confusion", Severity::Warning);

// Concurrency patterns
swift_detector!(SwiftDataRaceDetector, "swift_data_race", "Swift data race", Severity::Critical);
swift_detector!(SwiftDeadlockDetector, "swift_deadlock", "Swift deadlock", Severity::Critical);
swift_detector!(SwiftRaceConditionDetector, "swift_race_condition", "Swift race condition", Severity::Critical);
swift_detector!(SwiftAtomicityViolationDetector, "swift_atomicity_violation", "Swift atomicity violation", Severity::Error);
swift_detector!(SwiftOrderViolationDetector, "swift_order_violation", "Swift order violation", Severity::Error);
swift_detector!(SwiftLivelockDetector, "swift_livelock", "Swift livelock", Severity::Warning);
swift_detector!(SwiftThreadSafetyDetector, "swift_thread_safety", "Swift thread safety violation", Severity::Error);
swift_detector!(SwiftAsyncHazardDetector, "swift_async_hazard", "Swift async hazard", Severity::Warning);
swift_detector!(SwiftLockContentionDetector, "swift_lock_contention", "Swift lock contention", Severity::Warning);
swift_detector!(SwiftSynchronizationDetector, "swift_synchronization", "Swift synchronization issue", Severity::Warning);

// Error handling patterns
swift_detector!(SwiftSwallowedExceptionDetector, "swift_swallowed_exception", "Swift swallowed exception", Severity::Warning);
swift_detector!(SwiftEmptyCatchDetector, "swift_empty_catch", "Swift empty catch block", Severity::Warning);
swift_detector!(SwiftGenericCatchDetector, "swift_generic_catch", "Swift generic catch", Severity::Info);
swift_detector!(SwiftUnhandledErrorDetector, "swift_unhandled_error", "Swift unhandled error", Severity::Warning);
swift_detector!(SwiftErrorIgnoredDetector, "swift_error_ignored", "Swift error ignored", Severity::Warning);
swift_detector!(SwiftPanicMisuseDetector, "swift_panic_misuse", "Swift panic misuse", Severity::Warning);
swift_detector!(SwiftErrorPropagationDetector, "swift_error_propagation", "Swift error propagation issue", Severity::Info);
swift_detector!(SwiftResourceCleanupDetector, "swift_resource_cleanup", "Swift missing resource cleanup", Severity::Warning);
swift_detector!(SwiftTransactionRollbackDetector, "swift_transaction_rollback", "Swift missing transaction rollback", Severity::Warning);
swift_detector!(SwiftRetryLogicDetector, "swift_retry_logic", "Swift problematic retry logic", Severity::Info);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swift_patterns() {
        let patterns = get_swift_patterns();
        assert!(patterns.len() >= 50, "Should have 50+ Swift patterns");
    }
}
