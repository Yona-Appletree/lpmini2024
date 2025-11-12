use std::collections::BTreeMap;

use lp_data::kind::enum_unit::EnumUnitShape;
use lp_data::kind::fixed::FixedShape;
use lp_data::kind::kind::LpKind;
use lp_data::kind::record::{RecordFieldShape, RecordShape};
use lp_data::kind::shape::LpShape;

/// Generate Zod schema TypeScript code from a registry of shapes
pub fn generate_zod_schemas(registry: &BTreeMap<&'static str, &dyn LpShape>) -> String {
    let mut output = String::new();

    // Add header
    output.push_str("import { z } from 'zod';\n\n");
    output.push_str("export function ZodFactory<TSchema extends z.ZodTypeAny, TExtra>(schema: TSchema, extra?: TExtra) {
  return Object.assign((params: z.input<TSchema>) => schema.parse(params), {
    schema,
    ...extra,
  }) as {
    (params: z.input<TSchema>): z.output<TSchema>;
    schema: TSchema;
  } & (void extends TExtra ? Record<never, never> : TExtra);
}
\n\n");

    // Collect type names and sort them
    let mut type_names: Vec<&str> = registry.keys().copied().collect();
    type_names.sort();

    // Generate schemas for all types
    for type_name in &type_names {
        if let Some(&shape) = registry.get(type_name) {
            output.push_str(&generate_type_schema(type_name, shape, registry));
            output.push_str("\n\n");
        }
    }

    output
}

fn generate_type_schema(
    name: &str,
    shape: &dyn LpShape,
    all_types: &BTreeMap<&'static str, &dyn LpShape>,
) -> String {
    let schema_expr = lp_shape_to_zod(shape, all_types);
    let schema_name = name.to_string();

    format!("export const {schema_name} = ZodFactory('{schema_name}', {schema_expr});")
}

fn lp_shape_to_zod(
    shape: &dyn LpShape,
    all_types: &BTreeMap<&'static str, &dyn LpShape>,
) -> String {
    match shape.kind() {
        LpKind::Fixed => {
            // SAFETY: We know this is a Fixed because kind() returned Fixed
            // Shapes are 'static, so transmuting the reference is safe
            let fixed_shape: &dyn FixedShape = unsafe { core::mem::transmute(shape) };
            fixed_to_zod(fixed_shape)
        }
        LpKind::Int32 => "z.number().int()".to_string(),
        LpKind::Bool => "z.boolean()".to_string(),
        LpKind::Vec2 => "z.tuple([z.number(), z.number()])".to_string(),
        LpKind::Vec3 => "z.tuple([z.number(), z.number(), z.number()])".to_string(),
        LpKind::Vec4 => "z.tuple([z.number(), z.number(), z.number(), z.number()])".to_string(),
        LpKind::Record => {
            // SAFETY: We know this is a Record because kind() returned Record
            // Shapes are 'static, so transmuting the reference is safe
            let record_shape: &dyn RecordShape = unsafe { core::mem::transmute(shape) };
            record_to_zod(record_shape, all_types)
        }
        LpKind::EnumUnit => {
            // SAFETY: We know this is an Enum because kind() returned Enum
            // Shapes are 'static, so transmuting the reference is safe
            let enum_shape: &dyn EnumUnitShape = unsafe { core::mem::transmute(shape) };
            enum_to_zod(enum_shape, all_types)
        }
    }
}

fn fixed_to_zod(_fixed_shape: &dyn FixedShape) -> String {
    // For now, just return z.number()
    // TODO: Check metadata for slider min/max/step
    "z.number()".to_string()
}

fn record_to_zod(
    record_shape: &dyn RecordShape,
    all_types: &BTreeMap<&'static str, &dyn LpShape>,
) -> String {
    let mut fields = Vec::new();

    for i in 0..record_shape.field_count() {
        if let Some(field_shape) = record_shape.get_field(i) {
            let field_name = field_shape.name();
            let field_zod = field_shape_to_zod(field_shape, all_types);
            fields.push(format!("  {}: {}", field_name, field_zod));
        }
    }

    format!("z.object({{\n{}\n}})", fields.join(",\n"))
}

fn enum_to_zod(
    enum_shape: &dyn EnumUnitShape,
    _all_types: &BTreeMap<&'static str, &dyn LpShape>,
) -> String {
    let mut variants = Vec::new();

    for i in 0..enum_shape.variant_count() {
        if let Some(variant_shape) = enum_shape.get_variant(i) {
            variants.push(format!("'{}'", variant_shape.name()));
        }
    }

    if variants.is_empty() {
        "z.never()".to_string()
    } else {
        format!("z.enum([{}])", variants.join(", "))
    }
}

fn field_shape_to_zod(
    field_shape: &dyn RecordFieldShape,
    all_types: &BTreeMap<&'static str, &dyn LpShape>,
) -> String {
    let shape = field_shape.shape();

    // Check if this shape is registered as a named type
    // We need to compare shapes to see if they match a registered type
    // For now, we'll generate inline schemas
    lp_shape_to_zod(shape, all_types)
}
