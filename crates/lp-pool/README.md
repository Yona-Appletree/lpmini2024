# lp-pool

A memory pool allocator for embedded and `no_std` environments.

## Overview

`lp-pool` provides a fixed-size block allocator with thread-local access and custom collections optimized for constrained environments. It implements the `allocator-api2` trait, making it compatible with standard Rust patterns while maintaining `no_std` compatibility.

## Features

- **Fixed-size block allocator** with free list management
- **Thread-local pool access** via `LpMemoryPool::run()`
- **Pool-backed collections**: `LpVec`, `LpString`, `LpBTreeMap`, `LpBox`
- **allocator-api2 compatible** for use with standard collection patterns
- **Dynamic resizing** with grow/shrink support
- **Allocation tracking** (optional, via `alloc-meta` feature)

## Usage

```rust
use lp_pool::{LpMemoryPool, LpVec};
use core::ptr::NonNull;

// Allocate a memory region
let mut memory = [0u8; 4096];
let memory_ptr = NonNull::new(memory.as_mut_ptr()).unwrap();

// Create pool with 64-byte blocks
let pool = unsafe { LpMemoryPool::new(memory_ptr, 4096, 64).unwrap() };

// Use collections within the pool
pool.run(|| {
    let mut vec = LpVec::new();
    vec.try_push(1)?;
    vec.try_push(2)?;
    assert_eq!(vec.len(), 2);
    Ok::<(), lp_pool::AllocError>(())
}).unwrap();
```

## Limitations

- All allocations must fit within the configured `block_size`
- Blocks are aligned to `block_size` boundaries
- `BTreeMap` uses a simplified binary search tree (not a true B-tree)

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
