use crate::kind::value::{LpValue, LpValueBox, RecordValue};
use lp_math::fixed::Fixed;

/// Traverse the scene graph and print all data generically.
pub fn print_lp_value(value_box: LpValueBox, indent: usize) {
    match value_box {
        LpValueBox::Fixed(boxed) => {
            // Extract the Fixed value from the box
            let fixed_ref: &dyn LpValue = boxed.as_ref();
            // We need to downcast to get the actual Fixed value
            // For now, we'll use the shape to determine it's Fixed and access it
            // This is a bit of a hack - ideally we'd have a way to downcast
            let fixed_value = unsafe {
                // SAFETY: We know this is a Fixed because it's in the Fixed variant
                // The vtable pointer points to Fixed's implementation
                &*(boxed.as_ref() as *const dyn LpValue as *const Fixed)
            };
            println!("{:>indent$}Fixed: {}", "", fixed_value.to_f32());
        }
        LpValueBox::Record(boxed) => {
            let record_ref: &dyn RecordValue = boxed.as_ref();
            for (name, value) in record_ref.iter_fields() {
                println!("{:>indent$}  {}:", "", name);
                print_lp_value(value, indent + 2);
            }
        }
    }
}
