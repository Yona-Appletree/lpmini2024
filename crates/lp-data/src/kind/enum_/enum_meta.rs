//! Metadata types for Enum shapes.

use alloc::string::String;

/// Trait for Enum shape metadata.
///
/// This trait allows polymorphic access to metadata regardless of whether
/// it's stored as static strings (`&'static str`) or dynamic `String` values.
pub trait EnumMeta {
    /// Get the name of this enum type.
    fn name(&self) -> &str;

    /// Get the documentation for this enum type.
    fn docs(&self) -> Option<&str>;
}

/// Static metadata for an Enum shape.
///
/// Uses `&'static str` for zero-cost string storage.
/// Can be `Copy` since all fields are `Copy`.
#[derive(Debug, Clone, Copy)]
pub struct EnumMetaStatic {
    /// Name of this enum type.
    pub name: &'static str,

    /// Documentation for this enum type.
    pub docs: Option<&'static str>,
}

/// Dynamic metadata for an Enum shape.
///
/// Uses `String` for runtime-allocated strings.
#[derive(Debug)]
pub struct EnumMetaDyn {
    /// Name of this enum type.
    pub name: String,

    /// Documentation for this enum type.
    pub docs: Option<String>,
}

impl EnumMeta for EnumMetaStatic {
    fn name(&self) -> &str {
        self.name
    }

    fn docs(&self) -> Option<&str> {
        self.docs
    }
}

impl EnumMeta for EnumMetaDyn {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn docs(&self) -> Option<&str> {
        self.docs.as_ref().map(|s| s.as_str())
    }
}

/// Trait for enum variant metadata.
pub trait EnumVariantMeta {
    /// Get documentation for this variant.
    fn docs(&self) -> Option<&str>;
}

/// Static metadata for an enum variant.
///
/// Uses `&'static str` for zero-cost string storage.
#[derive(Debug, Clone, Copy)]
pub struct EnumVariantMetaStatic {
    /// Documentation for this variant.
    pub docs: Option<&'static str>,
}

/// Dynamic metadata for an enum variant.
///
/// Uses `String` for runtime-allocated strings.
#[derive(Debug)]
pub struct EnumVariantMetaDyn {
    /// Documentation for this variant.
    pub docs: Option<String>,
}

impl EnumVariantMeta for EnumVariantMetaStatic {
    fn docs(&self) -> Option<&str> {
        self.docs
    }
}

impl EnumVariantMeta for EnumVariantMetaDyn {
    fn docs(&self) -> Option<&str> {
        self.docs.as_ref().map(|s| s.as_str())
    }
}
