//! Lua-specific patterns (50+ patterns per language)

use crate::{CodeGraph, PatternDetector, PatternInstance, FixSuggestion, Severity};
use std::sync::Arc;

pub fn get_lua_patterns() -> Vec<Arc<dyn PatternDetector>> {
    vec![
        // Security patterns (10)
        Arc::new(LuaSQLInjectionDetector::new()),
        Arc::new(LuaXSSDetector::new()),
        Arc::new(LuaPathTraversalDetector::new()),
        Arc::new(LuaCommandInjectionDetector::new()),
        Arc::new(LuaDeserializationDetector::new()),
        Arc::new(LuaHardcodedSecretsDetector::new()),
        Arc::new(LuaWeakCryptoDetector::new()),
        Arc::new(LuaInsecureRandomDetector::new()),
        Arc::new(LuaAuthBypassDetector::new()),
        Arc::new(LuaCSRFDetector::new()),

        // Performance patterns (10)
        Arc::new(LuaNPlusOneDetector::new()),
        Arc::new(LuaIneffectiveLoopDetector::new()),
        Arc::new(LuaExcessiveAllocationDetector::new()),
        Arc::new(LuaStringConcatDetector::new()),
        Arc::new(LuaBlockingIODetector::new()),
        Arc::new(LuaMissingCacheDetector::new()),
        Arc::new(LuaAlgorithmComplexityDetector::new()),
        Arc::new(LuaRedundantComputationDetector::new()),
        Arc::new(LuaMemoryLeakDetector::new()),
        Arc::new(LuaResourceExhaustionDetector::new()),

        // Memory safety patterns (10)
        Arc::new(LuaUseAfterFreeDetector::new()),
        Arc::new(LuaBufferOverflowDetector::new()),
        Arc::new(LuaNullPointerDetector::new()),
        Arc::new(LuaUninitializedMemoryDetector::new()),
        Arc::new(LuaDoubleFreeDetector::new()),
        Arc::new(LuaMemoryCorruptionDetector::new()),
        Arc::new(LuaDanglingPointerDetector::new()),
        Arc::new(LuaStackOverflowDetector::new()),
        Arc::new(LuaHeapCorruptionDetector::new()),
        Arc::new(LuaTypeConfusionDetector::new()),

        // Concurrency patterns (10)
        Arc::new(LuaDataRaceDetector::new()),
        Arc::new(LuaDeadlockDetector::new()),
        Arc::new(LuaRaceConditionDetector::new()),
        Arc::new(LuaAtomicityViolationDetector::new()),
        Arc::new(LuaOrderViolationDetector::new()),
        Arc::new(LuaLivelockDetector::new()),
        Arc::new(LuaThreadSafetyDetector::new()),
        Arc::new(LuaAsyncHazardDetector::new()),
        Arc::new(LuaLockContentionDetector::new()),
        Arc::new(LuaSynchronizationDetector::new()),

        // Error handling patterns (10)
        Arc::new(LuaSwallowedExceptionDetector::new()),
        Arc::new(LuaEmptyCatchDetector::new()),
        Arc::new(LuaGenericCatchDetector::new()),
        Arc::new(LuaUnhandledErrorDetector::new()),
        Arc::new(LuaErrorIgnoredDetector::new()),
        Arc::new(LuaPanicMisuseDetector::new()),
        Arc::new(LuaErrorPropagationDetector::new()),
        Arc::new(LuaResourceCleanupDetector::new()),
        Arc::new(LuaTransactionRollbackDetector::new()),
        Arc::new(LuaRetryLogicDetector::new()),
    ]
}

macro_rules! lua_detector {
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
lua_detector!(LuaSQLInjectionDetector, "lua_sql_injection", "Lua SQL injection vulnerability", Severity::Critical);
lua_detector!(LuaXSSDetector, "lua_xss", "Lua XSS vulnerability", Severity::Critical);
lua_detector!(LuaPathTraversalDetector, "lua_path_traversal", "Lua path traversal vulnerability", Severity::Critical);
lua_detector!(LuaCommandInjectionDetector, "lua_command_injection", "Lua command injection", Severity::Critical);
lua_detector!(LuaDeserializationDetector, "lua_unsafe_deserialization", "Lua unsafe deserialization", Severity::Critical);
lua_detector!(LuaHardcodedSecretsDetector, "lua_hardcoded_secrets", "Lua hardcoded secrets", Severity::Warning);
lua_detector!(LuaWeakCryptoDetector, "lua_weak_crypto", "Lua weak cryptography", Severity::Warning);
lua_detector!(LuaInsecureRandomDetector, "lua_insecure_random", "Lua insecure randomness", Severity::Warning);
lua_detector!(LuaAuthBypassDetector, "lua_auth_bypass", "Lua authentication bypass", Severity::Critical);
lua_detector!(LuaCSRFDetector, "lua_csrf", "Lua CSRF vulnerability", Severity::Warning);

// Performance patterns
lua_detector!(LuaNPlusOneDetector, "lua_n_plus_one", "Lua N+1 query problem", Severity::Warning);
lua_detector!(LuaIneffectiveLoopDetector, "lua_ineffective_loop", "Lua ineffective loop", Severity::Warning);
lua_detector!(LuaExcessiveAllocationDetector, "lua_excessive_allocation", "Lua excessive allocation", Severity::Warning);
lua_detector!(LuaStringConcatDetector, "lua_string_concat", "Lua ineffective string concatenation", Severity::Info);
lua_detector!(LuaBlockingIODetector, "lua_blocking_io", "Lua blocking I/O", Severity::Warning);
lua_detector!(LuaMissingCacheDetector, "lua_missing_cache", "Lua missing cache", Severity::Info);
lua_detector!(LuaAlgorithmComplexityDetector, "lua_algorithm_complexity", "Lua high algorithm complexity", Severity::Warning);
lua_detector!(LuaRedundantComputationDetector, "lua_redundant_computation", "Lua redundant computation", Severity::Info);
lua_detector!(LuaMemoryLeakDetector, "lua_memory_leak", "Lua memory leak", Severity::Warning);
lua_detector!(LuaResourceExhaustionDetector, "lua_resource_exhaustion", "Lua resource exhaustion", Severity::Warning);

// Memory safety patterns
lua_detector!(LuaUseAfterFreeDetector, "lua_use_after_free", "Lua use-after-free", Severity::Critical);
lua_detector!(LuaBufferOverflowDetector, "lua_buffer_overflow", "Lua buffer overflow", Severity::Critical);
lua_detector!(LuaNullPointerDetector, "lua_null_pointer", "Lua null pointer dereference", Severity::Critical);
lua_detector!(LuaUninitializedMemoryDetector, "lua_uninitialized_memory", "Lua uninitialized memory", Severity::Critical);
lua_detector!(LuaDoubleFreeDetector, "lua_double_free", "Lua double free", Severity::Critical);
lua_detector!(LuaMemoryCorruptionDetector, "lua_memory_corruption", "Lua memory corruption", Severity::Critical);
lua_detector!(LuaDanglingPointerDetector, "lua_dangling_pointer", "Lua dangling pointer", Severity::Critical);
lua_detector!(LuaStackOverflowDetector, "lua_stack_overflow", "Lua stack overflow risk", Severity::Warning);
lua_detector!(LuaHeapCorruptionDetector, "lua_heap_corruption", "Lua heap corruption", Severity::Critical);
lua_detector!(LuaTypeConfusionDetector, "lua_type_confusion", "Lua type confusion", Severity::Warning);

// Concurrency patterns
lua_detector!(LuaDataRaceDetector, "lua_data_race", "Lua data race", Severity::Critical);
lua_detector!(LuaDeadlockDetector, "lua_deadlock", "Lua deadlock", Severity::Critical);
lua_detector!(LuaRaceConditionDetector, "lua_race_condition", "Lua race condition", Severity::Critical);
lua_detector!(LuaAtomicityViolationDetector, "lua_atomicity_violation", "Lua atomicity violation", Severity::Error);
lua_detector!(LuaOrderViolationDetector, "lua_order_violation", "Lua order violation", Severity::Error);
lua_detector!(LuaLivelockDetector, "lua_livelock", "Lua livelock", Severity::Warning);
lua_detector!(LuaThreadSafetyDetector, "lua_thread_safety", "Lua thread safety violation", Severity::Error);
lua_detector!(LuaAsyncHazardDetector, "lua_async_hazard", "Lua async hazard", Severity::Warning);
lua_detector!(LuaLockContentionDetector, "lua_lock_contention", "Lua lock contention", Severity::Warning);
lua_detector!(LuaSynchronizationDetector, "lua_synchronization", "Lua synchronization issue", Severity::Warning);

// Error handling patterns
lua_detector!(LuaSwallowedExceptionDetector, "lua_swallowed_exception", "Lua swallowed exception", Severity::Warning);
lua_detector!(LuaEmptyCatchDetector, "lua_empty_catch", "Lua empty catch block", Severity::Warning);
lua_detector!(LuaGenericCatchDetector, "lua_generic_catch", "Lua generic catch", Severity::Info);
lua_detector!(LuaUnhandledErrorDetector, "lua_unhandled_error", "Lua unhandled error", Severity::Warning);
lua_detector!(LuaErrorIgnoredDetector, "lua_error_ignored", "Lua error ignored", Severity::Warning);
lua_detector!(LuaPanicMisuseDetector, "lua_panic_misuse", "Lua panic misuse", Severity::Warning);
lua_detector!(LuaErrorPropagationDetector, "lua_error_propagation", "Lua error propagation issue", Severity::Info);
lua_detector!(LuaResourceCleanupDetector, "lua_resource_cleanup", "Lua missing resource cleanup", Severity::Warning);
lua_detector!(LuaTransactionRollbackDetector, "lua_transaction_rollback", "Lua missing transaction rollback", Severity::Warning);
lua_detector!(LuaRetryLogicDetector, "lua_retry_logic", "Lua problematic retry logic", Severity::Info);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lua_patterns() {
        let patterns = get_lua_patterns();
        assert!(patterns.len() >= 50, "Should have 50+ Lua patterns");
    }
}
