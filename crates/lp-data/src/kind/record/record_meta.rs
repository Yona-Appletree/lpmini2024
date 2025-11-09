//! Metadata types for Record field shapes.

use lp_pool::LpString;

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
/// Uses `LpString` for runtime-allocated strings.
#[derive(Debug)]
pub struct RecordFieldMetaDyn {
    /// Documentation for this field.
    pub docs: Option<LpString>,
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
