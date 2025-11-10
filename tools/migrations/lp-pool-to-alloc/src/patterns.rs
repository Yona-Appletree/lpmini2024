//! Pattern matching utilities for identifying lp-pool usage

use syn::Path;

/// Check if a path is `lp_pool::LpVec`
pub fn is_lp_vec(path: &Path) -> bool {
    path.segments.len() == 2
        && path.segments[0].ident == "lp_pool"
        && path.segments[1].ident == "LpVec"
}

/// Check if a path is `lp_pool::LpString`
pub fn is_lp_string(path: &Path) -> bool {
    path.segments.len() == 2
        && path.segments[0].ident == "lp_pool"
        && path.segments[1].ident == "LpString"
}

/// Check if a path is `lp_pool::LpBox`
pub fn is_lp_box(path: &Path) -> bool {
    path.segments.len() == 2
        && path.segments[0].ident == "lp_pool"
        && path.segments[1].ident == "LpBox"
}

/// Check if a path is `lp_pool::LpBTreeMap`
pub fn is_lp_btree_map(path: &Path) -> bool {
    path.segments.len() == 2
        && path.segments[0].ident == "lp_pool"
        && path.segments[1].ident == "LpBTreeMap"
}

/// Check if a path is `lp_pool::AllocError`
pub fn is_alloc_error(path: &Path) -> bool {
    path.segments.len() == 2
        && path.segments[0].ident == "lp_pool"
        && path.segments[1].ident == "AllocError"
}

/// Check if a path is `lp_pool::LpMemoryPool`
pub fn is_lp_memory_pool(path: &Path) -> bool {
    path.segments.len() == 2
        && path.segments[0].ident == "lp_pool"
        && path.segments[1].ident == "LpMemoryPool"
}

/// Check if a path is `lp_pool::LpBoxDyn`
pub fn is_lp_box_dyn(path: &Path) -> bool {
    path.segments.len() == 2
        && path.segments[0].ident == "lp_pool"
        && path.segments[1].ident == "LpBoxDyn"
}

/// Check if a method call is a try_* method
pub fn is_try_method(ident: &syn::Ident) -> bool {
    let name = ident.to_string();
    name.starts_with("try_")
        && (name == "try_push"
            || name == "try_reserve"
            || name == "try_push_str"
            || name == "try_push_char"
            || name == "try_from_str"
            || name == "try_new")
}

/// Check if a path segment is in the lp-script compiler directory
pub fn is_compiler_path(file_path: &std::path::Path) -> bool {
    file_path
        .to_string_lossy()
        .contains("lp-script/src/compiler")
}
