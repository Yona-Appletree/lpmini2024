use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, spanned::Spanned, Attribute, Data, DataStruct, DeriveInput, Error, Fields,
    Ident, LitStr, Type, TypePath,
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

        let field_shape = get_field_shape(struct_ident, field_ident, &field.ty)?;
        where_bounds.extend(field_shape.bounds);

        let is_fixed_type = matches!(&field.ty, Type::Path(path) if is_fixed(path));

        if is_fixed_type {
            field_getters.push(quote! {
                #index => Ok(crate::kind::value::LpValueRef::Fixed(&self.#field_ident as &dyn crate::kind::value::LpValue)),
            });
            field_getters_mut.push(quote! {
                #index => Ok(crate::kind::value::LpValueRefMut::Fixed(&mut self.#field_ident as &mut dyn crate::kind::value::LpValue)),
            });
        } else {
            let field_ty = &field.ty;
            where_bounds.push(quote! { #field_ty: crate::kind::record::record_value::RecordValue });
            field_getters.push(quote! {
                #index => Ok(crate::kind::value::LpValueRef::Record(&self.#field_ident as &dyn crate::kind::record::record_value::RecordValue)),
            });
            field_getters_mut.push(quote! {
                #index => Ok(crate::kind::value::LpValueRefMut::Record(&mut self.#field_ident as &mut dyn crate::kind::record::record_value::RecordValue)),
            });
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

    for field in fields {
        let field_ident = field
            .ident
            .as_ref()
            .ok_or_else(|| Error::new(field.span(), "expected named field"))?;
        let field_name = field_ident.to_string();

        let field_attrs = FieldAttrs::from_attrs(&field.attrs)?;
        let field_docs = merge_docs(extract_doc_comments(&field.attrs), field_attrs.docs.clone());

        let field_shape = get_field_shape(struct_ident, field_ident, &field.ty)?;
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
        const #fields_const_ident: &'static [crate::kind::record::record_static::RecordFieldStatic] = &[
            #(#field_exprs),*
        ];

        const #shape_const_ident: crate::kind::record::record_static::RecordShapeStatic = crate::kind::record::record_static::RecordShapeStatic {
            meta: #record_meta,
            fields: #fields_const_ident,
        };
    };

    Ok(tokens)
}

struct FieldShape {
    shape_expr: TokenStream2,
    bounds: Vec<TokenStream2>,
}

fn get_field_shape(
    _struct_ident: &Ident,
    _field_ident: &Ident,
    ty: &Type,
) -> Result<FieldShape, Error> {
    match ty {
        Type::Path(path) => {
            if is_fixed(path) {
                Ok(FieldShape {
                    shape_expr: quote! {
                        &crate::kind::fixed::fixed_static::FIXED_SHAPE
                    },
                    bounds: Vec::new(),
                })
            } else {
                // Assume it's a record type - reference its shape
                let mut bounds = Vec::new();
                bounds.push(quote! { #ty: crate::kind::value::LpValue });

                // For same-crate types, reference the generated shape constant directly
                let shape_ref = if path.path.segments.len() == 1 {
                    let type_ident = &path.path.segments[0].ident;
                    let shape_const_name =
                        format_ident!("__LP_VALUE_{}_SHAPE", type_ident.to_string().to_uppercase());
                    quote! { &#shape_const_name }
                } else {
                    // For external types, we need to call shape() at runtime
                    // But we can't do that in a const context, so we'll need a different approach
                    // For now, assume same-crate types
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
                })
            }
        }
        _ => Err(Error::new(
            ty.span(),
            "unsupported field type; expected Fixed or record types",
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
