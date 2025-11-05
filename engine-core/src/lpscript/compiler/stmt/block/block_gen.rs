/// Block statement code generation
extern crate alloc;
use crate::lpscript::compiler::ast::Stmt;
use crate::lpscript::compiler::codegen::CodeGenerator;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn gen_block(&mut self, stmts: &[Stmt]) {
        // Push a new scope for this block
        self.locals.push_scope();

        // Generate code for all statements
        for stmt in stmts {
            self.gen_stmt(stmt);
        }

        // Pop the scope, restoring any shadowed variables
        self.locals.pop_scope();
    }
}
