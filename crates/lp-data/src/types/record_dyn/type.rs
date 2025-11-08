//! Dynamic record/map type metadata.

/// Metadata for dynamic record types (maps).
///
/// Unlike static RecordType, MapType represents records that can be
/// created and modified at runtime with arbitrary field names.
#[derive(Debug, Clone, PartialEq)]
pub struct MapType {
    // For now, fully dynamic - any fields allowed
    // Future: could add optional schema for validation
}

impl MapType {
    pub const fn new() -> Self {
        Self {}
    }
}
