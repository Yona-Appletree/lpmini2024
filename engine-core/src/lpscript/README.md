# LightPlayer Script (LPS)

A lightweight GLSL-subset scripting language for per-pixel visualization generation on ESP32 microcontrollers.

## Design Goals

- **GLSL Compatibility**: Strict subset of GLSL - valid LPS programs should be valid GLSL
- **ESP32 First**: Optimized for 32-bit RISC-V at 160MHz, no FPU, ~100KB RAM
- **GPU Path**: Foundation for future GPU compilation via GLSL/SPIR-V
- **Low-res 2D**: Focused on individually addressable LED visualizations

## Architecture

```
Source (.lps) → Lexer → Parser → Type Checker → Codegen → LpsProgram → LpsVm → Pixels
```

### Components

**Lexer** (`lexer.rs`)
- Tokenizes source with span tracking for error reporting
- Supports GLSL numeric literals: `1.0f`, `1e-3`, `0xFF`, int/float distinction

**Parser** (`parser.rs`)
- Builds AST with span information on every node
- Recognizes vector constructors: `vec2(x, y)`, `vec3(x, y, z)`, `vec4(x, y, z, w)`

**AST** (`ast.rs`)
- Expression tree with metadata: `Expr { kind, span, ty }`
- Type field populated by type checker

**Type Checker** (planned: `typechecker.rs`)
- Infers and validates types
- Handles GLSL implicit conversions (int → float where allowed)

**Codegen** (`codegen.rs`)
- Emits typed opcodes based on type information
- Generates source maps for runtime error reporting

**VM** (`vm/`)
- Stack-based VM with typed opcodes
- Hybrid design: small args (indices, offsets) embedded, data on stack
- Reusable: create once, run per-pixel

## Type System

### Scalars
- `float` - 16.16 fixed-point (alias for `Fixed`)
- `int` - 32-bit signed integer

### Vectors (multiple stack slots)
- `vec2` - 2 consecutive Fixed values
- `vec3` - 3 consecutive Fixed values  
- `vec4` - 4 consecutive Fixed values

Vectors occupy consecutive stack slots for zero-memory overhead. Example:
```
AddVec2: pops 4 slots (2 vec2s), pushes 2 slots (1 vec2)
Dot3:    pops 6 slots (2 vec3s), pushes 1 slot (scalar)
```

### Textures and Locals
- `sampler2D` - Single-channel or RGBA8888 textures
- Local variables with type safety (read-only inputs, scratch space, outputs)

## OpCode Design (Hybrid)

**Rationale**: Balance between pure stack VM and register-based.

- **Data values**: Flow through stack (e.g., operands, results)
- **Small constants**: Embedded in opcodes (e.g., local indices, jump offsets)

Benefits:
- Smaller bytecode than pure stack (no index pushing)
- Simpler than registers (no allocation needed)
- Clear intent (e.g., `LoadLocalVec3(5)` vs generic load + index)

**Examples**:
```rust
AddFixed              // pops 2, pushes 1
LoadLocalVec3(idx)    // index embedded, pushes 3 Fixed
TextureSample_R(idx)  // texture index embedded, pops UV (2), pushes R (1)
```

## Why Stack-Based?

Unlike GPU VMs (SPIR-V uses SSA/registers), we chose stack-based for:
- **Simplicity**: No register allocation needed
- **Memory**: Smaller VM state (just stack + PC)
- **Bytecode size**: More compact than register references everywhere
- **Familiarity**: Easier to implement and debug

For GPU targets, we'll compile to GLSL which drivers convert to SPIR-V.

## Error Handling

### Compile-Time
- Lexer, parser, type checker errors with span information
- Rust-style error messages showing source snippet with caret

### Runtime
- Typed errors: `StackUnderflow`, `LocalTypeMismatch`, `DivisionByZero`, etc.
- VM error formatter shows: PC, opcode, stack state, local state, source snippet

### Type Safety
Opcodes encode expected types. Runtime validates:
```rust
LoadLocalVec3(5)  // expects locals[5] to be Vec3
                  // panics with LocalTypeMismatch if not
```

## Current Status

**Complete**:
- ✅ Vec3/Vec4 math types
- ✅ Module renamed expr → lpscript
- ✅ VM directory structure
- ✅ Typed OpCode enum
- ✅ Local variable system
- ✅ Error types (compile + runtime)
- ✅ AST with Type and Span
- ✅ Lexer with span tracking + GLSL literals
- ✅ Parser with vector constructors

**In Progress**:
- ⏳ Type checker
- ⏳ Full codegen (LpsProgram generation)
- ⏳ LpsVm executor
- ⏳ Opcode implementations
- ⏳ Test utilities and comprehensive tests

## Examples

```glsl
// Simple per-pixel math (0..1 normalized coordinates)
sin(time) * 0.5 + 0.5

// Perlin noise
cos(perlin3(xNorm*0.3, yNorm*0.3, time, 3))

// Ternary operator
centerDist < 0.5 ? 1.0 : 0.0

// Vector operations (planned)
vec2 uv = vec2(xNorm, yNorm);
float pattern = dot(uv, vec2(0.5, 0.5));
```

## Memory Constraints

ESP32 targets:
- **RAM**: ~100KB for engine
- **Storage**: 2MB for programs + data
- **Stack**: 64 slots × 4 bytes = 256 bytes
- **Locals**: As needed (textures, arrays, scratch)

Design optimizes for these constraints while maintaining expressiveness.

