//! Transformations for migrating from lp-pool to lp-alloc.

use crate::patterns::*;

/// Transform a Rust source file
pub fn transform_file(content: &str, file_path: &std::path::Path) -> String {
    let is_compiler = is_compiler_path(file_path);

    // First do simple string-based replacements
    let mut transformed = content.to_string();
    transformed = transform_imports(&transformed);
    transformed = transform_types(&transformed);
    transformed = transform_error_types(&transformed);
    transformed = remove_pool_scopes(&transformed);

    // Then parse and do AST-based transformations for complex cases
    if is_compiler {
        transformed = transform_compiler_try_calls(&transformed);
    }

    // Add test setup if needed
    transformed = add_test_setup(&transformed);

    transformed
}

/// Transform import statements
fn transform_imports(content: &str) -> String {
    let mut result = content.to_string();

    // Replace simple imports
    result = result.replace("use lp_pool::LpVec;", "use alloc::vec::Vec;");
    result = result.replace("use lp_pool::LpString;", "use alloc::string::String;");
    result = result.replace("use lp_pool::LpBox;", "use alloc::boxed::Box;");
    result = result.replace(
        "use lp_pool::LpBTreeMap;",
        "use alloc::collections::BTreeMap;",
    );
    result = result.replace("use lp_pool::AllocError;", "use lp_alloc::AllocLimitError;");
    result = result.replace("use lp_pool::LpBoxDyn;", "use alloc::boxed::Box;");

    // Handle multi-item imports with braces: use lp_pool::{LpVec, LpString};
    result = result.replace("lp_pool::{LpVec", "alloc::vec::{Vec");
    result = result.replace("lp_pool::{LpString", "alloc::string::{String");
    result = result.replace("lp_pool::{LpBox", "alloc::boxed::{Box");
    result = result.replace("lp_pool::{LpBTreeMap", "alloc::collections::{BTreeMap");
    result = result.replace("lp_pool::{AllocError", "lp_alloc::{AllocLimitError");

    // Handle path segments in imports
    result = result.replace("lp_pool::LpVec", "alloc::vec::Vec");
    result = result.replace("lp_pool::LpString", "alloc::string::String");
    result = result.replace("lp_pool::LpBox", "alloc::boxed::Box");
    result = result.replace("lp_pool::LpBTreeMap", "alloc::collections::BTreeMap");
    result = result.replace("lp_pool::AllocError", "lp_alloc::AllocLimitError");
    result = result.replace("lp_pool::LpBoxDyn", "alloc::boxed::Box");

    // Remove LpMemoryPool imports (but keep the use statement structure)
    result = result.replace("use lp_pool::LpMemoryPool;", "");
    result = result.replace(", LpMemoryPool", "");
    result = result.replace("LpMemoryPool,", "");
    result = result.replace("lp_pool::LpMemoryPool", "");

    // Remove other lp_pool specific imports that are no longer needed
    result = result.replace("use lp_pool::allow_global_alloc;", "");
    result = result.replace("use lp_pool::ScopedGlobalAllocGuard;", "");

    result
}

/// Transform type names in the code
fn transform_types(content: &str) -> String {
    let mut result = content.to_string();

    // Replace type names (being careful with word boundaries)
    // Use word boundaries where possible to avoid partial matches
    result = result.replace("LpVec<", "Vec<");
    result = result.replace("LpString", "String");
    result = result.replace("LpBox<", "Box<");
    result = result.replace("LpBTreeMap<", "BTreeMap<");
    result = result.replace("LpBoxDyn", "Box");

    // Handle type annotations and function signatures
    result = result.replace(": LpVec<", ": Vec<");
    result = result.replace(": LpString", ": String");
    result = result.replace(": LpBox<", ": Box<");
    result = result.replace(": LpBTreeMap<", ": BTreeMap<");
    result = result.replace(": LpBoxDyn", ": Box");

    // Handle return types
    result = result.replace("-> LpVec<", "-> Vec<");
    result = result.replace("-> LpString", "-> String");
    result = result.replace("-> LpBox<", "-> Box<");
    result = result.replace("-> LpBTreeMap<", "-> BTreeMap<");

    result
}

/// Transform error type references
fn transform_error_types(content: &str) -> String {
    let mut result = content.to_string();

    result = result.replace("AllocError", "AllocLimitError");
    result = result.replace("lp_pool::AllocError", "lp_alloc::AllocLimitError");
    result = result.replace(
        "AllocError::PoolExhausted",
        "AllocLimitError::SoftLimitExceeded",
    );
    result = result.replace(
        "AllocError::OutOfMemory",
        "AllocLimitError::SoftLimitExceeded",
    );
    result = result.replace(
        "AllocError::InvalidLayout",
        "AllocLimitError::SoftLimitExceeded",
    );

    result
}

/// Remove LpMemoryPool::run() and with_global_alloc() wrappers
fn remove_pool_scopes(content: &str) -> String {
    let mut result = content.to_string();

    // Remove common patterns:
    // pool.run(|| { ... })? -> extract inner code
    // LpMemoryPool::with_global_alloc(|| { ... }) -> extract inner code
    // This is simplified - full implementation would need proper AST parsing
    // For now, we'll handle this in Phase 2 manual cleanup

    // Simple string-based removal for common single-line cases
    result = result.replace("LpMemoryPool::with_global_alloc(|| ", "");
    result = result.replace("pool.run(|| ", "");

    result
}

/// Transform try_* method calls in compiler code to use try_alloc
fn transform_compiler_try_calls(content: &str) -> String {
    // This is complex and requires AST manipulation
    // For Phase 1, we'll do basic string replacements for common patterns
    // Full AST-based transformation will be done by the big model

    let mut result = content.to_string();

    // Basic replacements - these are placeholders
    // Full implementation needs AST parsing to properly wrap expressions
    // For now, mark that we need try_alloc import
    if result.contains(".try_push")
        || result.contains(".try_reserve")
        || result.contains("try_new")
        || result.contains("try_from_str")
    {
        // Add import if not present
        if !result.contains("use lp_alloc::try_alloc") {
            // Find first use statement and add after it
            if let Some(pos) = result.find("extern crate") {
                let insert_pos = result[pos..].find('\n').map(|i| pos + i + 1).unwrap_or(pos);
                result.insert_str(insert_pos, "use lp_alloc::try_alloc;\n");
            } else if let Some(pos) = result.find("use ") {
                let insert_pos = result[pos..].find('\n').map(|i| pos + i + 1).unwrap_or(pos);
                result.insert_str(insert_pos, "use lp_alloc::try_alloc;\n");
            }
        }
    }

    result
}

/// Add test setup (#[global_allocator] and limit initialization) to test modules
fn add_test_setup(content: &str) -> String {
    // Check if this file has #[cfg(test)] modules
    if !content.contains("#[cfg(test)]") {
        return content.to_string();
    }

    let mut result = content.to_string();

    // Check if setup_test_alloc! is already present
    if result.contains("setup_test_alloc!") || result.contains("#[global_allocator]") {
        return result;
    }

    // Find #[cfg(test)] and add setup after it
    if let Some(pos) = result.find("#[cfg(test)]") {
        // Find the mod tests { line
        if let Some(mod_pos) = result[pos..].find("mod tests") {
            let insert_pos = pos + mod_pos;
            // Find the opening brace
            if let Some(brace_pos) = result[insert_pos..].find('{') {
                let final_pos = insert_pos + brace_pos + 1;
                // Add setup macro
                let setup = "\n    lp_alloc::setup_test_alloc!();\n";
                result.insert_str(final_pos, setup);
            }
        }
    }

    // Also add the import if not present
    if !result.contains("use lp_alloc") && result.contains("#[cfg(test)]") {
        // Add import near other use statements in test module
        if let Some(pos) = result.find("#[cfg(test)]") {
            if let Some(use_pos) = result[pos..].find("use ") {
                let insert_pos = pos + use_pos;
                result.insert_str(insert_pos, "use lp_alloc;\n    ");
            }
        }
    }

    result
}
