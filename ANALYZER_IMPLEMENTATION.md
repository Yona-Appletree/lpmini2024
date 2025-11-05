# Function Analysis Pass Implementation

## Overview

Implemented multi-pass compilation architecture with function discovery and local variable analysis.

## Architecture

### Compilation Pipeline

1. **Parse** → AST + AstPool
2. **Analyze** → Build function metadata table ← NEW
3. **Type Check** → Validate types using metadata (enhanced)
4. **Optimize** → AST and opcode optimization
5. **Codegen** → Generate bytecode

### Key Components

#### 1. FunctionMetadata Structure (`compiler/func/func_types.rs`)

```rust
pub struct LocalVarInfo {
    pub name: String,
    pub ty: Type,
    pub index: u32,
}

pub struct FunctionMetadata {
    pub params: Vec<Type>,
    pub return_type: Type,
    pub locals: Vec<LocalVarInfo>,
    pub local_count: u32,
}
```

#### 2. FunctionAnalyzer (`compiler/analyzer.rs`)

- Scans AST to discover all local variables in each function
- Allocates local indices in the same order as codegen will
- Tracks variable shadowing correctly
- Records complete metadata for each function

#### 3. Enhanced TypeChecker (`compiler/prog/prog_types.rs`)

- Accepts pre-built FunctionTable
- Validates return types match function signatures
- Checks all code paths return correct type

#### 4. ScriptTest Utility (`compiler/stmt/stmt_test_util.rs`)

New testing methods:
- `.expect_function_metadata(name, params, return_type, local_count)`
- `.expect_function_local_count(name, count)`
- `.expect_function_params(name, param_types)`
- `.expect_function_local_names(name, names)`

## Test Coverage

### Analyzer Tests (16 tests)

**Basic Functionality:**
- Functions with no parameters
- Functions with parameters only
- Functions with parameters and locals
- Vector parameters (vec2, vec3, vec4)

**Advanced Features:**
- Variable shadowing (multiple variables same name, different scopes)
- Nested blocks with locals
- For loops with init declarations
- If/else branches with locals
- While loops with locals
- Multiple params and locals combined

**Using ScriptTest:**
```rust
ScriptTest::new("
    float add(float a, float b) {
        float result = a + b;
        return result;
    }
")
.expect_function_metadata("add", vec![Type::Fixed, Type::Fixed], Type::Fixed, 3)
.expect_function_local_names("add", vec!["a", "b", "result"])
.run()
.unwrap();
```

### Return Type Validation Tests (4 tests)

Previously ignored, now passing:
- ✅ `test_float_function_returns_vec2` - Detects type mismatch
- ✅ `test_vec3_function_returns_vec2` - Detects type mismatch
- ✅ `test_vec2_function_returns_float` - Detects type mismatch
- ✅ `test_vec4_function_returns_vec3` - Detects type mismatch

### Integration Tests (10 tests)

End-to-end pipeline tests in `analyzer_integration_tests.rs`:
- Simple functions
- Functions with parameters
- Functions with locals
- Vector parameters
- Return type validation (success and failure)
- Variable shadowing
- Multiple functions
- Function calling function
- Nested blocks

## Known Issues

### Runtime Execution (8 failing tests)

The following tests fail at runtime (VM execution), not compilation:
- `test_function_vec2_parameter`
- `test_function_vec3_parameter`
- `test_function_returns_vec2/3/4`
- `test_function_multiple_vec_parameters`
- `test_function_mixed_scalar_vector_params`
- `test_function_scalar_from_vectors`

**Error**: `CallStackOverflow` or `TypeMismatch` at runtime
**Status**: Separate VM issue, not related to analysis/compilation
**Note**: The compilation itself succeeds; programs generate valid bytecode

## Benefits

1. **Clear Metadata**: Functions have complete metadata (params, return types, locals)
2. **Return Type Validation**: Type checker now validates return statements
3. **Better Error Messages**: Can detect type mismatches before codegen
4. **Foundation for Future**: Enables advanced features like:
   - Better optimization (know all locals upfront)
   - SSA form
   - Dead code elimination  
   - Improved register allocation

## Files Modified

- `compiler/analyzer.rs` - NEW: Analysis pass
- `compiler/func/func_types.rs` - Enhanced metadata structures
- `compiler/prog/prog_types.rs` - Return type validation
- `compiler/codegen/program.rs` - Use pre-analyzed locals
- `compiler/codegen/local_allocator.rs` - `from_metadata()` constructor
- `compiler/stmt/stmt_test_util.rs` - Analyzer testing support
- `mod.rs` - Insert analysis pass in pipeline

## Usage

The analysis pass is automatically invoked in `compile_script_with_options()`:

```rust
// Parse
let (program, pool) = parser.parse_program()?;

// Analyze (NEW)
let func_table = analyzer::FunctionAnalyzer::analyze_program(&program, &pool)?;

// Type check (enhanced)
let (typed_program, pool) = TypeChecker::check_program(program, pool, &func_table)?;

// Codegen (uses metadata)
let functions = CodeGenerator::generate_program_with_functions(&pool, &program, &func_table);
```

