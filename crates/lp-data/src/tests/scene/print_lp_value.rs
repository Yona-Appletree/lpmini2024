use crate::kind::value::{LpValue, LpValueBox, RecordValue};
use lp_math::fixed::Fixed;

/// Traverse the scene graph and print all data generically.
pub fn print_lp_value(valueBox: LpValueBox, indent: usize) {
    match valueBox {
        LpValueBox::Fixed(value) => {
            println!("{:>indent$}Fixed: {}", "", value.to_f32());
        }
        LpValueBox::Record(value) => {
            value.iter_fields().for_each(|(name, value)| {
                println!("{:>indent$}  {}:", "", name);
                print_lp_value(value.clone(), indent + 2);
            });
        }
    }
}
