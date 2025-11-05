## Missing features

- not operator (!)
- remove uv built-in
- bitwise operators
- store local should not be "fixed" it should probably be int32 or something:
  ```
  Type::Bool | Type::Fixed | Type::Int32 => code.push(LpsOpCode::StoreLocalFixed(i as u32))
  ```
- get rid of LoadSource and such

## Tests

- test for protections against stack overflows, memory usage, etc.
- 