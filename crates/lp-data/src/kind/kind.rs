//! LpKind enumeration and related types.

/// Basic kinds of data in the lp-data system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LpKind {
    /// Fixed-point number (16.16 format)
    Fixed,

    /// Record (struct-like composite type)
    Record,
}
