//! Macros for defining primitive types in lp-data.
//!
//! These macros reduce boilerplate when adding new primitive types like Fixed, Int32, Bool, etc.

/// Defines the shape, meta, static, and dyn modules for a primitive type.
///
/// This macro generates:
/// - Shape trait (e.g., `FixedShape`)
/// - Meta trait and Static/Dyn implementations
/// - Static shape struct and const instance
/// - Dynamic shape struct
///
/// Usage:
/// ```ignore
/// define_primitive_shape! {
///     name: Fixed,
///     kind: Fixed,
///     shape_trait: FixedShape,
///     meta_trait: FixedMeta,
///     meta_static: FixedMetaStatic,
///     meta_dyn: FixedMetaDyn,
///     shape_static: FixedShapeStatic,
///     shape_dyn: FixedShapeDyn,
///     shape_const: FIXED_SHAPE,
/// }
/// ```
#[macro_export]
macro_rules! define_primitive_shape {
    (
        name: $name:ident,
        kind: $kind:ident,
        shape_trait: $shape_trait:ident,
        meta_trait: $meta_trait:ident,
        meta_static: $meta_static:ident,
        meta_dyn: $meta_dyn:ident,
        shape_static: $shape_static:ident,
        shape_dyn: $shape_dyn:ident,
        shape_const: $shape_const:ident,
    ) => {
        // Shape trait
        pub trait $shape_trait: $crate::kind::shape::LpShape {
            fn meta(&self) -> Option<&dyn $meta_trait>;
        }

        // Meta trait
        pub trait $meta_trait {
            fn label(&self) -> &str;
            fn desc_md(&self) -> Option<&str>;
            fn unit(&self) -> Option<&str>;
        }

        // Static metadata
        #[derive(Debug, Clone, Copy)]
        pub struct $meta_static {
            pub label: &'static str,
            pub desc_md: Option<&'static str>,
            pub unit: Option<&'static str>,
        }

        impl $meta_trait for $meta_static {
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

        // Dynamic metadata
        #[derive(Debug)]
        pub struct $meta_dyn {
            pub label: lp_pool::LpString,
            pub desc_md: Option<lp_pool::LpString>,
            pub unit: Option<lp_pool::LpString>,
        }

        impl $meta_trait for $meta_dyn {
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

        // Static shape
        pub struct $shape_static {
            pub meta: Option<$meta_static>,
        }

        impl $shape_static {
            pub const fn new() -> Self {
                Self { meta: None }
            }

            pub const fn with_meta(meta: $meta_static) -> Self {
                Self { meta: Some(meta) }
            }
        }

        impl $crate::kind::shape::LpShape for $shape_static {
            fn kind(&self) -> $crate::kind::kind::LpKind {
                $crate::kind::kind::LpKind::$kind
            }
        }

        impl $shape_trait for $shape_static {
            fn meta(&self) -> Option<&dyn $meta_trait> {
                self.meta.as_ref().map(|m| m as &dyn $meta_trait)
            }
        }

        pub const $shape_const: $shape_static = $shape_static::new();

        // Dynamic shape
        pub struct $shape_dyn {
            pub meta: Option<$meta_dyn>,
        }

        impl $shape_dyn {
            pub fn new() -> Self {
                Self { meta: None }
            }

            pub fn with_meta(meta: $meta_dyn) -> Self {
                Self { meta: Some(meta) }
            }
        }

        impl $crate::kind::shape::LpShape for $shape_dyn {
            fn kind(&self) -> $crate::kind::kind::LpKind {
                $crate::kind::kind::LpKind::$kind
            }
        }

        impl $shape_trait for $shape_dyn {
            fn meta(&self) -> Option<&dyn $meta_trait> {
                self.meta.as_ref().map(|m| m as &dyn $meta_trait)
            }
        }
    };
}

/// Defines the value implementation for a primitive type.
///
/// This macro generates:
/// - `LpValue` impl for the Rust type
/// - `From<T> for LpValueBox` impl
///
/// Usage:
/// ```ignore
/// define_primitive_value! {
///     rust_type: Fixed,
///     kind: Fixed,
///     shape_const: FIXED_SHAPE,
///     value_box_variant: Fixed,
/// }
/// ```
#[macro_export]
macro_rules! define_primitive_value {
    (
        rust_type: $rust_type:ty,
        kind: $kind:ident,
        shape_const: $shape_const:ident,
        value_box_variant: $value_box_variant:ident,
    ) => {
        impl $crate::kind::value::LpValue for $rust_type {
            fn shape(&self) -> &dyn $crate::kind::shape::LpShape {
                &$shape_const
            }
        }

        impl From<$rust_type> for $crate::kind::value::LpValueBox {
            fn from(value: $rust_type) -> Self {
                let boxed =
                    lp_pool::lp_box_dyn!(value, dyn $crate::kind::value::LpValue).expect(concat!(
                        "Failed to allocate ",
                        stringify!($rust_type),
                        " value in pool"
                    ));
                $crate::kind::value::LpValueBox::$value_box_variant(boxed)
            }
        }
    };
}
