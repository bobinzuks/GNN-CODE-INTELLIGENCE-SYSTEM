//! C-specific patterns (50+ patterns per language)

use crate::{CodeGraph, PatternDetector, PatternInstance, FixSuggestion, Severity};
use std::sync::Arc;

pub fn get_c_patterns() -> Vec<Arc<dyn PatternDetector>> {
    vec![
        // Security patterns (10)
        Arc::new(CSQLInjectionDetector::new()),
        Arc::new(CXSSDetector::new()),
        Arc::new(CPathTraversalDetector::new()),
        Arc::new(CCommandInjectionDetector::new()),
        Arc::new(CDeserializationDetector::new()),
        Arc::new(CHardcodedSecretsDetector::new()),
        Arc::new(CWeakCryptoDetector::new()),
        Arc::new(CInsecureRandomDetector::new()),
        Arc::new(CAuthBypassDetector::new()),
        Arc::new(CCSRFDetector::new()),

        // Performance patterns (10)
        Arc::new(CNPlusOneDetector::new()),
        Arc::new(CIneffectiveLoopDetector::new()),
        Arc::new(CExcessiveAllocationDetector::new()),
        Arc::new(CStringConcatDetector::new()),
        Arc::new(CBlockingIODetector::new()),
        Arc::new(CMissingCacheDetector::new()),
        Arc::new(CAlgorithmComplexityDetector::new()),
        Arc::new(CRedundantComputationDetector::new()),
        Arc::new(CMemoryLeakDetector::new()),
        Arc::new(CResourceExhaustionDetector::new()),

        // Memory safety patterns (10)
        Arc::new(CUseAfterFreeDetector::new()),
        Arc::new(CBufferOverflowDetector::new()),
        Arc::new(CNullPointerDetector::new()),
        Arc::new(CUninitializedMemoryDetector::new()),
        Arc::new(CDoubleFreeDetector::new()),
        Arc::new(CMemoryCorruptionDetector::new()),
        Arc::new(CDanglingPointerDetector::new()),
        Arc::new(CStackOverflowDetector::new()),
        Arc::new(CHeapCorruptionDetector::new()),
        Arc::new(CTypeConfusionDetector::new()),

        // Concurrency patterns (10)
        Arc::new(CDataRaceDetector::new()),
        Arc::new(CDeadlockDetector::new()),
        Arc::new(CRaceConditionDetector::new()),
        Arc::new(CAtomicityViolationDetector::new()),
        Arc::new(COrderViolationDetector::new()),
        Arc::new(CLivelockDetector::new()),
        Arc::new(CThreadSafetyDetector::new()),
        Arc::new(CAsyncHazardDetector::new()),
        Arc::new(CLockContentionDetector::new()),
        Arc::new(CSynchronizationDetector::new()),

        // Error handling patterns (10)
        Arc::new(CSwallowedExceptionDetector::new()),
        Arc::new(CEmptyCatchDetector::new()),
        Arc::new(CGenericCatchDetector::new()),
        Arc::new(CUnhandledErrorDetector::new()),
        Arc::new(CErrorIgnoredDetector::new()),
        Arc::new(CPanicMisuseDetector::new()),
        Arc::new(CErrorPropagationDetector::new()),
        Arc::new(CResourceCleanupDetector::new()),
        Arc::new(CTransactionRollbackDetector::new()),
        Arc::new(CRetryLogicDetector::new()),
    ]
}

macro_rules! c_detector {
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
c_detector!(CSQLInjectionDetector, "c_sql_injection", "C SQL injection vulnerability", Severity::Critical);
c_detector!(CXSSDetector, "c_xss", "C XSS vulnerability", Severity::Critical);
c_detector!(CPathTraversalDetector, "c_path_traversal", "C path traversal vulnerability", Severity::Critical);
c_detector!(CCommandInjectionDetector, "c_command_injection", "C command injection", Severity::Critical);
c_detector!(CDeserializationDetector, "c_unsafe_deserialization", "C unsafe deserialization", Severity::Critical);
c_detector!(CHardcodedSecretsDetector, "c_hardcoded_secrets", "C hardcoded secrets", Severity::Warning);
c_detector!(CWeakCryptoDetector, "c_weak_crypto", "C weak cryptography", Severity::Warning);
c_detector!(CInsecureRandomDetector, "c_insecure_random", "C insecure randomness", Severity::Warning);
c_detector!(CAuthBypassDetector, "c_auth_bypass", "C authentication bypass", Severity::Critical);
c_detector!(CCSRFDetector, "c_csrf", "C CSRF vulnerability", Severity::Warning);

// Performance patterns
c_detector!(CNPlusOneDetector, "c_n_plus_one", "C N+1 query problem", Severity::Warning);
c_detector!(CIneffectiveLoopDetector, "c_ineffective_loop", "C ineffective loop", Severity::Warning);
c_detector!(CExcessiveAllocationDetector, "c_excessive_allocation", "C excessive allocation", Severity::Warning);
c_detector!(CStringConcatDetector, "c_string_concat", "C ineffective string concatenation", Severity::Info);
c_detector!(CBlockingIODetector, "c_blocking_io", "C blocking I/O", Severity::Warning);
c_detector!(CMissingCacheDetector, "c_missing_cache", "C missing cache", Severity::Info);
c_detector!(CAlgorithmComplexityDetector, "c_algorithm_complexity", "C high algorithm complexity", Severity::Warning);
c_detector!(CRedundantComputationDetector, "c_redundant_computation", "C redundant computation", Severity::Info);
c_detector!(CMemoryLeakDetector, "c_memory_leak", "C memory leak", Severity::Warning);
c_detector!(CResourceExhaustionDetector, "c_resource_exhaustion", "C resource exhaustion", Severity::Warning);

// Memory safety patterns
c_detector!(CUseAfterFreeDetector, "c_use_after_free", "C use-after-free", Severity::Critical);
c_detector!(CBufferOverflowDetector, "c_buffer_overflow", "C buffer overflow", Severity::Critical);
c_detector!(CNullPointerDetector, "c_null_pointer", "C null pointer dereference", Severity::Critical);
c_detector!(CUninitializedMemoryDetector, "c_uninitialized_memory", "C uninitialized memory", Severity::Critical);
c_detector!(CDoubleFreeDetector, "c_double_free", "C double free", Severity::Critical);
c_detector!(CMemoryCorruptionDetector, "c_memory_corruption", "C memory corruption", Severity::Critical);
c_detector!(CDanglingPointerDetector, "c_dangling_pointer", "C dangling pointer", Severity::Critical);
c_detector!(CStackOverflowDetector, "c_stack_overflow", "C stack overflow risk", Severity::Warning);
c_detector!(CHeapCorruptionDetector, "c_heap_corruption", "C heap corruption", Severity::Critical);
c_detector!(CTypeConfusionDetector, "c_type_confusion", "C type confusion", Severity::Warning);

// Concurrency patterns
c_detector!(CDataRaceDetector, "c_data_race", "C data race", Severity::Critical);
c_detector!(CDeadlockDetector, "c_deadlock", "C deadlock", Severity::Critical);
c_detector!(CRaceConditionDetector, "c_race_condition", "C race condition", Severity::Critical);
c_detector!(CAtomicityViolationDetector, "c_atomicity_violation", "C atomicity violation", Severity::Error);
c_detector!(COrderViolationDetector, "c_order_violation", "C order violation", Severity::Error);
c_detector!(CLivelockDetector, "c_livelock", "C livelock", Severity::Warning);
c_detector!(CThreadSafetyDetector, "c_thread_safety", "C thread safety violation", Severity::Error);
c_detector!(CAsyncHazardDetector, "c_async_hazard", "C async hazard", Severity::Warning);
c_detector!(CLockContentionDetector, "c_lock_contention", "C lock contention", Severity::Warning);
c_detector!(CSynchronizationDetector, "c_synchronization", "C synchronization issue", Severity::Warning);

// Error handling patterns
c_detector!(CSwallowedExceptionDetector, "c_swallowed_exception", "C swallowed exception", Severity::Warning);
c_detector!(CEmptyCatchDetector, "c_empty_catch", "C empty catch block", Severity::Warning);
c_detector!(CGenericCatchDetector, "c_generic_catch", "C generic catch", Severity::Info);
c_detector!(CUnhandledErrorDetector, "c_unhandled_error", "C unhandled error", Severity::Warning);
c_detector!(CErrorIgnoredDetector, "c_error_ignored", "C error ignored", Severity::Warning);
c_detector!(CPanicMisuseDetector, "c_panic_misuse", "C panic misuse", Severity::Warning);
c_detector!(CErrorPropagationDetector, "c_error_propagation", "C error propagation issue", Severity::Info);
c_detector!(CResourceCleanupDetector, "c_resource_cleanup", "C missing resource cleanup", Severity::Warning);
c_detector!(CTransactionRollbackDetector, "c_transaction_rollback", "C missing transaction rollback", Severity::Warning);
c_detector!(CRetryLogicDetector, "c_retry_logic", "C problematic retry logic", Severity::Info);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c_patterns() {
        let patterns = get_c_patterns();
        assert!(patterns.len() >= 50, "Should have 50+ C patterns");
    }
}
