## Missing features

- not operator (!)
- remove uv built-in
- bitwise operators
- store local should not be "fixed" it should probably be int32 or something:
  ```
  Type::Bool | Type::Fixed | Type::Int32 => code.push(LpsOpCode::StoreLocalFixed(i as u32))
  ```
- get rid of LoadSource and such
- outputs like glsl

## Organization

- compiler/runtime errors need to be separated
- lpscript should be a library, runtime and compiler should be separate crates

## Optimization

- ast optimization
- opcode optimization

## Tests

- test for protections against stack overflows, memory usage, etc.
- test expressions with vector types
- test functions with vector types

## Error handling

- needs testing and probably a lot of work