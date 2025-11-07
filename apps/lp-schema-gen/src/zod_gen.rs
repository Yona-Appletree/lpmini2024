use crate::registry::SchemaRegistry;
use serde_json::Value as JsonValue;

/// Generate Zod schema TypeScript code from a schema registry
pub fn generate_zod_schemas(registry: &SchemaRegistry) -> String {
    let mut output = String::new();
    
    // Add header
    output.push_str("import { z } from 'zod';\n\n");
    
    // Convert schemas to JSON for easier processing
    let mut schemas_json: std::collections::BTreeMap<String, JsonValue> = std::collections::BTreeMap::new();
    for (name, schema) in registry.all_schemas() {
        let json = serde_json::to_value(schema).unwrap_or(JsonValue::Null);
        schemas_json.insert(name.clone(), json);
    }
    
    // Generate schemas for all types
    // First, collect and sort types, separating enums from structs
    let mut enum_names = Vec::new();
    let mut struct_names = Vec::new();
    let mut other_names = Vec::new();
    
    for (type_name, schema_json) in &schemas_json {
        match classify_schema_json(schema_json) {
            SchemaKind::Enum => enum_names.push(type_name.clone()),
            SchemaKind::Struct => struct_names.push(type_name.clone()),
            SchemaKind::Other => other_names.push(type_name.clone()),
        }
    }
    
    enum_names.sort();
    struct_names.sort();
    other_names.sort();
    
    // Generate enums first (they may be referenced by structs)
    for type_name in enum_names {
        if let Some(schema_json) = schemas_json.get(&type_name) {
            output.push_str(&generate_type_schema_from_json(&type_name, schema_json, &schemas_json));
            output.push_str("\n\n");
        }
    }
    
    // Then generate structs
    for type_name in struct_names {
        if let Some(schema_json) = schemas_json.get(&type_name) {
            output.push_str(&generate_type_schema_from_json(&type_name, schema_json, &schemas_json));
            output.push_str("\n\n");
        }
    }
    
    // Finally, generate other types
    for type_name in other_names {
        if let Some(schema_json) = schemas_json.get(&type_name) {
            output.push_str(&generate_type_schema_from_json(&type_name, schema_json, &schemas_json));
            output.push_str("\n\n");
        }
    }
    
    output
}

enum SchemaKind {
    Enum,
    Struct,
    Other,
}

fn classify_schema_json(schema: &JsonValue) -> SchemaKind {
    if let Some(obj) = schema.as_object() {
        if obj.get("type") == Some(&JsonValue::String("string".to_string()))
            && obj.get("enum").is_some() {
            return SchemaKind::Enum;
        }
        if obj.get("type") == Some(&JsonValue::String("object".to_string()))
            && obj.get("properties").is_some() {
            return SchemaKind::Struct;
        }
    }
    SchemaKind::Other
}

fn generate_type_schema_from_json(name: &str, schema: &JsonValue, all_schemas: &std::collections::BTreeMap<String, JsonValue>) -> String {
    if let Some(obj) = schema.as_object() {
        // Check if it's an enum
        if obj.get("type") == Some(&JsonValue::String("string".to_string()))
            && obj.get("enum").is_some() {
            return generate_enum_schema_from_json(name, obj);
        }
        
        // Check if it's a struct
        if obj.get("type") == Some(&JsonValue::String("object".to_string()))
            && obj.get("properties").is_some() {
            return generate_struct_schema_from_json(name, obj, all_schemas);
        }
    }
    
    // Primitive or other
    format!("export const {}Schema = {};", name, schema_json_to_zod(schema, all_schemas))
}

fn generate_enum_schema_from_json(name: &str, obj: &serde_json::Map<String, JsonValue>) -> String {
    if let Some(enum_values) = obj.get("enum").and_then(|v| v.as_array()) {
        let variants: Vec<String> = enum_values
            .iter()
            .filter_map(|v| {
                if let Some(s) = v.as_str() {
                    Some(format!("'{}'", s))
                } else {
                    None
                }
            })
            .collect();
        
        format!(
            "export const {}Schema = z.enum([{}]);",
            name,
            variants.join(", ")
        )
    } else {
        format!("export const {}Schema = z.string();", name)
    }
}

fn generate_struct_schema_from_json(name: &str, obj: &serde_json::Map<String, JsonValue>, all_schemas: &std::collections::BTreeMap<String, JsonValue>) -> String {
    let mut fields = Vec::new();
    
    if let Some(properties) = obj.get("properties").and_then(|v| v.as_object()) {
        for (field_name, field_schema) in properties {
            let zod_type = schema_json_to_zod(field_schema, all_schemas);
            fields.push(format!("  {}: {}", field_name, zod_type));
        }
    }
    
    format!(
        "export const {}Schema = z.object({{\n{}\n}});",
        name,
        fields.join(",\n")
    )
}

fn schema_json_to_zod(schema: &JsonValue, all_schemas: &std::collections::BTreeMap<String, JsonValue>) -> String {
    // Check for $ref
    if let Some(obj) = schema.as_object() {
        if let Some(ref_path) = obj.get("$ref").and_then(|v| v.as_str()) {
            // Extract type name from reference path (e.g., "#/definitions/MyType" -> "MyType")
            if let Some(type_name) = ref_path.strip_prefix("#/definitions/") {
                // Check if this type is registered
                if all_schemas.contains_key(type_name) {
                    return format!("{}Schema", type_name);
                }
            }
        }
        
        // Check type
        if let Some(type_str) = obj.get("type").and_then(|v| v.as_str()) {
            match type_str {
                "boolean" => return "z.boolean()".to_string(),
                "integer" => return "z.number().int()".to_string(),
                "number" => return "z.number()".to_string(),
                "string" => {
                    // Check if it's an enum
                    if let Some(enum_values) = obj.get("enum").and_then(|v| v.as_array()) {
                        let variants: Vec<String> = enum_values
                            .iter()
                            .filter_map(|v| {
                                if let Some(s) = v.as_str() {
                                    Some(format!("'{}'", s))
                                } else {
                                    None
                                }
                            })
                            .collect();
                        return format!("z.enum([{}])", variants.join(", "));
                    }
                    return "z.string()".to_string();
                }
                "array" => {
                    if let Some(items) = obj.get("items") {
                        let item_zod = schema_json_to_zod(items, all_schemas);
                        return format!("z.array({})", item_zod);
                    }
                    return "z.array(z.unknown())".to_string();
                }
                "object" => {
                    // Object type - generate inline
                    let mut fields = Vec::new();
                    if let Some(properties) = obj.get("properties").and_then(|v| v.as_object()) {
                        for (field_name, field_schema) in properties {
                            let zod_type = schema_json_to_zod(field_schema, all_schemas);
                            fields.push(format!("{}: {}", field_name, zod_type));
                        }
                    }
                    return format!("z.object({{\n    {}\n  }})", fields.join(",\n    "));
                }
                _ => {}
            }
        }
    }
    
    // Fallback for unknown types
    "z.unknown()".to_string()
}
