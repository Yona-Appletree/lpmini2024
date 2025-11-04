/// Statement code generation
extern crate alloc;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use alloc::string::String;

use crate::lpscript::ast::{Stmt, StmtKind, Expr};
use crate::lpscript::vm::opcodes::LpsOpCode;
use super::local_allocator::LocalAllocator;

mod var_decl;
mod assign;
mod return_stmt;
mod expr_stmt;
mod block;
mod if_stmt;
mod while_loop;
mod for_loop;

pub use var_decl::gen_var_decl;
pub use assign::gen_assignment;
pub use return_stmt::gen_return;
pub use expr_stmt::gen_expr_stmt;
pub use block::gen_block;
pub use if_stmt::gen_if;
pub use while_loop::gen_while;
pub use for_loop::gen_for;

/// Generate code for a statement
pub fn gen_stmt(
    stmt: &Stmt,
    code: &mut Vec<LpsOpCode>,
    locals: &mut LocalAllocator,
    func_offsets: &BTreeMap<String, u32>,
    gen_expr: impl Fn(&Expr, &mut Vec<LpsOpCode>, &mut LocalAllocator, &BTreeMap<String, u32>) + Copy,
) {
    match &stmt.kind {
        StmtKind::VarDecl { ty, name, init } => {
            gen_var_decl(ty, name, init, code, locals, func_offsets, gen_expr);
        }
        
        StmtKind::Assignment { name, value } => {
            gen_assignment(name, value, code, locals, func_offsets, gen_expr);
        }
        
        StmtKind::Return(expr) => {
            gen_return(expr, code, locals, func_offsets, gen_expr);
        }
        
        StmtKind::Expr(expr) => {
            gen_expr_stmt(expr, code, locals, func_offsets, gen_expr);
        }
        
        StmtKind::Block(stmts) => {
            gen_block(
                stmts,
                code,
                locals,
                func_offsets,
                |s, c, l, f| gen_stmt(s, c, l, f, gen_expr),
            );
        }
        
        StmtKind::If { condition, then_stmt, else_stmt } => {
            gen_if(
                condition,
                then_stmt,
                else_stmt,
                code,
                locals,
                func_offsets,
                gen_expr,
                |s, c, l, f| gen_stmt(s, c, l, f, gen_expr),
            );
        }
        
        StmtKind::While { condition, body } => {
            gen_while(
                condition,
                body,
                code,
                locals,
                func_offsets,
                gen_expr,
                |s, c, l, f| gen_stmt(s, c, l, f, gen_expr),
            );
        }
        
        StmtKind::For { init, condition, increment, body } => {
            gen_for(
                init,
                condition,
                increment,
                body,
                code,
                locals,
                func_offsets,
                gen_expr,
                |s, c, l, f| gen_stmt(s, c, l, f, gen_expr),
            );
        }
    }
}

