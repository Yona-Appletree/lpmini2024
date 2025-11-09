//! Metadata types for Fixed shapes.

use lp_pool::LpString;

/// Trait for Fixed shape metadata.
///
/// This trait allows polymorphic access to metadata regardless of whether
/// it's stored as static strings (`&'static str`) or dynamic strings (`LpString`).
pub trait FixedMeta {
    /// Get the label for this fixed value.
    fn label(&self) -> &str;

    /// Get the markdown documentation for this fixed value.
    fn desc_md(&self) -> Option<&str>;

    /// Get the unit string (e.g., "ms", "Hz", "px").
    fn unit(&self) -> Option<&str>;
}

/// Static metadata for a Fixed shape.
///
/// Uses `&'static str` for zero-cost string storage.
/// Can be `Copy` since all fields are `Copy`.
#[derive(Debug, Clone, Copy)]
pub struct FixedMetaStatic {
    /// Label for this fixed value (e.g., "Frequency").
    pub label: &'static str,

    /// Markdown documentation for this fixed value.
    pub desc_md: Option<&'static str>,

    /// Unit string (e.g., "ms", "Hz", "px").
    pub unit: Option<&'static str>,
}

/// Dynamic metadata for a Fixed shape.
///
/// Uses `LpString` for runtime-allocated strings.
#[derive(Debug)]
pub struct FixedMetaDyn {
    /// Label for this fixed value.
    pub label: LpString,

    /// Markdown documentation for this fixed value.
    pub desc_md: Option<LpString>,

    /// Unit string.
    pub unit: Option<LpString>,
}

impl FixedMeta for FixedMetaStatic {
    fn label(&self) -> &str {
        self.label
    }

    fn desc_md(&self) -> Option<&str> {
        self.desc_md
    }

    fn unit(&self) -> Option<&str> {
        self.unit
    }
}

impl FixedMeta for FixedMetaDyn {
    fn label(&self) -> &str {
        self.label.as_str()
    }

    fn desc_md(&self) -> Option<&str> {
        self.desc_md.as_ref().map(|s| s.as_str())
    }

    fn unit(&self) -> Option<&str> {
        self.unit.as_ref().map(|s| s.as_str())
    }
}

// Alternative approach using generics (commented out):
// This would work but has drawbacks:
// - Can't have `Copy` for the static version (would need conditional derives)
// - More complex trait bounds
// - Less flexible if we need different implementations later
//
// pub struct FixedMeta<S> {
//     pub label: S,
//     pub desc_md: Option<S>,
//     pub unit: Option<S>,
// }
//
// pub type FixedMetaStatic = FixedMeta<&'static str>;
// pub type FixedMetaDyn = FixedMeta<LpString>;
