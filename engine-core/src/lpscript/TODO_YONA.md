## Missing features

- [x] not operator (!), bitwise operators
- [ ] remove uv built-in
- [ ] store local should not be "fixed" it should probably be int32 or something:
  ```
  Type::Bool | Type::Fixed | Type::Int32 => code.push(LpsOpCode::StoreLocalFixed(i as u32))
  ```
- [ ] get rid of LoadSource and such
- [ ] outputs like glsl
- [ ] three operation modes (expr, script, shader)

## Nice to have features

- [ ] debugging outputs

## Organization

- [x] compiler/runtime errors need to be separated
- [ ] lpscript should be a library, runtime and compiler should be separate crates

## Optimization

- [ ] ast/opcode optimization

## Tests

- [ ] test for protections against stack overflows, memory usage, etc.
- [ ] test expressions with vector types
- [ ] test functions with vector types
- [ ] integration tests against real glsl shaders

## Error handling

- [ ] needs testing and probably a lot of work

## Other

- [ ] how big is this on the esp32?
- [ ] some performance testing