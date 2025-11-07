use lp_data::registry::TypeRegistry;
use lp_data::ty::{LpEnumType, LpPrimitive, LpStructType, LpType};


fn generate_enum_schema(enum_ty: &LpEnumType) -> String {
    let variants: Vec<String> = enum_ty
        .variants
        .iter()
        .map(|v| format!("'{}'", v.name))
        .collect();
    
    format!(
        "export const {}Schema = z.enum([{}]);",
        enum_ty.name,
        variants.join(", ")
    )
}

fn type_to_zod_with_registry(ty: &LpType, registry: Option<&TypeRegistry>) -> String {
    match ty {
        LpType::Primitive(prim) => primitive_to_zod(prim),
        LpType::Array(array_ty) => {
            format!("z.array({})", type_to_zod_with_registry(array_ty.element.as_ref(), registry))
        }
        LpType::Struct(struct_ty) => {
            let mut fields = Vec::new();
            for field in &struct_ty.fields {
                let zod_type = type_to_zod_with_registry(field.ty.as_ref(), registry);
                fields.push(format!("{}: {}", field.name, zod_type));
            }
            format!("z.object({{\n    {}\n  }})", fields.join(",\n    "))
        }
        LpType::Enum(enum_ty) => {
            // Check if this enum is registered separately, if so reference it
            if let Some(reg) = registry {
                if reg.get(enum_ty.name).is_some() {
                    return format!("{}Schema", enum_ty.name);
                }
            }
            // Otherwise generate inline
            let variants: Vec<String> = enum_ty
                .variants
                .iter()
                .map(|v| format!("'{}'", v.name))
                .collect();
            format!("z.enum([{}])", variants.join(", "))
        }
    }
}

/// Generate Zod schema TypeScript code from a type registry
pub fn generate_zod_schemas(registry: &TypeRegistry) -> String {
    let mut output = String::new();
    
    // Add header
    output.push_str("import { z } from 'zod';\n\n");
    
    // Generate schemas for all types
    // First, collect and sort types, separating enums from structs
    let mut enum_names = Vec::new();
    let mut struct_names = Vec::new();
    let mut other_names = Vec::new();
    
    for type_name in registry.type_names() {
        if let Some(ty) = registry.get(type_name) {
            match ty {
                LpType::Enum(_) => enum_names.push(type_name),
                LpType::Struct(_) => struct_names.push(type_name),
                _ => other_names.push(type_name),
            }
        }
    }
    
    enum_names.sort();
    struct_names.sort();
    other_names.sort();
    
    // Generate enums first (they may be referenced by structs)
    for type_name in enum_names {
        if let Some(ty) = registry.get(type_name) {
            output.push_str(&generate_type_schema_with_registry(type_name, ty, registry));
            output.push_str("\n\n");
        }
    }
    
    // Then generate structs
    for type_name in struct_names {
        if let Some(ty) = registry.get(type_name) {
            output.push_str(&generate_type_schema_with_registry(type_name, ty, registry));
            output.push_str("\n\n");
        }
    }
    
    // Finally, generate other types
    for type_name in other_names {
        if let Some(ty) = registry.get(type_name) {
            output.push_str(&generate_type_schema_with_registry(type_name, ty, registry));
            output.push_str("\n\n");
        }
    }
    
    output
}

fn generate_type_schema_with_registry(name: &str, ty: &LpType, registry: &TypeRegistry) -> String {
    match ty {
        LpType::Primitive(prim) => {
            format!("export const {}Schema = {};", name, primitive_to_zod(prim))
        }
        LpType::Struct(struct_ty) => {
            generate_struct_schema_with_registry(struct_ty, registry)
        }
        LpType::Enum(enum_ty) => {
            generate_enum_schema(enum_ty)
        }
        LpType::Array(_) => {
            // Arrays are typically used as field types, not top-level types
            format!("// Array type: {}", name)
        }
    }
}

fn generate_struct_schema_with_registry(struct_ty: &LpStructType, registry: &TypeRegistry) -> String {
    let mut fields = Vec::new();
    
    for field in &struct_ty.fields {
        let zod_type = type_to_zod_with_registry(field.ty.as_ref(), Some(registry));
        fields.push(format!("  {}: {}", field.name, zod_type));
    }
    
    format!(
        "export const {}Schema = z.object({{\n{}\n}});",
        struct_ty.name,
        fields.join(",\n")
    )
}

fn primitive_to_zod(prim: &LpPrimitive) -> String {
    match prim {
        LpPrimitive::Int32 => "z.number().int()".to_string(),
        LpPrimitive::Fixed32 => "z.number()".to_string(),
        LpPrimitive::Bool => "z.boolean()".to_string(),
        LpPrimitive::Vec2 => "z.object({ x: z.number(), y: z.number() })".to_string(),
        LpPrimitive::Vec3 => "z.object({ x: z.number(), y: z.number(), z: z.number() })".to_string(),
        LpPrimitive::Vec4 => "z.object({ x: z.number(), y: z.number(), z: z.number(), w: z.number() })".to_string(),
    }
}

