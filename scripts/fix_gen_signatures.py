#!/usr/bin/env python3
"""Fix _gen.rs function signatures: ExprId→&Expr, remove pool param"""

import re
from pathlib import Path

def fix_gen_file(content):
    """Transform codegen function signatures"""
    original = content
    
    # 1. Fix function signatures
    # fn gen_add_id(&mut self, pool: &AstPool, left: ExprId, right: ExprId, ...)
    # → fn gen_add(&mut self, left: &Expr, right: &Expr, ...)
    
    # Remove pool parameter
    content = re.sub(r',\s*pool:\s*&AstPool', '', content)
    content = re.sub(r'\(pool:\s*&AstPool,', '(', content)
    
    # Replace ExprId/StmtId parameters with &Expr/&Stmt
    content = re.sub(r'\bleft:\s*ExprId\b', 'left: &Expr', content)
    content = re.sub(r'\bright:\s*ExprId\b', 'right: &Expr', content)
    content = re.sub(r'\boperand:\s*ExprId\b', 'operand: &Expr', content)
    content = re.sub(r'\bexpr_id:\s*ExprId\b', 'expr: &Expr', content)
    content = re.sub(r'\bexpr:\s*ExprId\b', 'expr: &Expr', content)
    content = re.sub(r'\bcondition:\s*ExprId\b', 'condition: &Expr', content)
    content = re.sub(r'\btrue_expr:\s*ExprId\b', 'true_expr: &Expr', content)
    content = re.sub(r'\bfalse_expr:\s*ExprId\b', 'false_expr: &Expr', content)
    content = re.sub(r'\bvalue:\s*ExprId\b', 'value: &Expr', content)
    content = re.sub(r'\bstmt_id:\s*StmtId\b', 'stmt: &Stmt', content)
    content = re.sub(r'\bstmt:\s*StmtId\b', 'stmt: &Stmt', content)
    content = re.sub(r'\bbody:\s*StmtId\b', 'body: &Stmt', content)
    content = re.sub(r'\bthen_stmt:\s*StmtId\b', 'then_stmt: &Stmt', content)
    content = re.sub(r'\belse_stmt:\s*Option<StmtId>\b', 'else_stmt: Option<&Stmt>', content)
    content = re.sub(r'\binit:\s*&Option<StmtId>\b', 'init: Option<&Stmt>', content)
    content = re.sub(r'\bargs:\s*&Vec<ExprId>\b', 'args: &Vec<Expr>', content)
    content = re.sub(r'\bargs:\s*&\[ExprId\]\b', 'args: &[Expr]', content)
    
    # Remove _id suffix from function names
    content = re.sub(r'\bgen_(\w+)_id\b', r'gen_\1', content)
    
    # 2. Fix function calls
    # self.gen_expr_id(pool, left) → self.gen_expr(left)
    content = re.sub(r'self\.gen_expr_id\(pool,\s*', 'self.gen_expr(', content)
    content = re.sub(r'self\.gen_stmt_id\(pool,\s*', 'self.gen_stmt(', content)
    
    # 3. Remove pool.expr() and pool.stmt() dereferences
    content = re.sub(r'pool\.expr\((\w+)\)', r'\1', content)
    content = re.sub(r'pool\.stmt\((\w+)\)', r'\1', content)
    
    return content != original, content

def main():
    script_dir = Path(__file__).parent
    lpscript = script_dir.parent / "crates" / "lp-script" / "src"
    
    fixed = 0
    for pattern in ["**/*_gen.rs", "**/*_types.rs"]:
        for path in lpscript.glob(pattern):
            with open(path) as f:
                content = f.read()
            
            changed, new_content = fix_gen_file(content)
            if changed:
                with open(path, 'w') as f:
                    f.write(new_content)
                print(f"Fixed: {path}")
                fixed += 1
    
    print(f"\nFixed {fixed} files")

if __name__ == "__main__":
    main()

