use lp_data::{
    ArrayType, EnumType, EnumVariant, LpScalarType, LpType, LpTypeMeta, NumberUi, RecordType,
    TypeRef, TypeRegistry, Vec2Type, Vec3Type, Vec4Type,
};

/// Generate Zod schema TypeScript code from a TypeRegistry
pub fn generate_zod_schemas(registry: &TypeRegistry) -> String {
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

    let all_types = registry.all_types();

    // Collect type names and sort them
    let mut type_names: Vec<&str> = all_types.keys().copied().collect();
    type_names.sort();

    // Generate schemas for all types
    for type_name in &type_names {
        if let Some(meta) = all_types.get(type_name) {
            output.push_str(&generate_type_schema(type_name, meta, all_types));
            output.push_str("\n\n");
        }
    }

    output
}

fn generate_type_schema(
    name: &str,
    meta: &LpTypeMeta,
    all_types: &std::collections::BTreeMap<&'static str, &'static LpTypeMeta>,
) -> String {
    let schema_expr = lp_type_to_zod(&meta.ty, all_types);
    let schema_name = format!("{}", name);

    format!("export const {schema_name} = ZodFactory('{schema_name}', {schema_expr});")
}

fn lp_type_to_zod(
    ty: &LpType,
    all_types: &std::collections::BTreeMap<&'static str, &'static LpTypeMeta>,
) -> String {
    match ty {
        LpType::Scalar(scalar) => scalar_to_zod(scalar),
        LpType::Vec2(vec2) => vec2_to_zod(vec2),
        LpType::Vec3(vec3) => vec3_to_zod(vec3),
        LpType::Vec4(vec4) => vec4_to_zod(vec4),
        LpType::Array(array) => array_to_zod(array, all_types),
        LpType::Record(record) => record_to_zod(record, all_types),
        LpType::Enum(enum_ty) => enum_to_zod(enum_ty, all_types),
    }
}

fn scalar_to_zod(scalar: &LpScalarType) -> String {
    match scalar {
        LpScalarType::String(_) => "z.string()".to_string(),
        LpScalarType::Fixed(fixed) => {
            if let NumberUi::Slider(slider) = &fixed.ui {
                let mut zod = "z.number()".to_string();
                zod.push_str(&format!(".min({})", slider.min));
                zod.push_str(&format!(".max({})", slider.max));
                if let Some(step) = slider.step {
                    zod.push_str(&format!(".step({})", step));
                }
                zod
            } else {
                "z.number()".to_string()
            }
        }
        LpScalarType::Int32(int32) => {
            if let NumberUi::Slider(slider) = &int32.ui {
                let mut zod = "z.number().int()".to_string();
                zod.push_str(&format!(".min({})", slider.min as i32));
                zod.push_str(&format!(".max({})", slider.max as i32));
                if let Some(step) = slider.step {
                    zod.push_str(&format!(".step({})", step as i32));
                }
                zod
            } else {
                "z.number().int()".to_string()
            }
        }
        LpScalarType::Bool(_) => "z.boolean()".to_string(),
    }
}

fn vec2_to_zod(_vec2: &Vec2Type) -> String {
    "z.tuple([z.number(), z.number()])".to_string()
}

fn vec3_to_zod(_vec3: &Vec3Type) -> String {
    "z.tuple([z.number(), z.number(), z.number()])".to_string()
}

fn vec4_to_zod(_vec4: &Vec4Type) -> String {
    "z.tuple([z.number(), z.number(), z.number(), z.number()])".to_string()
}

fn array_to_zod(
    array: &ArrayType<TypeRef>,
    all_types: &std::collections::BTreeMap<&'static str, &'static LpTypeMeta>,
) -> String {
    let element_zod = type_ref_to_zod(array.element, all_types);
    format!("z.array({})", element_zod)
}

fn record_to_zod(
    record: &RecordType<TypeRef>,
    all_types: &std::collections::BTreeMap<&'static str, &'static LpTypeMeta>,
) -> String {
    let mut fields = Vec::new();

    for field in record.fields {
        let field_zod = type_ref_to_zod(field.ty, all_types);
        fields.push(format!("  {}: {}", field.name, field_zod));
    }

    format!("z.object({{\n{}\n}})", fields.join(",\n"))
}

fn enum_to_zod(
    enum_ty: &EnumType<TypeRef>,
    all_types: &std::collections::BTreeMap<&'static str, &'static LpTypeMeta>,
) -> String {
    match &enum_ty.variants[..] {
        [] => "z.never()".to_string(),
        variants => {
            let variant_strings: Vec<String> = variants
                .iter()
                .map(|v| match v {
                    EnumVariant::Unit { name } => format!("'{}'", name),
                    EnumVariant::Tuple { name, .. } => {
                        // For tuple variants, we'd need to generate a discriminated union
                        // For now, just use the name as a string literal
                        format!("'{}'", name)
                    }
                    EnumVariant::Struct { name, .. } => {
                        // For struct variants, we'd need to generate a discriminated union
                        // For now, just use the name as a string literal
                        format!("'{}'", name)
                    }
                })
                .collect();
            format!("z.enum([{}])", variant_strings.join(", "))
        }
    }
}

fn type_ref_to_zod(
    type_ref: TypeRef,
    all_types: &std::collections::BTreeMap<&'static str, &'static LpTypeMeta>,
) -> String {
    // Check if this is a reference to a registered type
    // We need to find the type name by searching through all_types
    // Compare by pointer address since these are static references
    let type_ref_ptr = type_ref as *const _;
    for (name, meta) in all_types {
        let meta_ptr = *meta as *const _;
        if std::ptr::eq(meta_ptr, type_ref_ptr) {
            return format!("{}Schema", name);
        }
    }

    // Not a registered type, generate inline
    lp_type_to_zod(&type_ref.ty, all_types)
}
