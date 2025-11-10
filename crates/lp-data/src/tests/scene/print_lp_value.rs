use crate::kind::value::{LpValue, LpValueBox, LpValueRef, RecordValue};
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
            println!("Record");
            // Iterate over fields using get_field_by_index and recursively print them
            for i in 0..record_ref.field_count() {
                if let Ok((field_name, field_value_ref)) = record_ref.get_field_by_index(i) {
                    print!("{:>indent$}  {}: ", "", field_name);
                    print_lp_value_ref(field_value_ref, indent + 2);
                }
            }
        }
    }
}
