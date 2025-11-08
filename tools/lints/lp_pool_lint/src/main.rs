use std::fs;
use std::path::{Component, Path};

use anyhow::{Context, Result};
use camino::Utf8PathBuf;
use clap::Parser;
use proc_macro2::Span;
use syn::visit::Visit;
use syn::{
    parse::ParseStream, punctuated::Punctuated, spanned::Spanned, Attribute, Expr, Item, Macro,
    Meta, Path as SynPath, PathArguments,
};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
struct Args {
    /// Paths to scan. Defaults to lp-script, lp-data, lp-math, engine-core.
    #[arg(value_name = "PATH")]
    paths: Vec<Utf8PathBuf>,

    /// Show files that were skipped (for debugging).
    #[arg(long)]
    verbose: bool,
}

#[derive(Debug)]
struct Violation {
    file: Utf8PathBuf,
    line: usize,
    column: usize,
    message: String,
}

#[derive(Clone)]
struct BannedCall {
    name: &'static str,
    message: &'static str,
}

#[derive(Clone)]
struct BannedMacro {
    name: &'static str,
    message: &'static str,
}

const BANNED_CALLS: &[BannedCall] = &[
    BannedCall {
        name: "Box::new",
        message: "use `lp_pool::collections::box_::LpBox::try_new` instead",
    },
    BannedCall {
        name: "Box::try_new",
        message: "use `LpBox::try_new` with the pool",
    },
    BannedCall {
        name: "Box::pin",
        message: "use pool-backed pinning helpers",
    },
    BannedCall {
        name: "Vec::new",
        message: "use `lp_pool::collections::vec::LpVec::new` instead",
    },
    BannedCall {
        name: "Vec::with_capacity",
        message: "use `LpVec::with_capacity` instead",
    },
    BannedCall {
        name: "Vec::from",
        message: "use the pool-backed conversions instead",
    },
    BannedCall {
        name: "Vec::from_iter",
        message: "use the pool-backed conversions instead",
    },
    BannedCall {
        name: "Vec::default",
        message: "use `LpVec::default` (pool-backed) instead",
    },
    BannedCall {
        name: "String::new",
        message: "use `lp_pool::collections::string::LpString::new` instead",
    },
    BannedCall {
        name: "String::with_capacity",
        message: "use `LpString::with_capacity` instead",
    },
    BannedCall {
        name: "String::from",
        message: "use pool-backed string conversion instead",
    },
    BannedCall {
        name: "VecDeque::new",
        message: "use the pool-backed deque instead",
    },
    BannedCall {
        name: "BinaryHeap::new",
        message: "use the pool-backed collection instead",
    },
    BannedCall {
        name: "LinkedList::new",
        message: "use the pool-backed collection instead",
    },
    BannedCall {
        name: "HashMap::new",
        message: "use the pool-backed map instead",
    },
    BannedCall {
        name: "HashMap::with_capacity",
        message: "use the pool-backed map instead",
    },
    BannedCall {
        name: "HashSet::new",
        message: "use the pool-backed set instead",
    },
    BannedCall {
        name: "HashSet::with_capacity",
        message: "use the pool-backed set instead",
    },
    BannedCall {
        name: "BTreeMap::new",
        message: "use `lp_pool::collections::map::LpBTreeMap::new` instead",
    },
    BannedCall {
        name: "BTreeSet::new",
        message: "use `lp_pool::collections::set::LpBTreeSet::new` instead",
    },
    BannedCall {
        name: "Rc::new",
        message: "use pool-backed shared pointers instead",
    },
    BannedCall {
        name: "Arc::new",
        message: "use pool-backed shared pointers instead",
    },
    BannedCall {
        name: "alloc::alloc",
        message: "allocate through `LpMemoryPool::try_alloc` instead",
    },
    BannedCall {
        name: "alloc::alloc_zeroed",
        message: "allocate through `LpMemoryPool::try_alloc_zeroed` instead",
    },
    BannedCall {
        name: "alloc::dealloc",
        message: "deallocate via pool APIs",
    },
];

const BANNED_MACROS: &[BannedMacro] = &[
    BannedMacro {
        name: "vec",
        message: "use `LpVec` constructors instead of `vec![]`",
    },
    BannedMacro {
        name: "format",
        message: "use pool-backed string builders instead of `format!`",
    },
];

const DEFAULT_PATHS: &[&str] = &[
    "crates/lp-script",
    "crates/lp-data",
    "crates/lp-math",
    "crates/engine-core",
];

fn main() -> Result<()> {
    let args = Args::parse();
    let mut paths = if args.paths.is_empty() {
        DEFAULT_PATHS
            .iter()
            .map(|p| Utf8PathBuf::from(p))
            .collect::<Vec<_>>()
    } else {
        args.paths
    };

    for path in &mut paths {
        if path.extension().is_none() && !path.ends_with("src") {
            path.push("src");
        }
    }

    let mut violations = Vec::new();

    for base in paths {
        let walker = WalkDir::new(&base)
            .into_iter()
            .filter_entry(|e| should_descend(e.path()));

        for entry in walker {
            let entry = match entry {
                Ok(e) => e,
                Err(e) => {
                    eprintln!("walk error: {e}");
                    continue;
                }
            };
            if !entry.file_type().is_file() {
                continue;
            }

            if entry.path().extension().and_then(|s| s.to_str()) != Some("rs") {
                continue;
            }

            let file_path = Utf8PathBuf::from_path_buf(entry.into_path())
                .expect("non-utf8 paths are not supported");
            let source = fs::read_to_string(&file_path)
                .with_context(|| format!("failed to read {}", file_path))?;

            match syn::parse_file(&source) {
                Ok(parsed) => {
                    if has_cfg_test(&parsed.attrs) {
                        continue;
                    }

                    let mut visitor = PoolLintVisitor::new(file_path.clone());
                    visitor.visit_file(&parsed);
                    violations.extend(visitor.violations);
                }
                Err(err) => {
                    eprintln!("failed to parse {file_path}: {err}");
                }
            }
        }
    }

    if !violations.is_empty() {
        for violation in &violations {
            eprintln!(
                "{}:{}:{}: {}",
                violation.file, violation.line, violation.column, violation.message
            );
        }
        anyhow::bail!("found {} lp_pool allocation violations", violations.len());
    }

    Ok(())
}

fn should_descend(path: &Path) -> bool {
    for component in path.components() {
        match component {
            Component::Normal(name) => {
                if let Some(name) = name.to_str() {
                    if matches!(
                        name,
                        "tests" | "test" | "benches" | "examples" | "fixtures" | "generated"
                    ) {
                        return false;
                    }
                }
            }
            _ => {}
        }
    }
    true
}

struct PoolLintVisitor {
    file: Utf8PathBuf,
    scope_allow: Vec<bool>,
    violations: Vec<Violation>,
}

impl PoolLintVisitor {
    fn new(file: Utf8PathBuf) -> Self {
        Self {
            file,
            scope_allow: vec![false],
            violations: Vec::new(),
        }
    }

    fn push_scope(&mut self, attrs: &[Attribute]) {
        let allow = self.scope_allow.last().copied().unwrap_or(false);
        let current = allow || has_allow(attrs) || has_cfg_test(attrs);
        self.scope_allow.push(current);
    }

    fn pop_scope(&mut self) {
        self.scope_allow.pop();
    }

    fn current_allowed(&self) -> bool {
        *self.scope_allow.last().unwrap_or(&false)
    }

    fn record_violation(&mut self, span: Span, message: &'static str) {
        if self.current_allowed() {
            return;
        }

        if let Some((line, column)) = span_start(span) {
            self.violations.push(Violation {
                file: self.file.clone(),
                line,
                column,
                message: message.to_string(),
            });
        } else {
            self.violations.push(Violation {
                file: self.file.clone(),
                line: 0,
                column: 0,
                message: message.to_string(),
            });
        }
    }
}

impl<'ast> Visit<'ast> for PoolLintVisitor {
    fn visit_item(&mut self, node: &'ast Item) {
        let attrs = item_attrs(node);
        self.push_scope(attrs);
        syn::visit::visit_item(self, node);
        self.pop_scope();
    }

    fn visit_expr(&mut self, node: &'ast Expr) {
        match node {
            Expr::Call(call) => {
                if let Expr::Path(path) = &*call.func {
                    let path_string = normalize_path(&path.path);
                    if let Some(banned) = BANNED_CALLS
                        .iter()
                        .find(|b| matches_path(&path_string, b.name))
                    {
                        self.record_violation(node.span(), banned.message);
                    }
                }
            }
            _ => {}
        }

        syn::visit::visit_expr(self, node);
    }

    fn visit_macro(&mut self, mac: &'ast Macro) {
        if let Some(banned) = macro_is_banned(mac) {
            self.record_violation(mac.span(), banned.message);
        }
        syn::visit::visit_macro(self, mac);
    }
}

fn matches_path(path_string: &str, banned: &str) -> bool {
    if path_string == banned {
        return true;
    }
    if let Some(stripped) = path_string.strip_prefix("std::") {
        if stripped == banned {
            return true;
        }
    }
    if let Some(stripped) = path_string.strip_prefix("alloc::") {
        if stripped == banned {
            return true;
        }
    }
    if let Some(stripped) = path_string.strip_prefix("core::") {
        return stripped == banned;
    }
    false
}

fn has_allow(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        if !attr.path().is_ident("allow") {
            return false;
        }
        attr.parse_args_with(|input: ParseStream| {
            let paths = input.call(Punctuated::<SynPath, syn::Token![,]>::parse_terminated)?;
            Ok(paths.iter().any(|path| path.is_ident("lp_pool_std_alloc")))
        })
        .unwrap_or(false)
    })
}

fn has_cfg_test(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        if let Some(meta) = attr_requires_test(attr) {
            return meta;
        }
        false
    })
}

fn macro_is_banned(mac: &Macro) -> Option<BannedMacro> {
    let path_string = normalize_path(&mac.path);
    let ident = mac.path.segments.last()?.ident.to_string();
    BANNED_MACROS
        .iter()
        .find(|b| b.name == ident || matches_path(&path_string, b.name))
        .cloned()
}

fn normalize_path(path: &SynPath) -> String {
    let mut segments = Vec::new();
    for segment in &path.segments {
        let mut piece = segment.ident.to_string();
        if let PathArguments::AngleBracketed(args) = &segment.arguments {
            if !args.args.is_empty() {
                piece.push_str("::<..>");
            }
        }
        segments.push(piece);
    }
    segments.join("::")
}

fn span_start(span: Span) -> Option<(usize, usize)> {
    let start = span.start();
    Some((start.line, start.column + 1))
}

fn item_attrs(item: &Item) -> &[Attribute] {
    match item {
        Item::Const(i) => &i.attrs,
        Item::Enum(i) => &i.attrs,
        Item::ExternCrate(i) => &i.attrs,
        Item::Fn(i) => &i.attrs,
        Item::Impl(i) => &i.attrs,
        Item::Macro(i) => &i.attrs,
        Item::Mod(i) => &i.attrs,
        Item::Static(i) => &i.attrs,
        Item::Struct(i) => &i.attrs,
        Item::Trait(i) => &i.attrs,
        Item::TraitAlias(i) => &i.attrs,
        Item::Type(i) => &i.attrs,
        Item::Union(i) => &i.attrs,
        Item::Use(i) => &i.attrs,
        Item::ForeignMod(i) => &i.attrs,
        Item::Verbatim(_) => &[],
        _ => &[],
    }
}

fn attr_requires_test(attr: &Attribute) -> Option<bool> {
    if !attr.path().is_ident("cfg") {
        return Some(false);
    }

    match attr.meta.clone() {
        Meta::List(list) => {
            if list.tokens.is_empty() {
                return Some(false);
            }
            match syn::parse2::<Expr>(list.tokens.clone()) {
                Ok(expr) => Some(expr_requires_test(&expr)),
                Err(_) => None,
            }
        }
        Meta::Path(_) | Meta::NameValue(_) => Some(false),
    }
}

fn expr_requires_test(expr: &Expr) -> bool {
    match expr {
        Expr::Path(path) => path_requires_test(&path.path),
        Expr::Group(group) => expr_requires_test(&group.expr),
        Expr::Paren(paren) => expr_requires_test(&paren.expr),
        Expr::Call(call) => {
            if let Expr::Path(func_path) = &*call.func {
                let ident = func_path
                    .path
                    .segments
                    .last()
                    .map(|segment| segment.ident.to_string());
                match ident.as_deref() {
                    Some("all") => call.args.iter().any(expr_requires_test),
                    Some("any") => call.args.iter().all(expr_requires_test),
                    Some("not") => false,
                    _ => false,
                }
            } else {
                false
            }
        }
        Expr::Assign(assign) => {
            expr_requires_test(&assign.left) || expr_requires_test(&assign.right)
        }
        Expr::Binary(binary) => {
            expr_requires_test(&binary.left) || expr_requires_test(&binary.right)
        }
        Expr::Tuple(tuple) => tuple.elems.iter().all(expr_requires_test),
        Expr::Reference(reference) => expr_requires_test(&reference.expr),
        Expr::Unary(unary) => expr_requires_test(&unary.expr),
        _ => false,
    }
}

fn path_requires_test(path: &SynPath) -> bool {
    path.segments.len() == 1 && path.is_ident("test")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lint_source(source: &str) -> Vec<String> {
        let file = Utf8PathBuf::from("test.rs");
        let parsed = syn::parse_file(source).expect("failed to parse source");
        if has_cfg_test(&parsed.attrs) {
            return Vec::new();
        }
        let mut visitor = PoolLintVisitor::new(file);
        visitor.visit_file(&parsed);
        visitor
            .violations
            .into_iter()
            .map(|violation| violation.message)
            .collect()
    }

    #[test]
    fn reports_banned_calls_in_normal_code() {
        let violations = lint_source("fn main() { let _v = Vec::new(); }");
        assert!(
            violations.iter().any(|msg| msg.contains("LpVec::new")),
            "expected Vec::new violation, got {violations:?}"
        );
    }

    #[test]
    fn allows_cfg_test_items() {
        let violations = lint_source(
            r#"
            #[cfg(test)]
            fn helper() {
                let _v = Vec::new();
            }
            "#,
        );
        assert!(
            violations.is_empty(),
            "expected no violations, got {violations:?}"
        );
    }

    #[test]
    fn allows_cfg_test_file() {
        let violations = lint_source(
            r#"
            #![cfg(test)]

            fn helper() {
                let _v = Vec::new();
            }
            "#,
        );
        assert!(
            violations.is_empty(),
            "expected no violations, got {violations:?}"
        );
    }
}
