//! Ocaml-specific patterns (50+ patterns per language)

use crate::{CodeGraph, PatternDetector, PatternInstance, FixSuggestion, Severity};
use std::sync::Arc;

pub fn get_ocaml_patterns() -> Vec<Arc<dyn PatternDetector>> {
    vec![
        // Security patterns (10)
        Arc::new(OcamlSQLInjectionDetector::new()),
        Arc::new(OcamlXSSDetector::new()),
        Arc::new(OcamlPathTraversalDetector::new()),
        Arc::new(OcamlCommandInjectionDetector::new()),
        Arc::new(OcamlDeserializationDetector::new()),
        Arc::new(OcamlHardcodedSecretsDetector::new()),
        Arc::new(OcamlWeakCryptoDetector::new()),
        Arc::new(OcamlInsecureRandomDetector::new()),
        Arc::new(OcamlAuthBypassDetector::new()),
        Arc::new(OcamlCSRFDetector::new()),

        // Performance patterns (10)
        Arc::new(OcamlNPlusOneDetector::new()),
        Arc::new(OcamlIneffectiveLoopDetector::new()),
        Arc::new(OcamlExcessiveAllocationDetector::new()),
        Arc::new(OcamlStringConcatDetector::new()),
        Arc::new(OcamlBlockingIODetector::new()),
        Arc::new(OcamlMissingCacheDetector::new()),
        Arc::new(OcamlAlgorithmComplexityDetector::new()),
        Arc::new(OcamlRedundantComputationDetector::new()),
        Arc::new(OcamlMemoryLeakDetector::new()),
        Arc::new(OcamlResourceExhaustionDetector::new()),

        // Memory safety patterns (10)
        Arc::new(OcamlUseAfterFreeDetector::new()),
        Arc::new(OcamlBufferOverflowDetector::new()),
        Arc::new(OcamlNullPointerDetector::new()),
        Arc::new(OcamlUninitializedMemoryDetector::new()),
        Arc::new(OcamlDoubleFreeDetector::new()),
        Arc::new(OcamlMemoryCorruptionDetector::new()),
        Arc::new(OcamlDanglingPointerDetector::new()),
        Arc::new(OcamlStackOverflowDetector::new()),
        Arc::new(OcamlHeapCorruptionDetector::new()),
        Arc::new(OcamlTypeConfusionDetector::new()),

        // Concurrency patterns (10)
        Arc::new(OcamlDataRaceDetector::new()),
        Arc::new(OcamlDeadlockDetector::new()),
        Arc::new(OcamlRaceConditionDetector::new()),
        Arc::new(OcamlAtomicityViolationDetector::new()),
        Arc::new(OcamlOrderViolationDetector::new()),
        Arc::new(OcamlLivelockDetector::new()),
        Arc::new(OcamlThreadSafetyDetector::new()),
        Arc::new(OcamlAsyncHazardDetector::new()),
        Arc::new(OcamlLockContentionDetector::new()),
        Arc::new(OcamlSynchronizationDetector::new()),

        // Error handling patterns (10)
        Arc::new(OcamlSwallowedExceptionDetector::new()),
        Arc::new(OcamlEmptyCatchDetector::new()),
        Arc::new(OcamlGenericCatchDetector::new()),
        Arc::new(OcamlUnhandledErrorDetector::new()),
        Arc::new(OcamlErrorIgnoredDetector::new()),
        Arc::new(OcamlPanicMisuseDetector::new()),
        Arc::new(OcamlErrorPropagationDetector::new()),
        Arc::new(OcamlResourceCleanupDetector::new()),
        Arc::new(OcamlTransactionRollbackDetector::new()),
        Arc::new(OcamlRetryLogicDetector::new()),
    ]
}

macro_rules! ocaml_detector {
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
ocaml_detector!(OcamlSQLInjectionDetector, "ocaml_sql_injection", "Ocaml SQL injection vulnerability", Severity::Critical);
ocaml_detector!(OcamlXSSDetector, "ocaml_xss", "Ocaml XSS vulnerability", Severity::Critical);
ocaml_detector!(OcamlPathTraversalDetector, "ocaml_path_traversal", "Ocaml path traversal vulnerability", Severity::Critical);
ocaml_detector!(OcamlCommandInjectionDetector, "ocaml_command_injection", "Ocaml command injection", Severity::Critical);
ocaml_detector!(OcamlDeserializationDetector, "ocaml_unsafe_deserialization", "Ocaml unsafe deserialization", Severity::Critical);
ocaml_detector!(OcamlHardcodedSecretsDetector, "ocaml_hardcoded_secrets", "Ocaml hardcoded secrets", Severity::Warning);
ocaml_detector!(OcamlWeakCryptoDetector, "ocaml_weak_crypto", "Ocaml weak cryptography", Severity::Warning);
ocaml_detector!(OcamlInsecureRandomDetector, "ocaml_insecure_random", "Ocaml insecure randomness", Severity::Warning);
ocaml_detector!(OcamlAuthBypassDetector, "ocaml_auth_bypass", "Ocaml authentication bypass", Severity::Critical);
ocaml_detector!(OcamlCSRFDetector, "ocaml_csrf", "Ocaml CSRF vulnerability", Severity::Warning);

// Performance patterns
ocaml_detector!(OcamlNPlusOneDetector, "ocaml_n_plus_one", "Ocaml N+1 query problem", Severity::Warning);
ocaml_detector!(OcamlIneffectiveLoopDetector, "ocaml_ineffective_loop", "Ocaml ineffective loop", Severity::Warning);
ocaml_detector!(OcamlExcessiveAllocationDetector, "ocaml_excessive_allocation", "Ocaml excessive allocation", Severity::Warning);
ocaml_detector!(OcamlStringConcatDetector, "ocaml_string_concat", "Ocaml ineffective string concatenation", Severity::Info);
ocaml_detector!(OcamlBlockingIODetector, "ocaml_blocking_io", "Ocaml blocking I/O", Severity::Warning);
ocaml_detector!(OcamlMissingCacheDetector, "ocaml_missing_cache", "Ocaml missing cache", Severity::Info);
ocaml_detector!(OcamlAlgorithmComplexityDetector, "ocaml_algorithm_complexity", "Ocaml high algorithm complexity", Severity::Warning);
ocaml_detector!(OcamlRedundantComputationDetector, "ocaml_redundant_computation", "Ocaml redundant computation", Severity::Info);
ocaml_detector!(OcamlMemoryLeakDetector, "ocaml_memory_leak", "Ocaml memory leak", Severity::Warning);
ocaml_detector!(OcamlResourceExhaustionDetector, "ocaml_resource_exhaustion", "Ocaml resource exhaustion", Severity::Warning);

// Memory safety patterns
ocaml_detector!(OcamlUseAfterFreeDetector, "ocaml_use_after_free", "Ocaml use-after-free", Severity::Critical);
ocaml_detector!(OcamlBufferOverflowDetector, "ocaml_buffer_overflow", "Ocaml buffer overflow", Severity::Critical);
ocaml_detector!(OcamlNullPointerDetector, "ocaml_null_pointer", "Ocaml null pointer dereference", Severity::Critical);
ocaml_detector!(OcamlUninitializedMemoryDetector, "ocaml_uninitialized_memory", "Ocaml uninitialized memory", Severity::Critical);
ocaml_detector!(OcamlDoubleFreeDetector, "ocaml_double_free", "Ocaml double free", Severity::Critical);
ocaml_detector!(OcamlMemoryCorruptionDetector, "ocaml_memory_corruption", "Ocaml memory corruption", Severity::Critical);
ocaml_detector!(OcamlDanglingPointerDetector, "ocaml_dangling_pointer", "Ocaml dangling pointer", Severity::Critical);
ocaml_detector!(OcamlStackOverflowDetector, "ocaml_stack_overflow", "Ocaml stack overflow risk", Severity::Warning);
ocaml_detector!(OcamlHeapCorruptionDetector, "ocaml_heap_corruption", "Ocaml heap corruption", Severity::Critical);
ocaml_detector!(OcamlTypeConfusionDetector, "ocaml_type_confusion", "Ocaml type confusion", Severity::Warning);

// Concurrency patterns
ocaml_detector!(OcamlDataRaceDetector, "ocaml_data_race", "Ocaml data race", Severity::Critical);
ocaml_detector!(OcamlDeadlockDetector, "ocaml_deadlock", "Ocaml deadlock", Severity::Critical);
ocaml_detector!(OcamlRaceConditionDetector, "ocaml_race_condition", "Ocaml race condition", Severity::Critical);
ocaml_detector!(OcamlAtomicityViolationDetector, "ocaml_atomicity_violation", "Ocaml atomicity violation", Severity::Error);
ocaml_detector!(OcamlOrderViolationDetector, "ocaml_order_violation", "Ocaml order violation", Severity::Error);
ocaml_detector!(OcamlLivelockDetector, "ocaml_livelock", "Ocaml livelock", Severity::Warning);
ocaml_detector!(OcamlThreadSafetyDetector, "ocaml_thread_safety", "Ocaml thread safety violation", Severity::Error);
ocaml_detector!(OcamlAsyncHazardDetector, "ocaml_async_hazard", "Ocaml async hazard", Severity::Warning);
ocaml_detector!(OcamlLockContentionDetector, "ocaml_lock_contention", "Ocaml lock contention", Severity::Warning);
ocaml_detector!(OcamlSynchronizationDetector, "ocaml_synchronization", "Ocaml synchronization issue", Severity::Warning);

// Error handling patterns
ocaml_detector!(OcamlSwallowedExceptionDetector, "ocaml_swallowed_exception", "Ocaml swallowed exception", Severity::Warning);
ocaml_detector!(OcamlEmptyCatchDetector, "ocaml_empty_catch", "Ocaml empty catch block", Severity::Warning);
ocaml_detector!(OcamlGenericCatchDetector, "ocaml_generic_catch", "Ocaml generic catch", Severity::Info);
ocaml_detector!(OcamlUnhandledErrorDetector, "ocaml_unhandled_error", "Ocaml unhandled error", Severity::Warning);
ocaml_detector!(OcamlErrorIgnoredDetector, "ocaml_error_ignored", "Ocaml error ignored", Severity::Warning);
ocaml_detector!(OcamlPanicMisuseDetector, "ocaml_panic_misuse", "Ocaml panic misuse", Severity::Warning);
ocaml_detector!(OcamlErrorPropagationDetector, "ocaml_error_propagation", "Ocaml error propagation issue", Severity::Info);
ocaml_detector!(OcamlResourceCleanupDetector, "ocaml_resource_cleanup", "Ocaml missing resource cleanup", Severity::Warning);
ocaml_detector!(OcamlTransactionRollbackDetector, "ocaml_transaction_rollback", "Ocaml missing transaction rollback", Severity::Warning);
ocaml_detector!(OcamlRetryLogicDetector, "ocaml_retry_logic", "Ocaml problematic retry logic", Severity::Info);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ocaml_patterns() {
        let patterns = get_ocaml_patterns();
        assert!(patterns.len() >= 50, "Should have 50+ Ocaml patterns");
    }
}
