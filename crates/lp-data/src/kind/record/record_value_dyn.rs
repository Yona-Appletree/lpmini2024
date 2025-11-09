//! Dynamic record value implementation.
//!
//! Dynamic record values are created at runtime and store their fields in a collection.
//! This is in contrast to static record values, which are Rust structs that implement
//! `RecordValue` directly via codegen.

// TODO: Implement dynamic record values once we resolve the open questions
// about value storage (see DESIGN.md).
