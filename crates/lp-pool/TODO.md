# TODO

## alloc-meta Feature

**Status**: Currently unstable and disabled by default

**Issue**: The `alloc-meta` feature uses `alloc::collections::BTreeMap` for tracking allocation metadata. This BTreeMap uses the native/global allocator, which conflicts with lp-pool's guard system that prevents global allocations.

**Problem**: When `alloc-meta` is enabled, operations like `LpBoxDyn::try_new_unsized` on structs containing pool-allocated types (`LpString`, `LpVec`) trigger global allocations during metadata tracking, causing panics even when `with_global_alloc` is used.

**Potential Solutions**:

1. Replace `alloc::collections::BTreeMap` with `LpBTreeMap` (pool-backed) for metadata storage
2. Refactor the guard/allowance system to properly handle nested allocations
3. Use a different data structure that doesn't require allocations (e.g., static arrays with fixed size limits)

**Current Workaround**: Disable `alloc-meta` feature by default. Users can enable it if they understand the limitations and don't use `LpBoxDyn` with complex types.

## LpBoxDyn Bitwise Copy Issue

**Status**: Known limitation

**Issue**: `LpBoxDyn::try_new_unsized` uses `core::ptr::copy_nonoverlapping` to bitwise copy values. This creates an invalid state when copying structs containing non-`Copy` types like `LpString` or `LpVec` (two owners of the same pool-allocated memory).

**Problem**: The bitwise copy may trigger global allocations or cause undefined behavior when the copied value is dropped, as both the original and copy will try to deallocate the same pool memory.

**Potential Solutions**:

1. Require `Clone` trait for values being boxed (limits what can be boxed)
2. Document that `LpBoxDyn` should not be used with structs containing pool-allocated types
3. Implement proper deep cloning for pool-allocated types (requires `Clone` implementations)

**Current Workaround**: Avoid using `LpBoxDyn` with structs containing `LpString`, `LpVec`, or other pool-allocated types. Use `LpBox<T>` for sized types instead.
