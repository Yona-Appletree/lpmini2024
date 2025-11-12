//! Metadata types for Union shapes.

use alloc::string::String;

/// Trait for Union shape metadata.
///
/// Provides polymorphic access regardless of whether metadata is stored
/// statically (`&'static str`) or dynamically (`String`).
pub trait EnumStructMeta {
    /// Get the name of this enum struct type.
    fn name(&self) -> &str;

    /// Get the documentation for this enum struct type, if any.
    fn docs(&self) -> Option<&str>;
}

/// Static metadata for a Union shape.
///
/// Uses `&'static str` for zero-cost storage.
#[derive(Debug, Clone, Copy)]
pub struct EnumStructMetaStatic {
    /// Name of this enum struct type.
    pub name: &'static str,

    /// Documentation for this enum struct type.
    pub docs: Option<&'static str>,
}

/// Dynamic metadata for a Union shape.
///
/// Uses `String` for runtime-allocated strings.
#[derive(Debug)]
pub struct EnumStructMetaDyn {
    /// Name of this enum struct type.
    pub name: String,

    /// Documentation for this enum struct type.
    pub docs: Option<String>,
}

impl EnumStructMeta for EnumStructMetaStatic {
    fn name(&self) -> &str {
        self.name
    }

    fn docs(&self) -> Option<&str> {
        self.docs
    }
}

impl EnumStructMeta for EnumStructMetaDyn {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn docs(&self) -> Option<&str> {
        self.docs.as_deref()
    }
}

/// Trait for Union variant metadata.
pub trait EnumStructVariantMeta {
    /// Get documentation for this variant, if any.
    fn docs(&self) -> Option<&str>;
}

/// Static metadata for a Union variant.
#[derive(Debug, Clone, Copy)]
pub struct EnumStructVariantMetaStatic {
    /// Documentation for this variant.
    pub docs: Option<&'static str>,
}

/// Dynamic metadata for a Union variant.
#[derive(Debug)]
pub struct EnumStructVariantMetaDyn {
    /// Documentation for this variant.
    pub docs: Option<String>,
}

impl EnumStructVariantMeta for EnumStructVariantMetaStatic {
    fn docs(&self) -> Option<&str> {
        self.docs
    }
}

impl EnumStructVariantMeta for EnumStructVariantMetaDyn {
    fn docs(&self) -> Option<&str> {
        self.docs.as_deref()
    }
}
