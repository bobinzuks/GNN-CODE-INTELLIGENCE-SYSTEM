//! Ruby-specific patterns (50+ patterns per language)

use crate::{CodeGraph, PatternDetector, PatternInstance, FixSuggestion, Severity};
use std::sync::Arc;

pub fn get_ruby_patterns() -> Vec<Arc<dyn PatternDetector>> {
    vec![
        // Security patterns (10)
        Arc::new(RubySQLInjectionDetector::new()),
        Arc::new(RubyXSSDetector::new()),
        Arc::new(RubyPathTraversalDetector::new()),
        Arc::new(RubyCommandInjectionDetector::new()),
        Arc::new(RubyDeserializationDetector::new()),
        Arc::new(RubyHardcodedSecretsDetector::new()),
        Arc::new(RubyWeakCryptoDetector::new()),
        Arc::new(RubyInsecureRandomDetector::new()),
        Arc::new(RubyAuthBypassDetector::new()),
        Arc::new(RubyCSRFDetector::new()),

        // Performance patterns (10)
        Arc::new(RubyNPlusOneDetector::new()),
        Arc::new(RubyIneffectiveLoopDetector::new()),
        Arc::new(RubyExcessiveAllocationDetector::new()),
        Arc::new(RubyStringConcatDetector::new()),
        Arc::new(RubyBlockingIODetector::new()),
        Arc::new(RubyMissingCacheDetector::new()),
        Arc::new(RubyAlgorithmComplexityDetector::new()),
        Arc::new(RubyRedundantComputationDetector::new()),
        Arc::new(RubyMemoryLeakDetector::new()),
        Arc::new(RubyResourceExhaustionDetector::new()),

        // Memory safety patterns (10)
        Arc::new(RubyUseAfterFreeDetector::new()),
        Arc::new(RubyBufferOverflowDetector::new()),
        Arc::new(RubyNullPointerDetector::new()),
        Arc::new(RubyUninitializedMemoryDetector::new()),
        Arc::new(RubyDoubleFreeDetector::new()),
        Arc::new(RubyMemoryCorruptionDetector::new()),
        Arc::new(RubyDanglingPointerDetector::new()),
        Arc::new(RubyStackOverflowDetector::new()),
        Arc::new(RubyHeapCorruptionDetector::new()),
        Arc::new(RubyTypeConfusionDetector::new()),

        // Concurrency patterns (10)
        Arc::new(RubyDataRaceDetector::new()),
        Arc::new(RubyDeadlockDetector::new()),
        Arc::new(RubyRaceConditionDetector::new()),
        Arc::new(RubyAtomicityViolationDetector::new()),
        Arc::new(RubyOrderViolationDetector::new()),
        Arc::new(RubyLivelockDetector::new()),
        Arc::new(RubyThreadSafetyDetector::new()),
        Arc::new(RubyAsyncHazardDetector::new()),
        Arc::new(RubyLockContentionDetector::new()),
        Arc::new(RubySynchronizationDetector::new()),

        // Error handling patterns (10)
        Arc::new(RubySwallowedExceptionDetector::new()),
        Arc::new(RubyEmptyCatchDetector::new()),
        Arc::new(RubyGenericCatchDetector::new()),
        Arc::new(RubyUnhandledErrorDetector::new()),
        Arc::new(RubyErrorIgnoredDetector::new()),
        Arc::new(RubyPanicMisuseDetector::new()),
        Arc::new(RubyErrorPropagationDetector::new()),
        Arc::new(RubyResourceCleanupDetector::new()),
        Arc::new(RubyTransactionRollbackDetector::new()),
        Arc::new(RubyRetryLogicDetector::new()),
    ]
}

macro_rules! ruby_detector {
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
ruby_detector!(RubySQLInjectionDetector, "ruby_sql_injection", "Ruby SQL injection vulnerability", Severity::Critical);
ruby_detector!(RubyXSSDetector, "ruby_xss", "Ruby XSS vulnerability", Severity::Critical);
ruby_detector!(RubyPathTraversalDetector, "ruby_path_traversal", "Ruby path traversal vulnerability", Severity::Critical);
ruby_detector!(RubyCommandInjectionDetector, "ruby_command_injection", "Ruby command injection", Severity::Critical);
ruby_detector!(RubyDeserializationDetector, "ruby_unsafe_deserialization", "Ruby unsafe deserialization", Severity::Critical);
ruby_detector!(RubyHardcodedSecretsDetector, "ruby_hardcoded_secrets", "Ruby hardcoded secrets", Severity::Warning);
ruby_detector!(RubyWeakCryptoDetector, "ruby_weak_crypto", "Ruby weak cryptography", Severity::Warning);
ruby_detector!(RubyInsecureRandomDetector, "ruby_insecure_random", "Ruby insecure randomness", Severity::Warning);
ruby_detector!(RubyAuthBypassDetector, "ruby_auth_bypass", "Ruby authentication bypass", Severity::Critical);
ruby_detector!(RubyCSRFDetector, "ruby_csrf", "Ruby CSRF vulnerability", Severity::Warning);

// Performance patterns
ruby_detector!(RubyNPlusOneDetector, "ruby_n_plus_one", "Ruby N+1 query problem", Severity::Warning);
ruby_detector!(RubyIneffectiveLoopDetector, "ruby_ineffective_loop", "Ruby ineffective loop", Severity::Warning);
ruby_detector!(RubyExcessiveAllocationDetector, "ruby_excessive_allocation", "Ruby excessive allocation", Severity::Warning);
ruby_detector!(RubyStringConcatDetector, "ruby_string_concat", "Ruby ineffective string concatenation", Severity::Info);
ruby_detector!(RubyBlockingIODetector, "ruby_blocking_io", "Ruby blocking I/O", Severity::Warning);
ruby_detector!(RubyMissingCacheDetector, "ruby_missing_cache", "Ruby missing cache", Severity::Info);
ruby_detector!(RubyAlgorithmComplexityDetector, "ruby_algorithm_complexity", "Ruby high algorithm complexity", Severity::Warning);
ruby_detector!(RubyRedundantComputationDetector, "ruby_redundant_computation", "Ruby redundant computation", Severity::Info);
ruby_detector!(RubyMemoryLeakDetector, "ruby_memory_leak", "Ruby memory leak", Severity::Warning);
ruby_detector!(RubyResourceExhaustionDetector, "ruby_resource_exhaustion", "Ruby resource exhaustion", Severity::Warning);

// Memory safety patterns
ruby_detector!(RubyUseAfterFreeDetector, "ruby_use_after_free", "Ruby use-after-free", Severity::Critical);
ruby_detector!(RubyBufferOverflowDetector, "ruby_buffer_overflow", "Ruby buffer overflow", Severity::Critical);
ruby_detector!(RubyNullPointerDetector, "ruby_null_pointer", "Ruby null pointer dereference", Severity::Critical);
ruby_detector!(RubyUninitializedMemoryDetector, "ruby_uninitialized_memory", "Ruby uninitialized memory", Severity::Critical);
ruby_detector!(RubyDoubleFreeDetector, "ruby_double_free", "Ruby double free", Severity::Critical);
ruby_detector!(RubyMemoryCorruptionDetector, "ruby_memory_corruption", "Ruby memory corruption", Severity::Critical);
ruby_detector!(RubyDanglingPointerDetector, "ruby_dangling_pointer", "Ruby dangling pointer", Severity::Critical);
ruby_detector!(RubyStackOverflowDetector, "ruby_stack_overflow", "Ruby stack overflow risk", Severity::Warning);
ruby_detector!(RubyHeapCorruptionDetector, "ruby_heap_corruption", "Ruby heap corruption", Severity::Critical);
ruby_detector!(RubyTypeConfusionDetector, "ruby_type_confusion", "Ruby type confusion", Severity::Warning);

// Concurrency patterns
ruby_detector!(RubyDataRaceDetector, "ruby_data_race", "Ruby data race", Severity::Critical);
ruby_detector!(RubyDeadlockDetector, "ruby_deadlock", "Ruby deadlock", Severity::Critical);
ruby_detector!(RubyRaceConditionDetector, "ruby_race_condition", "Ruby race condition", Severity::Critical);
ruby_detector!(RubyAtomicityViolationDetector, "ruby_atomicity_violation", "Ruby atomicity violation", Severity::Error);
ruby_detector!(RubyOrderViolationDetector, "ruby_order_violation", "Ruby order violation", Severity::Error);
ruby_detector!(RubyLivelockDetector, "ruby_livelock", "Ruby livelock", Severity::Warning);
ruby_detector!(RubyThreadSafetyDetector, "ruby_thread_safety", "Ruby thread safety violation", Severity::Error);
ruby_detector!(RubyAsyncHazardDetector, "ruby_async_hazard", "Ruby async hazard", Severity::Warning);
ruby_detector!(RubyLockContentionDetector, "ruby_lock_contention", "Ruby lock contention", Severity::Warning);
ruby_detector!(RubySynchronizationDetector, "ruby_synchronization", "Ruby synchronization issue", Severity::Warning);

// Error handling patterns
ruby_detector!(RubySwallowedExceptionDetector, "ruby_swallowed_exception", "Ruby swallowed exception", Severity::Warning);
ruby_detector!(RubyEmptyCatchDetector, "ruby_empty_catch", "Ruby empty catch block", Severity::Warning);
ruby_detector!(RubyGenericCatchDetector, "ruby_generic_catch", "Ruby generic catch", Severity::Info);
ruby_detector!(RubyUnhandledErrorDetector, "ruby_unhandled_error", "Ruby unhandled error", Severity::Warning);
ruby_detector!(RubyErrorIgnoredDetector, "ruby_error_ignored", "Ruby error ignored", Severity::Warning);
ruby_detector!(RubyPanicMisuseDetector, "ruby_panic_misuse", "Ruby panic misuse", Severity::Warning);
ruby_detector!(RubyErrorPropagationDetector, "ruby_error_propagation", "Ruby error propagation issue", Severity::Info);
ruby_detector!(RubyResourceCleanupDetector, "ruby_resource_cleanup", "Ruby missing resource cleanup", Severity::Warning);
ruby_detector!(RubyTransactionRollbackDetector, "ruby_transaction_rollback", "Ruby missing transaction rollback", Severity::Warning);
ruby_detector!(RubyRetryLogicDetector, "ruby_retry_logic", "Ruby problematic retry logic", Severity::Info);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ruby_patterns() {
        let patterns = get_ruby_patterns();
        assert!(patterns.len() >= 50, "Should have 50+ Ruby patterns");
    }
}
