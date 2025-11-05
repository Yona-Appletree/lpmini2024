## Missing features

- [ ] matrices
- [x] not operator (!), bitwise operators
  - [x] Bitwise: &, |, ^, ~, <<, >> (Int32 only)
  - [x] Logical NOT: ! (already implemented)
  - [x] Increment/Decrement: ++, -- (prefix and postfix)
  - [x] Compound assignments: +=, -=, \*=, /=, %=, &=, |=, ^=, <<=, >>=
  - [x] Removed ^ as exponentiation (use pow() function instead)
  - [x] Fixed StoreLocal/LoadLocal to use Int32 opcodes for Int32 types
  - [ ] **KNOWN BUG**: Compound assignments and increment/decrement cause infinite loop during parsing
    - Opcodes generate correctly for bitwise operators
    - Need to debug why statement parsing enters infinite loop

- [x] ~~store local should not be "fixed" it should probably be int32 or something~~ FIXED
  - Now uses StoreLocalInt32/LoadLocalInt32 for Int32 types
  - Fixed in: var_decl_gen.rs, assign_gen.rs, assign_expr_gen.rs, variable_gen.rs
- [ ] get rid of LoadSource and such
- [ ] outputs like glsl
- [ ] three operation modes (expr, script, shader)
- [ ] use of the term scalar vs fixed. many places should be scalar, where we don't know if its
      int32 or fixed, like in the stack.

## Safety

- [ ] compiler limits since we're on embedded

## Nice to have features

- [ ] debugging outputs

## Organization

- [x] compiler/runtime errors need to be separated
- [ ] lpscript should be a library, runtime and compiler should be separate crates

## Issues

- [ ] remove uv built-in

## Optimization

- [x] AST optimization
  - [x] Constant folding (arithmetic, math functions, comparisons, logical)
  - [x] Algebraic simplification (x*1, x+0, x*0, double negation, etc.)
  - [x] Dead code elimination (unreachable statements after return, constant if conditions)
  - [x] Multi-pass fixed-point iteration
  - [x] Configurable via `OptimizeOptions`
- [x] Opcode optimization
  - [x] Peephole patterns (Push/Drop, LoadLocal/StoreLocal same index, Dup/Drop)
  - [x] Dead code after unconditional jumps
  - [x] Configurable via `OptimizeOptions`

## Tests

- [ ] tests with vector types
- [ ] vector tests for functions (smoothstep)
- [ ] type error testing in \_types.rs files
- [ ] test for protections against stack overflows, memory usage, etc.
- [ ] integration tests against real glsl shaders

## Error handling

- [ ] needs testing and probably a lot of work

## Other

- [ ] how big is this on the esp32?
- [ ] some performance testing
