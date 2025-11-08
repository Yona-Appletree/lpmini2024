//! Enum metadata and UI hints.

use crate::metadata::record::RecordField;

/// Metadata for an enum type.
#[derive(Debug, Clone, PartialEq)]
pub struct EnumType<T: 'static> {
    pub name: &'static str,
    pub variants: &'static [EnumVariant<T>],
    pub ui: EnumUi,
}

impl<T: 'static> EnumType<T> {
    pub const fn new(name: &'static str, variants: &'static [EnumVariant<T>]) -> Self {
        Self {
            name,
            variants,
            ui: EnumUi::Dropdown,
        }
    }

    pub const fn with_ui(mut self, ui: EnumUi) -> Self {
        self.ui = ui;
        self
    }
}

/// Description of an enum variant.
#[derive(Debug, Clone, PartialEq)]
pub enum EnumVariant<T: 'static> {
    Unit {
        name: &'static str,
    },
    Tuple {
        name: &'static str,
        fields: &'static [T],
    },
    Struct {
        name: &'static str,
        fields: &'static [RecordField<T>],
    },
}

impl<T: 'static> EnumVariant<T> {
    pub const fn unit(name: &'static str) -> Self {
        Self::Unit { name }
    }

    pub const fn tuple(name: &'static str, fields: &'static [T]) -> Self {
        Self::Tuple { name, fields }
    }

    pub const fn struct_variant(name: &'static str, fields: &'static [RecordField<T>]) -> Self {
        Self::Struct { name, fields }
    }
}

/// UI hints for enums.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnumUi {
    Dropdown,
    SegmentedControl,
}
