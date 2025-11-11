<!-- Memory allocation policy overview -->

# Memory Allocation Policy

## Summary

The old `lp_pool` crate has been retired in favour of `lp_alloc`, a lightweight global allocator wrapper with configurable soft and hard limits. Runtime crates (`lp-script`, `lp-data`, `lp-math`, `engine-core`) now allocate through the standard library collections while the global allocator tracks usage and enforces limits.

## Guidelines

- **Global configuration:** use `lp_alloc::set_hard_limit`, `lp_alloc::set_soft_limit`, and `lp_alloc::allocated_bytes` to control memory at runtime.
- **Scoped limits:** wrap fallible operations with `lp_alloc::try_alloc` or `lp_alloc::with_alloc_limit` to ensure large bursts of work stay within budget.
- **Tests:** call `lp_alloc::init_test_allocator()` (or the `setup_test_alloc!` macro) to install the allocator with a 10 MB default limit. When a test needs extra headroom, call `lp_alloc::enter_global_alloc_allowance()` to temporarily lift the soft limit.
- **Dynamic helpers:** standard `String`, `Vec`, and `Box` types are used directly; allocator enforcement now happens entirely within `lp_alloc`.

## Exceptions

- Prefer `lp_alloc::with_alloc_limit` to keep limits explicit. If a module truly requires unlimited allocation, document the rationale in the code and restore the previous limit afterwards (see `enter_global_alloc_allowance`).
- Test-only modules can continue to use unrestricted allocations.

## Tooling

The custom `lp_pool_lint` tool is deprecated. Standard collections are allowed in production code; allocator limits are enforced through `lp_alloc` at runtime. Continuous integration runs `cargo check` and the existing test suite to ensure the allocator remains wired up correctly.

## Enforcement Matrix

| Crate         | Runtime policy                                                | Tests                          |
| ------------- | ------------------------------------------------------------- | ------------------------------ |
| `lp-script`   | `lp_alloc` soft limit via `VmLimits`                          | init allocator per test module |
| `lp-data`     | Standard `String`/`Vec` allocations routed through `lp_alloc` | init allocator per test module |
| `lp-math`     | Standard allocations, subject to global limits                | unrestricted                   |
| `engine-core` | Standard allocations, subject to global limits                | unrestricted                   |
