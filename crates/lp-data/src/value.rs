//! Runtime value system for lp-data.
//!
//! Provides `LpValue` enum for storing and accessing runtime values with type safety.

#[cfg(feature = "alloc")]
use alloc::{format, string::String, vec::Vec};

use lp_math::fixed::Fixed;
use lp_pool::collections::{LpBox, LpString, LpVec};
use lp_pool::error::AllocError;

use crate::shape::array::array_value::ArrayValue;
use crate::shape::map::MapValue;
use crate::shape::option::option_value::OptionValue;
use crate::shape::r#enum::enum_value::EnumValue;
use crate::shape::record::StructValue;
use crate::shape::shape_ref::ShapeRef;
use crate::shape::value::LpValueTrait;

/// Runtime error for value operations.
#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeError {
    TypeMismatch {
        expected: &'static str, // Shape name or description
        actual: &'static str,
    },
    FieldNotFound {
        record_name: &'static str,
        field_name: &'static str,
    },
    IndexOutOfBounds {
        array_len: usize,
        index: usize,
    },
    InvalidPath {
        path: &'static str,
        reason: &'static str,
    },
    AllocError(AllocError),
    NotAnOption,
    OptionIsNone,
    NotARecord,
    NotAnArray,
    NotAScalar,
    NotAVector,
}

impl From<AllocError> for RuntimeError {
    fn from(err: AllocError) -> Self {
        RuntimeError::AllocError(err)
    }
}

/// Runtime value that can hold any lp-data type.
///
/// Each variant stores the actual value and carries type information
/// for type-safe access and mutation.
pub enum LpValue {
    /// Fixed-point scalar value.
    Fixed(Fixed),
    /// 32-bit integer scalar value.
    Int32(i32),
    /// Boolean scalar value.
    Bool(bool),
    /// String scalar value (pool-allocated).
    String(LpString),
    /// 2D vector value.
    Vec2(Fixed, Fixed),
    /// 3D vector value.
    Vec3(Fixed, Fixed, Fixed),
    /// 4D vector value.
    Vec4(Fixed, Fixed, Fixed, Fixed),
    /// Optional value (None or Some).
    Option(OptionValue),
    /// Array of values (pool-allocated).
    Array(ArrayValue),
    /// Static struct value (from Rust structs).
    Struct(StructValue),
    /// Dynamic map value (runtime-created records).
    Map(MapValue),
    /// Enum value with variant name and optional payload.
    Enum(EnumValue),
}

impl LpValue {
    /// Get the shape reference for this value.
    pub fn shape(&self) -> &ShapeRef {
        match self {
            LpValue::Fixed(_) => {
                static DEFAULT: ShapeRef = ShapeRef::fixed_default();
                &DEFAULT
            }
            LpValue::Int32(_) => {
                static DEFAULT: ShapeRef = ShapeRef::int32_default();
                &DEFAULT
            }
            LpValue::Bool(_) => {
                static DEFAULT: ShapeRef = ShapeRef::bool_default();
                &DEFAULT
            }
            LpValue::String(_) => {
                static DEFAULT: ShapeRef = ShapeRef::string_default();
                &DEFAULT
            }
            LpValue::Vec2(_, _) => {
                static DEFAULT: ShapeRef = ShapeRef::vec2_default();
                &DEFAULT
            }
            LpValue::Vec3(_, _, _) => {
                static DEFAULT: ShapeRef = ShapeRef::vec3_default();
                &DEFAULT
            }
            LpValue::Vec4(_, _, _, _) => {
                static DEFAULT: ShapeRef = ShapeRef::vec4_default();
                &DEFAULT
            }
            LpValue::Option(opt) => &opt.shape,
            LpValue::Array(arr) => &arr.shape,
            LpValue::Struct(s) => &s.shape,
            LpValue::Map(m) => &m.shape,
            LpValue::Enum(e) => &e.shape,
        }
    }

    /// Construct a Fixed value.
    pub fn fixed(value: Fixed) -> Self {
        Self::Fixed(value)
    }

    /// Construct an Int32 value.
    pub fn int32(value: i32) -> Self {
        Self::Int32(value)
    }

    /// Construct a Bool value.
    pub fn bool(value: bool) -> Self {
        Self::Bool(value)
    }

    /// Construct a String value.
    pub fn string(value: LpString) -> Self {
        Self::String(value)
    }

    /// Construct a Vec2 value.
    pub fn vec2(x: Fixed, y: Fixed) -> Self {
        Self::Vec2(x, y)
    }

    /// Construct a Vec3 value.
    pub fn vec3(x: Fixed, y: Fixed, z: Fixed) -> Self {
        Self::Vec3(x, y, z)
    }

    /// Construct a Vec4 value.
    pub fn vec4(x: Fixed, y: Fixed, z: Fixed, w: Fixed) -> Self {
        Self::Vec4(x, y, z, w)
    }

    /// Construct an Option::None value.
    pub fn try_option_none(shape: ShapeRef) -> Result<Self, AllocError> {
        OptionValue::try_none(shape).map(Self::Option)
    }

    /// Construct an Option::Some value.
    pub fn try_option_some(shape: ShapeRef, value: LpValue) -> Result<Self, AllocError> {
        OptionValue::try_some(shape, value).map(Self::Option)
    }

    /// Construct a new struct value from a ShapeRef.
    ///
    /// Initializes all fields to default values based on their shapes.
    pub fn try_struct(shape: ShapeRef) -> Result<Self, AllocError> {
        StructValue::try_new(shape).map(Self::Struct)
    }

    /// Construct a new map value (dynamic record).
    pub fn try_map() -> Result<Self, AllocError> {
        MapValue::try_new().map(Self::Map)
    }

    /// Legacy alias for try_struct (for backwards compatibility).
    pub fn try_record(shape: ShapeRef) -> Result<Self, AllocError> {
        Self::try_struct(shape)
    }

    /// Construct a new array value.
    pub fn try_array(shape: ShapeRef, capacity: usize) -> Result<Self, AllocError> {
        ArrayValue::try_new(shape, capacity).map(Self::Array)
    }

    /// Construct a new value from a ShapeRef.
    ///
    /// Creates a default value based on the shape:
    /// - Scalars: zero/false/empty
    /// - Vectors: all zeros
    /// - Arrays: empty array
    /// - Records: all fields initialized to defaults
    /// - Options: None
    pub fn try_new_from_shape(shape: ShapeRef) -> Result<Self, AllocError> {
        match shape {
            ShapeRef::Fixed(_) => Ok(Self::Fixed(Fixed::ZERO)),
            ShapeRef::Int32(_) => Ok(Self::Int32(0)),
            ShapeRef::Bool(_) => Ok(Self::Bool(false)),
            ShapeRef::String(_) => Ok(Self::String(LpString::new())),
            ShapeRef::Vec2(_) => Ok(Self::Vec2(Fixed::ZERO, Fixed::ZERO)),
            ShapeRef::Vec3(_) => Ok(Self::Vec3(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)),
            ShapeRef::Vec4(_) => Ok(Self::Vec4(
                Fixed::ZERO,
                Fixed::ZERO,
                Fixed::ZERO,
                Fixed::ZERO,
            )),
            ShapeRef::Array(_) => {
                // For arrays, we need to create a new array value with the same shape
                // The shape will be stored in ArrayValue
                ArrayValue::try_new(shape, 0).map(Self::Array)
            }
            ShapeRef::Record(_) => StructValue::try_new(shape).map(Self::Struct),
            ShapeRef::Map(_) => Self::try_map(),
            ShapeRef::Enum(_) => {
                // Enums need a variant to be specified, can't create default
                Err(AllocError::InvalidLayout)
            }
            ShapeRef::Option(ref opt_ref) => {
                // Extract inner shape from OptionShape
                use crate::shape::shape::OptionShape;
                let _inner_shape = match opt_ref {
                    crate::shape::shape_ref::OptionShapeRef::Static(os) => os.inner(),
                    crate::shape::shape_ref::OptionShapeRef::Dynamic(_) => {
                        return Err(AllocError::InvalidLayout); // Dynamic options not yet supported
                    }
                };
                // Create Option::None with the Option shape itself
                // The shape will be stored in OptionValue
                Self::try_option_none(shape)
            }
            ShapeRef::Tuple(_) => {
                // Tuples need all elements specified
                Err(AllocError::InvalidLayout)
            }
        }
    }

    /// Get a field by name from a struct or map value.
    pub fn get_field(&self, name: &str) -> Result<&LpValue, RuntimeError> {
        match self {
            LpValue::Struct(s) => s.get_field(name),
            LpValue::Map(m) => m.get_field(name),
            _ => Err(RuntimeError::NotARecord),
        }
    }

    /// Get a mutable field by name from a struct or map value.
    pub fn get_field_mut(&mut self, name: &str) -> Result<&mut LpValue, RuntimeError> {
        match self {
            LpValue::Struct(s) => s.get_field_mut(name),
            LpValue::Map(m) => m.get_field_mut(name),
            _ => Err(RuntimeError::NotARecord),
        }
    }

    /// Get an element by index from an array value.
    pub fn get_element(&self, index: usize) -> Result<&LpValue, RuntimeError> {
        match self {
            LpValue::Array(arr) => arr.get(index),
            _ => Err(RuntimeError::NotAnArray),
        }
    }

    /// Get a mutable element by index from an array value.
    pub fn get_element_mut(&mut self, index: usize) -> Result<&mut LpValue, RuntimeError> {
        match self {
            LpValue::Array(arr) => arr.get_mut(index),
            _ => Err(RuntimeError::NotAnArray),
        }
    }

    /// Check if an Option value is Some.
    pub fn is_some(&self) -> bool {
        match self {
            LpValue::Option(opt) => opt.is_some(),
            _ => false,
        }
    }

    /// Check if an Option value is None.
    pub fn is_none(&self) -> bool {
        match self {
            LpValue::Option(opt) => opt.is_none(),
            _ => false,
        }
    }

    /// Unwrap an Option value, returning the inner value.
    pub fn try_unwrap(&self) -> Result<&LpValue, RuntimeError> {
        match self {
            LpValue::Option(opt) => opt.try_unwrap(),
            _ => Err(RuntimeError::NotAnOption),
        }
    }

    /// Unwrap an Option value mutably, returning the inner value.
    pub fn try_unwrap_mut(&mut self) -> Result<&mut LpValue, RuntimeError> {
        match self {
            LpValue::Option(opt) => opt.try_unwrap_mut(),
            _ => Err(RuntimeError::NotAnOption),
        }
    }

    /// Get value as Fixed, if it is a Fixed scalar.
    pub fn as_fixed(&self) -> Result<Fixed, RuntimeError> {
        fixed_value::as_fixed(self)
    }

    /// Get value as Int32, if it is an Int32 scalar.
    pub fn as_int32(&self) -> Result<i32, RuntimeError> {
        int32_value::as_int32(self)
    }

    /// Get value as Bool, if it is a Bool scalar.
    pub fn as_bool(&self) -> Result<bool, RuntimeError> {
        bool_value::as_bool(self)
    }

    /// Get value as string slice, if it is a String scalar.
    pub fn as_string(&self) -> Result<&LpString, RuntimeError> {
        match self {
            Self::String(s) => Ok(s),
            _ => Err(RuntimeError::TypeMismatch {
                expected: "String",
                actual: "other",
            }),
        }
    }

    /// Get value as Vec2, if it is a Vec2.
    pub fn as_vec2(&self) -> Result<(Fixed, Fixed), RuntimeError> {
        vec2_value::as_vec2(self)
    }

    /// Get value as Vec3, if it is a Vec3.
    pub fn as_vec3(&self) -> Result<(Fixed, Fixed, Fixed), RuntimeError> {
        vec3_value::as_vec3(self)
    }

    /// Get value as Vec4, if it is a Vec4.
    pub fn as_vec4(&self) -> Result<(Fixed, Fixed, Fixed, Fixed), RuntimeError> {
        vec4_value::as_vec4(self)
    }

    /// Set a field value in a struct or map.
    pub fn try_set_field(&mut self, name: &str, value: LpValue) -> Result<(), RuntimeError> {
        match self {
            LpValue::Struct(s) => s.try_set_field(name, value),
            LpValue::Map(m) => m.try_set_field(name, value),
            _ => Err(RuntimeError::NotARecord),
        }
    }

    /// Set an element value in an array.
    pub fn try_set_element(&mut self, index: usize, value: LpValue) -> Result<(), RuntimeError> {
        match self {
            LpValue::Array(arr) => arr.try_set(index, value),
            _ => Err(RuntimeError::NotAnArray),
        }
    }

    /// Push a value to an array.
    pub fn try_push_element(&mut self, value: LpValue) -> Result<(), RuntimeError> {
        match self {
            LpValue::Array(arr) => arr.try_push(value),
            _ => Err(RuntimeError::NotAnArray),
        }
    }

    /// Get a value by path (e.g., "nodes.lfo.output").
    ///
    /// Supports dot-separated paths for nested access.
    pub fn get_path(&self, path: &str) -> Result<&LpValue, RuntimeError> {
        if path.is_empty() {
            return Err(RuntimeError::InvalidPath {
                path: "",
                reason: "empty path",
            });
        }

        let mut current = self;
        for part in path.split('.') {
            if part.is_empty() {
                continue;
            }

            // Try as field name first (for records)
            if let Ok(field) = current.get_field(part) {
                current = field;
                continue;
            }

            // Try as array index
            if let Ok(index) = part.parse::<usize>() {
                if let Ok(element) = current.get_element(index) {
                    current = element;
                    continue;
                }
            }

            return Err(RuntimeError::InvalidPath {
                path: "",
                reason: "could not resolve path component",
            });
        }

        Ok(current)
    }

    /// Get a mutable value by path.
    ///
    /// Note: For nested paths (e.g., "a.b.c"), this requires multiple mutable borrows
    /// which Rust doesn't allow. For now, this only supports single-level paths.
    /// For nested paths, use `get_field_mut` repeatedly.
    pub fn get_path_mut(&mut self, path: &str) -> Result<&mut LpValue, RuntimeError> {
        if path.is_empty() {
            return Err(RuntimeError::InvalidPath {
                path: "",
                reason: "empty path",
            });
        }

        let mut parts = path.split('.');
        let first = parts.next().unwrap_or("");

        // For single-level paths, only support field access
        // For array access, use get_element_mut directly
        if parts.next().is_none() {
            return self
                .get_field_mut(first)
                .map_err(|_| RuntimeError::InvalidPath {
                    path: "",
                    reason: "could not resolve path component",
                });
        }

        // For nested paths, we need to use unsafe or a different approach
        // For now, return an error suggesting to use get_field_mut repeatedly
        Err(RuntimeError::InvalidPath {
            path: "",
            reason: "nested mutable path access requires multiple calls to get_field_mut",
        })
    }
}

impl LpValueTrait for LpValue {
    fn shape(&self) -> &ShapeRef {
        // Call the inherent method using fully qualified syntax to avoid recursion
        LpValue::shape(self)
    }
}

impl core::fmt::Debug for LpValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Fixed(v) => write!(f, "Fixed({:?})", v),
            Self::Int32(v) => write!(f, "Int32({})", v),
            Self::Bool(v) => write!(f, "Bool({})", v),
            Self::String(v) => write!(f, "String({:?})", v),
            Self::Vec2(x, y) => write!(f, "Vec2({:?}, {:?})", x, y),
            Self::Vec3(x, y, z) => write!(f, "Vec3({:?}, {:?}, {:?})", x, y, z),
            Self::Vec4(x, y, z, w) => write!(f, "Vec4({:?}, {:?}, {:?}, {:?})", x, y, z, w),
            Self::Option(opt) => write!(f, "Option({:?})", opt),
            Self::Array(arr) => write!(f, "Array({:?})", arr),
            Self::Struct(s) => write!(f, "Struct({:?})", s),
            Self::Map(m) => write!(f, "Map({:?})", m),
            Self::Enum(e) => write!(f, "Enum({:?})", e),
        }
    }
}
