//! Runtime value system for lp-data.
//!
//! Provides `LpValue` enum for storing and accessing runtime values with type safety.

#[cfg(feature = "alloc")]
use alloc::{format, string::String, vec::Vec};

use lp_math::fixed::Fixed;
use lp_pool::collections::{LpBox, LpString, LpVec};
use lp_pool::error::AllocError;

use crate::metadata::{LpType, LpTypeMeta, TypeRef};
use crate::types::{
    bool as bool_value, fixed as fixed_value, int32 as int32_value, string as string_value,
    vec2 as vec2_value, vec3 as vec3_value, vec4 as vec4_value,
};
use crate::types::{ArrayValue, EnumValue, MapValue, OptionValue, StructValue};

/// Runtime error for value operations.
#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeError {
    TypeMismatch {
        expected: LpType,
        actual: LpType,
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
    /// Get the type types for this value.
    ///
    /// Note: For scalar and vector types, this creates temporary types.
    /// For records, arrays, enums, and options, it returns the stored type reference.
    pub fn ty(&self) -> LpTypeMeta {
        match self {
            LpValue::Fixed(_) => LpTypeMeta::new(LpType::fixed()),
            LpValue::Int32(_) => LpTypeMeta::new(LpType::int32()),
            LpValue::Bool(_) => LpTypeMeta::new(LpType::boolean()),
            LpValue::String(_) => LpTypeMeta::new(LpType::string()),
            LpValue::Vec2(_, _) => LpTypeMeta::new(LpType::vec2()),
            LpValue::Vec3(_, _, _) => LpTypeMeta::new(LpType::vec3()),
            LpValue::Vec4(_, _, _, _) => LpTypeMeta::new(LpType::vec4()),
            LpValue::Option(opt) => LpTypeMeta::new(LpType::option(opt.inner_type)),
            LpValue::Array(arr) => LpTypeMeta::new(LpType::Array(crate::metadata::ArrayType::new(
                arr.element_type,
            ))),
            LpValue::Struct(s) => (*s.struct_type).clone(),
            LpValue::Map(m) => (*m.map_type.as_ref()).clone(),
            LpValue::Enum(e) => (*e.enum_type).clone(),
        }
    }

    /// Construct a Fixed value.
    pub fn fixed(value: Fixed) -> Self {
        fixed_value::fixed(value)
    }

    /// Construct an Int32 value.
    pub fn int32(value: i32) -> Self {
        int32_value::int32(value)
    }

    /// Construct a Bool value.
    pub fn bool(value: bool) -> Self {
        bool_value::bool(value)
    }

    /// Construct a String value.
    pub fn string(value: LpString) -> Self {
        string_value::string(value)
    }

    /// Construct a Vec2 value.
    pub fn vec2(x: Fixed, y: Fixed) -> Self {
        vec2_value::vec2(x, y)
    }

    /// Construct a Vec3 value.
    pub fn vec3(x: Fixed, y: Fixed, z: Fixed) -> Self {
        vec3_value::vec3(x, y, z)
    }

    /// Construct a Vec4 value.
    pub fn vec4(x: Fixed, y: Fixed, z: Fixed, w: Fixed) -> Self {
        vec4_value::vec4(x, y, z, w)
    }

    /// Construct an Option::None value.
    pub fn try_option_none(inner_type: TypeRef) -> Result<Self, AllocError> {
        OptionValue::try_none(inner_type).map(Self::Option)
    }

    /// Construct an Option::Some value.
    pub fn try_option_some(inner_type: TypeRef, value: LpValue) -> Result<Self, AllocError> {
        OptionValue::try_some(inner_type, value).map(Self::Option)
    }

    /// Construct a new struct value from RecordType metadata.
    ///
    /// Initializes all fields to default values based on their types.
    pub fn try_struct(struct_type: TypeRef) -> Result<Self, AllocError> {
        StructValue::try_new(struct_type).map(Self::Struct)
    }

    /// Construct a new map value (dynamic record).
    pub fn try_map() -> Result<Self, AllocError> {
        MapValue::try_new().map(Self::Map)
    }

    /// Legacy alias for try_struct (for backwards compatibility).
    pub fn try_record(record_type: TypeRef) -> Result<Self, AllocError> {
        Self::try_struct(record_type)
    }

    /// Construct a new array value.
    pub fn try_array(element_type: TypeRef, capacity: usize) -> Result<Self, AllocError> {
        ArrayValue::try_new(element_type, capacity).map(Self::Array)
    }

    /// Construct a new value from type types.
    ///
    /// Creates a default value based on the type:
    /// - Scalars: zero/false/empty
    /// - Vectors: all zeros
    /// - Arrays: empty array
    /// - Records: all fields initialized to defaults
    /// - Options: None
    pub fn try_new(ty: &LpTypeMeta) -> Result<Self, AllocError> {
        match &ty.ty {
            LpType::Fixed(_) => Ok(Self::Fixed(Fixed::ZERO)),
            LpType::Int32(_) => Ok(Self::Int32(0)),
            LpType::Bool(_) => Ok(Self::Bool(false)),
            LpType::String(_) => Ok(Self::String(LpString::new())),
            LpType::Vec2(_) => Ok(Self::Vec2(Fixed::ZERO, Fixed::ZERO)),
            LpType::Vec3(_) => Ok(Self::Vec3(Fixed::ZERO, Fixed::ZERO, Fixed::ZERO)),
            LpType::Vec4(_) => Ok(Self::Vec4(
                Fixed::ZERO,
                Fixed::ZERO,
                Fixed::ZERO,
                Fixed::ZERO,
            )),
            LpType::Array(array) => Self::try_array(array.element, 0),
            LpType::Record(_) => {
                // Records require static type types (TypeRef)
                // For non-static types, we can't create records
                Err(AllocError::InvalidLayout)
            }
            LpType::Map(_) => Self::try_map(),
            LpType::Enum(_) => {
                // Enums need a variant to be specified, can't create default
                Err(AllocError::InvalidLayout)
            }
            LpType::Option(opt) => Self::try_option_none(opt.inner),
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
    pub fn as_string(&self) -> Result<&str, RuntimeError> {
        string_value::as_string(self)
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
