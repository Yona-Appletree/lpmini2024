/// Function analysis pass - discovers locals and builds function metadata
extern crate alloc;
use alloc::vec::Vec;

use crate::compiler::ast::{Parameter, Program, Stmt, StmtKind};
use crate::compiler::codegen::LocalAllocator;
use crate::compiler::error::{TypeError, TypeErrorKind};
use crate::compiler::func::{FunctionMetadata, FunctionTable, LocalVarInfo};
use crate::shared::Type;

/// Function analyzer for discovering locals and building metadata
pub struct FunctionAnalyzer;

impl FunctionAnalyzer {
    /// Analyze a program and build function metadata table
    pub fn analyze_program(program: &Program, pool: &AstPool) -> Result<FunctionTable, TypeError> {
        let mut func_table = FunctionTable::new();

        // Analyze each function
        for func in &program.functions {
            let param_types: Vec<Type> = func.params.iter().map(|p| p.ty.clone()).collect();

            // Discover locals in function body
            let (locals, local_count) =
                Self::analyze_function_body(&func.body, &func.params, pool)?;

            let metadata = FunctionMetadata {
                params: param_types,
                return_type: func.return_type.clone(),
                locals,
                local_count,
            };

            func_table
                .declare_with_metadata(func.name.clone(), metadata)
                .map_err(|msg| TypeError {
                    kind: TypeErrorKind::UndefinedFunction(msg),
                    span: func.span,
                })?;
        }

        Ok(func_table)
    }

    /// Analyze a single function body to discover all local variables
    fn analyze_function_body(
        body: &[Stmt],
        params: &[Parameter],
        pool: &AstPool,
    ) -> Result<(Vec<LocalVarInfo>, u32), TypeError> {
        let mut locals = LocalAllocator::new();
        let mut local_infos = Vec::new();

        // First, allocate parameters as locals (they get the first indices)
        for param in params {
            let index = locals.allocate_typed(param.name.clone(), param.ty.clone());
            local_infos.push(LocalVarInfo {
                name: param.name.clone(),
                ty: param.ty.clone(),
                index,
            });
        }

        // Then discover all VarDecl statements in the function body
        // Pass local_infos so we can record each variable as it's declared
        for &stmt_id in body {
            Self::discover_locals_with_tracking(stmt_id, pool, &mut locals, &mut local_infos)?;
        }

        Ok((local_infos, locals.next_index))
    }

    /// Recursively discover all VarDecl statements with tracking
    fn discover_locals_with_tracking(
        stmt_id: Stmt,
        pool: &AstPool,
        locals: &mut LocalAllocator,
        local_infos: &mut Vec<LocalVarInfo>,
    ) -> Result<(), TypeError> {
        let stmt = pool.stmt(stmt_id);
        let stmt_kind = stmt.kind.clone();

        match stmt_kind {
            StmtKind::VarDecl { ty, name, .. } => {
                // Allocate a local for this variable and record it
                let index = locals.allocate_typed(name.clone(), ty.clone());
                local_infos.push(LocalVarInfo { name, ty, index });
            }

            StmtKind::Block(stmts) => {
                // Enter a new scope for the block
                locals.push_scope();
                for &inner_stmt_id in &stmts {
                    Self::discover_locals_with_tracking(inner_stmt_id, pool, locals, local_infos)?;
                }
                locals.pop_scope();
            }

            StmtKind::If {
                then_stmt,
                else_stmt,
                ..
            } => {
                // Analyze then branch
                Self::discover_locals_with_tracking(then_stmt, pool, locals, local_infos)?;

                // Analyze else branch if present
                if let Some(else_id) = else_stmt {
                    Self::discover_locals_with_tracking(else_id, pool, locals, local_infos)?;
                }
            }

            StmtKind::While { body, .. } => {
                Self::discover_locals_with_tracking(body, pool, locals, local_infos)?;
            }

            StmtKind::For { init, body, .. } => {
                // For loops create a scope for the init statement
                locals.push_scope();

                if let Some(init_id) = init {
                    Self::discover_locals_with_tracking(init_id, pool, locals, local_infos)?;
                }

                Self::discover_locals_with_tracking(body, pool, locals, local_infos)?;

                locals.pop_scope();
            }

            // These statements don't declare variables
            StmtKind::Return(_) | StmtKind::Expr(_) => {}
        }

        Ok(())
    }

    /// Recursively discover all VarDecl statements (legacy version without tracking)
    #[allow(dead_code)]
    fn discover_locals(
        stmt_id: Stmt,
        pool: &AstPool,
        locals: &mut LocalAllocator,
    ) -> Result<(), TypeError> {
        let stmt = pool.stmt(stmt_id);
        let stmt_kind = stmt.kind.clone();

        match stmt_kind {
            StmtKind::VarDecl { ty, name, .. } => {
                // Allocate a local for this variable
                locals.allocate_typed(name, ty);
            }

            StmtKind::Block(stmts) => {
                // Enter a new scope for the block
                locals.push_scope();
                for &inner_stmt_id in &stmts {
                    Self::discover_locals(inner_stmt_id, pool, locals)?;
                }
                locals.pop_scope();
            }

            StmtKind::If {
                then_stmt,
                else_stmt,
                ..
            } => {
                // Analyze then branch
                Self::discover_locals(then_stmt, pool, locals)?;

                // Analyze else branch if present
                if let Some(else_id) = else_stmt {
                    Self::discover_locals(else_id, pool, locals)?;
                }
            }

            StmtKind::While { body, .. } => {
                Self::discover_locals(body, pool, locals)?;
            }

            StmtKind::For { init, body, .. } => {
                // For loops create a scope for the init statement
                locals.push_scope();

                if let Some(init_id) = init {
                    Self::discover_locals(init_id, pool, locals)?;
                }

                Self::discover_locals(body, pool, locals)?;

                locals.pop_scope();
            }

            // These statements don't declare variables
            StmtKind::Return(_) | StmtKind::Expr(_) => {}
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::lexer::Lexer;
    use crate::compiler::parser::Parser;
    use crate::compiler::stmt::stmt_test_util::ScriptTest;

    #[test]
    fn test_analyze_simple_function() {
        ScriptTest::new(
            "
            float add(float a, float b) {
                float result = a + b;
                return result;
            }
        ",
        )
        .expect_function_metadata("add", vec![Type::Fixed, Type::Fixed], Type::Fixed, 3)
        .expect_function_local_names("add", vec!["a", "b", "result"])
        .run()
        .unwrap();
    }

    #[test]
    fn test_analyze_function_with_nested_scopes() {
        ScriptTest::new(
            "
            float test(float x) {
                float a = x;
                if (x > 0.0) {
                    float b = x * 2.0;
                    return b;
                }
                return a;
            }
        ",
        )
        .expect_function_local_count("test", 3)
        .expect_function_local_names("test", vec!["x", "a", "b"])
        .run()
        .unwrap();
    }

    #[test]
    fn test_analyze_no_locals() {
        ScriptTest::new(
            "
            float getPi() {
                return 3.14;
            }
        ",
        )
        .expect_function_metadata("getPi", vec![], Type::Fixed, 0)
        .run()
        .unwrap();
    }

    #[test]
    fn test_analyze_function_with_params_detailed() {
        ScriptTest::new(
            "
            float add(float a, float b) {
                return a + b;
            }
        ",
        )
        .expect_function_metadata("add", vec![Type::Fixed, Type::Fixed], Type::Fixed, 2)
        .expect_function_local_names("add", vec!["a", "b"])
        .run()
        .unwrap();
    }

    #[test]
    fn test_analyze_function_params_plus_local() {
        ScriptTest::new(
            "
            float calculate(float x) {
                float result = x * 2.0;
                return result;
            }
        ",
        )
        .expect_function_metadata("calculate", vec![Type::Fixed], Type::Fixed, 2)
        .expect_function_local_names("calculate", vec!["x", "result"])
        .run()
        .unwrap();
    }

    #[test]
    fn test_analyze_vec2_parameter() {
        ScriptTest::new(
            "
            float sumComponents(vec2 v) {
                return v.x + v.y;
            }
        ",
        )
        .expect_function_metadata("sumComponents", vec![Type::Vec2], Type::Fixed, 1)
        .expect_function_local_names("sumComponents", vec!["v"])
        .run()
        .unwrap();
    }

    #[test]
    fn test_analyze_shadowing() {
        ScriptTest::new(
            "
            float test() {
                float x = 1.0;
                {
                    float x = 2.0;
                }
                return x;
            }
        ",
        )
        .expect_function_local_count("test", 2)
        .expect_function_local_names("test", vec!["x", "x"])
        .run()
        .unwrap();
    }

    #[test]
    fn test_analyze_multiple_params_and_locals() {
        ScriptTest::new(
            "
            vec2 transform(vec2 pos, float scale, float offset) {
                vec2 scaled = pos * scale;
                vec2 result = scaled + vec2(offset, offset);
                return result;
            }
        ",
        )
        .expect_function_params("transform", vec![Type::Vec2, Type::Fixed, Type::Fixed])
        .expect_function_local_count("transform", 5) // pos, scale, offset, scaled, result
        .expect_function_local_names(
            "transform",
            vec!["pos", "scale", "offset", "scaled", "result"],
        )
        .run()
        .unwrap();
    }

    #[test]
    fn test_analyze_nested_blocks() {
        ScriptTest::new(
            "
            float test() {
                float a = 1.0;
                {
                    float b = 2.0;
                    {
                        float c = 3.0;
                        return a + b + c;
                    }
                }
            }
        ",
        )
        .expect_function_local_count("test", 3)
        .expect_function_local_names("test", vec!["a", "b", "c"])
        .run()
        .unwrap();
    }

    #[test]
    fn test_analyze_for_loop_with_init() {
        ScriptTest::new(
            "
            float test() {
                for (float i = 0.0; i < 10.0; i = i + 1.0) {
                    float x = i * 2.0;
                }
                return 0.0;
            }
        ",
        )
        .expect_function_local_count("test", 2) // i, x
        .expect_function_local_names("test", vec!["i", "x"])
        .run()
        .unwrap();
    }

    #[test]
    fn test_analyze_if_branches_with_locals() {
        ScriptTest::new(
            "
            float test(float x) {
                if (x > 0.0) {
                    float a = x * 2.0;
                    return a;
                } else {
                    float b = x * 3.0;
                    return b;
                }
            }
        ",
        )
        .expect_function_local_count("test", 3) // x, a, b
        .expect_function_local_names("test", vec!["x", "a", "b"])
        .run()
        .unwrap();
    }

    #[test]
    fn test_analyze_complex_shadowing_pattern() {
        ScriptTest::new(
            "
            float test(float x) {
                float y = x;
                {
                    float x = y * 2.0; // Shadow param x
                    float z = x + 1.0; // Uses shadowed x
                }
                return x + y; // Uses original param x
            }
        ",
        )
        .expect_function_local_count("test", 4) // param x, y, shadowed x, z
        .expect_function_local_names("test", vec!["x", "y", "x", "z"])
        .run()
        .unwrap();
    }

    #[test]
    fn test_analyze_while_loop_with_local() {
        ScriptTest::new(
            "
            float test() {
                float sum = 0.0;
                float i = 0.0;
                while (i < 10.0) {
                    float term = i * 2.0;
                    sum = sum + term;
                    i = i + 1.0;
                }
                return sum;
            }
        ",
        )
        .expect_function_local_count("test", 3) // sum, i, term
        .expect_function_local_names("test", vec!["sum", "i", "term"])
        .run()
        .unwrap();
    }

    #[test]
    fn test_analyze_vec3_and_vec4_params() {
        ScriptTest::new(
            "
            vec4 blend(vec3 color, float alpha) {
                return vec4(color.x, color.y, color.z, alpha);
            }
        ",
        )
        .expect_function_metadata("blend", vec![Type::Vec3, Type::Fixed], Type::Vec4, 2)
        .expect_function_local_names("blend", vec!["color", "alpha"])
        .run()
        .unwrap();
    }

    #[test]
    fn test_analyze_no_params_with_locals() {
        ScriptTest::new(
            "
            float compute() {
                float x = 1.0;
                float y = 2.0;
                float z = x + y;
                return z;
            }
        ",
        )
        .expect_function_params("compute", vec![])
        .expect_function_local_count("compute", 3)
        .expect_function_local_names("compute", vec!["x", "y", "z"])
        .run()
        .unwrap();
    }

    /// Comprehensive test demonstrating all ScriptTest analyzer capabilities
    #[test]
    fn test_comprehensive_analyzer_demo() {
        ScriptTest::new(
            "
            vec3 processColor(vec3 rgb, float brightness) {
                vec3 scaled = rgb * brightness;
                vec3 clamped = vec3(
                    min(scaled.x, 1.0),
                    min(scaled.y, 1.0),
                    min(scaled.z, 1.0)
                );
                return clamped;
            }
        ",
        )
        // Test signature
        .expect_function_params("processColor", vec![Type::Vec3, Type::Fixed])
        // Test return type
        .expect_function_metadata("processColor", vec![Type::Vec3, Type::Fixed], Type::Vec3, 4)
        // Test locals: rgb, brightness (params), scaled, clamped (locals)
        .expect_function_local_names(
            "processColor",
            vec!["rgb", "brightness", "scaled", "clamped"],
        )
        .run()
        .unwrap();
    }

    // Keep manual tests for detailed index checking
    #[test]
    fn test_analyze_shadowing_detailed() {
        let program_text = "
            float test() {
                float x = 1.0;
                {
                    float x = 2.0;
                }
                return x;
            }
        ";

        let mut lexer = Lexer::new(program_text);
        let tokens = lexer.tokenize();
        let parser = Parser::new(tokens);
        let (program, pool) = parser.parse_program().expect("parse should succeed");

        let func_table =
            FunctionAnalyzer::analyze_program(&program, &pool).expect("analysis should succeed");

        let metadata = func_table.lookup("test").expect("function should exist");

        // Should have 2 locals (both named x, different indices)
        assert_eq!(metadata.local_count, 2);
        assert_eq!(metadata.locals.len(), 2);

        // Outer x gets index 0
        assert_eq!(metadata.locals[0].name, "x");
        assert_eq!(metadata.locals[0].index, 0);

        // Inner x gets index 1 (shadowing)
        assert_eq!(metadata.locals[1].name, "x");
        assert_eq!(metadata.locals[1].index, 1);
    }
}
