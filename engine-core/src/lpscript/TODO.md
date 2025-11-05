# LPScript TODO

## Current Status

**Test Results**: ~620+ passing tests, 4 failing, 8 ignored

### Recent Fixes (Jan 2025)

- **✅ Integer Type System**
  - Added `Int32ToFixed` and `FixedToInt32` opcodes for bidirectional conversion
  - Fixed integer literal handling to preserve Int32 semantics
  - Fixed bitwise operators to work with raw Int32 values (8 tests)
  - Fixed power function to convert exponent from Fixed to i32 (2 tests)

- **✅ Control Flow Tests**
  - Converted if/while/for statement tests from opcode-checking to integration style
  - All control flow functionality works correctly (11 tests)

- **✅ Assignment Expression Tests**
  - Fixed test expectations to not require exact AST matching after type inference

## Known Issues

### Failing Tests (4 tests)

1. **`test_percent_eq_assignment`** - Modulo compound assignment edge case
   - Issue: `x %= 3.0` produces incorrect result
   - May be related to modulo function implementation with Fixed-point math

2. **`test_if_with_variable`** - TypeMismatch runtime error
   - Issue: `float x = 0.3; if (x > 0.5) {...}` fails with TypeMismatch at pc=9
   - May be related to type promotion or comparison codegen

3. **`demo_program::test_yint_load`** - Graphics rendering test
   - Issue: YInt values not being loaded correctly
   - Low priority - demo/graphics specific

4. **`demo_program::test_normalized_center_line`** - Graphics rendering test
   - Issue: Center row assertion fails
   - Low priority - demo/graphics specific

### Ignored Tests (8 tests)

#### Not Implemented Features (4 tests)

- `compiler/func/func_types.rs` - Function return type validation not implemented
  - Need to add validation that all code paths return correct type
  - 4 tests ignored

#### Pre-existing Bugs (2 tests)

- `compiler/expr/incdec/incdec_tests.rs` - Compound assignment has issues
  - 1 test ignored: `test_compound_assignment_opcodes`
- `tests/variables.rs` - Loops generate infinite bytecode
  - 1 test ignored: `test_variable_reassignment_in_loop`
  - Compiler bug in loop code generation

#### Needs Update (2 tests)

- `test_engine/mapping/sample.rs` - Needs update for integer-only circular_panel
  - 1 test ignored
- `test_engine/pipeline/rgb_utils.rs` - Needs fix for integer-only min/max
  - 1 test ignored

### Stack Overflow in Test Suite

When running all tests together, the test runner encounters a stack overflow. This appears to be a test runner limitation rather than a code issue, as individual tests pass when run separately. Tests that trigger this:

- `test_nested_blocks`
- `test_while_loop_sum`
- `test_for_loop_*` tests
- `test_if_else_chain`

**Workaround**: Run tests in smaller batches or skip these tests when running full suite.

## Implementation TODOs

### VM Executor

- [ ] `vm/executor.rs`: Could add frame pointer for local variables
- [ ] `vm/executor.rs`: Pass actual width/height instead of placeholders
- [ ] `vm/executor.rs`: Get actual opcode name for debugging

### VM Opcodes

- [ ] `vm/opcodes/arrays.rs`: Implement actual array access
- [ ] `vm/opcodes/textures.rs`: Implement actual texture sampling

### Typechecker

- [ ] `typechecker/mod.rs`: Implement function return type validation
  - Need to verify all code paths return correct type
  - Currently validates that paths exist but not their types
  - 4 tests waiting for this feature

### Modulo Operation

- [ ] Investigate modulo function accuracy with Fixed-point math
  - Current implementation: `x - (x / y) * y`
  - May have precision issues with certain values
  - Related to `test_percent_eq_assignment` failure

## Files Without Tests

### Typechecker

- [ ] `typechecker/func_table.rs`
- [ ] `typechecker/symbols.rs`

### VM

- [ ] `vm/mod.rs`
- [ ] `vm/program.rs`
- [ ] `vm/locals/mod.rs`
- [ ] `vm/locals/types.rs`

### Other

- [ ] `error.rs`
- [ ] `ast.rs`
- [ ] `parser/expr/mod.rs`
- [ ] `parser/stmt/mod.rs`
- [ ] `vm/opcodes/mod.rs`

### VM Opcodes

- [ ] `vm/opcodes/arrays.rs` (3 tests)
- [ ] `vm/opcodes/textures.rs` (3 tests)

## GLSL Compatibility and Limitations

LPS aims to be a strict subset of GLSL, meaning valid LPS programs should compile as GLSL. However, some GLSL features are not yet implemented:

### ✅ Supported Vector Operations

- **Component-wise arithmetic**: `vec + vec`, `vec - vec`, `vec * vec`, `vec / vec`
- **Vector-scalar operations**: `vec * scalar`, `scalar * vec`, `vec / scalar`
- **Vector functions**: `length()`, `normalize()`, `dot()`, `cross()` (vec3 only), `distance()`
- **Vector constructors**: `vec2(f,f)`, `vec3(vec2,f)`, `vec4(vec3,f)`, etc. (GLSL-style)
- **Vector swizzling**: `.xy`, `.rgb`, `.xyzw`, etc.
- **Bitwise operators**: `&`, `|`, `^`, `~`, `<<`, `>>` (Int32 only)
- **Increment/decrement**: `++`, `--` (prefix and postfix, scalar only)
- **Compound assignments**: `+=`, `-=`, `*=`, `/=`, `%=`, `&=`, `|=`, `^=`, `<<=`, `>>=`

### ❌ GLSL Features NOT Implemented

#### 1. Scalar / Vector Division

GLSL allows: `float / vec` → broadcast division

```glsl
// GLSL: valid
vec2 result = 1.0 / vec2(2.0, 4.0);  // → vec2(0.5, 0.25)

// LPS: NOT SUPPORTED
// Workaround: vec2(1.0, 1.0) / vec2(2.0, 4.0)
```

#### 2. Component-wise Math Functions

GLSL allows math functions on vectors (applied component-wise):

```glsl
// GLSL: valid
vec2 result = sin(vec2(0.0, 1.57));  // → vec2(0.0, 1.0)
vec3 abs_values = abs(vec3(-1, 2, -3));  // → vec3(1, 2, 3)

// LPS: NOT SUPPORTED
// Workaround: manual swizzle operations
vec2 v = vec2(0.0, 1.57);
vec2 result = vec2(sin(v.x), sin(v.y));
```

Functions affected: `sin`, `cos`, `tan`, `abs`, `floor`, `ceil`, `sqrt`, `sign`, `frac`, `saturate`

#### 3. Vector Comparison Functions

GLSL has component-wise comparison functions returning boolean vectors:

```glsl
// GLSL: valid
bvec2 result = lessThan(vec2(1,3), vec2(2,2));  // → bvec2(true, false)
bvec3 equals = equal(vec3(1,2,3), vec3(1,2,4));  // → bvec3(true, true, false)

// LPS: NOT SUPPORTED
// Workaround: use scalar comparisons
```

#### 4. Mix/Lerp with Vector Blend Factor

GLSL allows component-wise blending:

```glsl
// GLSL: valid
vec3 result = mix(vec3(0,0,0), vec3(1,1,1), vec3(0.5, 0.25, 0.75));

// LPS: Only supports scalar blend factor
vec3 result = mix(vec3(0,0,0), vec3(1,1,1), 0.5);  // Works
```

#### 5. Unary Negation on Vectors

GLSL allows: `-vec` → negate all components

```glsl
// GLSL: valid
vec2 negated = -vec2(1.0, 2.0);  // → vec2(-1.0, -2.0)

// LPS: NOT IMPLEMENTED
// Workaround: vec2(-1.0, -2.0) or vec2(0,0) - vec2(1,2)
```

#### 6. Ternary with Vector Results

The Select opcode only handles single stack values:

```glsl
// GLSL: valid
vec2 result = condition ? vec2(1,0) : vec2(0,1);

// LPS: NOT SUPPORTED - Select opcode limitation
// Workaround: Use separate scalar ternaries
vec2 result = vec2(
    condition ? 1.0 : 0.0,
    condition ? 0.0 : 1.0
);
```

### Future GLSL Feature Additions

Planned for future implementation:

- Component-wise math functions (`sin(vec)`, `abs(vec)`, etc.)
- Vector comparison functions (`lessThan`, `equal`, etc.)
- Unary negation for vectors
- Scalar / vector division
- Ternary operator with vector results (requires Select2/Select3/Select4 opcodes)
- Function return type validation

## Type System

### Integer vs Fixed-point

LPS uses two numeric types:

- **Int32**: Raw 32-bit integers for bitwise operations, array indices
- **Fixed**: 16.16 fixed-point for all floating-point math

#### Type Conversion Opcodes

- `Int32ToFixed`: Converts raw int32 to Fixed format (multiply by 2^16)
- `FixedToInt32`: Converts Fixed to raw int32 (divide by 2^16, truncate)

#### Automatic Promotion

Integer literals are parsed as Int32 but automatically promoted to Fixed when used in expressions with Float types:

```glsl
int x = 42;        // Stored as raw Int32
float y = x + 1.0; // x promoted to Fixed, result is Fixed
```

The compiler emits `Int32ToFixed` opcode when promotion is needed.
