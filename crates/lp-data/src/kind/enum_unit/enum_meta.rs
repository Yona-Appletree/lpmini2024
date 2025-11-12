//! Metadata types for Enum shapes.

use alloc::string::String;

/// Trait for Enum shape metadata.
///
/// This trait allows polymorphic access to metadata regardless of whether
/// it's stored as static strings (`&'static str`) or dynamic `String` values.
pub trait EnumUnitMeta {
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
pub struct EnumUnitMetaStatic {
    /// Name of this enum type.
    pub name: &'static str,

    /// Documentation for this enum type.
    pub docs: Option<&'static str>,
}

/// Dynamic metadata for an Enum shape.
///
/// Uses `String` for runtime-allocated strings.
#[derive(Debug)]
pub struct EnumUnitMetaDyn {
    /// Name of this enum type.
    pub name: String,

    /// Documentation for this enum type.
    pub docs: Option<String>,
}

impl EnumUnitMeta for EnumUnitMetaStatic {
    fn name(&self) -> &str {
        self.name
    }

    fn docs(&self) -> Option<&str> {
        self.docs
    }
}

impl EnumUnitMeta for EnumUnitMetaDyn {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn docs(&self) -> Option<&str> {
        self.docs.as_deref()
    }
}

/// Trait for enum variant metadata.
pub trait EnumUnitVariantMeta {
    /// Get documentation for this variant.
    fn docs(&self) -> Option<&str>;
}

/// Static metadata for an enum variant.
///
/// Uses `&'static str` for zero-cost string storage.
#[derive(Debug, Clone, Copy)]
pub struct EnumUnitVariantMetaStatic {
    /// Documentation for this variant.
    pub docs: Option<&'static str>,
}

/// Dynamic metadata for an enum variant.
///
/// Uses `String` for runtime-allocated strings.
#[derive(Debug)]
pub struct EnumUnitVariantMetaDyn {
    /// Documentation for this variant.
    pub docs: Option<String>,
}

impl EnumUnitVariantMeta for EnumUnitVariantMetaStatic {
    fn docs(&self) -> Option<&str> {
        self.docs
    }
}

impl EnumUnitVariantMeta for EnumUnitVariantMetaDyn {
    fn docs(&self) -> Option<&str> {
        self.docs.as_deref()
    }
}
