# lp-pool to lp-alloc Migration Tool

This tool helps migrate the codebase from `lp-pool` to `lp-alloc`.

## Usage

```bash
# Dry run to see what would change
cargo run --bin lp-pool-to-alloc -- --dry-run --path crates/ --path apps/

# Actually apply changes
cargo run --bin lp-pool-to-alloc -- --path crates/ --path apps/
```

## What It Does

1. **Import replacements**: Replaces `lp_pool::*` imports with standard `alloc::*` or `lp_alloc::*`
2. **Type replacements**: Replaces `LpVec`, `LpString`, `LpBox`, etc. with standard types
3. **Error type replacements**: Replaces `AllocError` with `AllocLimitError`
4. **Test setup**: Adds `#[global_allocator]` and `setup_test_alloc!()` to test modules
5. **Import additions**: Adds `use lp_alloc::try_alloc;` where needed in compiler code

## Limitations

- Complex AST transformations (like wrapping `try_*` calls in `try_alloc`) require manual work or a more sophisticated AST walker
- `LpMemoryPool::run()` wrapper removal is basic - may need manual cleanup
- Some edge cases in imports may need manual fixing

## Phase 2 Cleanup

After running this tool, Phase 2 will handle:

- Fixing compilation errors
- Completing try_alloc wrapping in compiler
- Removing remaining lp_pool references
- Updating documentation
