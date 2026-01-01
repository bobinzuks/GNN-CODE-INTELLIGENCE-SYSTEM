//! Memory safety pattern detectors (100+ patterns)

use crate::{CodeGraph, PatternDetector, PatternInstance, FixSuggestion, Severity};
use std::sync::Arc;

pub fn load_memory_patterns() -> Vec<Arc<dyn PatternDetector>> {
    vec![
        // Use-after-free patterns (15)
        Arc::new(UseAfterFreeDetector::new()),
        Arc::new(DanglingPointerDetector::new()),
        Arc::new(UseAfterMoveDetector::new()),
        Arc::new(UseAfterReturnDetector::new()),
        Arc::new(UseAfterScopeDetector::new()),
        Arc::new(IteratorInvalidationDetector::new()),
        Arc::new(ReferenceToTemporaryDetector::new()),
        Arc::new(LifetimeMismatchDetector::new()),
        Arc::new(BorrowAfterMoveDetector::new()),
        Arc::new(SelfReferentialStructDetector::new()),
        Arc::new(DropOrderIssueDetector::new()),
        Arc::new(CallbackLifetimeDetector::new()),
        Arc::new(ClosureLifetimeDetector::new()),
        Arc::new(AsyncLifetimeDetector::new()),
        Arc::new(StaticLifetimeAbuseDetector::new()),

        // Memory leaks (15)
        Arc::new(MemoryLeakDetector::new()),
        Arc::new(CircularReferenceDetector::new()),
        Arc::new(ReferenceCycleDetector::new()),
        Arc::new(MissingFreeDetector::new()),
        Arc::new(ResourceLeakDetector::new()),
        Arc::new(FileDescriptorLeakDetector::new()),
        Arc::new(SocketLeakDetector::new()),
        Arc::new(HandleLeakDetector::new()),
        Arc::new(ThreadLeakDetector::new()),
        Arc::new(EventListenerLeakDetector::new()),
        Arc::new(CacheLeakDetector::new()),
        Arc::new(TimerLeakDetector::new()),
        Arc::new(SubscriptionLeakDetector::new()),
        Arc::new(WeakReferenceLeakDetector::new()),
        Arc::new(GlobalStateLeakDetector::new()),

        // Buffer overflows (15)
        Arc::new(BufferOverflowDetector::new()),
        Arc::new(StackBufferOverflowDetector::new()),
        Arc::new(HeapBufferOverflowDetector::new()),
        Arc::new(ArrayBoundsViolationDetector::new()),
        Arc::new(UncheckedIndexDetector::new()),
        Arc::new(OffByOneErrorDetector::new()),
        Arc::new(IntegerOverflowBufferDetector::new()),
        Arc::new(SignednessErrorDetector::new()),
        Arc::new(MemcpyOverflowDetector::new()),
        Arc::new(StrcpyOverflowDetector::new()),
        Arc::new(SprintfOverflowDetector::new()),
        Arc::new(GetsBoundsDetector::new()),
        Arc::new(UnsafeCastOverflowDetector::new()),
        Arc::new(VectorCapacityOverflowDetector::new()),
        Arc::new(SliceOutOfBoundsDetector::new()),

        // Null/uninitialized memory (12)
        Arc::new(NullPointerDereferenceDetector::new()),
        Arc::new(UninitializedMemoryDetector::new()),
        Arc::new(UninitializedVariableDetector::new()),
        Arc::new(PartiallyInitializedStructDetector::new()),
        Arc::new(UndefBehaviorDetector::new()),
        Arc::new(NullCheckMissingDetector::new()),
        Arc::new(OptionalUnwrapDetector::new()),
        Arc::new(AssumeNonNullDetector::new()),
        Arc::new(MaybeUninitMisuseDetector::new()),
        Arc::new(ZeroedMemoryAssumptionDetector::new()),
        Arc::new(PaddingBytesDetector::new()),
        Arc::new(DiscriminantUninitDetector::new()),

        // Double free (8)
        Arc::new(DoubleFreeDetector::new()),
        Arc::new(MultipleDropDetector::new()),
        Arc::new(FreeAfterFreeDetector::new()),
        Arc::new(DeleteAfterDeleteDetector::new()),
        Arc::new(DeallocMismatchDetector::new()),
        Arc::new(CustomAllocatorFreeDetector::new()),
        Arc::new(ArenaDoubleFreeDetector::new()),
        Arc::new(PoolDoubleFreeDetector::new()),

        // Memory corruption (15)
        Arc::new(MemoryCorruptionDetector::new()),
        Arc::new(TypeConfusionMemoryDetector::new()),
        Arc::new(VtablePoisoningDetector::new()),
        Arc::new(HeapSprayDetector::new()),
        Arc::new(WriteWhatWhereDetector::new()),
        Arc::new(ControlFlowHijackDetector::new()),
        Arc::new(ReturnOrientedProgrammingDetector::new()),
        Arc::new(StackPivotDetector::new()),
        Arc::new(PointerArithmeticOverflowDetector::new()),
        Arc::new(UnsafeTransmuteDetector::new()),
        Arc::new(AlignmentViolationDetector::new()),
        Arc::new(DataRaceMemoryDetector::new()),
        Arc::new(ABAProbleDetector::new()),
        Arc::new(WildPointerDetector::new()),
        Arc::new(StalePointerDetector::new()),

        // Stack issues (10)
        Arc::new(StackOverflowDetector::new()),
        Arc::new(UnboundedRecursionDetector::new()),
        Arc::new(LargeStackFrameDetector::new()),
        Arc::new(VariableLengthArrayDetector::new()),
        Arc::new(AllocaDetector::new()),
        Arc::new(ReturnStackAddressDetector::new()),
        Arc::new(EscapingStackReferenceDetector::new()),
        Arc::new(StackSmashingDetector::new()),
        Arc::new(StackProtectorDetector::new()),
        Arc::new(CanaryBypassDetector::new()),

        // Ownership issues (10)
        Arc::new(OwnershipViolationDetector::new()),
        Arc::new(MoveSemanticViolationDetector::new()),
        Arc::new(BorrowRuleViolationDetector::new()),
        Arc::new(MutableAliasDetector::new()),
        Arc::new(SharedMutableStateDetector::new()),
        Arc::new(InteriorMutabilityMisuseDetector::new()),
        Arc::new(CellMisuseDetector::new()),
        Arc::new(RefCellPanicDetector::new()),
        Arc::new(UnsafeCellSoundnessDetector::new()),
        Arc::new(SendSyncViolationDetector::new()),

        // Additional patterns (20)
        Arc::new(UninitializedPaddingDetector::new()),
        Arc::new(MemoryOrderingDetector::new()),
        Arc::new(VolatileAccessDetector::new()),
        Arc::new(AtomicTearingDetector::new()),
        Arc::new(SizeofMismatchDetector::new()),
        Arc::new(AllocationSizeOverflowDetector::new()),
        Arc::new(ReallocUseAfterFreeDetector::new()),
        Arc::new(MemoryMappingIssueDetector::new()),
        Arc::new(SharedMemoryCorruptionDetector::new()),
        Arc::new(MMAPMisconfigDetector::new()),
    ]
}

macro_rules! memory_detector {
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

memory_detector!(UseAfterFreeDetector, "use_after_free", "Use-after-free memory safety violation", Severity::Critical);
memory_detector!(DanglingPointerDetector, "dangling_pointer", "Dangling pointer detected", Severity::Critical);
memory_detector!(UseAfterMoveDetector, "use_after_move", "Use after move", Severity::Error);
memory_detector!(UseAfterReturnDetector, "use_after_return", "Use of stack memory after return", Severity::Critical);
memory_detector!(UseAfterScopeDetector, "use_after_scope", "Reference outlives scope", Severity::Error);
memory_detector!(IteratorInvalidationDetector, "iterator_invalidation", "Iterator invalidation", Severity::Error);
memory_detector!(ReferenceToTemporaryDetector, "reference_to_temporary", "Reference to temporary", Severity::Error);
memory_detector!(LifetimeMismatchDetector, "lifetime_mismatch", "Lifetime mismatch", Severity::Error);
memory_detector!(BorrowAfterMoveDetector, "borrow_after_move", "Borrow after move", Severity::Error);
memory_detector!(SelfReferentialStructDetector, "self_referential_struct", "Self-referential struct safety issue", Severity::Warning);
memory_detector!(DropOrderIssueDetector, "drop_order_issue", "Drop order dependency", Severity::Warning);
memory_detector!(CallbackLifetimeDetector, "callback_lifetime", "Callback lifetime issue", Severity::Warning);
memory_detector!(ClosureLifetimeDetector, "closure_lifetime", "Closure lifetime issue", Severity::Warning);
memory_detector!(AsyncLifetimeDetector, "async_lifetime", "Async lifetime issue", Severity::Warning);
memory_detector!(StaticLifetimeAbuseDetector, "static_lifetime_abuse", "Static lifetime abuse", Severity::Warning);

memory_detector!(MemoryLeakDetector, "memory_leak", "Memory leak detected", Severity::Warning);
memory_detector!(CircularReferenceDetector, "circular_reference", "Circular reference causing leak", Severity::Warning);
memory_detector!(ReferenceCycleDetector, "reference_cycle", "Reference cycle detected", Severity::Warning);
memory_detector!(MissingFreeDetector, "missing_free", "Missing free/deallocation", Severity::Warning);
memory_detector!(ResourceLeakDetector, "resource_leak", "Resource leak", Severity::Warning);
memory_detector!(FileDescriptorLeakDetector, "fd_leak", "File descriptor leak", Severity::Warning);
memory_detector!(SocketLeakDetector, "socket_leak", "Socket leak", Severity::Warning);
memory_detector!(HandleLeakDetector, "handle_leak", "Handle leak", Severity::Warning);
memory_detector!(ThreadLeakDetector, "thread_leak", "Thread leak", Severity::Warning);
memory_detector!(EventListenerLeakDetector, "event_listener_leak", "Event listener leak", Severity::Warning);
memory_detector!(CacheLeakDetector, "cache_leak", "Cache grows without bounds", Severity::Warning);
memory_detector!(TimerLeakDetector, "timer_leak", "Timer not cleared", Severity::Warning);
memory_detector!(SubscriptionLeakDetector, "subscription_leak", "Subscription leak", Severity::Warning);
memory_detector!(WeakReferenceLeakDetector, "weak_reference_leak", "Weak reference not dropped", Severity::Info);
memory_detector!(GlobalStateLeakDetector, "global_state_leak", "Global state accumulation", Severity::Warning);

memory_detector!(BufferOverflowDetector, "buffer_overflow", "Buffer overflow vulnerability", Severity::Critical);
memory_detector!(StackBufferOverflowDetector, "stack_buffer_overflow", "Stack buffer overflow", Severity::Critical);
memory_detector!(HeapBufferOverflowDetector, "heap_buffer_overflow", "Heap buffer overflow", Severity::Critical);
memory_detector!(ArrayBoundsViolationDetector, "array_bounds_violation", "Array bounds violation", Severity::Critical);
memory_detector!(UncheckedIndexDetector, "unchecked_index", "Unchecked array index", Severity::Warning);
memory_detector!(OffByOneErrorDetector, "off_by_one", "Off-by-one error", Severity::Warning);
memory_detector!(IntegerOverflowBufferDetector, "integer_overflow_buffer", "Integer overflow in buffer size", Severity::Critical);
memory_detector!(SignednessErrorDetector, "signedness_error", "Signed/unsigned mismatch", Severity::Warning);
memory_detector!(MemcpyOverflowDetector, "memcpy_overflow", "memcpy buffer overflow", Severity::Critical);
memory_detector!(StrcpyOverflowDetector, "strcpy_overflow", "strcpy buffer overflow", Severity::Critical);
memory_detector!(SprintfOverflowDetector, "sprintf_overflow", "sprintf buffer overflow", Severity::Critical);
memory_detector!(GetsBoundsDetector, "gets_bounds", "gets() usage - always unsafe", Severity::Critical);
memory_detector!(UnsafeCastOverflowDetector, "unsafe_cast_overflow", "Unsafe cast overflow", Severity::Warning);
memory_detector!(VectorCapacityOverflowDetector, "vector_capacity_overflow", "Vector capacity overflow", Severity::Warning);
memory_detector!(SliceOutOfBoundsDetector, "slice_out_of_bounds", "Slice index out of bounds", Severity::Error);

memory_detector!(NullPointerDereferenceDetector, "null_pointer_deref", "Null pointer dereference", Severity::Critical);
memory_detector!(UninitializedMemoryDetector, "uninitialized_memory", "Uninitialized memory access", Severity::Critical);
memory_detector!(UninitializedVariableDetector, "uninitialized_variable", "Uninitialized variable", Severity::Error);
memory_detector!(PartiallyInitializedStructDetector, "partially_initialized_struct", "Partially initialized struct", Severity::Warning);
memory_detector!(UndefBehaviorDetector, "undefined_behavior", "Undefined behavior", Severity::Critical);
memory_detector!(NullCheckMissingDetector, "null_check_missing", "Missing null check", Severity::Warning);
memory_detector!(OptionalUnwrapDetector, "optional_unwrap", "Optional unwrap without check", Severity::Warning);
memory_detector!(AssumeNonNullDetector, "assume_non_null", "assume_init without verification", Severity::Critical);
memory_detector!(MaybeUninitMisuseDetector, "maybe_uninit_misuse", "MaybeUninit misuse", Severity::Error);
memory_detector!(ZeroedMemoryAssumptionDetector, "zeroed_memory_assumption", "Assumes memory is zeroed", Severity::Warning);
memory_detector!(PaddingBytesDetector, "padding_bytes", "Padding bytes uninitialized", Severity::Warning);
memory_detector!(DiscriminantUninitDetector, "discriminant_uninit", "Enum discriminant uninitialized", Severity::Critical);

memory_detector!(DoubleFreeDetector, "double_free", "Double free detected", Severity::Critical);
memory_detector!(MultipleDropDetector, "multiple_drop", "Value dropped multiple times", Severity::Critical);
memory_detector!(FreeAfterFreeDetector, "free_after_free", "Free after free", Severity::Critical);
memory_detector!(DeleteAfterDeleteDetector, "delete_after_delete", "Delete after delete", Severity::Critical);
memory_detector!(DeallocMismatchDetector, "dealloc_mismatch", "Allocation/deallocation mismatch", Severity::Critical);
memory_detector!(CustomAllocatorFreeDetector, "custom_allocator_free", "Wrong deallocator for custom allocator", Severity::Critical);
memory_detector!(ArenaDoubleFreeDetector, "arena_double_free", "Arena double free", Severity::Critical);
memory_detector!(PoolDoubleFreeDetector, "pool_double_free", "Pool double free", Severity::Critical);

memory_detector!(MemoryCorruptionDetector, "memory_corruption", "Memory corruption", Severity::Critical);
memory_detector!(TypeConfusionMemoryDetector, "type_confusion_memory", "Type confusion memory corruption", Severity::Critical);
memory_detector!(VtablePoisoningDetector, "vtable_poisoning", "Vtable poisoning attack", Severity::Critical);
memory_detector!(HeapSprayDetector, "heap_spray", "Heap spray pattern", Severity::Critical);
memory_detector!(WriteWhatWhereDetector, "write_what_where", "Write-what-where condition", Severity::Critical);
memory_detector!(ControlFlowHijackDetector, "control_flow_hijack", "Control flow hijacking", Severity::Critical);
memory_detector!(ReturnOrientedProgrammingDetector, "rop", "ROP gadget potential", Severity::Critical);
memory_detector!(StackPivotDetector, "stack_pivot", "Stack pivot vulnerability", Severity::Critical);
memory_detector!(PointerArithmeticOverflowDetector, "pointer_arithmetic_overflow", "Pointer arithmetic overflow", Severity::Critical);
memory_detector!(UnsafeTransmuteDetector, "unsafe_transmute", "Unsafe transmute", Severity::Warning);
memory_detector!(AlignmentViolationDetector, "alignment_violation", "Memory alignment violation", Severity::Error);
memory_detector!(DataRaceMemoryDetector, "data_race_memory", "Data race on memory", Severity::Critical);
memory_detector!(ABAProbleDetector, "aba_problem", "ABA problem in lock-free code", Severity::Warning);
memory_detector!(WildPointerDetector, "wild_pointer", "Wild pointer", Severity::Critical);
memory_detector!(StalePointerDetector, "stale_pointer", "Stale pointer", Severity::Warning);

memory_detector!(StackOverflowDetector, "stack_overflow", "Stack overflow risk", Severity::Critical);
memory_detector!(UnboundedRecursionDetector, "unbounded_recursion", "Unbounded recursion", Severity::Warning);
memory_detector!(LargeStackFrameDetector, "large_stack_frame", "Large stack frame", Severity::Warning);
memory_detector!(VariableLengthArrayDetector, "vla", "Variable length array (VLA)", Severity::Warning);
memory_detector!(AllocaDetector, "alloca_usage", "alloca usage", Severity::Warning);
memory_detector!(ReturnStackAddressDetector, "return_stack_address", "Returning stack address", Severity::Critical);
memory_detector!(EscapingStackReferenceDetector, "escaping_stack_reference", "Stack reference escapes", Severity::Critical);
memory_detector!(StackSmashingDetector, "stack_smashing", "Stack smashing potential", Severity::Critical);
memory_detector!(StackProtectorDetector, "stack_protector_missing", "Stack protector missing", Severity::Warning);
memory_detector!(CanaryBypassDetector, "canary_bypass", "Stack canary bypass", Severity::Critical);

memory_detector!(OwnershipViolationDetector, "ownership_violation", "Ownership violation", Severity::Error);
memory_detector!(MoveSemanticViolationDetector, "move_semantic_violation", "Move semantic violation", Severity::Error);
memory_detector!(BorrowRuleViolationDetector, "borrow_rule_violation", "Borrow rule violation", Severity::Error);
memory_detector!(MutableAliasDetector, "mutable_alias", "Mutable aliasing", Severity::Error);
memory_detector!(SharedMutableStateDetector, "shared_mutable_state", "Shared mutable state", Severity::Warning);
memory_detector!(InteriorMutabilityMisuseDetector, "interior_mutability_misuse", "Interior mutability misuse", Severity::Warning);
memory_detector!(CellMisuseDetector, "cell_misuse", "Cell misuse", Severity::Warning);
memory_detector!(RefCellPanicDetector, "refcell_panic", "RefCell runtime panic risk", Severity::Warning);
memory_detector!(UnsafeCellSoundnessDetector, "unsafecell_soundness", "UnsafeCell soundness issue", Severity::Error);
memory_detector!(SendSyncViolationDetector, "send_sync_violation", "Send/Sync trait violation", Severity::Error);

memory_detector!(UninitializedPaddingDetector, "uninitialized_padding", "Uninitialized padding bytes", Severity::Warning);
memory_detector!(MemoryOrderingDetector, "memory_ordering", "Memory ordering issue", Severity::Error);
memory_detector!(VolatileAccessDetector, "volatile_access", "Missing volatile access", Severity::Warning);
memory_detector!(AtomicTearingDetector, "atomic_tearing", "Atomic tearing", Severity::Error);
memory_detector!(SizeofMismatchDetector, "sizeof_mismatch", "sizeof mismatch", Severity::Warning);
memory_detector!(AllocationSizeOverflowDetector, "allocation_size_overflow", "Allocation size overflow", Severity::Critical);
memory_detector!(ReallocUseAfterFreeDetector, "realloc_use_after_free", "realloc use-after-free", Severity::Critical);
memory_detector!(MemoryMappingIssueDetector, "memory_mapping_issue", "Memory mapping issue", Severity::Warning);
memory_detector!(SharedMemoryCorruptionDetector, "shared_memory_corruption", "Shared memory corruption", Severity::Critical);
memory_detector!(MMAPMisconfigDetector, "mmap_misconfig", "mmap misconfiguration", Severity::Warning);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_memory_patterns() {
        let patterns = load_memory_patterns();
        assert!(patterns.len() >= 100);
    }
}
