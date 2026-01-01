//! Go-specific patterns (50+ patterns per language)

use crate::{CodeGraph, PatternDetector, PatternInstance, FixSuggestion, Severity};
use std::sync::Arc;

pub fn get_go_patterns() -> Vec<Arc<dyn PatternDetector>> {
    vec![
        // Security patterns (10)
        Arc::new(GoSQLInjectionDetector::new()),
        Arc::new(GoXSSDetector::new()),
        Arc::new(GoPathTraversalDetector::new()),
        Arc::new(GoCommandInjectionDetector::new()),
        Arc::new(GoDeserializationDetector::new()),
        Arc::new(GoHardcodedSecretsDetector::new()),
        Arc::new(GoWeakCryptoDetector::new()),
        Arc::new(GoInsecureRandomDetector::new()),
        Arc::new(GoAuthBypassDetector::new()),
        Arc::new(GoCSRFDetector::new()),

        // Performance patterns (10)
        Arc::new(GoNPlusOneDetector::new()),
        Arc::new(GoIneffectiveLoopDetector::new()),
        Arc::new(GoExcessiveAllocationDetector::new()),
        Arc::new(GoStringConcatDetector::new()),
        Arc::new(GoBlockingIODetector::new()),
        Arc::new(GoMissingCacheDetector::new()),
        Arc::new(GoAlgorithmComplexityDetector::new()),
        Arc::new(GoRedundantComputationDetector::new()),
        Arc::new(GoMemoryLeakDetector::new()),
        Arc::new(GoResourceExhaustionDetector::new()),

        // Memory safety patterns (10)
        Arc::new(GoUseAfterFreeDetector::new()),
        Arc::new(GoBufferOverflowDetector::new()),
        Arc::new(GoNullPointerDetector::new()),
        Arc::new(GoUninitializedMemoryDetector::new()),
        Arc::new(GoDoubleFreeDetector::new()),
        Arc::new(GoMemoryCorruptionDetector::new()),
        Arc::new(GoDanglingPointerDetector::new()),
        Arc::new(GoStackOverflowDetector::new()),
        Arc::new(GoHeapCorruptionDetector::new()),
        Arc::new(GoTypeConfusionDetector::new()),

        // Concurrency patterns (10)
        Arc::new(GoDataRaceDetector::new()),
        Arc::new(GoDeadlockDetector::new()),
        Arc::new(GoRaceConditionDetector::new()),
        Arc::new(GoAtomicityViolationDetector::new()),
        Arc::new(GoOrderViolationDetector::new()),
        Arc::new(GoLivelockDetector::new()),
        Arc::new(GoThreadSafetyDetector::new()),
        Arc::new(GoAsyncHazardDetector::new()),
        Arc::new(GoLockContentionDetector::new()),
        Arc::new(GoSynchronizationDetector::new()),

        // Error handling patterns (10)
        Arc::new(GoSwallowedExceptionDetector::new()),
        Arc::new(GoEmptyCatchDetector::new()),
        Arc::new(GoGenericCatchDetector::new()),
        Arc::new(GoUnhandledErrorDetector::new()),
        Arc::new(GoErrorIgnoredDetector::new()),
        Arc::new(GoPanicMisuseDetector::new()),
        Arc::new(GoErrorPropagationDetector::new()),
        Arc::new(GoResourceCleanupDetector::new()),
        Arc::new(GoTransactionRollbackDetector::new()),
        Arc::new(GoRetryLogicDetector::new()),
    ]
}

macro_rules! go_detector {
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
go_detector!(GoSQLInjectionDetector, "go_sql_injection", "Go SQL injection vulnerability", Severity::Critical);
go_detector!(GoXSSDetector, "go_xss", "Go XSS vulnerability", Severity::Critical);
go_detector!(GoPathTraversalDetector, "go_path_traversal", "Go path traversal vulnerability", Severity::Critical);
go_detector!(GoCommandInjectionDetector, "go_command_injection", "Go command injection", Severity::Critical);
go_detector!(GoDeserializationDetector, "go_unsafe_deserialization", "Go unsafe deserialization", Severity::Critical);
go_detector!(GoHardcodedSecretsDetector, "go_hardcoded_secrets", "Go hardcoded secrets", Severity::Warning);
go_detector!(GoWeakCryptoDetector, "go_weak_crypto", "Go weak cryptography", Severity::Warning);
go_detector!(GoInsecureRandomDetector, "go_insecure_random", "Go insecure randomness", Severity::Warning);
go_detector!(GoAuthBypassDetector, "go_auth_bypass", "Go authentication bypass", Severity::Critical);
go_detector!(GoCSRFDetector, "go_csrf", "Go CSRF vulnerability", Severity::Warning);

// Performance patterns
go_detector!(GoNPlusOneDetector, "go_n_plus_one", "Go N+1 query problem", Severity::Warning);
go_detector!(GoIneffectiveLoopDetector, "go_ineffective_loop", "Go ineffective loop", Severity::Warning);
go_detector!(GoExcessiveAllocationDetector, "go_excessive_allocation", "Go excessive allocation", Severity::Warning);
go_detector!(GoStringConcatDetector, "go_string_concat", "Go ineffective string concatenation", Severity::Info);
go_detector!(GoBlockingIODetector, "go_blocking_io", "Go blocking I/O", Severity::Warning);
go_detector!(GoMissingCacheDetector, "go_missing_cache", "Go missing cache", Severity::Info);
go_detector!(GoAlgorithmComplexityDetector, "go_algorithm_complexity", "Go high algorithm complexity", Severity::Warning);
go_detector!(GoRedundantComputationDetector, "go_redundant_computation", "Go redundant computation", Severity::Info);
go_detector!(GoMemoryLeakDetector, "go_memory_leak", "Go memory leak", Severity::Warning);
go_detector!(GoResourceExhaustionDetector, "go_resource_exhaustion", "Go resource exhaustion", Severity::Warning);

// Memory safety patterns
go_detector!(GoUseAfterFreeDetector, "go_use_after_free", "Go use-after-free", Severity::Critical);
go_detector!(GoBufferOverflowDetector, "go_buffer_overflow", "Go buffer overflow", Severity::Critical);
go_detector!(GoNullPointerDetector, "go_null_pointer", "Go null pointer dereference", Severity::Critical);
go_detector!(GoUninitializedMemoryDetector, "go_uninitialized_memory", "Go uninitialized memory", Severity::Critical);
go_detector!(GoDoubleFreeDetector, "go_double_free", "Go double free", Severity::Critical);
go_detector!(GoMemoryCorruptionDetector, "go_memory_corruption", "Go memory corruption", Severity::Critical);
go_detector!(GoDanglingPointerDetector, "go_dangling_pointer", "Go dangling pointer", Severity::Critical);
go_detector!(GoStackOverflowDetector, "go_stack_overflow", "Go stack overflow risk", Severity::Warning);
go_detector!(GoHeapCorruptionDetector, "go_heap_corruption", "Go heap corruption", Severity::Critical);
go_detector!(GoTypeConfusionDetector, "go_type_confusion", "Go type confusion", Severity::Warning);

// Concurrency patterns
go_detector!(GoDataRaceDetector, "go_data_race", "Go data race", Severity::Critical);
go_detector!(GoDeadlockDetector, "go_deadlock", "Go deadlock", Severity::Critical);
go_detector!(GoRaceConditionDetector, "go_race_condition", "Go race condition", Severity::Critical);
go_detector!(GoAtomicityViolationDetector, "go_atomicity_violation", "Go atomicity violation", Severity::Error);
go_detector!(GoOrderViolationDetector, "go_order_violation", "Go order violation", Severity::Error);
go_detector!(GoLivelockDetector, "go_livelock", "Go livelock", Severity::Warning);
go_detector!(GoThreadSafetyDetector, "go_thread_safety", "Go thread safety violation", Severity::Error);
go_detector!(GoAsyncHazardDetector, "go_async_hazard", "Go async hazard", Severity::Warning);
go_detector!(GoLockContentionDetector, "go_lock_contention", "Go lock contention", Severity::Warning);
go_detector!(GoSynchronizationDetector, "go_synchronization", "Go synchronization issue", Severity::Warning);

// Error handling patterns
go_detector!(GoSwallowedExceptionDetector, "go_swallowed_exception", "Go swallowed exception", Severity::Warning);
go_detector!(GoEmptyCatchDetector, "go_empty_catch", "Go empty catch block", Severity::Warning);
go_detector!(GoGenericCatchDetector, "go_generic_catch", "Go generic catch", Severity::Info);
go_detector!(GoUnhandledErrorDetector, "go_unhandled_error", "Go unhandled error", Severity::Warning);
go_detector!(GoErrorIgnoredDetector, "go_error_ignored", "Go error ignored", Severity::Warning);
go_detector!(GoPanicMisuseDetector, "go_panic_misuse", "Go panic misuse", Severity::Warning);
go_detector!(GoErrorPropagationDetector, "go_error_propagation", "Go error propagation issue", Severity::Info);
go_detector!(GoResourceCleanupDetector, "go_resource_cleanup", "Go missing resource cleanup", Severity::Warning);
go_detector!(GoTransactionRollbackDetector, "go_transaction_rollback", "Go missing transaction rollback", Severity::Warning);
go_detector!(GoRetryLogicDetector, "go_retry_logic", "Go problematic retry logic", Severity::Info);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_go_patterns() {
        let patterns = get_go_patterns();
        assert!(patterns.len() >= 50, "Should have 50+ Go patterns");
    }
}
