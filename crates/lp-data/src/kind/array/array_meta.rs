//! Metadata types for Array shapes.

use alloc::string::String;

/// Trait for Array shape metadata.
///
/// This trait allows polymorphic access to metadata regardless of whether
/// it's stored as static strings (`&'static str`) or dynamic `String` values.
pub trait ArrayMeta {
    /// Get the name of this array type.
    fn name(&self) -> &str;

    /// Get the documentation for this array type.
    fn docs(&self) -> Option<&str>;
}

/// Static metadata for an Array shape.
///
/// Uses `&'static str` for zero-cost string storage.
/// Can be `Copy` since all fields are `Copy`.
#[derive(Debug, Clone, Copy)]
pub struct ArrayMetaStatic {
    /// Name of this array type.
    pub name: &'static str,

    /// Documentation for this array type.
    pub docs: Option<&'static str>,
}

/// Dynamic metadata for an Array shape.
///
/// Uses `String` for runtime-allocated strings.
#[derive(Debug)]
pub struct ArrayMetaDyn {
    /// Name of this array type.
    pub name: String,

    /// Documentation for this array type.
    pub docs: Option<String>,
}

impl ArrayMeta for ArrayMetaStatic {
    fn name(&self) -> &str {
        self.name
    }

    fn docs(&self) -> Option<&str> {
        self.docs
    }
}

impl ArrayMeta for ArrayMetaDyn {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn docs(&self) -> Option<&str> {
        self.docs.as_deref()
    }
}
