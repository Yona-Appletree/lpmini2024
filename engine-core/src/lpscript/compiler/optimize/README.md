# LPScript Optimization System

This module provides configurable optimization passes for the LPScript compiler.

## Architecture

The optimization system operates at two levels:

1. **AST-level optimizations** - High-level semantic optimizations applied after type checking
2. **Opcode-level optimizations** - Low-level bytecode optimizations applied after code generation

## Optimization Passes

### AST Level

#### Constant Folding (`ast/constant_fold.rs`)

Evaluates expressions with constant operands at compile time.

**Examples:**

- `2.0 + 3.0` → `5.0`
- `sin(0.0)` → `0.0`
- `max(2.0, 3.0)` → `3.0`
- `2.0 < 3.0` → `1.0` (true)

#### Algebraic Simplification (`ast/algebraic.rs`)

Applies mathematical identities to simplify expressions.

**Examples:**

- `x * 1.0` → `x`
- `x + 0.0` → `x`
- `x * 0.0` → `0.0`
- `x && true` → `x`
- `!(!x)` → `x`

#### Dead Code Elimination (`ast/dead_code.rs`)

Removes unreachable code.

**Examples:**

- Statements after `return`
- `if (true) A else B` → `A`
- `if (false) A else B` → `B`

### Opcode Level

#### Peephole Optimization (`ops/peephole.rs`)

Pattern-matches and optimizes local instruction sequences.

**Examples:**

- `Push x; Drop1` → _(delete)_
- `LoadLocal(x); StoreLocal(x)` → _(delete)_
- `Dup1; Drop1` → _(delete)_
- Unreachable code after unconditional `Jump`

## Usage

### Default (All Optimizations Enabled)

```rust
use engine_core::lpscript::compile_expr;

let program = compile_expr("2.0 + 3.0 * 1.0").unwrap();
// Result: Push(5.0), Return
```

### Custom Options

```rust
use engine_core::lpscript::{compile_expr_with_options, OptimizeOptions};

// Disable all optimizations
let program = compile_expr_with_options(
    "2.0 + 3.0",
    &OptimizeOptions::none()
).unwrap();

// Custom configuration
let mut options = OptimizeOptions::none();
options.constant_folding = true;
options.max_ast_passes = 5;
let program = compile_expr_with_options("2.0 + 3.0", &options).unwrap();
```

## Configuration

```rust
pub struct OptimizeOptions {
    /// Enable constant folding (e.g., `2 + 3` → `5`)
    pub constant_folding: bool,

    /// Enable algebraic simplification (e.g., `x * 1` → `x`)
    pub algebraic_simplification: bool,

    /// Enable dead code elimination
    pub dead_code_elimination: bool,

    /// Enable opcode peephole optimization
    pub peephole_optimization: bool,

    /// Maximum number of AST optimization passes (to reach fixed point)
    pub max_ast_passes: usize,
}
```

## Multi-Pass Optimization

The AST optimizer runs multiple passes until either:

1. No changes are detected (fixed point reached)
2. Maximum iterations exceeded (`max_ast_passes`)

**Example:**

```
Input: (2.0 + 3.0) * 1.0

Pass 1 (constant folding): 5.0 * 1.0
Pass 2 (algebraic simplification): 5.0
Pass 3: No changes → stop

Result: Push(5.0)
```

## Performance Characteristics

### Compile-Time Overhead

- **Minimal** for simple expressions (< 1ms)
- **Low** for complex expressions (~1-5ms)
- Optimization happens once at compile time

### Runtime Benefit

- **High** for expressions with constants (up to 90% fewer opcodes)
- **Medium** for algebraic simplifications (20-50% fewer opcodes)
- **Low** for expressions without optimizable patterns

### ESP32 Benefits

- Smaller bytecode size (less flash usage)
- Faster execution (fewer VM operations)
- Lower stack usage (simpler evaluation)

## Safety Guarantees

All optimizations preserve program semantics:

- Optimized code produces identical results to unoptimized code
- Floating-point precision is maintained
- Side effects are preserved (though none exist in pure expressions)

## Testing

Comprehensive test suite in `tests.rs`:

- Constant folding tests
- Algebraic simplification tests
- Dead code elimination tests
- Semantic preservation tests
- Integration tests with real programs

Run tests:

```bash
cargo test --lib optimize
```

## Future Enhancements

Potential optimizations not yet implemented:

- Common subexpression elimination (CSE)
- Loop invariant code motion
- Strength reduction (e.g., `x * 2` → `x + x`)
- Constant propagation across statements
- Function inlining

