/// Statement compilation modules grouped by feature

pub mod assign;
pub mod block;
pub mod expr_stmt;
pub mod for_loop;
pub mod if_stmt;
pub mod return_stmt;
pub mod var_decl;
pub mod while_loop;

#[cfg(test)]
pub mod stmt_test_util;

#[cfg(test)]
pub mod stmt_test_ast;

