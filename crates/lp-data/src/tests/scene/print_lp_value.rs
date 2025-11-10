use crate::kind::value::{LpValue, LpValueBox, LpValueRef};
use lp_math::fixed::Fixed;

/// Traverse the scene graph and print all data generically.
pub fn print_lp_value(value_box: LpValueBox, indent: usize) {
    match value_box {
        LpValueBox::Fixed(boxed) => {
            print_lp_value_ref(LpValueRef::Fixed(boxed.as_ref()), indent);
        }
        LpValueBox::Record(boxed) => {
            print_lp_value_ref(LpValueRef::Record(boxed.as_ref()), indent);
        }
        LpValueBox::Enum(boxed) => {
            print_lp_value_ref(LpValueRef::Enum(boxed.as_ref()), indent);
        }
    }
}

/// Traverse the scene graph and return a string representation.
pub fn print_lp_value_to_string(value_box: LpValueBox, indent: usize) -> String {
    match value_box {
        LpValueBox::Fixed(boxed) => {
            print_lp_value_ref_to_string(LpValueRef::Fixed(boxed.as_ref()), indent)
        }
        LpValueBox::Record(boxed) => {
            print_lp_value_ref_to_string(LpValueRef::Record(boxed.as_ref()), indent)
        }
        LpValueBox::Enum(boxed) => {
            print_lp_value_ref_to_string(LpValueRef::Enum(boxed.as_ref()), indent)
        }
    }
}

/// Print a value reference recursively.
fn print_lp_value_ref(value_ref: LpValueRef, indent: usize) {
    match value_ref {
        LpValueRef::Fixed(fixed_ref) => {
            let fixed_value = unsafe {
                // SAFETY: We know this is a Fixed because it's in the Fixed variant
                // The vtable pointer points to Fixed's implementation
                &*(fixed_ref as *const dyn LpValue as *const Fixed)
            };
            println!("Fixed({})", fixed_value.to_f32());
        }
        LpValueRef::Record(record_ref) => {
            use crate::kind::record::record_value::RecordValue;
            let record_name = RecordValue::shape(record_ref).meta().name();
            if record_name.is_empty() {
                println!("Record (anonymous)");
            } else {
                println!("Record({})", record_name);
            }
            // Iterate over fields using get_field_by_index and recursively print them
            let shape = RecordValue::shape(record_ref);
            for i in 0..shape.field_count() {
                if let Some(field_shape) = shape.get_field(i) {
                    if let Ok(field_value_ref) = record_ref.get_field_by_index(i) {
                        print!("{:>indent$}  {}: ", "", field_shape.name());
                        print_lp_value_ref(field_value_ref, indent + 2);
                    }
                }
            }
        }
        LpValueRef::Enum(enum_ref) => {
            use crate::kind::enum_::enum_value::EnumValue;
            let enum_name = EnumValue::shape(enum_ref).meta().name();
            if let Ok(variant_name) = enum_ref.variant_name() {
                if enum_name.is_empty() {
                    println!("Enum::{}", variant_name);
                } else {
                    println!("Enum({})::{}", enum_name, variant_name);
                }
            } else {
                println!("Enum({})", enum_name);
            }
        }
    }
}

/// Print a value reference recursively to a string.
fn print_lp_value_ref_to_string(value_ref: LpValueRef, indent: usize) -> String {
    match value_ref {
        LpValueRef::Fixed(fixed_ref) => {
            let fixed_value = unsafe {
                // SAFETY: We know this is a Fixed because it's in the Fixed variant
                // The vtable pointer points to Fixed's implementation
                &*(fixed_ref as *const dyn LpValue as *const Fixed)
            };
            format!("Fixed({})\n", fixed_value.to_f32())
        }
        LpValueRef::Record(record_ref) => {
            use crate::kind::record::record_value::RecordValue;
            let record_name = RecordValue::shape(record_ref).meta().name();
            let mut output = if record_name.is_empty() {
                "Record (anonymous)\n".to_string()
            } else {
                format!("Record({})\n", record_name)
            };
            // Iterate over fields using get_field_by_index and recursively print them
            let shape = RecordValue::shape(record_ref);
            let field_count = shape.field_count();

            // Shape should always match the actual fields now
            for i in 0..field_count {
                if let Some(field_shape) = shape.get_field(i) {
                    if let Ok(field_value_ref) = record_ref.get_field_by_index(i) {
                        output.push_str(&format!("{:>indent$}  {}: ", "", field_shape.name()));
                        output.push_str(&print_lp_value_ref_to_string(field_value_ref, indent + 2));
                    }
                }
            }
            output
        }
        LpValueRef::Enum(enum_ref) => {
            use crate::kind::enum_::enum_value::EnumValue;
            let enum_name = EnumValue::shape(enum_ref).meta().name();
            if let Ok(variant_name) = enum_ref.variant_name() {
                if enum_name.is_empty() {
                    format!("Enum::{}\n", variant_name)
                } else {
                    format!("Enum({})::{}\n", enum_name, variant_name)
                }
            } else {
                format!("Enum({})\n", enum_name)
            }
        }
    }
}
