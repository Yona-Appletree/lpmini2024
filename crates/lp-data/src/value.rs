//! Runtime value system for lp-data.
//!
//! Provides `LpValue` enum for storing and accessing runtime values with type safety.

#[cfg(feature = "alloc")]
use alloc::{format, string::String, vec::Vec};

use lp_math::fixed::Fixed;
use lp_pool::collections::{LpBox, LpString, LpVec};
use lp_pool::error::AllocError;

use crate::metadata::{LpType, LpTypeMeta, TypeRef};

/// Runtime error for value operations.
#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeError {
    TypeMismatch {
        expected: LpType,
        actual: LpType,
    },
    FieldNotFound {
        record_name: String,
        field_name: String,
    },
    IndexOutOfBounds {
        array_len: usize,
        index: usize,
    },
    InvalidPath {
        path: String,
        reason: String,
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
    Option {
        inner_type: TypeRef,
        value: Option<LpBox<LpValue>>,
    },
    /// Array of values (pool-allocated).
    Array {
        element_type: TypeRef,
        values: LpVec<LpValue>,
    },
    /// Record/struct value (pool-allocated fields).
    Record {
        record_type: TypeRef,
        fields: LpVec<LpValue>,
    },
    /// Enum value with variant name and optional payload.
    Enum {
        enum_type: TypeRef,
        variant_name: LpString,
        payload: Option<LpBox<LpValue>>,
    },
}

impl LpValue {
    /// Get the type metadata for this value.
    ///
    /// Note: For scalar and vector types, this creates temporary metadata.
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
            LpValue::Option { inner_type, .. } => LpTypeMeta::new(LpType::option(*inner_type)),
            LpValue::Array { element_type, .. } => LpTypeMeta::new(LpType::Array(
                crate::metadata::ArrayType::new(*element_type),
            )),
            LpValue::Record { record_type, .. } => (*record_type).clone(),
            LpValue::Enum { enum_type, .. } => (*enum_type).clone(),
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
    pub fn try_option_none(inner_type: TypeRef) -> Result<Self, AllocError> {
        Ok(Self::Option {
            inner_type,
            value: None,
        })
    }

    /// Construct an Option::Some value.
    pub fn try_option_some(inner_type: TypeRef, value: LpValue) -> Result<Self, AllocError> {
        let boxed = LpBox::try_new(value)?;
        Ok(Self::Option {
            inner_type,
            value: Some(boxed),
        })
    }

    /// Construct a new record value from RecordType metadata.
    ///
    /// Initializes all fields to default values based on their types.
    pub fn try_record(record_type: TypeRef) -> Result<Self, AllocError> {
        let record_ty = match &record_type.ty {
            LpType::Record(rt) => rt,
            _ => return Err(AllocError::InvalidLayout), // Wrong type
        };

        let mut fields = LpVec::new();
        for field in record_ty.fields.iter() {
            let field_value = Self::try_new(&field.ty)?;
            fields.try_push(field_value)?;
        }

        Ok(Self::Record {
            record_type,
            fields,
        })
    }

    /// Construct a new array value.
    pub fn try_array(element_type: TypeRef, capacity: usize) -> Result<Self, AllocError> {
        let mut values = LpVec::new();
        if capacity > 0 {
            values.try_reserve(capacity)?;
        }
        Ok(Self::Array {
            element_type,
            values,
        })
    }

    /// Construct a new value from type metadata.
    ///
    /// Creates a default value based on the type:
    /// - Scalars: zero/false/empty
    /// - Vectors: all zeros
    /// - Arrays: empty array
    /// - Records: all fields initialized to defaults
    /// - Options: None
    pub fn try_new(ty: &LpTypeMeta) -> Result<Self, AllocError> {
        match &ty.ty {
            LpType::Scalar(scalar) => match scalar {
                crate::metadata::LpScalarType::Fixed(_) => Ok(Self::Fixed(Fixed::ZERO)),
                crate::metadata::LpScalarType::Int32(_) => Ok(Self::Int32(0)),
                crate::metadata::LpScalarType::Bool(_) => Ok(Self::Bool(false)),
                crate::metadata::LpScalarType::String(_) => Ok(Self::String(LpString::new())),
            },
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
                // Records require static type metadata (TypeRef)
                // For non-static types, we can't create records
                Err(AllocError::InvalidLayout)
            }
            LpType::Enum(_) => {
                // Enums need a variant to be specified, can't create default
                Err(AllocError::InvalidLayout)
            }
            LpType::Option(opt) => Self::try_option_none(opt.inner),
        }
    }

    /// Get a field by name from a record value.
    pub fn get_field(&self, name: &str) -> Result<&LpValue, RuntimeError> {
        let (record_type, fields) = match self {
            LpValue::Record {
                record_type,
                fields,
            } => (record_type, fields),
            _ => return Err(RuntimeError::NotARecord),
        };

        let record_ty = match &record_type.ty {
            LpType::Record(rt) => rt,
            _ => return Err(RuntimeError::NotARecord),
        };

        let field_index = record_ty
            .fields
            .iter()
            .position(|field| field.name == name)
            .ok_or_else(|| RuntimeError::FieldNotFound {
                record_name: String::from(record_ty.name),
                field_name: String::from(name),
            })?;

        fields
            .get(field_index)
            .ok_or_else(|| RuntimeError::IndexOutOfBounds {
                array_len: fields.len(),
                index: field_index,
            })
    }

    /// Get a mutable field by name from a record value.
    pub fn get_field_mut(&mut self, name: &str) -> Result<&mut LpValue, RuntimeError> {
        // First, get the field index without holding mutable borrow
        let field_index = match self {
            LpValue::Record { record_type, .. } => {
                let record_ty = match &record_type.ty {
                    LpType::Record(rt) => rt,
                    _ => return Err(RuntimeError::NotARecord),
                };
                record_ty
                    .fields
                    .iter()
                    .position(|field| field.name == name)
                    .ok_or_else(|| RuntimeError::FieldNotFound {
                        record_name: String::from(record_ty.name),
                        field_name: String::from(name),
                    })?
            }
            _ => return Err(RuntimeError::NotARecord),
        };

        // Get array length before mutable borrow
        let array_len = match self {
            LpValue::Record { fields, .. } => fields.len(),
            _ => return Err(RuntimeError::NotARecord),
        };

        // Now get mutable access to fields
        match self {
            LpValue::Record { fields, .. } => {
                fields
                    .get_mut(field_index)
                    .ok_or_else(|| RuntimeError::IndexOutOfBounds {
                        array_len,
                        index: field_index,
                    })
            }
            _ => Err(RuntimeError::NotARecord),
        }
    }

    /// Get an element by index from an array value.
    pub fn get_element(&self, index: usize) -> Result<&LpValue, RuntimeError> {
        match self {
            LpValue::Array { values, .. } => {
                values
                    .get(index)
                    .ok_or_else(|| RuntimeError::IndexOutOfBounds {
                        array_len: values.len(),
                        index,
                    })
            }
            _ => Err(RuntimeError::NotAnArray),
        }
    }

    /// Get a mutable element by index from an array value.
    pub fn get_element_mut(&mut self, index: usize) -> Result<&mut LpValue, RuntimeError> {
        // First check the length without holding mutable borrow
        let array_len = match self {
            LpValue::Array { values, .. } => values.len(),
            _ => return Err(RuntimeError::NotAnArray),
        };

        // Now get mutable access
        match self {
            LpValue::Array { values, .. } => values
                .get_mut(index)
                .ok_or_else(|| RuntimeError::IndexOutOfBounds { array_len, index }),
            _ => Err(RuntimeError::NotAnArray),
        }
    }

    /// Check if an Option value is Some.
    pub fn is_some(&self) -> bool {
        match self {
            LpValue::Option { value, .. } => value.is_some(),
            _ => false,
        }
    }

    /// Check if an Option value is None.
    pub fn is_none(&self) -> bool {
        match self {
            LpValue::Option { value, .. } => value.is_none(),
            _ => false,
        }
    }

    /// Unwrap an Option value, returning the inner value.
    pub fn try_unwrap(&self) -> Result<&LpValue, RuntimeError> {
        match self {
            LpValue::Option { value, .. } => value
                .as_ref()
                .map(|v| v.as_ref())
                .ok_or(RuntimeError::OptionIsNone),
            _ => Err(RuntimeError::NotAnOption),
        }
    }

    /// Unwrap an Option value mutably, returning the inner value.
    pub fn try_unwrap_mut(&mut self) -> Result<&mut LpValue, RuntimeError> {
        match self {
            LpValue::Option { value, .. } => value
                .as_mut()
                .map(|v| v.as_mut())
                .ok_or(RuntimeError::OptionIsNone),
            _ => Err(RuntimeError::NotAnOption),
        }
    }

    /// Get value as Fixed, if it is a Fixed scalar.
    pub fn as_fixed(&self) -> Result<Fixed, RuntimeError> {
        match self {
            LpValue::Fixed(f) => Ok(*f),
            _ => Err(RuntimeError::NotAScalar),
        }
    }

    /// Get value as Int32, if it is an Int32 scalar.
    pub fn as_int32(&self) -> Result<i32, RuntimeError> {
        match self {
            LpValue::Int32(i) => Ok(*i),
            _ => Err(RuntimeError::NotAScalar),
        }
    }

    /// Get value as Bool, if it is a Bool scalar.
    pub fn as_bool(&self) -> Result<bool, RuntimeError> {
        match self {
            LpValue::Bool(b) => Ok(*b),
            _ => Err(RuntimeError::NotAScalar),
        }
    }

    /// Get value as string slice, if it is a String scalar.
    pub fn as_string(&self) -> Result<&str, RuntimeError> {
        match self {
            LpValue::String(s) => Ok(s.as_str()),
            _ => Err(RuntimeError::NotAScalar),
        }
    }

    /// Get value as Vec2, if it is a Vec2.
    pub fn as_vec2(&self) -> Result<(Fixed, Fixed), RuntimeError> {
        match self {
            LpValue::Vec2(x, y) => Ok((*x, *y)),
            _ => Err(RuntimeError::NotAVector),
        }
    }

    /// Get value as Vec3, if it is a Vec3.
    pub fn as_vec3(&self) -> Result<(Fixed, Fixed, Fixed), RuntimeError> {
        match self {
            LpValue::Vec3(x, y, z) => Ok((*x, *y, *z)),
            _ => Err(RuntimeError::NotAVector),
        }
    }

    /// Get value as Vec4, if it is a Vec4.
    pub fn as_vec4(&self) -> Result<(Fixed, Fixed, Fixed, Fixed), RuntimeError> {
        match self {
            LpValue::Vec4(x, y, z, w) => Ok((*x, *y, *z, *w)),
            _ => Err(RuntimeError::NotAVector),
        }
    }

    /// Set a field value in a record.
    pub fn try_set_field(&mut self, name: &str, value: LpValue) -> Result<(), RuntimeError> {
        let field = self.get_field_mut(name)?;
        *field = value;
        Ok(())
    }

    /// Set an element value in an array.
    pub fn try_set_element(&mut self, index: usize, value: LpValue) -> Result<(), RuntimeError> {
        let element = self.get_element_mut(index)?;
        *element = value;
        Ok(())
    }

    /// Push a value to an array.
    pub fn try_push_element(&mut self, value: LpValue) -> Result<(), RuntimeError> {
        match self {
            LpValue::Array { values, .. } => {
                values.try_push(value).map_err(RuntimeError::AllocError)?;
                Ok(())
            }
            _ => Err(RuntimeError::NotAnArray),
        }
    }

    /// Get a value by path (e.g., "nodes.lfo.output").
    ///
    /// Supports dot-separated paths for nested access.
    pub fn get_path(&self, path: &str) -> Result<&LpValue, RuntimeError> {
        let parts: Vec<&str> = path.split('.').collect();
        if parts.is_empty() {
            return Err(RuntimeError::InvalidPath {
                path: String::from(path),
                reason: String::from("empty path"),
            });
        }

        let mut current = self;
        for part in parts {
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
                path: String::from(path),
                reason: format!("could not resolve '{}'", part),
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
        let parts: Vec<&str> = path.split('.').collect();
        if parts.is_empty() {
            return Err(RuntimeError::InvalidPath {
                path: String::from(path),
                reason: String::from("empty path"),
            });
        }

        // For single-level paths, only support field access
        // For array access, use get_element_mut directly
        if parts.len() == 1 {
            return self
                .get_field_mut(parts[0])
                .map_err(|_| RuntimeError::InvalidPath {
                    path: String::from(path),
                    reason: format!("could not resolve '{}'", parts[0]),
                });
        }

        // For nested paths, we need to use unsafe or a different approach
        // For now, return an error suggesting to use get_field_mut repeatedly
        Err(RuntimeError::InvalidPath {
            path: String::from(path),
            reason: String::from(
                "nested mutable path access requires multiple calls to get_field_mut",
            ),
        })
    }
}
