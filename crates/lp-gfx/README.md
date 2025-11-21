# lp-gfx

Graphics abstraction layer for managing textures and executing shaders.

## Overview

`lp-gfx` provides a graphics abstraction layer with trait-based design (`GfxContext`) supporting both CPU rendering (using lp-script VM) and GPU rendering (using miniquad/OpenGL). It's designed for embedded environments and desktop/web targets.

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

### CPU Context

```rust
use lp_gfx::context::cpu::CpuContext;
use lp_gfx::context::GfxContext;
use lp_gfx::texture::format::TextureFormat;

let mut ctx = CpuContext::new();
let texture = ctx.create_texture(256, 256, TextureFormat::RGBA8).unwrap();
```

### GPU Context (OpenGL)

```rust
use lp_gfx::context::gpu::GpuContext;
use lp_gfx::context::GfxContext;
use lp_gfx::texture::format::TextureFormat;

let mut ctx = GpuContext::new();
let texture = ctx.create_texture(256, 256, TextureFormat::RGBA8).unwrap();
```
