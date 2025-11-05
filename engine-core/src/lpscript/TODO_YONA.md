## Missing features

- [ ] matrices
- [ ] get rid of LoadSource and such
- [ ] outputs like glsl
- [ ] three operation modes (expr, script, shader)
- [ ] use of the term scalar vs fixed. many places should be scalar, where we don't know if its
  int32 or fixed, like in the stack.
- [ ] arrays
- [ ] texture sampling
- [ ] compiler limits since we're on embedded

## Bugs

- [ ] Compiler uses floats in some places instead of Fixed.

## Nice to have features

- [ ] debugging outputs

## Organization

- [ ] locals should be a struct like the stack
- [x] compiler/runtime errors need to be separated
- [ ] lpscript should be a library, runtime and compiler should be separate crates

## Issues

- [ ] remove uv built-in
- [ ] functions should really be stored separately from the main code / no top level code, right?
- [ ] is boolean coericion really allowed in glsl?
- [ ] division by zero error? if b == 0 { return Err(RuntimeError::DivisionByZero); }

## Optimization

- [x] AST optimization
- [x] Opcode optimization
- [ ] Push2,3,4 opcodes? run tests?
-

## Tests

- [ ] tests with vector types
- [ ] vector tests for functions (smoothstep)
- [ ] tests for fixed/int32 to make sure they are't mixed up
- [ ] type error testing in \_types.rs files
- [ ] test for protections against stack overflows, memory usage, etc.
- [ ] integration tests against real glsl shaders

## Error handling

- [ ] needs testing and probably a lot of work

## Other

- [ ] how big is this on the esp32?
- [ ] some performance testing

## Visualizations

start with checkerboard.
A program that uses a perlin to create a sort of magnification
effect as if looking at something through a changing lens.