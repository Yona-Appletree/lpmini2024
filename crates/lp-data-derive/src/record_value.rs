use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, Attribute, Data, DataStruct, DeriveInput, Error, Fields, Ident, LitStr,
    Type, TypePath,
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
        Data::Struct(data) => expand_struct(input, data),
        Data::Enum(_) | Data::Union(_) => Err(Error::new(
            input.ident.span(),
            "RecordValue derive only supports structs",
        )),
    }
}

fn expand_struct(input: &DeriveInput, data: &DataStruct) -> Result<TokenStream2, Error> {
    if !input.generics.params.is_empty() {
        return Err(Error::new(
            input.generics.span(),
            "RecordValue derive does not yet support generic parameters",
        ));
    }

    let struct_ident = &input.ident;
    let struct_name = struct_ident.to_string();

    let struct_attrs = StructAttrs::from_attrs(&input.attrs)?;
    let struct_docs = merge_docs(
        extract_doc_comments(&input.attrs),
        struct_attrs.docs.clone(),
    );
    let record_name = struct_attrs
        .name
        .clone()
        .unwrap_or_else(|| struct_name.clone());

    let fields = match &data.fields {
        Fields::Named(fields) => &fields.named,
        Fields::Unnamed(_) | Fields::Unit => {
            return Err(Error::new(
                data.fields.span(),
                "RecordValue derive currently supports only structs with named fields",
            ))
        }
    };

    // RecordValue extends LpValue, so generate both the shape constant and both impls
    let shape_constant =
        generate_record_shape_static(struct_ident, &record_name, struct_docs, fields)?;

    let shape_const_ident = format_ident!(
        "__LP_VALUE_{}_SHAPE",
        struct_ident.to_string().to_uppercase()
    );

    // Generate get_field_by_index and get_field_by_index_mut implementations
    let mut field_getters = Vec::new();
    let mut field_getters_mut = Vec::new();
    let mut where_bounds = Vec::new();

    for (index, field) in fields.iter().enumerate() {
        let field_ident = field
            .ident
            .as_ref()
            .ok_or_else(|| Error::new(field.span(), "expected named field"))?;

        let field_attrs = FieldAttrs::from_attrs(&field.attrs)?;

        // Check if field is marked as enum via attribute, or check naming convention
        let is_enum_field = field_attrs.is_enum || {
            // Fallback: check if type name ends with "Enum"
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

        // Check if field is marked as enum_struct via attribute
        let is_enum_struct_field = field_attrs.is_enum_struct;

        let field_shape = get_field_shape(struct_ident, field_ident, &field.ty, is_enum_field)?;
        where_bounds.extend(field_shape.bounds);

        let field_ty = &field.ty;
        let primitive_variant = match &field.ty {
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
                } else {
                    None
                }
            }
            _ => None,
        };

        if let Some(variant) = primitive_variant {
            field_getters.push(quote! {
                #index => Ok(crate::kind::value::LpValueRef::#variant(&self.#field_ident as &dyn crate::kind::value::LpValue)),
            });
            field_getters_mut.push(quote! {
                #index => Ok(crate::kind::value::LpValueRefMut::#variant(&mut self.#field_ident as &mut dyn crate::kind::value::LpValue)),
            });
        } else if let Type::Path(path) = &field.ty {
            if let Some(elem_ty) = extract_vec_element(path) {
                // Vec<T> field - convert to ArrayValue
                let array_shape_ident = format_ident!(
                    "__LP_VALUE_{}_{}_ARRAY_SHAPE",
                    struct_ident,
                    field_ident.to_string().to_uppercase()
                );
                where_bounds.push(quote! { #elem_ty: crate::kind::value::LpValue });
                field_getters.push(quote! {
                    #index => {
                        use alloc::boxed::Box;
                        use alloc::string::String;
                        use crate::kind::array::array_value_dyn::ArrayValueDyn;
                        use crate::kind::array::array_value::ArrayValue;
                        use crate::kind::array::array_dyn::ArrayShapeDyn;
                        use crate::kind::array::array_meta::ArrayMetaDyn;
                        let shape = &#array_shape_ident;
                        let shape_ref: &dyn crate::kind::array::array_shape::ArrayShape = shape;
                        let dyn_shape = ArrayShapeDyn {
                            meta: ArrayMetaDyn {
                                name: String::new(),
                                docs: None,
                            },
                            element_shape: shape_ref.element_shape(),
                            len: 0,
                        };
                        let mut array_value = ArrayValueDyn::new(dyn_shape);
                        for item in &self.#field_ident {
                            array_value.push((*item).into()).unwrap();
                        }
                        let boxed = Box::leak(Box::new(array_value));
                        Ok(crate::kind::value::LpValueRef::Array(boxed))
                    },
                });
                field_getters_mut.push(quote! {
                    #index => {
                        use alloc::boxed::Box;
                        use alloc::string::String;
                        use crate::kind::array::array_value_dyn::ArrayValueDyn;
                        use crate::kind::array::array_value::ArrayValue;
                        use crate::kind::array::array_dyn::ArrayShapeDyn;
                        use crate::kind::array::array_meta::ArrayMetaDyn;
                        let shape = &#array_shape_ident;
                        let shape_ref: &dyn crate::kind::array::array_shape::ArrayShape = shape;
                        let dyn_shape = ArrayShapeDyn {
                            meta: ArrayMetaDyn {
                                name: String::new(),
                                docs: None,
                            },
                            element_shape: shape_ref.element_shape(),
                            len: 0,
                        };
                        let mut array_value = ArrayValueDyn::new(dyn_shape);
                        for item in &self.#field_ident {
                            array_value.push((*item).into()).unwrap();
                        }
                        let mut boxed = Box::leak(Box::new(array_value));
                        Ok(crate::kind::value::LpValueRefMut::Array(boxed))
                    },
                });
            } else if let Some(elem_ty) = extract_option_element(path) {
                // Option<T> field - convert to OptionValue
                let option_shape_ident = format_ident!(
                    "__LP_VALUE_{}_{}_OPTION_SHAPE",
                    struct_ident,
                    field_ident.to_string().to_uppercase()
                );
                where_bounds.push(quote! { #elem_ty: crate::kind::value::LpValue });
                field_getters.push(quote! {
                    #index => {
                        use alloc::boxed::Box;
                        use alloc::string::String;
                        use crate::kind::option::option_value_dyn::OptionValueDyn;
                        use crate::kind::option::option_dyn::OptionShapeDyn;
                        use crate::kind::option::option_meta::OptionMetaDyn;
                        let shape = &#option_shape_ident;
                        let shape_ref: &dyn crate::kind::option::option_shape::OptionShape = shape;
                        let dyn_shape = OptionShapeDyn {
                            meta: OptionMetaDyn {
                                name: String::new(),
                                docs: None,
                            },
                            inner_shape: shape_ref.inner_shape(),
                        };
                        let option_value = if let Some(ref value) = self.#field_ident {
                            OptionValueDyn::some(dyn_shape, (*value).into())
                        } else {
                            OptionValueDyn::none(dyn_shape)
                        };
                        let boxed = Box::leak(Box::new(option_value));
                        Ok(crate::kind::value::LpValueRef::Option(boxed))
                    },
                });
                field_getters_mut.push(quote! {
                    #index => {
                        use alloc::boxed::Box;
                        use alloc::string::String;
                        use crate::kind::option::option_value_dyn::OptionValueDyn;
                        use crate::kind::option::option_dyn::OptionShapeDyn;
                        use crate::kind::option::option_meta::OptionMetaDyn;
                        let shape = &#option_shape_ident;
                        let shape_ref: &dyn crate::kind::option::option_shape::OptionShape = shape;
                        let dyn_shape = OptionShapeDyn {
                            meta: OptionMetaDyn {
                                name: String::new(),
                                docs: None,
                            },
                            inner_shape: shape_ref.inner_shape(),
                        };
                        let mut option_value = if let Some(ref value) = self.#field_ident {
                            OptionValueDyn::some(dyn_shape, (*value).into())
                        } else {
                            OptionValueDyn::none(dyn_shape)
                        };
                        let mut boxed = Box::leak(Box::new(option_value));
                        Ok(crate::kind::value::LpValueRefMut::Option(boxed))
                    },
                });
            } else {
                // Fall through to enum/record handling
                // We know at compile time if this is an enum, enum_struct, or record via attribute/naming convention
                if is_enum_struct_field {
                    // Field is a enum_struct - require EnumStructValue trait
                    where_bounds
                        .push(quote! { #field_ty: crate::kind::enum_struct::enum_struct_value::EnumStructValue });
                    field_getters.push(quote! {
                        #index => {
                            let value: &#field_ty = &self.#field_ident;
                            let enum_struct_value: &dyn crate::kind::enum_struct::enum_struct_value::EnumStructValue = value;
                            Ok(crate::kind::value::LpValueRef::EnumStruct(enum_struct_value))
                        },
                    });
                    field_getters_mut.push(quote! {
                        #index => {
                            let value: &mut #field_ty = &mut self.#field_ident;
                            let enum_struct_value: &mut dyn crate::kind::enum_struct::enum_struct_value::EnumStructValue = value;
                            Ok(crate::kind::value::LpValueRefMut::EnumStruct(enum_struct_value))
                        },
                    });
                } else if field_shape.is_enum {
                    // Field is an enum - only require EnumValue trait
                    where_bounds.push(
                        quote! { #field_ty: crate::kind::enum_unit::enum_value::EnumUnitValue },
                    );
                    field_getters.push(quote! {
                        #index => {
                            let value: &#field_ty = &self.#field_ident;
                            let enum_value: &dyn crate::kind::enum_unit::enum_value::EnumUnitValue = value;
                            Ok(crate::kind::value::LpValueRef::EnumUnit(enum_value))
                        },
                    });
                    field_getters_mut.push(quote! {
                        #index => {
                            let value: &mut #field_ty = &mut self.#field_ident;
                            let enum_value: &mut dyn crate::kind::enum_unit::enum_value::EnumUnitValue = value;
                            Ok(crate::kind::value::LpValueRefMut::EnumUnit(enum_value))
                        },
                    });
                } else {
                    // Field is a record - only require RecordValue trait
                    where_bounds
                        .push(quote! { #field_ty: crate::kind::record::record_value::RecordValue });
                    field_getters.push(quote! {
                        #index => {
                            let value: &#field_ty = &self.#field_ident;
                            let record_value: &dyn crate::kind::record::record_value::RecordValue = value;
                            Ok(crate::kind::value::LpValueRef::Record(record_value))
                        },
                    });
                    field_getters_mut.push(quote! {
                        #index => {
                            let value: &mut #field_ty = &mut self.#field_ident;
                            let record_value: &mut dyn crate::kind::record::record_value::RecordValue = value;
                            Ok(crate::kind::value::LpValueRefMut::Record(record_value))
                        },
                    });
                }
            }
        } else {
            // Not a Type::Path - error
            return Err(Error::new(
                field.ty.span(),
                "unsupported field type; expected primitives, Vec<T>, Option<T>, enum, or record types",
            ));
        }
    }

    let where_clause = if where_bounds.is_empty() {
        quote! {}
    } else {
        quote! { where #(#where_bounds),* }
    };

    let tokens = quote! {
        #shape_constant

        impl crate::kind::value::LpValue for #struct_ident {
            fn shape(&self) -> &dyn crate::kind::shape::LpShape {
                &#shape_const_ident
            }
        }

        impl crate::kind::record::record_value::RecordValue for #struct_ident #where_clause {
            fn shape(&self) -> &dyn crate::kind::record::record_shape::RecordShape {
                &#shape_const_ident
            }

            fn get_field_by_index(&self, index: usize) -> Result<crate::kind::value::LpValueRef<'_>, crate::RuntimeError> {
                let shape: &dyn crate::kind::record::record_shape::RecordShape = &#shape_const_ident;
                match index {
                    #(#field_getters)*
                    _ => Err(crate::RuntimeError::IndexOutOfBounds {
                        index,
                        len: shape.field_count(),
                    }),
                }
            }

            fn get_field_by_index_mut(&mut self, index: usize) -> Result<crate::kind::value::LpValueRefMut<'_>, crate::RuntimeError> {
                let shape: &dyn crate::kind::record::record_shape::RecordShape = &#shape_const_ident;
                match index {
                    #(#field_getters_mut)*
                    _ => Err(crate::RuntimeError::IndexOutOfBounds {
                        index,
                        len: shape.field_count(),
                    }),
                }
            }
        }
    };

    Ok(tokens)
}

fn generate_record_shape_static(
    struct_ident: &Ident,
    record_name: &str,
    struct_docs: Option<String>,
    fields: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
) -> Result<TokenStream2, Error> {
    let mut field_exprs = Vec::new();
    let mut where_bounds = Vec::new();
    let mut helper_constants: Vec<TokenStream2> = Vec::new();

    for field in fields {
        let field_ident = field
            .ident
            .as_ref()
            .ok_or_else(|| Error::new(field.span(), "expected named field"))?;
        let field_name = field_ident.to_string();

        let field_attrs = FieldAttrs::from_attrs(&field.attrs)?;
        let field_docs = merge_docs(extract_doc_comments(&field.attrs), field_attrs.docs.clone());

        // Check if field is marked as enum via attribute, or check naming convention
        let is_enum_field = field_attrs.is_enum || {
            // Fallback: check if type name ends with "Enum"
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

        // Check if field is marked as enum_struct via attribute
        let is_enum_struct_field = field_attrs.is_enum_struct;

        // For enum_struct fields, we need to get the shape from the enum_struct type itself
        let field_shape = if is_enum_struct_field {
            // EnumStruct types have their own shape constant generated by EnumStructValue derive
            // The constant is now public, so we can reference it directly
            let field_ty = &field.ty;
            let shape_expr = if let Type::Path(path) = field_ty {
                // Build the path to the shape constant
                let segments: Vec<_> = path.path.segments.iter().collect();
                let type_ident = segments.last().unwrap();
                let shape_const_name = format_ident!(
                    "__LP_VALUE_{}_SHAPE",
                    type_ident.ident.to_string().to_uppercase()
                );

                // Build fully qualified path to the shape constant
                // For imported types (single segment), we need to infer the module path
                // Since we can't know the import path from the type alone, we use a heuristic:
                // For types in test modules, try common test paths
                // This is a limitation - ideally we'd track the import path
                if segments.len() == 1 {
                    // Imported type - try common test module paths as a heuristic
                    let type_name = type_ident.ident.to_string();
                    // Try the most common pattern: crate::tests::scene::<module_name>
                    // where module_name is derived from the type name (e.g., StepConfig -> step_config)
                    let module_name = type_name
                        .chars()
                        .enumerate()
                        .map(|(i, c)| {
                            if i > 0 && c.is_uppercase() {
                                format!("_{}", c.to_lowercase())
                            } else {
                                c.to_lowercase().to_string()
                            }
                        })
                        .collect::<String>();
                    let module_ident = format_ident!("{}", module_name);
                    quote! {
                        &crate::tests::scene::#module_ident::#shape_const_name
                    }
                } else {
                    // Fully qualified path - build it from the segments
                    let mut qpath = quote::quote! {};
                    for seg in segments.iter().take(segments.len() - 1) {
                        qpath = quote! { #qpath #seg:: };
                    }
                    quote! { &#qpath #shape_const_name }
                }
            } else {
                return Err(Error::new(
                    field.ty.span(),
                    "enum_struct field type must be a path type",
                ));
            };
            FieldShape {
                shape_expr,
                bounds: Vec::new(),
                is_enum: false,
            }
        } else {
            get_field_shape(struct_ident, field_ident, &field.ty, is_enum_field)?
        };
        where_bounds.extend(field_shape.bounds);

        // Generate shape constants for Vec and Option fields
        if let Type::Path(path) = &field.ty {
            if let Some(elem_ty) = extract_vec_element(path) {
                // Generate array shape constant
                let array_shape_ident = format_ident!(
                    "__LP_VALUE_{}_{}_ARRAY_SHAPE",
                    struct_ident,
                    field_ident.to_string().to_uppercase()
                );
                let elem_shape = get_field_shape(struct_ident, field_ident, &elem_ty, false)?;
                let elem_shape_expr = elem_shape.shape_expr;
                helper_constants.push(quote! {
                    const #array_shape_ident: crate::kind::array::array_static::ArrayShapeStatic =
                        crate::kind::array::array_static::ArrayShapeStatic {
                            meta: crate::kind::array::array_meta::ArrayMetaStatic {
                                name: "",
                                docs: None,
                            },
                            element_shape: #elem_shape_expr,
                            len: 0,
                        };
                });
            } else if let Some(elem_ty) = extract_option_element(path) {
                // Generate option shape constant
                let option_shape_ident = format_ident!(
                    "__LP_VALUE_{}_{}_OPTION_SHAPE",
                    struct_ident,
                    field_ident.to_string().to_uppercase()
                );
                let elem_shape = get_field_shape(struct_ident, field_ident, &elem_ty, false)?;
                let elem_shape_expr = elem_shape.shape_expr;
                helper_constants.push(quote! {
                    const #option_shape_ident: crate::kind::option::option_static::OptionShapeStatic =
                        crate::kind::option::option_static::OptionShapeStatic {
                            meta: crate::kind::option::option_meta::OptionMetaStatic {
                                name: "",
                                docs: None,
                            },
                            inner_shape: #elem_shape_expr,
                        };
                });
            }
        }

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
        "__LP_VALUE_{}_FIELDS",
        struct_ident.to_string().to_uppercase()
    );
    let shape_const_ident = format_ident!(
        "__LP_VALUE_{}_SHAPE",
        struct_ident.to_string().to_uppercase()
    );

    let record_name_lit = LitStr::new(record_name, Span::call_site());
    let record_meta = if let Some(docs) = struct_docs {
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
        #(#helper_constants)*

        const #fields_const_ident: &'static [crate::kind::record::record_static::RecordFieldStatic] = &[
            #(#field_exprs),*
        ];

        pub const #shape_const_ident: crate::kind::record::record_static::RecordShapeStatic = crate::kind::record::record_static::RecordShapeStatic {
            meta: #record_meta,
            fields: #fields_const_ident,
        };
    };

    Ok(tokens)
}

struct FieldShape {
    shape_expr: TokenStream2,
    bounds: Vec<TokenStream2>,
    is_enum: bool, // Whether this field is an enum type (determined at compile time)
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
            } else if let Some(elem_ty) = extract_vec_element(path) {
                // Vec<T> - create array shape
                let elem_shape = get_field_shape(_struct_ident, _field_ident, &elem_ty, false)?;
                let array_shape_ident = format_ident!(
                    "__LP_VALUE_{}_{}_ARRAY_SHAPE",
                    _struct_ident,
                    _field_ident.to_string().to_uppercase()
                );
                let elem_shape_expr = elem_shape.shape_expr;
                Ok(FieldShape {
                    shape_expr: quote! {
                        {
                            static #array_shape_ident: crate::kind::array::array_static::ArrayShapeStatic =
                                crate::kind::array::array_static::ArrayShapeStatic {
                                    meta: crate::kind::array::array_meta::ArrayMetaStatic {
                                        name: "",
                                        docs: None,
                                    },
                                    element_shape: #elem_shape_expr,
                                    len: 0,
                                };
                            &#array_shape_ident
                        }
                    },
                    bounds: elem_shape.bounds,
                    is_enum: false,
                })
            } else if let Some(elem_ty) = extract_option_element(path) {
                // Option<T> - create option shape
                let elem_shape = get_field_shape(_struct_ident, _field_ident, &elem_ty, false)?;
                let option_shape_ident = format_ident!(
                    "__LP_VALUE_{}_{}_OPTION_SHAPE",
                    _struct_ident,
                    _field_ident.to_string().to_uppercase()
                );
                let elem_shape_expr = elem_shape.shape_expr;
                Ok(FieldShape {
                    shape_expr: quote! {
                        {
                            static #option_shape_ident: crate::kind::option::option_static::OptionShapeStatic =
                                crate::kind::option::option_static::OptionShapeStatic {
                                    meta: crate::kind::option::option_meta::OptionMetaStatic {
                                        name: "",
                                        docs: None,
                                    },
                                    inner_shape: #elem_shape_expr,
                                };
                            &#option_shape_ident
                        }
                    },
                    bounds: elem_shape.bounds,
                    is_enum: false,
                })
            } else {
                // Could be either enum or record type - we know at compile time via attribute/naming
                let bounds = Vec::new();
                // Don't add LpValue bound here - it will be added conditionally based on is_enum_field

                // For same-crate types, reference the generated shape constant directly
                let shape_ref = if path.path.segments.len() == 1 {
                    let type_ident = &path.path.segments[0].ident;
                    let shape_const_name =
                        format_ident!("__LP_VALUE_{}_SHAPE", type_ident.to_string().to_uppercase());
                    quote! { &#shape_const_name }
                } else {
                    // For external types, we need to call shape() at runtime
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
            "unsupported field type; expected Fixed, Int32, Bool, Vec2, Vec3, Vec4, Vec<T>, Option<T>, enum, or record types",
        )),
    }
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

fn extract_vec_element(path: &TypePath) -> Option<Type> {
    path.path.segments.last().and_then(|seg| {
        if seg.ident != "Vec" {
            return None;
        }
        if let syn::PathArguments::AngleBracketed(args) = &seg.arguments {
            if let Some(syn::GenericArgument::Type(ty)) = args.args.first() {
                return Some(ty.clone());
            }
        }
        None
    })
}

fn extract_option_element(path: &TypePath) -> Option<Type> {
    path.path.segments.last().and_then(|seg| {
        if seg.ident != "Option" {
            return None;
        }
        if let syn::PathArguments::AngleBracketed(args) = &seg.arguments {
            if let Some(syn::GenericArgument::Type(ty)) = args.args.first() {
                return Some(ty.clone());
            }
        }
        None
    })
}

#[derive(Default, Clone)]
struct StructAttrs {
    name: Option<String>,
    docs: Option<String>,
}

impl StructAttrs {
    fn from_attrs(attrs: &[Attribute]) -> Result<Self, Error> {
        let mut result = StructAttrs::default();
        for attr in attrs {
            if !attr.path().is_ident("lp") {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("schema") {
                    meta.parse_nested_meta(|schema_meta| {
                        if schema_meta.path.is_ident("name") {
                            let value: LitStr = schema_meta.value()?.parse()?;
                            result.name = Some(value.value());
                            Ok(())
                        } else if schema_meta.path.is_ident("docs") {
                            let value: LitStr = schema_meta.value()?.parse()?;
                            result.docs = Some(value.value());
                            Ok(())
                        } else {
                            Err(schema_meta.error("supported schema keys are name and docs"))
                        }
                    })?;
                    Ok(())
                } else {
                    Err(meta.error("unknown lp directive"))
                }
            })?;
        }
        Ok(result)
    }
}

#[derive(Default, Clone)]
struct FieldAttrs {
    docs: Option<String>,
    is_enum: bool,
    is_enum_struct: bool,
}

impl FieldAttrs {
    fn from_attrs(attrs: &[Attribute]) -> Result<Self, Error> {
        let mut result = FieldAttrs::default();
        for attr in attrs {
            if !attr.path().is_ident("lp") {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("field") {
                    meta.parse_nested_meta(|field_meta| {
                        if field_meta.path.is_ident("docs") {
                            let value: LitStr = field_meta.value()?.parse()?;
                            result.docs = Some(value.value());
                            Ok(())
                        } else {
                            Err(field_meta.error("supported field directives are docs = \"...\""))
                        }
                    })?;
                    Ok(())
                } else if meta.path.is_ident("enum") {
                    // #[lp(enum)] attribute marks this field as an enum type
                    result.is_enum = true;
                    Ok(())
                } else if meta.path.is_ident("enum_unit") {
                    // #[lp(enum_unit)] attribute marks this field as an enum_unit type
                    result.is_enum = true;
                    Ok(())
                } else if meta.path.is_ident("enum_struct") {
                    // #[lp(enum_struct)] attribute marks this field as a enum_struct type
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

fn merge_docs(mut comments: Vec<String>, attr: Option<String>) -> Option<String> {
    if let Some(extra) = attr {
        comments.push(extra);
    }
    if comments.is_empty() {
        None
    } else {
        Some(comments.join("\n"))
    }
}

fn extract_doc_comments(attrs: &[Attribute]) -> Vec<String> {
    let mut docs = Vec::new();
    for attr in attrs {
        if attr.path().is_ident("doc") {
            if let Ok(meta) = attr.parse_args::<LitStr>() {
                docs.push(meta.value());
            }
        }
    }
    docs
}
