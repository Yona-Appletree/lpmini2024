# LPScript TODO

## Critical Issues

### Compiler Bugs

- **Control Flow**: If statements generate invalid bytecode
    - [ ] `tests/control_flow.rs`: 5 ignored tests related to if statements and loops
- **Loops**: While/for loops generate infinite bytecode
    - [ ] `tests/control_flow.rs`: Multiple ignored tests
    - [ ] `tests/variables.rs`: Loop-related tests ignored
    - [ ] `tests/functions.rs`: Loop tests ignored
- **Variable Scoping**: Block scoping generates invalid bytecode
    - [ ] `tests/variables.rs`: Nested scopes and block scoping tests ignored
- **Stack Overflow**: Compiler crashes on certain patterns
    - [ ] `tests/variables.rs`: 2 ignored tests for stack overflow
- **Negative Literals**: Parser treats `-5.0` as `0 - 5.0` (affects AST tests)
    - [ ] `compiler/expr/literals/literals_tests.rs`: test_negative_literal fails
    - [ ] `compiler/expr/call/call_tests.rs`: test_function_call_abs fails (due to negative literal in test)

### Parser Issues

- **Assignment Expression Recursion**: Parser fails on assignment expressions
    - [ ] `parser/expr/assign_expr.rs`: 2 ignored tests
    - Need to fix assignment and chained assignment parsing

### VM/Function Issues

- **Function Execution**: Multiple issues with function calls
    - [ ] `tests/functions.rs`: Vec parameters need special handling (multiple stack values)
    - [ ] `tests/functions.rs`: Recursive function execution broken
    - [ ] `tests/functions.rs`: Multiple function execution issues
    - [ ] `tests/functions.rs`: Function return value propagation broken
    - [ ] `tests/functions.rs`: Vec return types need more VM opcodes
- **Floating Point Precision**: Fixed-point math tolerance issues
    - [ ] `compiler/expr/call/call_tests.rs`: test_function_call_cos (expected 1.0, got 0.9996948)
    - [ ] `compiler/expr/call/call_tests.rs`: test_function_call_nested (precision drift in nested calls)

## Implementation TODOs

### Codegen

- [ ] `codegen/expr/binary.rs`: Add proper pow implementation
- [ ] `codegen/expr/binary.rs`: Implement proper modulo operation (currently placeholder)
- [ ] `codegen/expr/swizzle.rs`: Implement general vec3/vec4 swizzling (line 107-114)
    - Currently only vec2 swizzles work
    - [ ] `compiler/expr/swizzle/swizzle_tests.rs`: test_swizzle_two_components fails (vec3.xy)
- [ ] `codegen/expr/variable.rs`: Need type information to use correct Load opcode
- [ ] `codegen/expr/literals.rs`: Keep integers as int32 instead of converting to fixed point

### VM Executor

- [x] `vm/executor.rs`: Refactor run() to return Vec<Fixed> for vector support (COMPLETED)
    - [x] Added run_scalar(), run_vec2(), run_vec3(), run_vec4() convenience methods
    - [x] Updated all test utilities to use appropriate run_*() methods
    - [x] All vector constructor tests now pass (7/7)
- [ ] `vm/executor.rs`: Could add frame pointer for local variables
- [ ] `vm/executor.rs`: Pass actual width/height instead of placeholders (line 292)
- [ ] `vm/executor.rs`: Get actual opcode name for debugging (line 963)

### VM Opcodes

- [ ] `vm/opcodes/arrays.rs`: Implement actual array access (lines 25, 53)
- [ ] `vm/opcodes/textures.rs`: Implement actual texture sampling (lines 26, 55)

### Typechecker

- [ ] `typechecker/mod.rs`: Verify all code paths return a value (if return_type != Void)

## Ignored Tests Summary

### tests/control_flow.rs (7 tests, 7 ignored)

1. `test_if_statement` - if statements generate invalid bytecode
2. `test_if_else` - if statements generate invalid bytecode
3. `test_while_loop` - while loops generate infinite bytecode
4. `test_for_loop` - loops generate infinite bytecode
5. `test_break` - loops generate infinite bytecode
6. `test_continue` - loops generate infinite bytecode
7. `test_nested_if` - if statements generate invalid bytecode

### tests/variables.rs (8 tests, 5 ignored)

1. `test_block_scope` - block scoping generates invalid bytecode
2. `test_nested_scopes` - nested scopes generate invalid bytecode
3. `test_variable_reassignment_in_loop` - loops generate infinite bytecode
4. `test_shadowing_across_functions` - stack overflow in compiler
5. `test_multiple_shadowing` - stack overflow in compiler

### tests/functions.rs (9 tests, 6 ignored)

1. `test_function_with_vec_param` - Vec parameters need special handling
2. `test_recursive_function` - recursive function execution broken
3. `test_multiple_functions` - multiple function execution broken
4. `test_function_composition` - return value propagation issue
5. `test_function_with_loop` - loops generate infinite bytecode
6. `test_function_returning_vec` - Vec return types need more opcodes

### parser/expr/assign_expr.rs (2 tests, 2 ignored)

1. `test_simple_assignment` - assignment expression parser recursion
2. `test_chained_assignment` - chained assignment recursion

### vm/opcodes/locals.rs (4 tests, 1 ignored)

1. `test_locals_auto_grow` - Auto-grow removed to prevent memory leaks

## Codegen Tests Added

### Expression Codegen (All files now have tests)

- [x] `codegen/expr/literals.rs` - 6 tests (5 passing, 1 failing due to negative literal parsing)
- [x] `codegen/expr/binary.rs` - 25 tests (Fixed/Vec2/Vec3/Vec4 arithmetic), 2 ignored (pow, mod) ✅ ALL PASSING
- [x] `codegen/expr/comparison.rs` - 16 tests (all comparison operators with bytecode + execution) ✅ ALL PASSING
- [x] `codegen/expr/logical.rs` - 9 tests (&&, || with bytecode + execution) ✅ ALL PASSING
- [x] `codegen/expr/variable.rs` - 8 tests (uv, coord, time, local variables) ✅ ALL PASSING
- [x] `codegen/expr/constructors.rs` - 8 tests (vec2/vec3/vec4 constructors, GLSL-style) ✅ ALL PASSING
- [x] `codegen/expr/swizzle.rs` - 10 tests (9 passing, 1 failing due to unimplemented vec3->vec2 swizzle)
- [x] `codegen/expr/ternary.rs` - 6 tests (ternary operator, nested ternaries) ✅ ALL PASSING
- [x] `codegen/expr/call.rs` - 20 tests (18 passing, 2 failing due to floating point precision)
- [x] `codegen/expr/assign_expr.rs` - 4 tests + 1 ignored (assignment expressions) ✅ ALL PASSING

### Statement Codegen (All files now have tests)

- [x] `codegen/stmt/var_decl.rs` - 5 tests (variable declarations with/without init) ✅ ALL PASSING
- [x] `codegen/stmt/assign.rs` - 4 tests (simple and complex assignments) ✅ ALL PASSING
- [x] `codegen/stmt/expr_stmt.rs` - 3 tests (expression statements with Drop opcode) ✅ ALL PASSING
    - [x] Fixed: Expression statements now properly drop unused results based on type
- [x] `codegen/stmt/return_stmt.rs` - 4 tests (return with literals, expressions, variables) ✅ ALL PASSING
- [x] `codegen/stmt/block.rs` - 2 tests + 2 ignored (simple blocks, scoping issues)
- [x] `codegen/stmt/if_stmt.rs` - 1 test + 5 ignored (if/else bytecode generation issues)
- [x] `codegen/stmt/while_loop.rs` - 1 test + 2 ignored (while loops generate infinite bytecode)
- [x] `codegen/stmt/for_loop.rs` - 1 test + 3 ignored (for loops generate infinite bytecode)

### Program-Level Codegen

- [x] `codegen/local_allocator.rs` - 7 tests (variable allocation and lookup)
- [ ] `codegen/program.rs` - Tested via integration tests
- [ ] `codegen/functions.rs` - Tested via integration tests
- [ ] `codegen/native_functions.rs` - Tested indirectly via call tests

### Test Results Summary

**Compiler Tests: 95 of 100 passing** (5 failures, 13 ignored)

**Current Failures** (5 tests):
1. `test_negative_literal` - Parser generates `0 - 5.0` instead of `-5.0`
2. `test_function_call_abs` - AST mismatch due to negative literal in test
3. `test_function_call_cos` - Floating point precision (0.9996948 vs 1.0)
4. `test_function_call_nested` - Floating point precision drift
5. `test_swizzle_two_components` - Vec3->Vec2 swizzle not implemented

**Ignored Tests** (13 tests):
- 7 control flow tests (if statements, loops generate invalid bytecode)
- 5 variable scoping tests (block scoping issues, stack overflow)
- 1 vec2 variable declaration test

## Files Without Tests (Updated)

### Typechecker

- [ ] `typechecker/func_table.rs` (0 tests)
- [ ] `typechecker/symbols.rs` (0 tests)

### VM

- [ ] `vm/mod.rs` (0 tests)
- [ ] `vm/program.rs` (0 tests)
- [ ] `vm/locals/mod.rs` (0 tests)
- [ ] `vm/locals/types.rs` (0 tests)

### Other

- [ ] `error.rs` (0 tests)
- [ ] `ast.rs` (0 tests)
- [ ] `parser/expr/mod.rs` (0 tests)
- [ ] `parser/stmt/mod.rs` (0 tests)
- [ ] `vm/opcodes/mod.rs` (0 tests)

## Files With Minimal Tests (1-2 tests)

### Parser Statements

- [ ] `parser/stmt/var_decl.rs` (1 test)
- [ ] `parser/stmt/while_loop.rs` (1 test)
- [ ] `parser/stmt/for_loop.rs` (1 test)
- [ ] `parser/stmt/expr_stmt.rs` (1 test)
- [ ] `parser/stmt/assign.rs` (1 test)
- [ ] `parser/stmt/return_stmt.rs` (1 test)
- [ ] `parser/stmt/if_stmt.rs` (2 tests)
- [ ] `parser/stmt/block.rs` (2 tests)

### Parser Expressions

- [ ] `parser/expr/constructors.rs` (1 test)
- [ ] `parser/expr/call.rs` (1 test)
- [ ] `parser/expr/variable.rs` (1 test)
- [ ] `parser/expr/swizzle.rs` (1 test)
- [ ] `parser/expr/comparison.rs` (1 test)
- [ ] `parser/expr/ternary.rs` (1 test)
- [ ] `parser/expr/assign_expr.rs` (2 tests, both ignored)
- [ ] `parser/expr/literals.rs` (2 tests)
- [ ] `parser/expr/logical.rs` (2 tests)

### VM Opcodes

- [ ] `vm/opcodes/arrays.rs` (3 tests)
- [ ] `vm/opcodes/textures.rs` (3 tests)

## Priority Order

1. **Fix Remaining Test Failures** (5 tests)
    - [ ] Fix negative literal parsing (`-5.0` should parse as negative number, not `0 - 5.0`)
    - [ ] Relax floating point tolerance for Fixed-point math tests
    - [ ] Implement vec3/vec4 swizzle extraction to vec2

2. **Fix Compiler Bugs** - Critical blockers preventing 13 ignored tests from running
    - Control flow (if/else statements)
    - Loop bytecode generation
    - Stack overflow issues
    - Variable scoping

3. **Fix Function Execution** - Functions are core feature, need to work properly
    - Vec parameters and return types
    - Recursive functions
    - Multiple function definitions

4. **Implement Missing Features** - Arrays, textures, proper operators
    - Pow implementation
    - Modulo operation
    - Array access
    - Texture sampling

5. **Expand Parser Test Coverage** - Many parser modules have only 1-2 tests

## Recent Completions

- [x] **VM Vector Return Support** (Dec 2024)
    - Refactored `LpsVm::run()` to return `Vec<Fixed>` for proper vector support
    - Added `run_scalar()`, `run_vec2()`, `run_vec3()`, `run_vec4()` convenience methods
    - Updated all test utilities to call appropriate run method based on expected type
    - All 7 vector constructor tests now pass
    
- [x] **Expression Statement Fix** (Dec 2024)
    - Fixed expression statements to drop unused results based on type
    - Prevents stack pollution when expressions used for side effects only
    - All 3 expression statement tests now pass
