#!/usr/bin/env python3
"""Fix imports: ExprId/StmtId/AstPool → Expr/Stmt"""

import re
import sys
from pathlib import Path

def fix_imports(content):
    """Fix imports in a single file"""
    original = content
    
    # 1. Remove AstPool from imports
    content = re.sub(r',\s*AstPool\s*,', ', ', content)
    content = re.sub(r',\s*AstPool\s*}', '}', content)
    content = re.sub(r'\{\s*AstPool\s*,', '{', content)
    content = re.sub(r'use\s+crate::compiler::ast::\{\s*AstPool\s*\};?\n', '', content)
    
    # 2. Replace ExprId with Expr in imports
    content = re.sub(r'\bExprId\b(?=.*;\s*$|.*,|.*})', 'Expr', content, flags=re.MULTILINE)
    
    # 3. Replace StmtId with Stmt in imports  
    content = re.sub(r'\bStmtId\b(?=.*;\s*$|.*,|.*})', 'Stmt', content, flags=re.MULTILINE)
    
    return content != original, content

def process_file(path):
    with open(path) as f:
        content = f.read()
    
    changed, new_content = fix_imports(content)
    
    if changed:
        with open(path, 'w') as f:
            f.write(new_content)
        print(f"Fixed: {path}")
        return True
    return False

if __name__ == "__main__":
    script_dir = Path(__file__).parent
    lpscript_src = script_dir.parent / "crates" / "lp-script" / "src"
    
    fixed = 0
    for rs_file in lpscript_src.rglob("*.rs"):
        if process_file(rs_file):
            fixed += 1
    
    print(f"\nFixed {fixed} files")

