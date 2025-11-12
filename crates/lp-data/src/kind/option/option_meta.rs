//! Metadata types for Option shapes.

#[cfg(feature = "alloc")]
use alloc::string::String;

/// Trait for Option shape metadata.
///
/// Provides polymorphic access regardless of whether metadata is stored
/// statically (`&'static str`) or dynamically (`String`).
pub trait OptionMeta {
    /// Get the name of this Option type.
    fn name(&self) -> &str;

    /// Get the documentation for this Option type, if any.
    fn docs(&self) -> Option<&str>;
}

/// Static metadata for an Option shape.
///
/// Uses `&'static str` for zero-cost storage.
#[derive(Debug, Clone, Copy)]
pub struct OptionMetaStatic {
    /// Name of this Option type.
    pub name: &'static str,

    /// Documentation for this Option type.
    pub docs: Option<&'static str>,
}

/// Dynamic metadata for an Option shape.
///
/// Uses `String` for runtime-allocated strings.
#[derive(Debug)]
pub struct OptionMetaDyn {
    /// Name of this Option type.
    pub name: String,

    /// Documentation for this Option type.
    pub docs: Option<String>,
}

impl OptionMeta for OptionMetaStatic {
    fn name(&self) -> &str {
        self.name
    }

    fn docs(&self) -> Option<&str> {
        self.docs
    }
}

impl OptionMeta for OptionMetaDyn {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn docs(&self) -> Option<&str> {
        self.docs.as_deref()
    }
}
