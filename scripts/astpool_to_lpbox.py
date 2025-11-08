#!/usr/bin/env python3
"""
Codemod: Convert AstPool (ExprId/StmtId indexed) to direct LpBox<Expr> recursive AST

Based on reversing commit f44de202 but using LpBox instead of Box.

Transformations:
1. ExprId → Expr (return full expressions)
2. StmtId → Stmt (return full statements)  
3. pool.alloc_expr(kind, span) → Expr::new(kind, span)
4. ExprKind::Add(id1, id2) → ExprKind::Add(LpBox::try_new(e1)?, LpBox::try_new(e2)?)
5. pool.expr(id) → &expr (direct reference)
6. Remove AstPool parameter threading
"""

import re
import sys
from pathlib import Path

def process_file(filepath):
    """Process a single Rust file"""
    print(f"Processing: {filepath}")
    
    with open(filepath, 'r') as f:
        content = f.read()
    
    original = content
    
    # 1. Replace type signatures
    content = re.sub(r'\bExprId\b', 'Expr', content)
    content = re.sub(r'\bStmtId\b', 'Stmt', content)
    
    # 2. Replace Box<Expr> and Box<Stmt> with LpBox versions
    content = re.sub(r'Box<Expr>', 'LpBox<Expr>', content)
    content = re.sub(r'Box<Stmt>', 'LpBox<Stmt>', content)
    
    # 3. Replace Box::new with LpBox::try_new
    content = re.sub(r'Box::new\(', 'LpBox::try_new(', content)
    
    # 4. Remove AstPool parameters from function signatures
    # This is tricky - need to handle both fn signatures and calls
    # Pattern: ", pool: AstPool" or ", mut pool: AstPool" or "pool: AstPool,"
    content = re.sub(r',\s*(mut\s+)?pool:\s*AstPool', '', content)
    content = re.sub(r'\(pool:\s*AstPool,', '(', content)
    content = re.sub(r'\(pool:\s*AstPool\)', '()', content)
    
    # 5. Remove AstPool return types
    # Pattern: "-> (ExprId, AstPool)" becomes "-> Expr"
    content = re.sub(r'->\s*\(Expr,\s*AstPool\)', '-> Expr', content)
    content = re.sub(r'->\s*\(Stmt,\s*AstPool\)', '-> Stmt', content)
    
    # 6. Remove pool.expr(id) dereferences - becomes just expr reference
    # This is complex and may need manual fixing
    content = re.sub(r'pool\.expr\((\w+)\)', r'\1', content)
    content = re.sub(r'pool\.expr_mut\((\w+)\)', r'\1', content)
    content = re.sub(r'pool\.stmt\((\w+)\)', r'\1', content)
    
    # 7. Remove pool.alloc_expr wrapping
    # pool.alloc_expr(kind, span)? → Expr::new(kind, span)
    content = re.sub(r'pool\.alloc_expr\(([^,]+),\s*([^)]+)\)\?', r'Expr::new(\1, \2)', content)
    
    # 8. Add LpBox import if file was modified and doesn't have it
    if content != original and 'use lp_pool::LpBox' not in content and 'LpBox' in content:
        # Find first use statement or after extern crate
        if 'extern crate alloc;' in content:
            content = content.replace('extern crate alloc;', 'extern crate alloc;\nuse lp_pool::LpBox;', 1)
        elif '\nuse ' in content:
            # Insert before first use
            content = re.sub(r'(\n)(use )', r'\1use lp_pool::LpBox;\n\2', content, 1)
    
    # 9. Remove Box import if no longer needed
    if 'Box<' not in content and 'Box::' not in content:
        content = re.sub(r'use alloc::boxed::Box;\n', '', content)
    
    # 10. Remove AstPool/ExprId/StmtId imports
    content = re.sub(r'use crate::(?:lpscript::)?compiler::ast::\{[^}]*AstPool[^}]*\};?\n', '', content)
    content = re.sub(r',\s*AstPool', '', content)  # Remove from import lists
    content = re.sub(r',\s*ExprId', '', content)
    content = re.sub(r',\s*StmtId', '', content)
    
    # Only write if changed
    if content != original:
        with open(filepath, 'w') as f:
            f.write(content)
        return True
    return False

def main():
    script_dir = Path(__file__).parent
    lpscript_dir = script_dir.parent / "crates" / "lp-script" / "src"
    
    if not lpscript_dir.exists():
        print(f"Error: {lpscript_dir} not found")
        sys.exit(1)
    
    print(f"Running AstPool → LpBox codemod on {lpscript_dir}")
    
    # Process all .rs files
    modified = 0
    for rs_file in lpscript_dir.rglob("*.rs"):
        if process_file(rs_file):
            modified += 1
    
    print(f"\nCodemod complete. Modified {modified} files.")
    print("\nNext steps:")
    print("  1. cargo build -p lp-script (expect many errors)")
    print("  2. Fix function returns: add .unwrap() or ? for LpBox::try_new")
    print("  3. Fix pool threading: remove pool parameters and returns")
    print("  4. Fix pattern matches on ExprKind (no more IDs)")

if __name__ == "__main__":
    main()

