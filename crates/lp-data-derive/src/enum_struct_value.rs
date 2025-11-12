use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, Attribute, Data, DataEnum, DeriveInput, Error, Fields, Ident, LitStr, Type,
    TypePath, Variant,
};

pub fn derive(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    match expand(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn expand(input: &DeriveInput) -> Result<TokenStream2, Error> {
    match &input.data {
        Data::Enum(data) => expand_enum_struct(input, data),
        Data::Struct(_) | Data::Union(_) => Err(Error::new(
            input.ident.span(),
            "EnumStructValue derive only supports enums",
        )),
    }
}

fn expand_enum_struct(input: &DeriveInput, data: &DataEnum) -> Result<TokenStream2, Error> {
    if !input.generics.params.is_empty() {
        return Err(Error::new(
            input.generics.span(),
            "EnumStructValue derive does not yet support generic parameters",
        ));
    }

    let enum_ident = &input.ident;
    let enum_name = enum_ident.to_string();

    let struct_attrs = StructAttrs::from_attrs(&input.attrs)?;
    let struct_docs = merge_docs(
        extract_doc_comments(&input.attrs),
        struct_attrs.docs.clone(),
    );
    let enum_struct_name_str = struct_attrs
        .name
        .clone()
        .unwrap_or_else(|| enum_name.clone());

    // Check that all variants have named fields (struct-like variants)
    for variant in &data.variants {
        match &variant.fields {
            Fields::Named(_) => {
                // Good - struct-like variant with named fields
            }
            Fields::Unnamed(_) => {
                return Err(Error::new(
    variant.span(),
    "EnumStructValue derive only supports struct-like variants with named fields (e.g., Variant { field: Type })",
));
            }
            Fields::Unit => {
                return Err(Error::new(
    variant.span(),
    "EnumStructValue derive requires variants with fields. Use EnumValue for unit variants.",
));
            }
        }
    }

    // Generate shape constant
    let variants: Vec<_> = data.variants.iter().collect();
    let shape_constant = generate_enum_struct_shape_static(
        enum_ident,
        &enum_struct_name_str,
        struct_docs,
        &variants,
    )?;

    let shape_const_ident =
        format_ident!("__LP_VALUE_{}_SHAPE", enum_ident.to_string().to_uppercase());

    // Generate variant_index and variant_value implementations
    let mut variant_index_matchers = Vec::new();
    let mut variant_value_matchers = Vec::new();
    let mut variant_value_mut_matchers = Vec::new();
    let mut where_bounds = Vec::new();

    for (index, variant) in data.variants.iter().enumerate() {
        let variant_ident = &variant.ident;
        variant_index_matchers.push(quote! {
            #enum_ident::#variant_ident { .. } => #index,
        });

        // Extract named fields from the variant
        let fields = match &variant.fields {
            Fields::Named(fields) => &fields.named,
            _ => unreachable!(),
        };

        // Generate code to create RecordValueDyn from variant fields
        let shape_const_ident = format_ident!(
            "__LP_VALUE_{}_{}_SHAPE",
            enum_ident.to_string().to_uppercase(),
            variant_ident.to_string().to_uppercase()
        );

        // Generate field accessors for value extraction
        let mut field_value_extractions = Vec::new();
        let mut field_value_extractions_mut = Vec::new();
        let mut field_names = Vec::new();
        let mut match_pattern_fields = Vec::new();

        for field in fields {
            let field_ident = field.ident.as_ref().unwrap();
            let field_name = field_ident.to_string();
            let field_ty = &field.ty;

            field_names.push(field_name.clone());
            match_pattern_fields.push(quote! { #field_ident });

            // Determine how to extract the field value
            let field_attrs = FieldAttrs::from_attrs(&field.attrs)?;
            let is_enum_field = {
                if let Type::Path(path) = field_ty {
                    if let Some(seg) = path.path.segments.last() {
                        seg.ident.to_string().ends_with("Enum")
                    } else {
                        false
                    }
                } else {
                    false
                }
            };
            let is_enum_struct_field = field_attrs.is_enum_struct;

            let field_shape = get_field_shape(enum_ident, field_ident, field_ty, is_enum_field)?;
            where_bounds.extend(field_shape.bounds);

            // Generate value extraction based on field type
            let primitive_variant = match field_ty {
                Type::Path(path) => {
                    if is_fixed(path) {
                        Some(quote! { Fixed })
                    } else if is_i32(path) {
                        Some(quote! { Int32 })
                    } else if is_bool(path) {
                        Some(quote! { Bool })
                    } else if is_vec2(path) {
                        Some(quote! { Vec2 })
                    } else if is_vec3(path) {
                        Some(quote! { Vec3 })
                    } else if is_vec4(path) {
                        Some(quote! { Vec4 })
                    } else if is_mat3(path) {
                        Some(quote! { Mat3 })
                    } else {
                        None
                    }
                }
                _ => None,
            };

            if let Some(variant) = primitive_variant {
                field_value_extractions.push(quote! {
    crate::kind::value::LpValueBox::#variant(alloc::boxed::Box::new(#field_ident.clone()))
});
                field_value_extractions_mut.push(quote! {
    crate::kind::value::LpValueBox::#variant(alloc::boxed::Box::new(#field_ident.clone()))
});
            } else if is_enum_struct_field {
                where_bounds.push(quote! { #field_ty: crate::kind::enum_struct::enum_struct_value::EnumStructValue + Clone });
                field_value_extractions.push(quote! {
    crate::kind::value::LpValueBox::EnumStruct(alloc::boxed::Box::new(#field_ident.clone()))
});
                field_value_extractions_mut.push(quote! {
    crate::kind::value::LpValueBox::EnumStruct(alloc::boxed::Box::new(#field_ident.clone()))
});
            } else if field_shape.is_enum {
                where_bounds
                    .push(quote! { #field_ty: crate::kind::enum_::enum_value::EnumValue + Clone });
                field_value_extractions.push(quote! {
    crate::kind::value::LpValueBox::Enum(alloc::boxed::Box::new(#field_ident.clone()))
});
                field_value_extractions_mut.push(quote! {
    crate::kind::value::LpValueBox::Enum(alloc::boxed::Box::new(#field_ident.clone()))
});
            } else {
                where_bounds.push(
                    quote! { #field_ty: crate::kind::record::record_value::RecordValue + Clone },
                );
                field_value_extractions.push(quote! {
    crate::kind::value::LpValueBox::Record(alloc::boxed::Box::new(#field_ident.clone()))
});
                field_value_extractions_mut.push(quote! {
    crate::kind::value::LpValueBox::Record(alloc::boxed::Box::new(#field_ident.clone()))
});
            }
        }

        // Generate value extraction code that creates RecordValueDyn
        variant_value_matchers.push(quote! {
                    #enum_ident::#variant_ident { #(#match_pattern_fields),* } => {
        use alloc::vec::Vec;
        use alloc::string::String;
        use crate::kind::record::record_dyn::{RecordFieldDyn, RecordShapeDyn};
        use crate::kind::record::record_meta::RecordFieldMetaDyn;
        use crate::kind::record::RecordValueDyn;

        // Create RecordShapeDyn from static shape
        let static_shape = &#shape_const_ident;
        let mut dyn_shape = RecordShapeDyn::new();
        dyn_shape.meta.name = String::from(static_shape.meta.name);
        dyn_shape.meta.docs = static_shape.meta.docs.map(String::from);
        for field in static_shape.fields {
            dyn_shape.fields.push(RecordFieldDyn {
                name: String::from(field.name),
                shape: field.shape,
                meta: RecordFieldMetaDyn {
                    docs: field.meta.docs.map(String::from),
                },
            });
        }

        let mut record = RecordValueDyn::new(dyn_shape);
        #(
            record.add_field(String::from(#field_names), #field_value_extractions).unwrap();
        )*
        // Box and leak the record to get a 'static reference
        // This is a workaround - the boxed value will never be freed.
        let boxed_record = alloc::boxed::Box::leak(alloc::boxed::Box::new(record));
        Ok(crate::kind::value::LpValueRef::Record(boxed_record))
                    }
                });

        variant_value_mut_matchers.push(quote! {
                    #enum_ident::#variant_ident { #(#match_pattern_fields),* } => {
        use alloc::vec::Vec;
        use alloc::string::String;
        use alloc::boxed::Box;
        use crate::kind::record::record_dyn::{RecordFieldDyn, RecordShapeDyn};
        use crate::kind::record::record_meta::RecordFieldMetaDyn;
        use crate::kind::record::RecordValueDyn;
        // Create RecordShapeDyn from static shape
        let static_shape = &#shape_const_ident;
        let mut dyn_shape = RecordShapeDyn::new();
        dyn_shape.meta.name = String::from(static_shape.meta.name);
        dyn_shape.meta.docs = static_shape.meta.docs.map(String::from);
        for field in static_shape.fields {
            dyn_shape.fields.push(RecordFieldDyn {
                name: String::from(field.name),
                shape: field.shape,
                meta: RecordFieldMetaDyn {
                    docs: field.meta.docs.map(String::from),
                },
            });
        }
        // Create RecordValueDyn from variant fields
        // Note: For mutable access, we create a new RecordValueDyn.
        // Mutations won't affect the original enum variant.
        // TODO: Implement proper mutable access that can update the original variant
        let mut record = RecordValueDyn::new(dyn_shape);
        #(
            record.add_field(String::from(#field_names), #field_value_extractions_mut).unwrap();
        )*
        // For mutable access, we box the RecordValueDyn and leak it to get a 'static reference.
        // This is a workaround - the boxed value will never be freed.
        // Mutations won't affect the original enum variant.
        // TODO: Implement proper mutable access that can update the original variant.
        let boxed = Box::leak(Box::new(record));
        Ok(crate::kind::value::LpValueRefMut::Record(boxed))
                    }
                });
    }

    let where_clause = if where_bounds.is_empty() {
        quote! {}
    } else {
        quote! { where #(#where_bounds),* }
    };

    let tokens = quote! {
            #shape_constant

            impl crate::kind::value::LpValue for #enum_ident {
                fn shape(&self) -> &dyn crate::kind::shape::LpShape {
    &#shape_const_ident
                }
            }

            impl crate::kind::enum_struct::enum_struct_value::EnumStructValue for #enum_ident #where_clause {
                fn shape(&self) -> &dyn crate::kind::enum_struct::enum_struct_shape::EnumStructShape {
    &#shape_const_ident
                }

                fn variant_index(&self) -> usize {
    match self {
        #(#variant_index_matchers)*
    }
                }

                fn variant_value(&self) -> Result<crate::kind::value::LpValueRef<'_>, crate::RuntimeError> {
    match self {
        #(#variant_value_matchers)*
    }
                }

                fn variant_value_mut(&mut self) -> Result<crate::kind::value::LpValueRefMut<'_>, crate::RuntimeError> {
    match self {
        #(#variant_value_mut_matchers)*
    }
                }
            }
        };

    Ok(tokens)
}

struct FieldShape {
    shape_expr: TokenStream2,
    bounds: Vec<TokenStream2>,
    is_enum: bool,
}

fn get_field_shape(
    _struct_ident: &Ident,
    _field_ident: &Ident,
    ty: &Type,
    is_enum_field: bool,
) -> Result<FieldShape, Error> {
    match ty {
        Type::Path(path) => {
            if is_fixed(path) {
Ok(FieldShape {
    shape_expr: quote! {
        &crate::kind::fixed::fixed_static::FIXED_SHAPE
    },
    bounds: Vec::new(),
    is_enum: false,
})
            } else if is_i32(path) {
Ok(FieldShape {
    shape_expr: quote! {
        &crate::kind::int32::int32_static::INT32_SHAPE
    },
    bounds: Vec::new(),
    is_enum: false,
})
            } else if is_bool(path) {
Ok(FieldShape {
    shape_expr: quote! {
        &crate::kind::bool::bool_static::BOOL_SHAPE
    },
    bounds: Vec::new(),
    is_enum: false,
})
            } else if is_vec2(path) {
Ok(FieldShape {
    shape_expr: quote! {
        &crate::kind::vec2::vec2_static::VEC2_SHAPE
    },
    bounds: Vec::new(),
    is_enum: false,
})
            } else if is_vec3(path) {
Ok(FieldShape {
    shape_expr: quote! {
        &crate::kind::vec3::vec3_static::VEC3_SHAPE
    },
    bounds: Vec::new(),
    is_enum: false,
})
            } else if is_vec4(path) {
Ok(FieldShape {
    shape_expr: quote! {
        &crate::kind::vec4::vec4_static::VEC4_SHAPE
    },
    bounds: Vec::new(),
    is_enum: false,
})
            } else if is_mat3(path) {
Ok(FieldShape {
    shape_expr: quote! {
        &crate::kind::mat3::mat3_static::MAT3_SHAPE
    },
    bounds: Vec::new(),
    is_enum: false,
})
            } else {
// Could be either enum or record type
let bounds = Vec::new();
let shape_ref = if path.path.segments.len() == 1 {
    let type_ident = &path.path.segments[0].ident;
    let shape_const_name =
        format_ident!("__LP_VALUE_{}_SHAPE", type_ident.to_string().to_uppercase());
    quote! { &#shape_const_name }
} else {
    let type_ident = path.path.segments.last().unwrap();
    let shape_const_name = format_ident!(
        "__LP_VALUE_{}_SHAPE",
        type_ident.ident.to_string().to_uppercase()
    );
    quote! { &#shape_const_name }
};

Ok(FieldShape {
    shape_expr: shape_ref,
    bounds,
    is_enum: is_enum_field,
})
            }
        }
        _ => Err(Error::new(
            ty.span(),
            "unsupported field type; expected Fixed, Int32, Bool, Vec2, Vec3, Vec4, Mat3, enum, or record types",
        )),
    }
}

fn generate_variant_record_shape_static(
    enum_ident: &Ident,
    variant_ident: &Ident,
    variant_name: &str,
    variant_docs: Option<String>,
    fields: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
) -> Result<(TokenStream2, Vec<TokenStream2>), Error> {
    let mut field_exprs = Vec::new();
    let mut where_bounds = Vec::new();

    for field in fields {
        let field_ident = field
            .ident
            .as_ref()
            .ok_or_else(|| Error::new(field.span(), "expected named field"))?;
        let field_name = field_ident.to_string();

        let field_attrs = FieldAttrs::from_attrs(&field.attrs)?;
        let field_docs = merge_docs(extract_doc_comments(&field.attrs), field_attrs.docs.clone());

        // Check if field is marked as enum via attribute, or check naming convention
        let is_enum_field = {
            if let Type::Path(path) = &field.ty {
                if let Some(seg) = path.path.segments.last() {
                    seg.ident.to_string().ends_with("Enum")
                } else {
                    false
                }
            } else {
                false
            }
        };

        let field_shape = get_field_shape(enum_ident, field_ident, &field.ty, is_enum_field)?;
        where_bounds.extend(field_shape.bounds);

        let field_meta = if let Some(docs) = field_docs {
            let docs_lit = LitStr::new(&docs, Span::call_site());
            quote! {
            crate::kind::record::record_meta::RecordFieldMetaStatic {
                docs: Some(#docs_lit),
            }
                        }
        } else {
            quote! {
            crate::kind::record::record_meta::RecordFieldMetaStatic {
                docs: None,
            }
                        }
        };

        let field_name_lit = LitStr::new(&field_name, Span::call_site());
        let shape_expr = field_shape.shape_expr;
        field_exprs.push(quote! {
                    crate::kind::record::record_static::RecordFieldStatic {
        name: #field_name_lit,
        shape: #shape_expr,
        meta: #field_meta,
                    }
                });
    }

    let fields_const_ident = format_ident!(
        "__LP_VALUE_{}_{}_FIELDS",
        enum_ident.to_string().to_uppercase(),
        variant_ident.to_string().to_uppercase()
    );
    let shape_const_ident = format_ident!(
        "__LP_VALUE_{}_{}_SHAPE",
        enum_ident.to_string().to_uppercase(),
        variant_ident.to_string().to_uppercase()
    );

    let record_name_lit = LitStr::new(variant_name, Span::call_site());
    let record_meta = if let Some(docs) = variant_docs {
        let docs_lit = LitStr::new(&docs, Span::call_site());
        quote! {
                    crate::kind::record::record_meta::RecordMetaStatic {
        name: #record_name_lit,
        docs: Some(#docs_lit),
                    }
                }
    } else {
        quote! {
                    crate::kind::record::record_meta::RecordMetaStatic {
        name: #record_name_lit,
        docs: None,
                    }
                }
    };

    let tokens = quote! {
        const #fields_const_ident: &'static [crate::kind::record::record_static::RecordFieldStatic] = &[
            #(#field_exprs),*
        ];

        const #shape_const_ident: crate::kind::record::record_static::RecordShapeStatic = crate::kind::record::record_static::RecordShapeStatic {
            meta: #record_meta,
            fields: #fields_const_ident,
        };
    };

    Ok((tokens, where_bounds))
}

fn generate_enum_struct_shape_static(
    enum_ident: &Ident,
    enum_struct_name: &str,
    enum_struct_docs: Option<String>,
    variants: &[&Variant],
) -> Result<TokenStream2, Error> {
    let mut variant_exprs = Vec::new();
    let mut variant_shape_constants = Vec::new();

    for variant in variants {
        let variant_ident = &variant.ident;
        let variant_name = variant_ident.to_string();

        let variant_attrs = FieldAttrs::from_attrs(&variant.attrs)?;
        let variant_docs = merge_docs(
            extract_doc_comments(&variant.attrs),
            variant_attrs.docs.clone(),
        );

        // Extract named fields from the variant
        let fields = match &variant.fields {
            Fields::Named(fields) => &fields.named,
            _ => unreachable!(),
        };

        // Generate record shape for this variant
        let (shape_constant, _bounds) = generate_variant_record_shape_static(
            enum_ident,
            variant_ident,
            &variant_name,
            variant_docs.clone(),
            fields,
        )?;
        variant_shape_constants.push(shape_constant);

        let shape_const_ident = format_ident!(
            "__LP_VALUE_{}_{}_SHAPE",
            enum_ident.to_string().to_uppercase(),
            variant_ident.to_string().to_uppercase()
        );
        let shape_expr = quote! { &#shape_const_ident };

        let variant_meta = if let Some(docs) = variant_docs {
            let docs_lit = LitStr::new(&docs, Span::call_site());
            quote! {
            crate::kind::enum_struct::enum_struct_meta::EnumStructVariantMetaStatic {
                docs: Some(#docs_lit),
            }
                        }
        } else {
            quote! {
            crate::kind::enum_struct::enum_struct_meta::EnumStructVariantMetaStatic {
                docs: None,
            }
                        }
        };

        let variant_name_lit = LitStr::new(&variant_name, Span::call_site());
        variant_exprs.push(quote! {
                    crate::kind::enum_struct::enum_struct_static::EnumStructVariantStatic {
        name: #variant_name_lit,
        shape: #shape_expr,
        meta: #variant_meta,
                    }
                });
    }

    let variants_const_ident = format_ident!(
        "__LP_VALUE_{}_VARIANTS",
        enum_ident.to_string().to_uppercase()
    );
    let shape_const_ident =
        format_ident!("__LP_VALUE_{}_SHAPE", enum_ident.to_string().to_uppercase());

    let enum_struct_name_lit = LitStr::new(enum_struct_name, Span::call_site());
    let enum_struct_meta = if let Some(docs) = enum_struct_docs {
        let docs_lit = LitStr::new(&docs, Span::call_site());
        quote! {
                    crate::kind::enum_struct::enum_struct_meta::EnumStructMetaStatic {
        name: #enum_struct_name_lit,
        docs: Some(#docs_lit),
                    }
                }
    } else {
        quote! {
                    crate::kind::enum_struct::enum_struct_meta::EnumStructMetaStatic {
        name: #enum_struct_name_lit,
        docs: None,
                    }
                }
    };

    let tokens = quote! {
        #(#variant_shape_constants)*

        const #variants_const_ident: &'static [crate::kind::enum_struct::enum_struct_static::EnumStructVariantStatic] = &[
            #(#variant_exprs),*
        ];

        pub const #shape_const_ident: crate::kind::enum_struct::enum_struct_static::EnumStructShapeStatic = crate::kind::enum_struct::enum_struct_static::EnumStructShapeStatic {
            meta: #enum_struct_meta,
            variants: #variants_const_ident,
        };
    };

    Ok(tokens)
}

fn is_fixed(path: &TypePath) -> bool {
    path.path
        .segments
        .last()
        .map(|seg| seg.ident == "Fixed")
        .unwrap_or(false)
}

fn is_i32(path: &TypePath) -> bool {
    path.path.is_ident("i32")
}

fn is_bool(path: &TypePath) -> bool {
    path.path.is_ident("bool")
}

fn is_vec2(path: &TypePath) -> bool {
    path.path
        .segments
        .last()
        .map(|seg| seg.ident == "Vec2")
        .unwrap_or(false)
}

fn is_vec3(path: &TypePath) -> bool {
    path.path
        .segments
        .last()
        .map(|seg| seg.ident == "Vec3")
        .unwrap_or(false)
}

fn is_vec4(path: &TypePath) -> bool {
    path.path
        .segments
        .last()
        .map(|seg| seg.ident == "Vec4")
        .unwrap_or(false)
}

fn is_mat3(path: &TypePath) -> bool {
    path.path
        .segments
        .last()
        .map(|seg| seg.ident == "Mat3")
        .unwrap_or(false)
}

// Helper structs and functions (similar to enum_value.rs)
struct StructAttrs {
    name: Option<String>,
    docs: Option<String>,
}

impl StructAttrs {
    fn from_attrs(_attrs: &[Attribute]) -> Result<Self, Error> {
        let name = None;
        let docs = None;

        Ok(StructAttrs { name, docs })
    }
}

struct FieldAttrs {
    docs: Option<String>,
    is_enum_struct: bool,
}

impl FieldAttrs {
    fn from_attrs(attrs: &[Attribute]) -> Result<Self, Error> {
        let mut result = FieldAttrs {
            docs: extract_doc_comments(attrs),
            is_enum_struct: false,
        };
        for attr in attrs {
            if !attr.path().is_ident("lp") {
                continue;
            }
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("enum_struct") {
                    result.is_enum_struct = true;
                    Ok(())
                } else {
                    Err(meta.error("unknown lp directive"))
                }
            })?;
        }
        Ok(result)
    }
}

fn extract_doc_comments(attrs: &[Attribute]) -> Option<String> {
    let mut docs = Vec::new();
    for attr in attrs {
        if attr.path().is_ident("doc") {
            if let syn::Meta::NameValue(nv) = &attr.meta {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(s),
                    ..
                }) = &nv.value
                {
                    docs.push(s.value());
                }
            }
        }
    }
    if docs.is_empty() {
        None
    } else {
        Some(docs.join("\n"))
    }
}

fn merge_docs(doc_comments: Option<String>, attr_docs: Option<String>) -> Option<String> {
    match (doc_comments, attr_docs) {
        (Some(a), Some(b)) => Some(format!("{}\n{}", a, b)),
        (Some(a), None) | (None, Some(a)) => Some(a),
        (None, None) => None,
    }
}
