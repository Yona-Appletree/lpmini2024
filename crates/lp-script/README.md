# lp-script

A GLSL/HLSL-inspired expression language with compiler and virtual machine.

## Overview

`lp-script` provides a shader-like scripting language that compiles to bytecode and runs on a stack-based VM. It's designed for embedded environments, supporting both simple expressions and full scripts with control flow.

## Features

- **Expression compiler** with comprehensive operator support
- **Full script support** with variables, functions, and control flow
- **GLSL/HLSL compatibility** for familiar shader functions
- **Optimization pipeline** with constant folding, algebraic simplification, and dead code elimination
- **Stack-based VM** optimized for fixed-point arithmetic
- **no_std compatible** for embedded targets

## Language Features

### Operators

- Arithmetic: `+`, `-`, `*`, `/`, `%`
- Bitwise (int): `&`, `|`, `^`, `~`, `<<`, `>>`
- Comparisons: `<`, `>`, `<=`, `>=`, `==`, `!=`
- Logical: `&&`, `||`, `!`
- Compound: `+=`, `-=`, `*=`, `/=`, etc.
- Ternary: `condition ? true_val : false_val`

### Built-in Functions

- **Math**: `sin`, `cos`, `abs`, `floor`, `ceil`, `sqrt`, `pow`, `min`, `max`
- **Interpolation**: `lerp`, `mix`, `smoothstep`, `clamp`, `saturate`
- **Noise**: `perlin3(vec3)` or `perlin3(vec3, octaves)`
- **Vector**: `.x`, `.xy`, `.rgb`, swizzling

### Built-in Variables

- `uv`: vec2, normalized coordinates (0..1)
- `coord`: vec2, pixel coordinates
- `time`: float, time value

## Usage

### Simple Expressions

```rust
use lp_script::compile_expr;

// Compile expression to bytecode
let program = compile_expr("sin(time) * 0.5 + 0.5").unwrap();

// With vector swizzling
let program = compile_expr("uv.x * 2.0").unwrap();

// With noise
let program = compile_expr("perlin3(vec3(uv * 0.3, time), 3)").unwrap();
```

### Full Scripts

```rust
use lp_script::compile_script;

let script = "
    float radius = length(uv - vec2(0.5));
    if (radius < 0.3) {
        return sin(time);
    } else {
        return 0.0;
    }
";
let program = compile_script(script).unwrap();
```

### Optimization Control

```rust
use lp_script::{compile_expr_with_options, OptimizeOptions};

// Disable optimizations for debugging
let program = compile_expr_with_options(
    "2.0 + 3.0",
    &OptimizeOptions::none()
).unwrap();

// Custom optimization settings
let mut options = OptimizeOptions::default();
options.constant_folding = true;
options.algebraic_simplification = false;
let program = compile_expr_with_options("x * 1.0", &options).unwrap();
```

## Optimization

The compiler automatically optimizes code by default:

- **Constant folding**: `sin(0.0)` → `0.0`
- **Algebraic simplification**: `x * 1.0` → `x`
- **Dead code elimination**: Remove unreachable code
- **Peephole optimization**: Eliminate redundant opcodes
