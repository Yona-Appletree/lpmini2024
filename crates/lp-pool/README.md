# lp-pool

A variable-size memory pool allocator for embedded and `no_std` environments.

## Overview

`lp-pool` provides a variable-size block allocator with thread-local access and custom collections optimized for constrained environments. It implements the `allocator-api2` trait, making it compatible with standard Rust patterns while maintaining `no_std` compatibility.

## Features

- **Variable-size block allocator** with free list management and automatic coalescing
- **Thread-local pool access** via `LpMemoryPool::run()`
- **Pool-backed collections**: `LpVec`, `LpString`, `LpBTreeMap`, `LpBox`
- **allocator-api2 compatible** for use with standard collection patterns
- **Dynamic resizing** with grow/shrink support
- **Efficient memory usage** - allocations are sized to fit the requested size
- **Block coalescing** - adjacent free blocks are automatically merged
- **Allocation tracking** (optional, via `alloc-meta` feature)

## Usage

```rust
use lp_pool::{LpMemoryPool, LpVec};
use core::ptr::NonNull;

// Allocate a memory region
let mut memory = [0u8; 4096];
let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

// Create pool
let pool = unsafe { LpMemoryPool::new(memory_ptr, 4096).unwrap() };

// Use collections within the pool
pool.run(|| {
    let mut vec = LpVec::new();
    vec.try_push(1)?;
    vec.try_push(2)?;
    assert_eq!(vec.len(), 2);
    Ok::<(), lp_pool::AllocError>(())
}).unwrap();
```

## Preventing stray allocations

When the crate is running on a host with a global allocator (e.g. during tests), `LpMemoryPool::run`
automatically disables the process-wide allocator so any stray `Box::new`, `alloc::vec![]`, or similar
calls fail fast. If test code needs a short-lived escape hatch, wrap the operation in
`with_global_alloc`:

```rust
use lp_pool::{with_global_alloc, LpMemoryPool};

pool.run(|| {
    // This would panic because the guard is active:
    // let _vec = alloc::vec![1, 2, 3];

    // Allow one standard allocation within the closure.
    let host_vec = with_global_alloc(|| alloc::vec![1, 2, 3]);
    assert_eq!(host_vec, [1, 2, 3]);

    Ok::<(), lp_pool::AllocError>(())
})?;
```

## Architecture

The allocator uses a variable-size block approach with metadata headers:

- **Block Headers**: Each allocated or free block has a header containing its size and allocation status
- **Free List**: Free blocks are linked together via a free list for fast allocation
- **First-Fit**: Allocations use a first-fit strategy, finding the first free block large enough
- **Block Splitting**: Large free blocks are split when allocating smaller sizes
- **Coalescing**: Adjacent free blocks are merged on deallocation to reduce fragmentation

### Memory Overhead

- **Header size**: ~16 bytes per allocation
- **Minimum block size**: 24 bytes (header + free list pointer)

## Performance Characteristics

- **Allocation**: O(n) where n = number of free blocks (first-fit search)
- **Deallocation**: O(n) where n = number of blocks (for coalescing with previous block)
- **Memory efficiency**: Better than fixed-size for varied allocation sizes
- **Fragmentation**: Reduced through automatic coalescing

## Limitations

- **Coalescing overhead**: Finding the previous block requires scanning from the start of memory
- **BTreeMap**: Uses a simplified binary search tree (not a true B-tree), may degrade with unbalanced data
- **Alignment**: Blocks must meet alignment requirements; misaligned blocks are not split for padding

## Allocation Tracking

The `alloc-meta` feature enables detailed tracking of allocations by type and scope. When enabled, the pool records metadata about each allocation, including the type name and optional scope identifier. This is useful for debugging memory usage and identifying allocation patterns in production.

### Usage

Enable the feature in your `Cargo.toml`:

```toml
[dependencies]
lp-pool = { version = "0.1", features = ["alloc-meta"] }
```

Print allocation statistics:

```rust
use lp_pool::{LpMemoryPool, LpVec, LpBox, print_memory_stats_with};

pool.run(|| {
    let _vec = LpVec::<i32>::new();
    let _boxed = LpBox::try_new(42)?;

    // Print statistics (no_std compatible)
    print_memory_stats_with(|s| {
        // Use your custom print function
        // e.g., serial.write_str(s)
    });

    Ok::<(), lp_pool::AllocError>(())
}).unwrap();
```

Output format:

```
Memory Statistics by Type and Scope:
----------------------------------------------------------------------------------------
Type                                     Scope                    Count      Bytes
----------------------------------------------------------------------------------------
lp_pool::collections::vec::LpVec<i32>  (none)                        1        128
lp_pool::collections::box::LpBox<i32>  (none)                        1        128
----------------------------------------------------------------------------------------
TOTAL                                                                 2        256
```

## Feature Flags

- `alloc-meta`: Enable allocation metadata tracking with type and scope information

## Testing

The crate includes comprehensive tests with per-test timeouts using cargo-nextest:

```bash
cargo nextest run --lib
```

This ensures tests complete within 5 seconds and prevents hanging tests from blocking CI/development.
