//! Shape trait hierarchy for type metadata.

use crate::shape::kind::LpKind;
use crate::shape::shape_ref::ShapeRef;

/// Base trait for all shapes (type metadata).
pub trait LpShape: core::fmt::Debug {
    /// Returns the kind of this shape.
    fn kind(&self) -> LpKind;
}

/// Shape for record/struct types.
pub trait RecordShape: LpShape {
    /// Returns the name of the record type.
    fn name(&self) -> &str;

    /// Returns the fields of the record.
    fn fields(&self) -> &[crate::shape::record::RecordField];
}

/// Shape for array types.
pub trait ArrayShape: LpShape {
    /// Returns the element shape reference.
    fn element(&self) -> &ShapeRef;
}

/// Shape for option types.
pub trait OptionShape: LpShape {
    /// Returns the inner shape reference.
    fn inner(&self) -> &ShapeRef;
}

/// Shape for tuple types.
pub trait TupleShape: LpShape {
    /// Returns the element shapes.
    fn elements(&self) -> &[ShapeRef];
}

/// Shape for map/dynamic record types.
pub trait MapShape: LpShape {
    // No additional metadata needed for maps
}

/// Shape for enum types.
pub trait EnumShape: LpShape {
    /// Returns the variants of the enum.
    fn variants(&self) -> &[crate::shape::r#enum::EnumVariant];
}
