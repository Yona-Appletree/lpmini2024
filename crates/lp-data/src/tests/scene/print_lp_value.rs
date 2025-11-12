use lp_math::fixed::{Fixed, Vec2, Vec3, Vec4};

use crate::kind::value::{LpValue, LpValueBox, LpValueRef};

#[allow(dead_code)]
/// Traverse the scene graph and print all data generically.
pub fn print_lp_value(value_box: LpValueBox, indent: usize) {
    match value_box {
        LpValueBox::Fixed(boxed) => {
            print_lp_value_ref(LpValueRef::Fixed(boxed.as_ref()), indent);
        }
        LpValueBox::Int32(boxed) => {
            print_lp_value_ref(LpValueRef::Int32(boxed.as_ref()), indent);
        }
        LpValueBox::Bool(boxed) => {
            print_lp_value_ref(LpValueRef::Bool(boxed.as_ref()), indent);
        }
        LpValueBox::Vec2(boxed) => {
            print_lp_value_ref(LpValueRef::Vec2(boxed.as_ref()), indent);
        }
        LpValueBox::Vec3(boxed) => {
            print_lp_value_ref(LpValueRef::Vec3(boxed.as_ref()), indent);
        }
        LpValueBox::Vec4(boxed) => {
            print_lp_value_ref(LpValueRef::Vec4(boxed.as_ref()), indent);
        }
        LpValueBox::Record(boxed) => {
            print_lp_value_ref(LpValueRef::Record(boxed.as_ref()), indent);
        }
        LpValueBox::EnumUnit(boxed) => {
            print_lp_value_ref(LpValueRef::EnumUnit(boxed.as_ref()), indent);
        }
        LpValueBox::EnumStruct(boxed) => {
            print_lp_value_ref(LpValueRef::EnumStruct(boxed.as_ref()), indent);
        }
    }
}

/// Traverse the scene graph and return a string representation.
pub fn print_lp_value_to_string(value_box: LpValueBox, indent: usize) -> String {
    match value_box {
        LpValueBox::Fixed(boxed) => {
            print_lp_value_ref_to_string(LpValueRef::Fixed(boxed.as_ref()), indent)
        }
        LpValueBox::Int32(boxed) => {
            print_lp_value_ref_to_string(LpValueRef::Int32(boxed.as_ref()), indent)
        }
        LpValueBox::Bool(boxed) => {
            print_lp_value_ref_to_string(LpValueRef::Bool(boxed.as_ref()), indent)
        }
        LpValueBox::Vec2(boxed) => {
            print_lp_value_ref_to_string(LpValueRef::Vec2(boxed.as_ref()), indent)
        }
        LpValueBox::Vec3(boxed) => {
            print_lp_value_ref_to_string(LpValueRef::Vec3(boxed.as_ref()), indent)
        }
        LpValueBox::Vec4(boxed) => {
            print_lp_value_ref_to_string(LpValueRef::Vec4(boxed.as_ref()), indent)
        }
        LpValueBox::Record(boxed) => {
            print_lp_value_ref_to_string(LpValueRef::Record(boxed.as_ref()), indent)
        }
        LpValueBox::EnumUnit(boxed) => {
            print_lp_value_ref_to_string(LpValueRef::EnumUnit(boxed.as_ref()), indent)
        }
        LpValueBox::EnumStruct(boxed) => {
            print_lp_value_ref_to_string(LpValueRef::EnumStruct(boxed.as_ref()), indent)
        }
    }
}

#[allow(dead_code)]
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
        LpValueRef::Int32(int32_ref) => {
            let int32_value = unsafe {
                // SAFETY: We know this is an i32 because it's in the Int32 variant
                &*(int32_ref as *const dyn LpValue as *const i32)
            };
            println!("Int32({})", int32_value);
        }
        LpValueRef::Bool(bool_ref) => {
            let bool_value = unsafe {
                // SAFETY: We know this is a bool because it's in the Bool variant
                &*(bool_ref as *const dyn LpValue as *const bool)
            };
            println!("Bool({})", bool_value);
        }
        LpValueRef::Vec2(vec2_ref) => {
            let vec2_value = unsafe {
                // SAFETY: We know this is a Vec2 because it's in the Vec2 variant
                &*(vec2_ref as *const dyn LpValue as *const Vec2)
            };
            println!("Vec2({}, {})", vec2_value.x.to_f32(), vec2_value.y.to_f32());
        }
        LpValueRef::Vec3(vec3_ref) => {
            let vec3_value = unsafe {
                // SAFETY: We know this is a Vec3 because it's in the Vec3 variant
                &*(vec3_ref as *const dyn LpValue as *const Vec3)
            };
            println!(
                "Vec3({}, {}, {})",
                vec3_value.x.to_f32(),
                vec3_value.y.to_f32(),
                vec3_value.z.to_f32()
            );
        }
        LpValueRef::Vec4(vec4_ref) => {
            let vec4_value = unsafe {
                // SAFETY: We know this is a Vec4 because it's in the Vec4 variant
                &*(vec4_ref as *const dyn LpValue as *const Vec4)
            };
            println!(
                "Vec4({}, {}, {}, {})",
                vec4_value.x.to_f32(),
                vec4_value.y.to_f32(),
                vec4_value.z.to_f32(),
                vec4_value.w.to_f32()
            );
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
        LpValueRef::EnumUnit(enum_ref) => {
            use crate::kind::enum_unit::enum_value::EnumUnitValue;
            let enum_name = EnumUnitValue::shape(enum_ref).meta().name();
            if let Ok(variant_name) = enum_ref.variant_name() {
                if enum_name.is_empty() {
                    println!("EnumUnit::{}", variant_name);
                } else {
                    println!("EnumUnit({})::{}", enum_name, variant_name);
                }
            } else {
                println!("EnumUnit({})", enum_name);
            }
        }
        LpValueRef::EnumStruct(union_ref) => {
            use crate::kind::enum_struct::enum_struct_value::EnumStructValue;
            let union_name = EnumStructValue::shape(union_ref).meta().name();
            if let Ok(variant_name) = union_ref.variant_name() {
                if union_name.is_empty() {
                    println!("Union::{}", variant_name);
                } else {
                    println!("Union({})::{}", union_name, variant_name);
                }
                // Print the variant's value
                if let Ok(variant_value) = union_ref.variant_value() {
                    print!("{:>indent$}  value: ", "");
                    print_lp_value_ref(variant_value, indent + 2);
                }
            } else {
                println!("Union({})", union_name);
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
        LpValueRef::Int32(int32_ref) => {
            let int32_value = unsafe {
                // SAFETY: We know this is an i32 because it's in the Int32 variant
                &*(int32_ref as *const dyn LpValue as *const i32)
            };
            format!("Int32({})\n", int32_value)
        }
        LpValueRef::Bool(bool_ref) => {
            let bool_value = unsafe {
                // SAFETY: We know this is a bool because it's in the Bool variant
                &*(bool_ref as *const dyn LpValue as *const bool)
            };
            format!("Bool({})\n", bool_value)
        }
        LpValueRef::Vec2(vec2_ref) => {
            let vec2_value = unsafe {
                // SAFETY: We know this is a Vec2 because it's in the Vec2 variant
                &*(vec2_ref as *const dyn LpValue as *const Vec2)
            };
            format!(
                "Vec2({}, {})\n",
                vec2_value.x.to_f32(),
                vec2_value.y.to_f32()
            )
        }
        LpValueRef::Vec3(vec3_ref) => {
            let vec3_value = unsafe {
                // SAFETY: We know this is a Vec3 because it's in the Vec3 variant
                &*(vec3_ref as *const dyn LpValue as *const Vec3)
            };
            format!(
                "Vec3({}, {}, {})\n",
                vec3_value.x.to_f32(),
                vec3_value.y.to_f32(),
                vec3_value.z.to_f32()
            )
        }
        LpValueRef::Vec4(vec4_ref) => {
            let vec4_value = unsafe {
                // SAFETY: We know this is a Vec4 because it's in the Vec4 variant
                &*(vec4_ref as *const dyn LpValue as *const Vec4)
            };
            format!(
                "Vec4({}, {}, {}, {})\n",
                vec4_value.x.to_f32(),
                vec4_value.y.to_f32(),
                vec4_value.z.to_f32(),
                vec4_value.w.to_f32()
            )
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
        LpValueRef::EnumUnit(enum_ref) => {
            use crate::kind::enum_unit::enum_value::EnumUnitValue;
            let enum_name = EnumUnitValue::shape(enum_ref).meta().name();
            if let Ok(variant_name) = enum_ref.variant_name() {
                if enum_name.is_empty() {
                    format!("EnumUnit::{}\n", variant_name)
                } else {
                    format!("EnumUnit({})::{}\n", enum_name, variant_name)
                }
            } else {
                format!("EnumUnit({})\n", enum_name)
            }
        }
        LpValueRef::EnumStruct(union_ref) => {
            use crate::kind::enum_struct::enum_struct_value::EnumStructValue;
            let union_name = EnumStructValue::shape(union_ref).meta().name();
            let mut output = if let Ok(variant_name) = union_ref.variant_name() {
                if union_name.is_empty() {
                    format!("Union::{}\n", variant_name)
                } else {
                    format!("Union({})::{}\n", union_name, variant_name)
                }
            } else {
                format!("Union({})\n", union_name)
            };
            // Print the variant's value
            if let Ok(variant_value) = union_ref.variant_value() {
                output.push_str(&format!("{:>indent$}  value: ", ""));
                output.push_str(&print_lp_value_ref_to_string(variant_value, indent + 2));
            }
            output
        }
    }
}
