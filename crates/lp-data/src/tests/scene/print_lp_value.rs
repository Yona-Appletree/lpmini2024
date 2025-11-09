use crate::kind::value::{LpValue, LpValueBox, RecordValue};
use lp_math::fixed::Fixed;

/// Traverse the scene graph and print all data generically.
pub fn print_lp_value(value_box: LpValueBox, indent: usize) {
    match value_box {
        LpValueBox::Fixed(boxed) => {
            // Extract the Fixed value from the box
            // We need to downcast to get the actual Fixed value
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
            // Note: iter_fields was removed to avoid cloning
            // For now, we can't iterate over fields without knowing field names
            // This is a limitation - field iteration requires knowing field names
            println!(
                "{:>indent$}Record ({} fields)",
                "",
                record_ref.field_count()
            );
        }
    }
}
