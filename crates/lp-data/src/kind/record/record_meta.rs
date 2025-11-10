//! Metadata types for Record field shapes and Record shapes.

use alloc::string::String;

/// Trait for record field metadata.
pub trait RecordFieldMeta {
    /// Get documentation for this field.
    fn docs(&self) -> Option<&str>;
}

/// Static metadata for a record field.
///
/// Uses `&'static str` for zero-cost string storage.
#[derive(Debug, Clone, Copy)]
pub struct RecordFieldMetaStatic {
    /// Documentation for this field.
    pub docs: Option<&'static str>,
}

/// Dynamic metadata for a record field.
///
/// Uses `String` for runtime-allocated strings.
#[derive(Debug)]
pub struct RecordFieldMetaDyn {
    /// Documentation for this field.
    pub docs: Option<String>,
}

impl RecordFieldMeta for RecordFieldMetaStatic {
    fn docs(&self) -> Option<&str> {
        self.docs
    }
}

impl RecordFieldMeta for RecordFieldMetaDyn {
    fn docs(&self) -> Option<&str> {
        self.docs.as_ref().map(|s| s.as_str())
    }
}

/// Trait for Record shape metadata.
///
/// This trait allows polymorphic access to metadata regardless of whether
/// it's stored as static strings (`&'static str`) or dynamic `String` values.
pub trait RecordMeta {
    /// Get the name of this record type.
    fn name(&self) -> &str;

    /// Get the documentation for this record type.
    fn docs(&self) -> Option<&str>;
}

/// Static metadata for a Record shape.
///
/// Uses `&'static str` for zero-cost string storage.
/// Can be `Copy` since all fields are `Copy`.
#[derive(Debug, Clone, Copy)]
pub struct RecordMetaStatic {
    /// Name of this record type.
    pub name: &'static str,

    /// Documentation for this record type.
    pub docs: Option<&'static str>,
}

/// Dynamic metadata for a Record shape.
///
/// Uses `String` for runtime-allocated strings.
#[derive(Debug)]
pub struct RecordMetaDyn {
    /// Name of this record type.
    pub name: String,

    /// Documentation for this record type.
    pub docs: Option<String>,
}

impl RecordMeta for RecordMetaStatic {
    fn name(&self) -> &str {
        self.name
    }

    fn docs(&self) -> Option<&str> {
        self.docs
    }
}

impl RecordMeta for RecordMetaDyn {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn docs(&self) -> Option<&str> {
        self.docs.as_ref().map(|s| s.as_str())
    }
}
