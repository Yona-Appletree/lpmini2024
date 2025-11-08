//! Option/Optional type metadata.

/// Metadata for optional types.
#[derive(Debug, Clone, PartialEq)]
pub struct OptionType<T> {
    pub inner: T,
}

impl<T> OptionType<T> {
    pub const fn new(inner: T) -> Self {
        Self { inner }
    }
}
