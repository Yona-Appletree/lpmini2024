# lp-gfx Crate Implementation

## Overview

The `lp-gfx` crate provides a graphics abstraction layer for managing textures and executing shaders. The crate uses a trait-based design (`GfxContext`) with fully isolated implementations: `CpuContext` (using lp-script VM) and `GpuContext` (using miniquad/OpenGL).

**Note**: lp-script (the shader language compiler and VM) is part of lp-gfx, located at `src/gfx_context/lp_script/`. It's not a separate crate - it's integrated into the graphics system.

## Structure

### Crate Setup

- Create `crates/lp-gfx/` with standard Rust crate structure
- Add to workspace `Cargo.toml`
- Features:
  - `cpu` (always enabled) - CPU/software rendering
  - `opengl` (optional, disabled by default) - GPU/OpenGL rendering via miniquad
  - `std` (for tests) - Standard library support
- Default build: `no_std` compatible, `cpu` only
- Tests: Enable `std` + `opengl` by default

### Core Types (Top-Level Traits and Types)

**Texture Format Enum** (`src/texture_format.rs`):

```rust
pub enum TextureFormat {
    RGBA8,   // 8 bits per channel (R, G, B, A) - 32 bits per pixel total
    Dec32,   // 32-bit decimal greyscale (single channel, Dec32 format)
    Mono1,   // 1-bit per channel monochrome - 1 bit per pixel total
}

// IMPORTANT: OpenGL requires format specification at creation time.
// Format includes both layout (RGBA vs single channel) AND bit depth/type.
// Cannot reinterpret later - format is fixed at creation.
// For GPU context, this maps to OpenGL internal formats:
// - RGBA8 -> GL_RGBA8 (or miniquad equivalent)
// - Dec32 -> GL_R32F (32-bit float, single channel)
// - Mono1 -> GL_R8 (8-bit single channel, stored efficiently)
```

**Texture Reference** (`src/texture_ref.rs`):

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TexRef {
    id: u32,  // Wrapper around integer ID (matches OpenGL texture IDs)
}
```

**Shader Reference** (`src/shader_ref.rs`):

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShaderRef {
    id: u32,  // Wrapper around integer ID for shader programs
}
```

**GfxContext Trait** (`src/gfx_context.rs`):

```rust
pub trait GfxContext {
    // Texture management
    // Format MUST be specified at creation - OpenGL cannot reinterpret later
    fn create_texture(&mut self, width: usize, height: usize, format: TextureFormat) -> Result<TexRef, GfxError>;
    fn delete_texture(&mut self, texture: TexRef) -> Result<(), GfxError>;

    // Texture metadata access
    fn get_texture_format(&self, texture: TexRef) -> Result<TextureFormat, GfxError>;
    fn get_texture_size(&self, texture: TexRef) -> Result<(usize, usize), GfxError>;

    // Texture sampling (for use in shaders)
    // Sample texture at normalized UV coordinates (0.0-1.0)
    fn sample_texture(&self, texture: TexRef, u: Dec32, v: Dec32) -> Result<Dec32, GfxError>;  // Single channel
    fn sample_texture_rgba(&self, texture: TexRef, u: Dec32, v: Dec32) -> Result<(Dec32, Dec32, Dec32, Dec32), GfxError>;  // RGBA

    // Shader compilation and execution
    fn compile_shader(&mut self, source: &str) -> Result<ShaderRef, GfxError>;
    fn delete_shader(&mut self, shader: ShaderRef) -> Result<(), GfxError>;
    fn execute_shader(&mut self, shader: ShaderRef, output: TexRef, inputs: &[TexRef], time: Dec32) -> Result<(), GfxError>;

    // Texture data access
    // CPU context: direct access to underlying buffers
    fn get_texture_data(&self, texture: TexRef) -> Result<&[u8], GfxError>;
    fn get_texture_data_mut(&mut self, texture: TexRef) -> Result<&mut [u8], GfxError>;

    // GPU context: download texture data from GPU to CPU
    // Returns raw texture data in the texture's format
    fn download_texture(&self, texture: TexRef, output: &mut [u8]) -> Result<(), GfxError>;
}
```

### Implementations (Implementation-First Structure)

**CpuContext** (`src/cpu/cpu_context.rs`):

- Uses lp-script VM (from `gfx_context::lp_script`) for shader execution
- Implements GfxContext trait
- Executes shaders pixel-by-pixel using `execute_program_lps` / `execute_program_lps_vec3`
- Compiles shaders using `lp_script::compile_expr` / `lp_script::compile_script`

**CpuTexture** (`src/cpu/cpu_texture.rs`):

- Stores textures as in-memory buffers (Vec<u8> or Vec<Dec32>)
- Handles texture format conversions
- Tracks width, height, format

**GpuContext** (`src/gpu/gpu_context.rs`):

- Uses miniquad for OpenGL/WebGL
- Manages OpenGL shader programs
- Compiles GLSL shaders (may need lp-script → GLSL translation layer later)
- Executes shaders on GPU
- Feature-gated behind `opengl` feature

**GpuTexture** (`src/gpu/gpu_texture.rs`):

- Manages OpenGL textures via miniquad
- Tracks width, height, format
- Handles texture format mapping to OpenGL internal formats

### Error Types

**GfxError** (`src/gfx_error.rs`):

- Texture creation failures
- Shader compilation errors
- Invalid texture references
- Format mismatches
- Runtime execution errors

### Module Structure (Implementation-First)

```
crates/lp-gfx/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   │
│   ├── # Top-level traits and shared types
│   ├── gfx_context.rs      # GfxContext trait
│   ├── texture_format.rs   # TextureFormat enum
│   ├── texture_ref.rs      # TexRef wrapper
│   ├── shader_ref.rs       # ShaderRef wrapper
│   └── gfx_error.rs        # GfxError enum
│   │
│   ├── # Graphics context implementation (shader language)
│   ├── gfx_context/
│   │   ├── mod.rs          # Graphics context module exports
│   │   └── lp_script/      # lp-script shader language (compiler + VM)
│   │       ├── mod.rs
│   │       ├── compiler/   # Shader compiler (lexer, parser, typechecker, codegen)
│   │       ├── vm/         # Virtual machine for shader execution
│   │       └── shared/     # Shared types (Type, Span, etc.)
│   │
│   ├── # CPU implementation
│   ├── cpu/
│   │   ├── mod.rs          # CPU module exports
│   │   ├── cpu_context.rs  # CpuContext implementation (uses lp_script VM)
│   │   └── cpu_texture.rs  # CPU texture storage
│   │
│   └── # GPU implementation (feature-gated)
│       └── gpu/
│           ├── mod.rs          # GPU module exports
│           ├── gpu_context.rs # GpuContext implementation
│           └── gpu_texture.rs # GPU texture management
│
└── tests/
    ├── util/
    │   └── mod.rs          # Test utilities
    │       # run_shader_test(context) -> TexRef - Runs simple shader on context
    │       # compare_textures(cpu_ctx, gpu_ctx, cpu_tex, gpu_tex) -> bool - Compares two textures
    └── basic_test.rs       # Integration test: CPU vs GPU shader execution
```

## Implementation Details

### Texture Format Requirements

**OpenGL/GPU Context:**

- Format must be specified at texture creation time
- Format includes both layout (RGBA vs single channel) AND bit depth/type (8-bit vs 32-bit float)
- Cannot reinterpret texture data later - format is fixed
- Mapping to OpenGL internal formats:
  - `RGBA8` → `GL_RGBA8` (8-bit per channel, 32 bits per pixel)
  - `Dec32` → `GL_R32F` (32-bit float, single channel)
  - `Mono1` → `GL_R8` (8-bit single channel, stored efficiently)

**CPU Context:**

- More flexible - can store data in different formats internally
- Still tracks format for API consistency
- May optimize storage (e.g., `Vec<Dec32>` for Dec32 format)

### Texture Storage

- CPU context: Store as `Vec<u8>` for RGBA8/Mono1, `Vec<Dec32>` for Dec32
- GPU context: Use miniquad texture handles with fixed OpenGL formats
- Both track width, height, format for metadata access

### Shader Execution

- **lp-script location**: `src/gfx_context/lp_script/` - Part of the graphics context system
- **CPU**: Uses `gfx_context::lp_script::vm::execute_program_lps` functions
- **GPU**: Compile GLSL via miniquad (may need lp-script → GLSL compiler later)
- Both accept lp-script `LpsProgram` initially
- GPU path may need translation layer (future work)
- Shaders are compiled once and stored with `ShaderRef` for reuse
- lp-script provides the shader language compiler and VM used by CpuContext

### Texture Sampling

- CPU context: Direct pixel lookup with bilinear interpolation
- GPU context: Uses OpenGL texture sampling (hardware-accelerated)
- Both support normalized UV coordinates (0.0-1.0)
- Single channel (Dec32) and RGBA sampling methods

### Texture Download (GPU → CPU)

- GPU context: Download texture data from GPU memory to CPU
- Used for testing/validation (comparing CPU vs GPU results)
- Returns raw texture data in the texture's format
- May be slow - use sparingly

### Isolation

- Contexts are completely independent
- No shared state between implementations
- Trait ensures API compatibility
- Implementation-first structure: each backend (cpu/gpu) in its own directory
- Traits and shared types at top level for easy discovery

## Dependencies

- `lp-math` - For Dec32 type (used by lp-script)
- `miniquad` - For GPU/OpenGL support (feature-gated)
- Note: lp-script is now part of lp-gfx (`src/gfx_context/lp_script/`), not a separate crate

## Testing Strategy

- Unit tests for each context type
- Integration tests comparing CPU vs GPU results (when both available)
- Format conversion tests
- Error handling tests
- Tests enable `std` + `opengl` features

### Integration Test Structure

**Test Utilities** (`tests/util/mod.rs`):

```rust
// Runs a simple shader test on any context, returns output texture
// Uses a checkerboard pattern shader (no texture sampling required)
pub fn run_shader_test<C: GfxContext>(ctx: &mut C) -> Result<TexRef, GfxError> {
    // Create output texture (RGBA8 format, e.g., 64x64)
    let output = ctx.create_texture(64, 64, TextureFormat::RGBA8)?;

    // Compile checkerboard pattern shader
    // Example: "float checker = mod(floor(uv.x * 8.0) + floor(uv.y * 8.0), 2.0); return vec3(checker, checker, checker);"
    // Or simpler: "float c = mod(floor(uv.x * 8.0) + floor(uv.y * 8.0), 2.0); return vec3(c);"
    let shader_source = "float c = mod(floor(uv.x * 8.0) + floor(uv.y * 8.0), 2.0); return vec3(c, c, c);";
    let shader = ctx.compile_shader(shader_source)?;

    // Execute shader with no input textures (empty inputs array)
    ctx.execute_shader(shader, output, &[], Dec32::ZERO)?;

    // Return output texture reference
    Ok(output)
}
```

**Note**: The test shader generates a checkerboard pattern using only built-in variables (`uv`) and math functions. Texture sampling is not yet supported in lp-script, so the test avoids using input textures.

// Compares two textures from different contexts
// Downloads GPU texture if needed, compares pixel-by-pixel
pub fn compare_textures<C1: GfxContext, C2: GfxContext>(
ctx1: &C1,
ctx2: &C2,
tex1: TexRef,
tex2: TexRef,
) -> Result<bool, GfxError> {
// Get texture sizes and formats
// Download GPU texture data if needed
// Compare pixel-by-pixel with tolerance for floating point
// Return true if textures match
}

````

**Basic Integration Test** (`tests/basic_test.rs`):

```rust
#[cfg(all(feature = "std", feature = "opengl"))]
#[test]
fn test_cpu_gpu_shader_match() {
    // Create CPU context
    let mut cpu_ctx = CpuContext::new();

    // Create GPU context
    let mut gpu_ctx = GpuContext::new();

    // Run same shader on both
    let cpu_result = run_shader_test(&mut cpu_ctx).unwrap();
    let gpu_result = run_shader_test(&mut gpu_ctx).unwrap();

    // Compare results
    assert!(compare_textures(&cpu_ctx, &gpu_ctx, cpu_result, gpu_result).unwrap());
}
````

This structure allows:

- Reusable test utilities for running shaders and comparing textures
- Easy addition of more integration tests
- Clear separation between test logic and test cases

## Migration Notes

- Existing `Buffer`/`BufferRef` in `engine-core` remain unchanged (migration strategy: new_only)
- New code can use `lp-gfx` textures
- `TexRef` in `lpcore` may need coordination (check if we reuse or create new)

## Decisions Made

1. **TexRef uses u32** - Matches OpenGL texture ID convention
2. **Texture sampling API included** - Both `sample_texture` (single channel) and `sample_texture_rgba` methods
3. **Shader compilation in trait** - `compile_shader` returns `ShaderRef`, shaders can be deleted with `delete_shader`
4. **Texture download supported** - `download_texture` method for GPU context to transfer data to CPU
5. **ShaderRef type** - Similar to TexRef, wraps u32 ID for shader programs

## Future Work

1. How to handle lp-script → GLSL translation for GPU path? (Needed for full GPU shader support)
2. Texture upload (CPU → GPU) - May be needed for initializing GPU textures from CPU data
