<!-- lp_pool allocation policy -->

# LpPool Allocation Policy

## Scope

This policy applies to all non-test Rust code in the crates `lp-script`, `lp-data`, `lp-math`, and `engine-core`. Unit tests, benchmarks, and dev-only utilities may continue to use standard allocation APIs.

## Rationale

We must limit dynamic memory to the custom `LpPool` allocator to guarantee deterministic usage on constrained targets. Standard library heap allocations are disallowed unless a specific exception is documented and reviewed.

## Banned APIs

The linter flags the following constructors and macros whenever they appear outside of allowed scopes:

- `Box::{new, try_new, pin}`
- `Vec::{new, with_capacity, from, from_iter, default}`
- `String::{new, with_capacity, from}`
- `VecDeque::new`, `BinaryHeap::new`, `LinkedList::new`
- `HashMap::{new, with_capacity}`, `HashSet::{new, with_capacity}`
- `BTreeMap::new`, `BTreeSet::new`
- `Rc::new`, `Arc::new`
- `alloc::alloc`, `alloc::alloc_zeroed`, `alloc::dealloc`
- Macros: `vec![]`, `format!`

The list is intentionally conservative—if a standard allocator entry point is missing, add it to the tool’s deny-list before merging new usage.

## Allowed Alternatives

Use the corresponding `LpPool` collections:

- `lp_pool::collections::box_::LpBox`
- `lp_pool::collections::vec::LpVec`
- `lp_pool::collections::string::LpString`
- `lp_pool::collections::map::LpBTreeMap`
- `lp_pool::collections::set::LpBTreeSet`
- `lp_pool::collections::deque::LpVecDeque` (if available)

Direct allocations must go through `LpMemoryPool::try_alloc` or helpers on `LpPoolHandle`.

## Exceptions

- Mark narrowly-scoped exceptions with `#[allow(lp_pool_std_alloc)]` and add a comment explaining why the allocation is required.
- Prefer wrapping the smallest enclosing expression or function rather than entire modules.
- File a follow-up issue for every permanent exception.

Annotating a module or function with `#[cfg(test)]` automatically suppresses lint findings in that scope.

## Tooling

Run the linter locally before pushing:

```
$ cargo run -p lp_pool_lint
```

By default it scans `crates/lp-script`, `crates/lp-data`, `crates/lp-math`, and `crates/engine-core`. Provide extra paths if needed:

```
$ cargo run -p lp_pool_lint -- crates/engine-core/src/test_engine
```

The tool exits with a non-zero status when it encounters disallowed allocations.

## Enforcement Matrix

| Crate         | Non-test code | Tests/benches |
| ------------- | ------------- | ------------- |
| `lp-script`   | `LpPool` only | unrestricted  |
| `lp-data`     | `LpPool` only | unrestricted  |
| `lp-math`     | `LpPool` only | unrestricted  |
| `engine-core` | `LpPool` only | unrestricted  |
