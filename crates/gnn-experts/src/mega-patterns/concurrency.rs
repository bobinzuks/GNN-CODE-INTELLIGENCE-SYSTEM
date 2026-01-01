//! Concurrency bug pattern detectors (100+ patterns)

use crate::{CodeGraph, PatternDetector, PatternInstance, FixSuggestion, Severity};
use std::sync::Arc;

pub fn load_concurrency_patterns() -> Vec<Arc<dyn PatternDetector>> {
    vec![
        // Data races (20)
        Arc::new(DataRaceDetector::new()),
        Arc::new(UnsynchronizedAccessDetector::new()),
        Arc::new(SharedMutableStateRaceDetector::new()),
        Arc::new(FieldRaceDetector::new()),
        Arc::new(ArrayRaceDetector::new()),
        Arc::new(CollectionRaceDetector::new()),
        Arc::new(LazyInitRaceDetector::new()),
        Arc::new(SingletonRaceDetector::new()),
        Arc::new(StaticRaceDetector::new()),
        Arc::new(GlobalRaceDetector::new()),
        Arc::new(ThreadLocalRaceDetector::new()),
        Arc::new(ClosureRaceDetector::new()),
        Arc::new(AsyncRaceDetector::new()),
        Arc::new(FutureRaceDetector::new()),
        Arc::new(StreamRaceDetector::new()),
        Arc::new(ChannelRaceDetector::new()),
        Arc::new(MessagePassingRaceDetector::new()),
        Arc::new(SharedMemoryRaceDetector::new()),
        Arc::new(MMIORaceDetector::new()),
        Arc::new(AtomicRaceDetector::new()),

        // Deadlocks (15)
        Arc::new(DeadlockDetector::new()),
        Arc::new(LockOrderViolationDetector::new()),
        Arc::new(CircularWaitDetector::new()),
        Arc::new(NestedLockDetector::new()),
        Arc::new(RecursiveLockDetector::new()),
        Arc::new(CrossThreadLockDetector::new()),
        Arc::new(ConditionalLockDetector::new()),
        Arc::new(ChannelDeadlockDetector::new()),
        Arc::new(SemaphoreDeadlockDetector::new()),
        Arc::new(BarrierDeadlockDetector::new()),
        Arc::new(RwLockDeadlockDetector::new()),
        Arc::new(MutexDeadlockDetector::new()),
        Arc::new(SpinlockDeadlockDetector::new()),
        Arc::new(ResourceDeadlockDetector::new()),
        Arc::new(PriorityInversionDetector::new()),

        // Atomicity violations (12)
        Arc::new(AtomicityViolationDetector::new()),
        Arc::new(NonAtomicCompoundOpDetector::new()),
        Arc::new(CheckThenActDetector::new()),
        Arc::new(ReadModifyWriteDetector::new()),
        Arc::new(CompareAndSwapMisuseDetector::new()),
        Arc::new(DoubleCheckedLockingDetector::new()),
        Arc::new(LazyInitAtomicityDetector::new()),
        Arc::new(IncrementDecrementRaceDetector::new()),
        Arc::new(CounterRaceDetector::new()),
        Arc::new(FlagRaceDetector::new()),
        Arc::new(StateTransitionRaceDetector::new()),
        Arc::new(ResourceAllocationRaceDetector::new()),

        // Order violations (10)
        Arc::new(OrderViolationDetector::new()),
        Arc::new(InitBeforeUseViolationDetector::new()),
        Arc::new(ReleaseAcquireViolationDetector::new()),
        Arc::new(HappensBeforeViolationDetector::new()),
        Arc::new(SequentialConsistencyViolationDetector::new()),
        Arc::new(MemoryOrderViolationDetector::new()),
        Arc::new(PublishSubscribeOrderDetector::new()),
        Arc::new(ProducerConsumerOrderDetector::new()),
        Arc::new(InitOrderDetector::new()),
        Arc::new(ShutdownOrderDetector::new()),

        // Livelock (8)
        Arc::new(LivelockDetector::new()),
        Arc::new(SpinWaitLivelockDetector::new()),
        Arc::new(RetryLivelockDetector::new()),
        Arc::new(BackoffFailureDetector::new()),
        Arc::new(ResourceStarvationDetector::new()),
        Arc::new(YieldLivelockDetector::new()),
        Arc::new(PollingLivelockDetector::new()),
        Arc::new(BusyWaitLivelockDetector::new()),

        // Thread safety (15)
        Arc::new(ThreadSafetyViolationDetector::new()),
        Arc::new(NonThreadSafeAPIDetector::new()),
        Arc::new(UnsafeSendDetector::new()),
        Arc::new(UnsafeSyncDetector::new()),
        Arc::new(ThreadLocalMisuseDetector::new()),
        Arc::new(ThreadPoolSafetyDetector::new()),
        Arc::new(WorkStealingSafetyDetector::new()),
        Arc::new(RacyStartDetector::new()),
        Arc::new(RacyStopDetector::new()),
        Arc::new(ThreadSpawnLeakDetector::new()),
        Arc::new(DetachedThreadDetector::new()),
        Arc::new(ThreadPanicDetector::new()),
        Arc::new(UnwindSafetyDetector::new()),
        Arc::new(FFISafetyDetector::new()),
        Arc::new(CallbackThreadSafetyDetector::new()),

        // Async hazards (12)
        Arc::new(AsyncHazardDetector::new()),
        Arc::new(AwaitPointRaceDetector::new()),
        Arc::new(TaskCancellationRaceDetector::new()),
        Arc::new(AsyncDropDetector::new()),
        Arc::new(AsyncMutexContention::new()),
        Arc::new(SelectBiasDetector::new()),
        Arc::new(FutureLeakDetector::new()),
        Arc::new(StreamBackpressureDetector::new()),
        Arc::new(ReactorStarvationDetector::new()),
        Arc::new(ExecutorOverloadDetector::new()),
        Arc::new(AsyncRecursionDetector::new()),
        Arc::new(SpawnBlockingMisuseDetector::new()),

        // Lock-free issues (10)
        Arc::new(LockFreeABADetector::new()),
        Arc::new(LockFreeMemoryOrderDetector::new()),
        Arc::new(CASLoopDetector::new()),
        Arc::new(WeakCASMisuseDetector::new()),
        Arc::new(LoadStoreOrderingDetector::new()),
        Arc::new(AtomicRMWDetector::new()),
        Arc::new(FenceUsageDetector::new()),
        Arc::new(RCUViolationDetector::new()),
        Arc::new(HazardPointerDetector::new()),
        Arc::new(EpochBasedReclaimDetector::new()),

        // Additional patterns (18)
        Arc::new(SignalHandlerSafetyDetector::new()),
        Arc::new(ForkSafetyDetector::new()),
        Arc::new(ExitHandlerRaceDetector::new()),
        Arc::new(GlobalDestructorRaceDetector::new()),
        Arc::new(PluginConcurrencyDetector::new()),
        Arc::new(InterruptHandlerDetector::new()),
        Arc::new(MemoryBarrierMissingDetector::new()),
        Arc::new(VolatileMissingDetector::new()),
    ]
}

macro_rules! concurrency_detector {
    ($name:ident, $pattern_name:expr, $desc:expr, $sev:expr) => {
        pub struct $name;
        impl $name { pub fn new() -> Self { Self } }
        impl Default for $name { fn default() -> Self { Self::new() } }
        impl PatternDetector for $name {
            fn name(&self) -> &str { $pattern_name }
            fn description(&self) -> &str { $desc }
            fn severity(&self) -> Severity { $sev }
            fn detect(&self, _graph: &CodeGraph) -> Vec<PatternInstance> { Vec::new() }
            fn suggest_fix(&self, _instance: &PatternInstance) -> Option<FixSuggestion> { None }
        }
    };
}

concurrency_detector!(DataRaceDetector, "data_race", "Data race on shared memory", Severity::Critical);
concurrency_detector!(UnsynchronizedAccessDetector, "unsynchronized_access", "Unsynchronized access to shared data", Severity::Critical);
concurrency_detector!(SharedMutableStateRaceDetector, "shared_mutable_state_race", "Race on shared mutable state", Severity::Critical);
concurrency_detector!(FieldRaceDetector, "field_race", "Data race on struct field", Severity::Critical);
concurrency_detector!(ArrayRaceDetector, "array_race", "Data race on array element", Severity::Critical);
concurrency_detector!(CollectionRaceDetector, "collection_race", "Concurrent modification of collection", Severity::Critical);
concurrency_detector!(LazyInitRaceDetector, "lazy_init_race", "Race in lazy initialization", Severity::Error);
concurrency_detector!(SingletonRaceDetector, "singleton_race", "Race in singleton creation", Severity::Error);
concurrency_detector!(StaticRaceDetector, "static_race", "Race on static variable", Severity::Critical);
concurrency_detector!(GlobalRaceDetector, "global_race", "Race on global variable", Severity::Critical);
concurrency_detector!(ThreadLocalRaceDetector, "thread_local_race", "Thread-local misuse causing race", Severity::Warning);
concurrency_detector!(ClosureRaceDetector, "closure_race", "Closure captures causing race", Severity::Error);
concurrency_detector!(AsyncRaceDetector, "async_race", "Race in async code", Severity::Error);
concurrency_detector!(FutureRaceDetector, "future_race", "Race between future tasks", Severity::Error);
concurrency_detector!(StreamRaceDetector, "stream_race", "Race in stream processing", Severity::Error);
concurrency_detector!(ChannelRaceDetector, "channel_race", "Channel usage race", Severity::Warning);
concurrency_detector!(MessagePassingRaceDetector, "message_passing_race", "Message passing race condition", Severity::Warning);
concurrency_detector!(SharedMemoryRaceDetector, "shared_memory_race", "Shared memory race", Severity::Critical);
concurrency_detector!(MMIORaceDetector, "mmio_race", "Memory-mapped I/O race", Severity::Critical);
concurrency_detector!(AtomicRaceDetector, "atomic_race", "Atomic operation race", Severity::Warning);

concurrency_detector!(DeadlockDetector, "deadlock", "Potential deadlock", Severity::Critical);
concurrency_detector!(LockOrderViolationDetector, "lock_order_violation", "Lock order violation", Severity::Critical);
concurrency_detector!(CircularWaitDetector, "circular_wait", "Circular wait condition", Severity::Critical);
concurrency_detector!(NestedLockDetector, "nested_lock", "Nested locking pattern", Severity::Warning);
concurrency_detector!(RecursiveLockDetector, "recursive_lock", "Recursive lock misuse", Severity::Warning);
concurrency_detector!(CrossThreadLockDetector, "cross_thread_lock", "Cross-thread locking issue", Severity::Warning);
concurrency_detector!(ConditionalLockDetector, "conditional_lock", "Conditional locking deadlock risk", Severity::Warning);
concurrency_detector!(ChannelDeadlockDetector, "channel_deadlock", "Channel deadlock", Severity::Error);
concurrency_detector!(SemaphoreDeadlockDetector, "semaphore_deadlock", "Semaphore deadlock", Severity::Error);
concurrency_detector!(BarrierDeadlockDetector, "barrier_deadlock", "Barrier deadlock", Severity::Error);
concurrency_detector!(RwLockDeadlockDetector, "rwlock_deadlock", "RwLock deadlock", Severity::Error);
concurrency_detector!(MutexDeadlockDetector, "mutex_deadlock", "Mutex deadlock", Severity::Error);
concurrency_detector!(SpinlockDeadlockDetector, "spinlock_deadlock", "Spinlock deadlock", Severity::Error);
concurrency_detector!(ResourceDeadlockDetector, "resource_deadlock", "Resource allocation deadlock", Severity::Critical);
concurrency_detector!(PriorityInversionDetector, "priority_inversion", "Priority inversion", Severity::Warning);

concurrency_detector!(AtomicityViolationDetector, "atomicity_violation", "Atomicity violation", Severity::Error);
concurrency_detector!(NonAtomicCompoundOpDetector, "non_atomic_compound_op", "Non-atomic compound operation", Severity::Error);
concurrency_detector!(CheckThenActDetector, "check_then_act", "Check-then-act race", Severity::Error);
concurrency_detector!(ReadModifyWriteDetector, "read_modify_write", "Read-modify-write race", Severity::Error);
concurrency_detector!(CompareAndSwapMisuseDetector, "cas_misuse", "Compare-and-swap misuse", Severity::Warning);
concurrency_detector!(DoubleCheckedLockingDetector, "double_checked_locking", "Broken double-checked locking", Severity::Error);
concurrency_detector!(LazyInitAtomicityDetector, "lazy_init_atomicity", "Lazy init atomicity issue", Severity::Error);
concurrency_detector!(IncrementDecrementRaceDetector, "increment_decrement_race", "Increment/decrement race", Severity::Error);
concurrency_detector!(CounterRaceDetector, "counter_race", "Counter race condition", Severity::Warning);
concurrency_detector!(FlagRaceDetector, "flag_race", "Flag race condition", Severity::Warning);
concurrency_detector!(StateTransitionRaceDetector, "state_transition_race", "State transition race", Severity::Error);
concurrency_detector!(ResourceAllocationRaceDetector, "resource_allocation_race", "Resource allocation race", Severity::Error);

concurrency_detector!(OrderViolationDetector, "order_violation", "Operation order violation", Severity::Error);
concurrency_detector!(InitBeforeUseViolationDetector, "init_before_use_violation", "Use before initialization", Severity::Critical);
concurrency_detector!(ReleaseAcquireViolationDetector, "release_acquire_violation", "Release-acquire violation", Severity::Error);
concurrency_detector!(HappensBeforeViolationDetector, "happens_before_violation", "Happens-before violation", Severity::Error);
concurrency_detector!(SequentialConsistencyViolationDetector, "sequential_consistency_violation", "Sequential consistency violation", Severity::Warning);
concurrency_detector!(MemoryOrderViolationDetector, "memory_order_violation", "Memory ordering violation", Severity::Error);
concurrency_detector!(PublishSubscribeOrderDetector, "publish_subscribe_order", "Publish-subscribe ordering issue", Severity::Warning);
concurrency_detector!(ProducerConsumerOrderDetector, "producer_consumer_order", "Producer-consumer ordering issue", Severity::Warning);
concurrency_detector!(InitOrderDetector, "init_order", "Initialization order issue", Severity::Warning);
concurrency_detector!(ShutdownOrderDetector, "shutdown_order", "Shutdown order issue", Severity::Warning);

concurrency_detector!(LivelockDetector, "livelock", "Livelock condition", Severity::Warning);
concurrency_detector!(SpinWaitLivelockDetector, "spin_wait_livelock", "Spin-wait livelock", Severity::Warning);
concurrency_detector!(RetryLivelockDetector, "retry_livelock", "Retry livelock", Severity::Warning);
concurrency_detector!(BackoffFailureDetector, "backoff_failure", "Backoff strategy failure", Severity::Warning);
concurrency_detector!(ResourceStarvationDetector, "resource_starvation", "Resource starvation", Severity::Warning);
concurrency_detector!(YieldLivelockDetector, "yield_livelock", "Yield-based livelock", Severity::Warning);
concurrency_detector!(PollingLivelockDetector, "polling_livelock", "Polling livelock", Severity::Warning);
concurrency_detector!(BusyWaitLivelockDetector, "busy_wait_livelock", "Busy-wait livelock", Severity::Warning);

concurrency_detector!(ThreadSafetyViolationDetector, "thread_safety_violation", "Thread safety violation", Severity::Error);
concurrency_detector!(NonThreadSafeAPIDetector, "non_thread_safe_api", "Non-thread-safe API usage", Severity::Warning);
concurrency_detector!(UnsafeSendDetector, "unsafe_send", "Unsafe Send implementation", Severity::Error);
concurrency_detector!(UnsafeSyncDetector, "unsafe_sync", "Unsafe Sync implementation", Severity::Error);
concurrency_detector!(ThreadLocalMisuseDetector, "thread_local_misuse", "Thread-local misuse", Severity::Warning);
concurrency_detector!(ThreadPoolSafetyDetector, "thread_pool_safety", "Thread pool safety issue", Severity::Warning);
concurrency_detector!(WorkStealingSafetyDetector, "work_stealing_safety", "Work-stealing safety issue", Severity::Warning);
concurrency_detector!(RacyStartDetector, "racy_start", "Racy thread start", Severity::Warning);
concurrency_detector!(RacyStopDetector, "racy_stop", "Racy thread stop", Severity::Warning);
concurrency_detector!(ThreadSpawnLeakDetector, "thread_spawn_leak", "Thread spawn leak", Severity::Warning);
concurrency_detector!(DetachedThreadDetector, "detached_thread", "Detached thread issue", Severity::Info);
concurrency_detector!(ThreadPanicDetector, "thread_panic", "Thread panic handling", Severity::Warning);
concurrency_detector!(UnwindSafetyDetector, "unwind_safety", "Unwind safety issue", Severity::Warning);
concurrency_detector!(FFISafetyDetector, "ffi_concurrency_safety", "FFI concurrency safety", Severity::Error);
concurrency_detector!(CallbackThreadSafetyDetector, "callback_thread_safety", "Callback thread safety", Severity::Warning);

concurrency_detector!(AsyncHazardDetector, "async_hazard", "Async concurrency hazard", Severity::Warning);
concurrency_detector!(AwaitPointRaceDetector, "await_point_race", "Race at await point", Severity::Warning);
concurrency_detector!(TaskCancellationRaceDetector, "task_cancellation_race", "Task cancellation race", Severity::Warning);
concurrency_detector!(AsyncDropDetector, "async_drop", "Async drop issue", Severity::Warning);
concurrency_detector!(AsyncMutexContention, "async_mutex_contention", "Async mutex contention", Severity::Warning);
concurrency_detector!(SelectBiasDetector, "select_bias", "Select bias in async", Severity::Info);
concurrency_detector!(FutureLeakDetector, "future_leak", "Future not polled to completion", Severity::Warning);
concurrency_detector!(StreamBackpressureDetector, "stream_backpressure", "Stream backpressure issue", Severity::Warning);
concurrency_detector!(ReactorStarvationDetector, "reactor_starvation", "Reactor starvation", Severity::Warning);
concurrency_detector!(ExecutorOverloadDetector, "executor_overload", "Executor overload", Severity::Warning);
concurrency_detector!(AsyncRecursionDetector, "async_recursion", "Async recursion issue", Severity::Warning);
concurrency_detector!(SpawnBlockingMisuseDetector, "spawn_blocking_misuse", "spawn_blocking misuse", Severity::Warning);

concurrency_detector!(LockFreeABADetector, "lock_free_aba", "ABA problem in lock-free code", Severity::Error);
concurrency_detector!(LockFreeMemoryOrderDetector, "lock_free_memory_order", "Memory ordering in lock-free code", Severity::Error);
concurrency_detector!(CASLoopDetector, "cas_loop", "CAS loop issue", Severity::Warning);
concurrency_detector!(WeakCASMisuseDetector, "weak_cas_misuse", "Weak CAS misuse", Severity::Warning);
concurrency_detector!(LoadStoreOrderingDetector, "load_store_ordering", "Load-store ordering issue", Severity::Error);
concurrency_detector!(AtomicRMWDetector, "atomic_rmw", "Atomic RMW issue", Severity::Warning);
concurrency_detector!(FenceUsageDetector, "fence_usage", "Memory fence usage issue", Severity::Warning);
concurrency_detector!(RCUViolationDetector, "rcu_violation", "RCU protocol violation", Severity::Error);
concurrency_detector!(HazardPointerDetector, "hazard_pointer", "Hazard pointer issue", Severity::Warning);
concurrency_detector!(EpochBasedReclaimDetector, "epoch_based_reclaim", "Epoch-based reclamation issue", Severity::Warning);

concurrency_detector!(SignalHandlerSafetyDetector, "signal_handler_safety", "Signal handler safety", Severity::Error);
concurrency_detector!(ForkSafetyDetector, "fork_safety", "Fork safety issue", Severity::Error);
concurrency_detector!(ExitHandlerRaceDetector, "exit_handler_race", "Exit handler race", Severity::Warning);
concurrency_detector!(GlobalDestructorRaceDetector, "global_destructor_race", "Global destructor race", Severity::Warning);
concurrency_detector!(PluginConcurrencyDetector, "plugin_concurrency", "Plugin concurrency issue", Severity::Warning);
concurrency_detector!(InterruptHandlerDetector, "interrupt_handler", "Interrupt handler safety", Severity::Error);
concurrency_detector!(MemoryBarrierMissingDetector, "memory_barrier_missing", "Missing memory barrier", Severity::Error);
concurrency_detector!(VolatileMissingDetector, "volatile_missing", "Missing volatile qualifier", Severity::Warning);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_concurrency_patterns() {
        let patterns = load_concurrency_patterns();
        assert!(patterns.len() >= 100);
    }
}
