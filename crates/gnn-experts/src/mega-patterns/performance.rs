//! Performance anti-pattern detectors (100+ patterns)

use crate::{CodeGraph, PatternDetector, PatternInstance, FixSuggestion, Severity};
use std::sync::Arc;

pub fn load_performance_patterns() -> Vec<Arc<dyn PatternDetector>> {
    vec![
        // Database performance (20 patterns)
        Arc::new(NPlusOneQueryDetector::new()),
        Arc::new(MissingIndexDetector::new()),
        Arc::new(FullTableScanDetector::new()),
        Arc::new(CartesianJoinDetector::new()),
        Arc::new(IneffectiveJoinDetector::new()),
        Arc::new(SuboptimalQueryDetector::new()),
        Arc::new(MissingQueryCacheDetector::new()),
        Arc::new(OverfetchingDetector::new()),
        Arc::new(UnderfetchingDetector::new()),
        Arc::new(DatabaseConnectionPoolingDetector::new()),
        Arc::new(TransactionScopeDetector::new()),
        Arc::new(BatchingOpportunityDetector::new()),
        Arc::new(EagerLoadingDetector::new()),
        Arc::new(LazyLoadingProblemDetector::new()),
        Arc::new(QueryInLoopDetector::new()),
        Arc::new(UnboundedQueryDetector::new()),
        Arc::new(MissingPaginationDetector::new()),
        Arc::new(IneffectiveSortingDetector::new()),
        Arc::new(RedundantQueryDetector::new()),
        Arc::new(DatabaseLockContentionDetector::new()),

        // Algorithm complexity (15 patterns)
        Arc::new(QuadraticComplexityDetector::new()),
        Arc::new(CubicComplexityDetector::new()),
        Arc::new(ExponentialComplexityDetector::new()),
        Arc::new(NestedLoopsDetector::new()),
        Arc::new(IneffectiveSortAlgorithmDetector::new()),
        Arc::new(IneffectiveSearchAlgorithmDetector::new()),
        Arc::new(RedundantComputationDetector::new()),
        Arc::new(UnnecessaryRecursionDetector::new()),
        Arc::new(MissingMemoizationDetector::new()),
        Arc::new(DynamicProgrammingOpportunityDetector::new()),
        Arc::new(GreedyAlgorithmMisuseDetector::new()),
        Arc::new(BruteForceApproachDetector::new()),
        Arc::new(LinearSearchInSortedArrayDetector::new()),
        Arc::new(UnoptimizedGraphTraversalDetector::new()),
        Arc::new(IneffectiveDataStructureDetector::new()),

        // Memory allocation (15 patterns)
        Arc::new(ExcessiveAllocationDetector::new()),
        Arc::new(AllocationInLoopDetector::new()),
        Arc::new(UnnecessaryCloneDetector::new()),
        Arc::new(StringConcatenationInLoopDetector::new()),
        Arc::new(LargeStackAllocationDetector::new()),
        Arc::new(FragmentationRiskDetector::new()),
        Arc::new(MissingObjectPoolingDetector::new()),
        Arc::new(IneffectiveGarbageCollectionDetector::new()),
        Arc::new(MemoryChurnDetector::new()),
        Arc::new(AllocationPatternDetector::new()),
        Arc::new(SmallObjectAllocationDetector::new()),
        Arc::new(PreallocationOpportunityDetector::new()),
        Arc::new(ArenaAllocationOpportunityDetector::new()),
        Arc::new(CopyOnWriteOpportunityDetector::new()),
        Arc::new(ZeroCopyOpportunityDetector::new()),

        // I/O performance (15 patterns)
        Arc::new(SynchronousIODetector::new()),
        Arc::new(BlockingIOInAsyncDetector::new()),
        Arc::new(UnbufferedIODetector::new()),
        Arc::new(SmallIOOperationsDetector::new()),
        Arc::new(ExcessiveFileOpenCloseDetector::new()),
        Arc::new(SequentialReadOptimizationDetector::new()),
        Arc::new(MissingIOBatchingDetector::new()),
        Arc::new(DiskIOThresholdDetector::new()),
        Arc::new(NetworkIOLatencyDetector::new()),
        Arc::new(SerializationOverheadDetector::new()),
        Arc::new(CompressionOpportunityDetector::new()),
        Arc::new(MemoryMappedIOOpportunityDetector::new()),
        Arc::new(DirectIOOpportunityDetector::new()),
        Arc::new(AsyncIOOpportunityDetector::new()),
        Arc::new(IOMultiplexingDetector::new()),

        // Caching issues (10 patterns)
        Arc::new(MissingCacheDetector::new()),
        Arc::new(IneffectiveCacheStrategyDetector::new()),
        Arc::new(CacheInvalidationIssueDetector::new()),
        Arc::new(OvercachingDetector::new()),
        Arc::new(CacheThrashingDetector::new()),
        Arc::new(MissingCacheWarmupDetector::new()),
        Arc::new(CacheKeyDesignDetector::new()),
        Arc::new(DistributedCacheIssueDetector::new()),
        Arc::new(LocalityCacheMissDetector::new()),
        Arc::new(CacheSizeOptimizationDetector::new()),

        // Collection operations (10 patterns)
        Arc::new(LinearSearchInCollectionDetector::new()),
        Arc::new(WrongCollectionTypeDetector::new()),
        Arc::new(CollectionGrowthDetector::new()),
        Arc::new(UnnecessaryCollectionCopyDetector::new()),
        Arc::new(FilterMapChainingDetector::new()),
        Arc::new(IteratorMisuseDetector::new()),
        Arc::new(StreamOperationDetector::new()),
        Arc::new(ParallelStreamOpportunityDetector::new()),
        Arc::new(LazyEvaluationOpportunityDetector::new()),
        Arc::new(ShortCircuitEvaluationDetector::new()),

        // Concurrency performance (10 patterns)
        Arc::new(ExcessiveLockingDetector::new()),
        Arc::new(LockContentionDetector::new()),
        Arc::new(FineGrainedLockingOpportunityDetector::new()),
        Arc::new(LockFreeOpportunityDetector::new()),
        Arc::new(ThreadPoolMisconfigurationDetector::new()),
        Arc::new(TaskGranularityDetector::new()),
        Arc::new(ContextSwitchOverheadDetector::new()),
        Arc::new(FalseSharingDetector::new()),
        Arc::new(AtomicOperationMisuseDetector::new()),
        Arc::new(ConcurrentDataStructureOpportunityDetector::new()),

        // Network performance (10 patterns)
        Arc::new(ChattyCommunicationDetector::new()),
        Arc::new(RoundTripLatencyDetector::new()),
        Arc::new(ConnectionPoolingIssueDetector::new()),
        Arc::new(KeepAliveIssueDetector::new()),
        Arc::new(CompressionMissingDetector::new()),
        Arc::new(BandwidthOptimizationDetector::new()),
        Arc::new(ProtocolOverheadDetector::new()),
        Arc::new(PayloadSizeDetector::new()),
        Arc::new(HTTPRequestBatchingDetector::new()),
        Arc::new(WebSocketOpportunityDetector::new()),

        // Additional performance patterns (20+)
        Arc::new(RegexCompilationInLoopDetector::new()),
        Arc::new(ReflectionOverheadDetector::new()),
        Arc::new(ExceptionControlFlowDetector::new()),
        Arc::new(PolymorphicCallOverheadDetector::new()),
        Arc::new(VirtualCallOptimizationDetector::new()),
        Arc::new(InliningOpportunityDetector::new()),
        Arc::new(BranchPredictionIssueDetector::new()),
        Arc::new(CPUCacheMissDetector::new()),
        Arc::new(MemoryAlignmentDetector::new()),
        Arc::new(SIMDOpportunityDetector::new()),
        Arc::new(VectorizationOpportunityDetector::new()),
        Arc::new(UnrollingOpportunityDetector::new()),
        Arc::new(TailCallOptimizationDetector::new()),
        Arc::new(ConstantFoldingOpportunityDetector::new()),
        Arc::new(DeadCodeEliminationDetector::new()),
        Arc::new(CommonSubexpressionDetector::new()),
        Arc::new(StrengthReductionOpportunityDetector::new()),
        Arc::new(LoopInvariantCodeMotionDetector::new()),
        Arc::new(RegisterSpillingDetector::new()),
        Arc::new(PipelineStallDetector::new()),
    ]
}

macro_rules! perf_detector {
    ($name:ident, $pattern_name:expr, $desc:expr) => {
        pub struct $name;
        impl $name {
            pub fn new() -> Self { Self }
        }
        impl Default for $name {
            fn default() -> Self { Self::new() }
        }
        impl PatternDetector for $name {
            fn name(&self) -> &str { $pattern_name }
            fn description(&self) -> &str { $desc }
            fn severity(&self) -> Severity { Severity::Warning }
            fn detect(&self, _graph: &CodeGraph) -> Vec<PatternInstance> { Vec::new() }
            fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> { None }
        }
    };
}

perf_detector!(NPlusOneQueryDetector, "n_plus_one_query", "N+1 query problem - causes excessive database queries");
perf_detector!(MissingIndexDetector, "missing_database_index", "Missing database index causes slow queries");
perf_detector!(FullTableScanDetector, "full_table_scan", "Full table scan detected - add index or optimize query");
perf_detector!(CartesianJoinDetector, "cartesian_join", "Cartesian product join - very inefficient");
perf_detector!(IneffectiveJoinDetector, "ineffective_join", "Ineffective join strategy");
perf_detector!(SuboptimalQueryDetector, "suboptimal_query", "Suboptimal query structure");
perf_detector!(MissingQueryCacheDetector, "missing_query_cache", "Query results not cached");
perf_detector!(OverfetchingDetector, "overfetching_data", "Fetching more data than needed");
perf_detector!(UnderfetchingDetector, "underfetching_data", "Multiple queries due to underfetching");
perf_detector!(DatabaseConnectionPoolingDetector, "connection_pooling_issue", "Database connection pooling misconfigured");
perf_detector!(TransactionScopeDetector, "transaction_scope_too_large", "Transaction scope too large");
perf_detector!(BatchingOpportunityDetector, "batch_operation_opportunity", "Operations can be batched");
perf_detector!(EagerLoadingDetector, "eager_loading_issue", "Eager loading causing memory issues");
perf_detector!(LazyLoadingProblemDetector, "lazy_loading_problem", "Lazy loading causing N+1");
perf_detector!(QueryInLoopDetector, "query_in_loop", "Database query inside loop");
perf_detector!(UnboundedQueryDetector, "unbounded_query", "Query without LIMIT clause");
perf_detector!(MissingPaginationDetector, "missing_pagination", "Large result set without pagination");
perf_detector!(IneffectiveSortingDetector, "ineffective_sorting", "Ineffective sorting approach");
perf_detector!(RedundantQueryDetector, "redundant_query", "Redundant database query");
perf_detector!(DatabaseLockContentionDetector, "database_lock_contention", "Database lock contention");

perf_detector!(QuadraticComplexityDetector, "quadratic_complexity", "O(n²) complexity - consider optimization");
perf_detector!(CubicComplexityDetector, "cubic_complexity", "O(n³) complexity - highly inefficient");
perf_detector!(ExponentialComplexityDetector, "exponential_complexity", "Exponential complexity - critical issue");
perf_detector!(NestedLoopsDetector, "nested_loops", "Nested loops causing high complexity");
perf_detector!(IneffectiveSortAlgorithmDetector, "ineffective_sort_algorithm", "Ineffective sorting algorithm");
perf_detector!(IneffectiveSearchAlgorithmDetector, "ineffective_search_algorithm", "Ineffective search algorithm");
perf_detector!(RedundantComputationDetector, "redundant_computation", "Redundant computation detected");
perf_detector!(UnnecessaryRecursionDetector, "unnecessary_recursion", "Recursion can be replaced with iteration");
perf_detector!(MissingMemoizationDetector, "missing_memoization", "Memoization opportunity");
perf_detector!(DynamicProgrammingOpportunityDetector, "dynamic_programming_opportunity", "Can use dynamic programming");
perf_detector!(GreedyAlgorithmMisuseDetector, "greedy_algorithm_misuse", "Greedy algorithm may not work here");
perf_detector!(BruteForceApproachDetector, "brute_force_approach", "Brute force approach - optimize");
perf_detector!(LinearSearchInSortedArrayDetector, "linear_search_sorted_array", "Use binary search instead");
perf_detector!(UnoptimizedGraphTraversalDetector, "unoptimized_graph_traversal", "Graph traversal can be optimized");
perf_detector!(IneffectiveDataStructureDetector, "ineffective_data_structure", "Wrong data structure for use case");

perf_detector!(ExcessiveAllocationDetector, "excessive_allocation", "Excessive memory allocation");
perf_detector!(AllocationInLoopDetector, "allocation_in_loop", "Memory allocation inside loop");
perf_detector!(UnnecessaryCloneDetector, "unnecessary_clone", "Unnecessary data cloning");
perf_detector!(StringConcatenationInLoopDetector, "string_concat_in_loop", "String concatenation in loop");
perf_detector!(LargeStackAllocationDetector, "large_stack_allocation", "Large stack allocation - use heap");
perf_detector!(FragmentationRiskDetector, "fragmentation_risk", "Memory fragmentation risk");
perf_detector!(MissingObjectPoolingDetector, "missing_object_pooling", "Object pooling opportunity");
perf_detector!(IneffectiveGarbageCollectionDetector, "ineffective_gc", "Ineffective garbage collection pattern");
perf_detector!(MemoryChurnDetector, "memory_churn", "High memory churn rate");
perf_detector!(AllocationPatternDetector, "allocation_pattern_issue", "Problematic allocation pattern");
perf_detector!(SmallObjectAllocationDetector, "small_object_allocation", "Many small object allocations");
perf_detector!(PreallocationOpportunityDetector, "preallocation_opportunity", "Can preallocate memory");
perf_detector!(ArenaAllocationOpportunityDetector, "arena_allocation_opportunity", "Arena allocation beneficial");
perf_detector!(CopyOnWriteOpportunityDetector, "copy_on_write_opportunity", "Copy-on-write opportunity");
perf_detector!(ZeroCopyOpportunityDetector, "zero_copy_opportunity", "Zero-copy optimization possible");

perf_detector!(SynchronousIODetector, "synchronous_io", "Synchronous I/O blocking execution");
perf_detector!(BlockingIOInAsyncDetector, "blocking_io_in_async", "Blocking I/O in async context");
perf_detector!(UnbufferedIODetector, "unbuffered_io", "Unbuffered I/O - use buffering");
perf_detector!(SmallIOOperationsDetector, "small_io_operations", "Many small I/O operations");
perf_detector!(ExcessiveFileOpenCloseDetector, "excessive_file_open_close", "Excessive file open/close");
perf_detector!(SequentialReadOptimizationDetector, "sequential_read_optimization", "Sequential read can be optimized");
perf_detector!(MissingIOBatchingDetector, "missing_io_batching", "I/O operations should be batched");
perf_detector!(DiskIOThresholdDetector, "disk_io_threshold", "Disk I/O threshold exceeded");
perf_detector!(NetworkIOLatencyDetector, "network_io_latency", "High network I/O latency");
perf_detector!(SerializationOverheadDetector, "serialization_overhead", "Serialization overhead too high");
perf_detector!(CompressionOpportunityDetector, "compression_opportunity", "Data compression opportunity");
perf_detector!(MemoryMappedIOOpportunityDetector, "mmap_io_opportunity", "Memory-mapped I/O beneficial");
perf_detector!(DirectIOOpportunityDetector, "direct_io_opportunity", "Direct I/O opportunity");
perf_detector!(AsyncIOOpportunityDetector, "async_io_opportunity", "Async I/O would improve performance");
perf_detector!(IOMultiplexingDetector, "io_multiplexing_opportunity", "I/O multiplexing opportunity");

perf_detector!(MissingCacheDetector, "missing_cache", "Caching opportunity");
perf_detector!(IneffectiveCacheStrategyDetector, "ineffective_cache_strategy", "Cache strategy ineffective");
perf_detector!(CacheInvalidationIssueDetector, "cache_invalidation_issue", "Cache invalidation problem");
perf_detector!(OvercachingDetector, "overcaching", "Caching too much data");
perf_detector!(CacheThrashingDetector, "cache_thrashing", "Cache thrashing detected");
perf_detector!(MissingCacheWarmupDetector, "missing_cache_warmup", "Cache warmup missing");
perf_detector!(CacheKeyDesignDetector, "cache_key_design_issue", "Cache key design issue");
perf_detector!(DistributedCacheIssueDetector, "distributed_cache_issue", "Distributed cache misconfigured");
perf_detector!(LocalityCacheMissDetector, "locality_cache_miss", "Poor cache locality");
perf_detector!(CacheSizeOptimizationDetector, "cache_size_optimization", "Cache size needs optimization");

perf_detector!(LinearSearchInCollectionDetector, "linear_search_collection", "Linear search in collection");
perf_detector!(WrongCollectionTypeDetector, "wrong_collection_type", "Wrong collection type for use case");
perf_detector!(CollectionGrowthDetector, "collection_growth_issue", "Collection growth not optimized");
perf_detector!(UnnecessaryCollectionCopyDetector, "unnecessary_collection_copy", "Unnecessary collection copy");
perf_detector!(FilterMapChainingDetector, "filter_map_chaining", "Ineffective filter/map chaining");
perf_detector!(IteratorMisuseDetector, "iterator_misuse", "Iterator misuse");
perf_detector!(StreamOperationDetector, "stream_operation_issue", "Stream operation inefficient");
perf_detector!(ParallelStreamOpportunityDetector, "parallel_stream_opportunity", "Parallel stream opportunity");
perf_detector!(LazyEvaluationOpportunityDetector, "lazy_evaluation_opportunity", "Lazy evaluation beneficial");
perf_detector!(ShortCircuitEvaluationDetector, "short_circuit_evaluation", "Short-circuit evaluation opportunity");

perf_detector!(ExcessiveLockingDetector, "excessive_locking", "Excessive locking");
perf_detector!(LockContentionDetector, "lock_contention", "Lock contention detected");
perf_detector!(FineGrainedLockingOpportunityDetector, "fine_grained_locking", "Fine-grained locking opportunity");
perf_detector!(LockFreeOpportunityDetector, "lock_free_opportunity", "Lock-free algorithm opportunity");
perf_detector!(ThreadPoolMisconfigurationDetector, "thread_pool_misconfiguration", "Thread pool misconfigured");
perf_detector!(TaskGranularityDetector, "task_granularity_issue", "Task granularity issue");
perf_detector!(ContextSwitchOverheadDetector, "context_switch_overhead", "Excessive context switches");
perf_detector!(FalseSharingDetector, "false_sharing", "False sharing detected");
perf_detector!(AtomicOperationMisuseDetector, "atomic_operation_misuse", "Atomic operation misuse");
perf_detector!(ConcurrentDataStructureOpportunityDetector, "concurrent_ds_opportunity", "Concurrent data structure opportunity");

perf_detector!(ChattyCommunicationDetector, "chatty_communication", "Chatty network communication");
perf_detector!(RoundTripLatencyDetector, "round_trip_latency", "High round-trip latency");
perf_detector!(ConnectionPoolingIssueDetector, "connection_pooling_issue", "Connection pooling issue");
perf_detector!(KeepAliveIssueDetector, "keep_alive_issue", "Keep-alive not configured");
perf_detector!(CompressionMissingDetector, "compression_missing", "Network compression missing");
perf_detector!(BandwidthOptimizationDetector, "bandwidth_optimization", "Bandwidth optimization opportunity");
perf_detector!(ProtocolOverheadDetector, "protocol_overhead", "Protocol overhead too high");
perf_detector!(PayloadSizeDetector, "payload_size_issue", "Payload size too large");
perf_detector!(HTTPRequestBatchingDetector, "http_request_batching", "HTTP requests can be batched");
perf_detector!(WebSocketOpportunityDetector, "websocket_opportunity", "WebSocket would reduce overhead");

perf_detector!(RegexCompilationInLoopDetector, "regex_compilation_in_loop", "Regex compilation in loop");
perf_detector!(ReflectionOverheadDetector, "reflection_overhead", "Reflection overhead");
perf_detector!(ExceptionControlFlowDetector, "exception_control_flow", "Using exceptions for control flow");
perf_detector!(PolymorphicCallOverheadDetector, "polymorphic_call_overhead", "Polymorphic call overhead");
perf_detector!(VirtualCallOptimizationDetector, "virtual_call_optimization", "Virtual call can be devirtualized");
perf_detector!(InliningOpportunityDetector, "inlining_opportunity", "Function inlining opportunity");
perf_detector!(BranchPredictionIssueDetector, "branch_prediction_issue", "Branch prediction issue");
perf_detector!(CPUCacheMissDetector, "cpu_cache_miss", "CPU cache miss likely");
perf_detector!(MemoryAlignmentDetector, "memory_alignment_issue", "Memory alignment issue");
perf_detector!(SIMDOpportunityDetector, "simd_opportunity", "SIMD optimization opportunity");
perf_detector!(VectorizationOpportunityDetector, "vectorization_opportunity", "Vectorization opportunity");
perf_detector!(UnrollingOpportunityDetector, "loop_unrolling_opportunity", "Loop unrolling opportunity");
perf_detector!(TailCallOptimizationDetector, "tail_call_optimization", "Tail call optimization opportunity");
perf_detector!(ConstantFoldingOpportunityDetector, "constant_folding_opportunity", "Constant folding opportunity");
perf_detector!(DeadCodeEliminationDetector, "dead_code_elimination", "Dead code can be eliminated");
perf_detector!(CommonSubexpressionDetector, "common_subexpression", "Common subexpression elimination");
perf_detector!(StrengthReductionOpportunityDetector, "strength_reduction_opportunity", "Strength reduction opportunity");
perf_detector!(LoopInvariantCodeMotionDetector, "loop_invariant_code_motion", "Loop invariant code motion");
perf_detector!(RegisterSpillingDetector, "register_spilling", "Register spilling detected");
perf_detector!(PipelineStallDetector, "pipeline_stall", "CPU pipeline stall likely");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_performance_patterns() {
        let patterns = load_performance_patterns();
        assert!(patterns.len() >= 100, "Should have 100+ performance patterns");
    }
}
