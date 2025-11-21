use std::collections::BTreeSet;

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::meta::ParseNestedMeta;
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, Attribute, Data, DataEnum, DataStruct, DeriveInput, Error, Fields, Ident,
    LitStr, Type, TypePath,
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
        Data::Enum(data) => expand_enum(input, data),
        Data::Union(_) => Err(Error::new(
            input.ident.span(),
            "LpSchema derive does not support enum_structs",
        )),
    }
}

fn expand_enum(input: &DeriveInput, data: &DataEnum) -> Result<TokenStream2, Error> {
    if !input.generics.params.is_empty() {
        return Err(Error::new(
            input.generics.span(),
            "LpSchema derive does not yet support generic parameters",
        ));
    }

    let enum_ident = &input.ident;
    let enum_name = enum_ident.to_string();

    let enum_attrs = StructAttrs::from_attrs(&input.attrs)?;
    let enum_docs = merge_docs(extract_doc_comments(&input.attrs), enum_attrs.docs.clone());
    let enum_display_name = enum_attrs.name.clone().unwrap_or_else(|| enum_name.clone());

    let mut variant_exprs = Vec::new();
    for variant in &data.variants {
        if !variant.fields.is_empty() {
            return Err(Error::new(
                variant.span(),
                "LpSchema currently supports only unit-style enum variants",
            ));
        }
        let variant_name = variant.ident.to_string();
        let variant_lit = LitStr::new(&variant_name, Span::call_site());
        variant_exprs
            .push(quote! { lp_data::shape::enum::enum_meta::EnumVariant::unit(#variant_lit) });
    }

    let variants_const_ident = format_ident!(
        "__LP_SCHEMA_{}_VARIANTS",
        enum_ident.to_string().to_uppercase()
    );
    let schema_const_ident = format_ident!("__LP_SCHEMA_{}", enum_ident.to_string().to_uppercase());
    let variants_const = quote! {
        const #variants_const_ident: &'static [lp_data::EnumVariant<lp_data::TypeRef>] = &[
            #(#variant_exprs),*
        ];
    };

    let enum_name_lit = LitStr::new(&enum_display_name, Span::call_site());
    let mut enum_type_expr = quote! {
        lp_data::EnumType::new(#enum_name_lit, #variants_const_ident)
    };
    if let Some(ui) = enum_attrs.enum_ui {
        let ui_expr = match ui {
            EnumUiAttr::Dropdown => quote! { lp_data::EnumUi::Dropdown },
            EnumUiAttr::SegmentedControl => quote! { lp_data::EnumUi::SegmentedControl },
        };
        enum_type_expr = quote! { #enum_type_expr.with_ui(#ui_expr) };
    }

    let enum_type_expr = quote! {
        lp_data::LpType::Enum(#enum_type_expr)
    };
    let doc_expr = enum_docs.map(|doc| {
        let doc_lit = LitStr::new(&doc, Span::call_site());
        quote! { .with_docs(#doc_lit) }
    });
    let type_name_literal = enum_attrs
        .type_name_override
        .clone()
        .unwrap_or_else(|| enum_name.clone());
    let type_name_lit = LitStr::new(&type_name_literal, Span::call_site());

    let serde_assert = quote! {
        #[cfg(feature = "serde")]
        const _: fn() = || {
            fn assert_impls<T>()
            where
                T: ::serde::Serialize + for<'de> ::serde::Deserialize<'de>,
            {}
            assert_impls::<#enum_ident>();
        };
    };

    let tokens = quote! {
        #[allow(unused_imports)]

        #variants_const

        const #schema_const_ident: lp_data::LpTypeMeta =
            lp_data::LpTypeMeta::new(#enum_type_expr) #doc_expr;

        impl lp_data::LpDescribe for #enum_ident {
            const TYPE_NAME: &'static str = #type_name_lit;

            fn lp_schema() -> &'static lp_data::LpTypeMeta {
                &#schema_const_ident
            }
        }

        #serde_assert
    };

    Ok(tokens)
}

fn expand_struct(input: &DeriveInput, data: &DataStruct) -> Result<TokenStream2, Error> {
    if !input.generics.params.is_empty() {
        return Err(Error::new(
            input.generics.span(),
            "LpSchema derive does not yet support generic parameters",
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
                "LpSchema derive currently supports only structs with named fields",
            ))
        }
    };

    let mut helper_consts = Vec::new();
    let mut field_exprs = Vec::new();
    let mut where_bounds = Vec::new();
    let mut seen_bounds = BTreeSet::new();

    for field in fields {
        let field_ident = field
            .ident
            .as_ref()
            .ok_or_else(|| Error::new(field.span(), "expected named field"))?;
        let field_name = field_ident.to_string();

        let field_attrs = FieldAttrs::from_attrs(&field.attrs)?;
        let field_docs = merge_docs(extract_doc_comments(&field.attrs), field_attrs.docs.clone());

        let type_tokens =
            TypeTokens::for_field(struct_ident, field_ident, &field.ty, field_attrs.ui.clone())?;

        helper_consts.extend(type_tokens.helpers);
        for bound in type_tokens.bounds {
            let key = bound.to_string();
            if seen_bounds.insert(key) {
                where_bounds.push(bound);
            }
        }

        let record_field_expr = build_field_expr(&field_name, field_docs, type_tokens.type_ref);
        field_exprs.push(record_field_expr);
    }

    let fields_const_ident = format_ident!("__LP_SCHEMA_{}_FIELDS", struct_ident);
    let schema_const_ident =
        format_ident!("__LP_SCHEMA_{}", struct_ident.to_string().to_uppercase());
    let schema_shape_ref_ident = format_ident!(
        "__LP_SCHEMA_{}_SHAPE_REF",
        struct_ident.to_string().to_uppercase()
    );
    let type_name_literal = struct_attrs
        .type_name_override
        .clone()
        .unwrap_or_else(|| struct_name.clone());
    let record_name_lit = LitStr::new(&record_name, Span::call_site());
    let type_name_lit = LitStr::new(&type_name_literal, Span::call_site());

    let _doc_expr = struct_docs.map(|doc| {
        let doc_lit = LitStr::new(&doc, Span::call_site());
        quote! { .with_docs(#doc_lit) }
    });

    let where_clause = if where_bounds.is_empty() {
        quote! {}
    } else {
        quote! { where #(#where_bounds),* }
    };

    let serde_assert = quote! {
        #[cfg(feature = "serde")]
        const _: fn() = || {
            fn assert_impls<T>()
            where
                T: ::serde::Serialize + for<'de> ::serde::Deserialize<'de>,
            {}
            assert_impls::<#struct_ident>();
        };
    };

    let tokens = quote! {
        #[allow(unused_imports)]

        #(#helper_consts)*

        const #fields_const_ident: &'static [lp_data::shape::record::RecordField] = &[
            #(#field_exprs),*
        ];

        static #schema_const_ident: lp_data::shape::record::StaticRecordShape =
            lp_data::shape::record::StaticRecordShape {
                name: #record_name_lit,
                fields: #fields_const_ident,
                ui: lp_data::shape::record::RecordUi { collapsible: false },
            };

        static #schema_shape_ref_ident: lp_data::shape::shape_ref::ShapeRef =
            lp_data::shape::shape_ref::ShapeRef::Record(
                lp_data::shape::shape_ref::RecordShapeRef::Static(&#schema_const_ident)
            );

        impl lp_data::LpDescribe for #struct_ident #where_clause {
            const TYPE_NAME: &'static str = #type_name_lit;

            fn lp_schema() -> &'static lp_data::shape::shape_ref::ShapeRef {
                &#schema_shape_ref_ident
            }
        }

        #serde_assert
    };

    Ok(tokens)
}

fn build_field_expr(
    field_name: &str,
    docs: Option<String>,
    type_ref: TokenStream2,
) -> TokenStream2 {
    let field_name_lit = LitStr::new(field_name, Span::call_site());
    let base = quote! { lp_data::shape::record::RecordField::new(#field_name_lit, #type_ref) };
    if let Some(doc) = docs {
        let doc_lit = LitStr::new(&doc, Span::call_site());
        quote! { #base.with_docs(#doc_lit) }
    } else {
        base
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

#[derive(Default, Clone)]
struct StructAttrs {
    name: Option<String>,
    type_name_override: Option<String>,
    docs: Option<String>,
    enum_ui: Option<EnumUiAttr>,
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
                            return Ok(());
                        }
                        if schema_meta.path.is_ident("type_name") {
                            let value: LitStr = schema_meta.value()?.parse()?;
                            result.type_name_override = Some(value.value());
                            return Ok(());
                        }
                        if schema_meta.path.is_ident("docs") {
                            let value: LitStr = schema_meta.value()?.parse()?;
                            result.docs = Some(value.value());
                            return Ok(());
                        }
                        if schema_meta.path.is_ident("ui") {
                            result.enum_ui = Some(parse_enum_ui(schema_meta)?);
                            return Ok(());
                        }
                        Err(schema_meta
                            .error("supported schema keys are name, type_name, docs, and ui"))
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

#[derive(Clone)]
enum EnumUiAttr {
    Dropdown,
    SegmentedControl,
}

#[derive(Default, Clone)]
struct FieldAttrs {
    ui: Option<UiAttr>,
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
                        if field_meta.path.is_ident("ui") {
                            if result.ui.is_some() {
                                return Err(
                                    field_meta.error("ui directive specified more than once")
                                );
                            }
                            result.ui = Some(parse_ui_from_meta(field_meta)?);
                            Ok(())
                        } else if field_meta.path.is_ident("docs") {
                            let value: LitStr = field_meta.value()?.parse()?;
                            result.docs = Some(value.value());
                            Ok(())
                        } else {
                            Err(field_meta
                                .error("supported field directives are ui(...) and docs = \"...\""))
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

#[derive(Clone, Debug)]
enum UiAttr {
    NumberSlider {
        min: TokenStream2,
        max: TokenStream2,
        step: Option<TokenStream2>,
    },
    NumberTextbox,
    StringMultiline,
    StringSingleLine,
    VectorRaw,
    VectorPosition,
    VectorColor,
    BoolToggle,
    BoolCheckbox,
}

fn parse_enum_ui(meta: ParseNestedMeta) -> Result<EnumUiAttr, Error> {
    let value: LitStr = meta.value()?.parse()?;
    match value.value().as_str() {
        "dropdown" => Ok(EnumUiAttr::Dropdown),
        "segmented" | "segmented_control" => Ok(EnumUiAttr::SegmentedControl),
        other => Err(meta.error(format!(
            "unknown enum ui `{other}`; expected `dropdown` or `segmented`"
        ))),
    }
}

fn parse_ui_from_meta(meta: ParseNestedMeta) -> Result<UiAttr, Error> {
    // Try to parse nested content (e.g., ui(slider(...)) or ui(textbox))
    let mut result: Option<UiAttr> = None;
    let parse_result = meta.parse_nested_meta(|nested| {
        if nested.path.is_ident("slider") {
            result = Some(parse_slider_from_meta(nested)?);
            Ok(())
        } else if nested.path.is_ident("textbox") {
            result = Some(UiAttr::NumberTextbox);
            Ok(())
        } else if nested.path.is_ident("single_line") {
            result = Some(UiAttr::StringSingleLine);
            Ok(())
        } else if nested.path.is_ident("multiline") {
            result = Some(UiAttr::StringMultiline);
            Ok(())
        } else if nested.path.is_ident("raw") {
            result = Some(UiAttr::VectorRaw);
            Ok(())
        } else if nested.path.is_ident("position") {
            result = Some(UiAttr::VectorPosition);
            Ok(())
        } else if nested.path.is_ident("color") {
            result = Some(UiAttr::VectorColor);
            Ok(())
        } else if nested.path.is_ident("toggle") {
            result = Some(UiAttr::BoolToggle);
            Ok(())
        } else if nested.path.is_ident("checkbox") {
            result = Some(UiAttr::BoolCheckbox);
            Ok(())
        } else {
            Err(nested.error(
                "unknown ui option; expected textbox, single_line, multiline, raw, position, color, toggle, checkbox, or slider(...)",
            ))
        }
    });

    if let Some(attr) = result {
        Ok(attr)
    } else {
        parse_result?;
        Err(meta.error("failed to parse ui attribute"))
    }
}

fn parse_slider_from_meta(meta: ParseNestedMeta) -> Result<UiAttr, Error> {
    let mut min = None;
    let mut max = None;
    let mut step = None;

    meta.parse_nested_meta(|slider_meta| {
        if slider_meta.path.is_ident("min") {
            let value: syn::Lit = slider_meta.value()?.parse()?;
            let tokens = match value {
                syn::Lit::Int(int) => quote! { #int },
                syn::Lit::Float(float) => quote! { #float },
                _ => return Err(slider_meta.error("slider min must be a numeric literal")),
            };
            min = Some(tokens);
            Ok(())
        } else if slider_meta.path.is_ident("max") {
            let value: syn::Lit = slider_meta.value()?.parse()?;
            let tokens = match value {
                syn::Lit::Int(int) => quote! { #int },
                syn::Lit::Float(float) => quote! { #float },
                _ => return Err(slider_meta.error("slider max must be a numeric literal")),
            };
            max = Some(tokens);
            Ok(())
        } else if slider_meta.path.is_ident("step") {
            let value: syn::Lit = slider_meta.value()?.parse()?;
            let tokens = match value {
                syn::Lit::Int(int) => quote! { #int },
                syn::Lit::Float(float) => quote! { #float },
                _ => return Err(slider_meta.error("slider step must be a numeric literal")),
            };
            step = Some(tokens);
            Ok(())
        } else {
            Err(slider_meta.error("slider supports min, max, and optional step"))
        }
    })?;

    let Some(min) = min else {
        return Err(meta.error("slider requires a min value"));
    };
    let Some(max) = max else {
        return Err(meta.error("slider requires a max value"));
    };

    Ok(UiAttr::NumberSlider { min, max, step })
}

struct TypeTokens {
    helpers: Vec<TokenStream2>,
    type_ref: TokenStream2,
    bounds: Vec<TokenStream2>,
}

impl TypeTokens {
    fn for_field(
        struct_ident: &Ident,
        field_ident: &Ident,
        ty: &Type,
        ui: Option<UiAttr>,
    ) -> Result<Self, Error> {
        match classify_type(ty)? {
            TypeInfo::Primitive(kind) => {
                let shape_ident = format_ident!(
                    "__LP_SCHEMA_{}_{}_SHAPE",
                    struct_ident,
                    field_ident.to_string().to_uppercase()
                );
                let shape_ref_ident = format_ident!(
                    "__LP_SCHEMA_{}_{}_SHAPE_REF",
                    struct_ident,
                    field_ident.to_string().to_uppercase()
                );
                let (shape_type, shape_expr, variant_name, shape_ref_type) =
                    primitive_shape_expr(kind, &ui, ty.span())?;
                let helper = quote! {
                    static #shape_ident: #shape_type = #shape_expr;
                    static #shape_ref_ident: lp_data::shape::shape_ref::ShapeRef =
                        lp_data::shape::shape_ref::ShapeRef::#variant_name(
                            lp_data::shape::shape_ref::#shape_ref_type::Static(&#shape_ident)
                        );
                };
                Ok(TypeTokens {
                    helpers: vec![helper],
                    type_ref: quote! { &#shape_ref_ident },
                    bounds: Vec::new(),
                })
            }
            TypeInfo::Vector(kind) => {
                let const_ident = format_ident!(
                    "__LP_SCHEMA_{}_{}_TYPE",
                    struct_ident,
                    field_ident.to_string().to_uppercase()
                );
                let helper_expr = vector_meta_expr(kind, &ui, ty.span())?;
                let helper = quote! {
                    const #const_ident: lp_data::LpTypeMeta = #helper_expr;
                };
                Ok(TypeTokens {
                    helpers: vec![helper],
                    type_ref: quote! { &#const_ident },
                    bounds: Vec::new(),
                })
            }
            TypeInfo::Bool => {
                let shape_ident = format_ident!(
                    "__LP_SCHEMA_{}_{}_SHAPE",
                    struct_ident,
                    field_ident.to_string().to_uppercase()
                );
                let shape_ref_ident = format_ident!(
                    "__LP_SCHEMA_{}_{}_SHAPE_REF",
                    struct_ident,
                    field_ident.to_string().to_uppercase()
                );
                let (shape_type, shape_expr) = bool_shape_expr(&ui, ty.span())?;
                let helper = quote! {
                    static #shape_ident: #shape_type = #shape_expr;
                    static #shape_ref_ident: lp_data::shape::shape_ref::ShapeRef =
                        lp_data::shape::shape_ref::ShapeRef::Bool(
                            lp_data::shape::shape_ref::BoolShapeRef::Static(&#shape_ident)
                        );
                };
                Ok(TypeTokens {
                    helpers: vec![helper],
                    type_ref: quote! { &#shape_ref_ident },
                    bounds: Vec::new(),
                })
            }
            TypeInfo::Array(element_ty) => {
                let element_tokens =
                    TypeTokens::for_nested(struct_ident, field_ident, *element_ty, ui)?;
                let array_shape_ident = format_ident!(
                    "__LP_SCHEMA_{}_{}_ARRAY_SHAPE",
                    struct_ident,
                    field_ident.to_string().to_uppercase()
                );
                let array_shape_ref_ident = format_ident!(
                    "__LP_SCHEMA_{}_{}_ARRAY_SHAPE_REF",
                    struct_ident,
                    field_ident.to_string().to_uppercase()
                );
                let element_shape_ref = element_tokens.type_ref;
                let array_helper = quote! {
                    static #array_shape_ident: lp_data::shape::array::StaticArrayShape =
                        lp_data::shape::array::StaticArrayShape {
                            element: #element_shape_ref,
                            ui: lp_data::shape::array::array_meta::ArrayUi::List,
                        };
                    static #array_shape_ref_ident: lp_data::shape::shape_ref::ShapeRef =
                        lp_data::shape::shape_ref::ShapeRef::Array(
                            lp_data::shape::shape_ref::ArrayShapeRef::Static(&#array_shape_ident)
                        );
                };
                let mut helpers = element_tokens.helpers;
                helpers.push(array_helper);
                let bounds = element_tokens.bounds;
                Ok(TypeTokens {
                    helpers,
                    type_ref: quote! { &#array_shape_ref_ident },
                    bounds,
                })
            }
            TypeInfo::Describe(path) => {
                // For types that implement LpDescribe, we can reference the static ShapeRef directly
                let mut bounds = Vec::new();
                let path = path.as_ref();
                bounds.push(quote! { #path: lp_data::LpDescribe });

                // Check if this is a same-crate type (simple ident, not a path)
                let type_ref = if let Type::Path(type_path) = path {
                    if type_path.path.segments.len() == 1 {
                        // Same-crate type - reference the static ShapeRef directly
                        let type_ident = &type_path.path.segments[0].ident;
                        let shape_ref_name = format_ident!(
                            "__LP_SCHEMA_{}_SHAPE_REF",
                            type_ident.to_string().to_uppercase()
                        );
                        quote! { #shape_ref_name }
                    } else {
                        // External type - call lp_schema() which returns &'static ShapeRef
                        quote! { *<#path as lp_data::LpDescribe>::lp_schema() }
                    }
                } else {
                    // Not a TypePath - fall back to lp_schema()
                    quote! { *<#path as lp_data::LpDescribe>::lp_schema() }
                };

                Ok(TypeTokens {
                    helpers: Vec::new(),
                    type_ref,
                    bounds,
                })
            }
        }
    }

    fn for_nested(
        struct_ident: &Ident,
        field_ident: &Ident,
        info: TypeInfo,
        ui: Option<UiAttr>,
    ) -> Result<Self, Error> {
        match info {
            TypeInfo::Primitive(kind) => {
                let shape_ident = format_ident!(
                    "__LP_SCHEMA_{}_{}_ELEMENT_SHAPE",
                    struct_ident,
                    field_ident.to_string().to_uppercase()
                );
                let shape_ref_ident = format_ident!(
                    "__LP_SCHEMA_{}_{}_ELEMENT_SHAPE_REF",
                    struct_ident,
                    field_ident.to_string().to_uppercase()
                );
                if ui.is_some() {
                    return Err(Error::new(
                        field_ident.span(),
                        "array element UI cannot be overridden at the array level",
                    ));
                }
                let (shape_type, shape_expr, variant_name, shape_ref_type) =
                    primitive_shape_expr(kind, &None, field_ident.span())?;
                let helper = quote! {
                    static #shape_ident: #shape_type = #shape_expr;
                    static #shape_ref_ident: lp_data::shape::shape_ref::ShapeRef =
                        lp_data::shape::shape_ref::ShapeRef::#variant_name(
                            lp_data::shape::shape_ref::#shape_ref_type::Static(&#shape_ident)
                        );
                };
                Ok(TypeTokens {
                    helpers: vec![helper],
                    type_ref: quote! { &#shape_ref_ident },
                    bounds: Vec::new(),
                })
            }
            TypeInfo::Vector(kind) => {
                let shape_ident = format_ident!(
                    "__LP_SCHEMA_{}_{}_ELEMENT_SHAPE",
                    struct_ident,
                    field_ident.to_string().to_uppercase()
                );
                let shape_ref_ident = format_ident!(
                    "__LP_SCHEMA_{}_{}_ELEMENT_SHAPE_REF",
                    struct_ident,
                    field_ident.to_string().to_uppercase()
                );
                if ui.is_some() {
                    return Err(Error::new(
                        field_ident.span(),
                        "array element UI cannot be overridden at the array level",
                    ));
                }
                let (shape_type, shape_expr, variant_name, shape_ref_type) =
                    vector_shape_expr(kind, &None, field_ident.span())?;
                let helper = quote! {
                    static #shape_ident: #shape_type = #shape_expr;
                    static #shape_ref_ident: lp_data::shape::shape_ref::ShapeRef =
                        lp_data::shape::shape_ref::ShapeRef::#variant_name(
                            lp_data::shape::shape_ref::#shape_ref_type::Static(&#shape_ident)
                        );
                };
                Ok(TypeTokens {
                    helpers: vec![helper],
                    type_ref: quote! { &#shape_ref_ident },
                    bounds: Vec::new(),
                })
            }
            TypeInfo::Bool => {
                let shape_ident = format_ident!(
                    "__LP_SCHEMA_{}_{}_ELEMENT_SHAPE",
                    struct_ident,
                    field_ident.to_string().to_uppercase()
                );
                let shape_ref_ident = format_ident!(
                    "__LP_SCHEMA_{}_{}_ELEMENT_SHAPE_REF",
                    struct_ident,
                    field_ident.to_string().to_uppercase()
                );
                if ui.is_some() {
                    return Err(Error::new(
                        field_ident.span(),
                        "array element UI cannot be overridden at the array level",
                    ));
                }
                let (shape_type, shape_expr) = bool_shape_expr(&None, field_ident.span())?;
                let helper = quote! {
                    static #shape_ident: #shape_type = #shape_expr;
                    static #shape_ref_ident: lp_data::shape::shape_ref::ShapeRef =
                        lp_data::shape::shape_ref::ShapeRef::Bool(
                            lp_data::shape::shape_ref::BoolShapeRef::Static(&#shape_ident)
                        );
                };
                Ok(TypeTokens {
                    helpers: vec![helper],
                    type_ref: quote! { &#shape_ref_ident },
                    bounds: Vec::new(),
                })
            }
            TypeInfo::Array(_) => Err(Error::new(
                field_ident.span(),
                "nested arrays are not currently supported",
            )),
            TypeInfo::Describe(path) => {
                // For same-crate types, reference the static ShapeRef directly
                let path = path.as_ref();
                let type_ref = if let Type::Path(type_path) = path {
                    if type_path.path.segments.len() == 1 {
                        let type_ident = &type_path.path.segments[0].ident;
                        let shape_ref_name = format_ident!(
                            "__LP_SCHEMA_{}_SHAPE_REF",
                            type_ident.to_string().to_uppercase()
                        );
                        quote! { #shape_ref_name }
                    } else {
                        quote! { *<#path as lp_data::LpDescribe>::lp_schema() }
                    }
                } else {
                    quote! { *<#path as lp_data::LpDescribe>::lp_schema() }
                };

                Ok(TypeTokens {
                    helpers: Vec::new(),
                    type_ref,
                    bounds: vec![quote! { #path: lp_data::LpDescribe }],
                })
            }
        }
    }
}

#[derive(Clone)]
enum TypeInfo {
    Primitive(PrimitiveKind),
    Vector(VectorKind),
    Bool,
    Array(Box<TypeInfo>),
    Describe(Box<Type>),
}

#[derive(Clone, Copy)]
enum PrimitiveKind {
    String,
    Int32,
    Dec32,
}

#[derive(Clone, Copy)]
enum VectorKind {
    Vec2,
    Vec3,
    Vec4,
}

fn classify_type(ty: &Type) -> Result<TypeInfo, Error> {
    match ty {
        Type::Path(path) => {
            if path.path.is_ident("String") {
                Ok(TypeInfo::Primitive(PrimitiveKind::String))
            } else if path.path.is_ident("i32") {
                Ok(TypeInfo::Primitive(PrimitiveKind::Int32))
            } else if path.path.is_ident("bool") {
                Ok(TypeInfo::Bool)
            } else if is_dec32(path) {
                Ok(TypeInfo::Primitive(PrimitiveKind::Dec32))
            } else if is_lp_int32(path) {
                // LpInt32 is a newtype wrapper around i32, treat it as i32
                Ok(TypeInfo::Primitive(PrimitiveKind::Int32))
            } else if is_lp_vec2(path) {
                // LpVec2 is a newtype wrapper around Vec2, treat it as Vec2
                Ok(TypeInfo::Vector(VectorKind::Vec2))
            } else if is_lp_vec3(path) {
                // LpVec3 is a newtype wrapper around Vec3, treat it as Vec3
                Ok(TypeInfo::Vector(VectorKind::Vec3))
            } else if is_lp_vec4(path) {
                // LpVec4 is a newtype wrapper around Vec4, treat it as Vec4
                Ok(TypeInfo::Vector(VectorKind::Vec4))
            } else if is_vec2(path) {
                Ok(TypeInfo::Vector(VectorKind::Vec2))
            } else if is_vec3(path) {
                Ok(TypeInfo::Vector(VectorKind::Vec3))
            } else if is_vec4(path) {
                Ok(TypeInfo::Vector(VectorKind::Vec4))
            } else if let Some(elem) = extract_vec_element(path) {
                let inner = classify_type(&elem)?;
                Ok(TypeInfo::Array(Box::new(inner)))
            } else {
                Ok(TypeInfo::Describe(Box::new(Type::Path(path.clone()))))
            }
        }
        _ => Err(Error::new(
            ty.span(),
            "unsupported type; expected primitives, vectors, Vec<T>, or other LpDescribe types",
        )),
    }
}

fn is_lp_int32(path: &TypePath) -> bool {
    // Check for lp_data::shape::int32::LpInt32 or just LpInt32
    if path.path.segments.len() == 1 {
        path.path.segments[0].ident == "LpInt32"
    } else if path.path.segments.len() == 4 {
        // lp_data::shape::int32::LpInt32
        path.path.segments[0].ident == "lp_data"
            && path.path.segments[1].ident == "shape"
            && path.path.segments[2].ident == "int32"
            && path.path.segments[3].ident == "LpInt32"
    } else {
        false
    }
}

fn is_lp_vec2(path: &TypePath) -> bool {
    // Check for lp_data::shape::vec2::LpVec2 or just LpVec2
    if path.path.segments.len() == 1 {
        path.path.segments[0].ident == "LpVec2"
    } else if path.path.segments.len() == 4 {
        // lp_data::shape::vec2::LpVec2
        path.path.segments[0].ident == "lp_data"
            && path.path.segments[1].ident == "shape"
            && path.path.segments[2].ident == "vec2"
            && path.path.segments[3].ident == "LpVec2"
    } else {
        false
    }
}

fn is_lp_vec3(path: &TypePath) -> bool {
    // Check for lp_data::shape::vec3::LpVec3 or just LpVec3
    if path.path.segments.len() == 1 {
        path.path.segments[0].ident == "LpVec3"
    } else if path.path.segments.len() == 4 {
        // lp_data::shape::vec3::LpVec3
        path.path.segments[0].ident == "lp_data"
            && path.path.segments[1].ident == "shape"
            && path.path.segments[2].ident == "vec3"
            && path.path.segments[3].ident == "LpVec3"
    } else {
        false
    }
}

fn is_lp_vec4(path: &TypePath) -> bool {
    // Check for lp_data::shape::vec4::LpVec4 or just LpVec4
    if path.path.segments.len() == 1 {
        path.path.segments[0].ident == "LpVec4"
    } else if path.path.segments.len() == 4 {
        // lp_data::shape::vec4::LpVec4
        path.path.segments[0].ident == "lp_data"
            && path.path.segments[1].ident == "shape"
            && path.path.segments[2].ident == "vec4"
            && path.path.segments[3].ident == "LpVec4"
    } else {
        false
    }
}

fn is_dec32(path: &TypePath) -> bool {
    path.path
        .segments
        .last()
        .map(|seg| seg.ident == "Dec32")
        .unwrap_or(false)
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

fn primitive_shape_expr(
    kind: PrimitiveKind,
    ui: &Option<UiAttr>,
    span: Span,
) -> Result<(TokenStream2, TokenStream2, TokenStream2, TokenStream2), Error> {
    match kind {
        PrimitiveKind::String => {
            let shape_type = quote! { lp_data::shape::string::StaticStringShape };
            let shape_expr = match ui {
                None | Some(UiAttr::StringSingleLine) | Some(UiAttr::NumberTextbox) => {
                    quote! { lp_data::shape::string::StaticStringShape::default() }
                }
                Some(UiAttr::StringMultiline) => {
                    quote! { lp_data::shape::string::StaticStringShape::new(lp_data::shape::string::string_meta::StringUi::MultiLine) }
                }
                Some(other) => {
                    return Err(Error::new(
                        span,
                        format!(
                            "ui {:?} not valid for string fields",
                            other_variant_name(other)
                        ),
                    ));
                }
            };
            Ok((
                shape_type,
                shape_expr,
                quote! { String },
                quote! { StringShapeRef },
            ))
        }
        PrimitiveKind::Int32 => {
            let shape_type = quote! { lp_data::shape::int32::StaticInt32Shape };
            let shape_expr = match ui {
                None | Some(UiAttr::NumberTextbox) => {
                    quote! { lp_data::shape::int32::StaticInt32Shape::default() }
                }
                Some(UiAttr::NumberSlider { min, max, step }) => {
                    let _ = step; // step is parsed but not yet used in shape generation
                    let slider_ui = quote! { lp_data::shape::int32::int32_meta::Int32Ui::Slider { min: (#min) as i32, max: (#max) as i32 } };
                    quote! { lp_data::shape::int32::StaticInt32Shape::new(#slider_ui) }
                }
                Some(other) => {
                    return Err(Error::new(
                        span,
                        format!(
                            "ui {:?} not valid for numeric fields",
                            other_variant_name(other)
                        ),
                    ));
                }
            };
            Ok((
                shape_type,
                shape_expr,
                quote! { Int32 },
                quote! { Int32ShapeRef },
            ))
        }
        PrimitiveKind::Dec32 => {
            let shape_type = quote! { lp_data::shape::dec32::StaticDec32Shape };
            let shape_expr = match ui {
                None | Some(UiAttr::NumberTextbox) => {
                    quote! { lp_data::shape::dec32::StaticDec32Shape::default() }
                }
                Some(UiAttr::NumberSlider { min, max, step }) => {
                    let _ = step; // step is parsed but not yet used in shape generation
                    let slider_ui = quote! { lp_data::shape::dec32::dec32_meta::Dec32Ui::Slider { min: (#min) as i32, max: (#max) as i32 } };
                    quote! { lp_data::shape::dec32::StaticDec32Shape::new(#slider_ui) }
                }
                Some(other) => {
                    return Err(Error::new(
                        span,
                        format!(
                            "ui {:?} not valid for numeric fields",
                            other_variant_name(other)
                        ),
                    ));
                }
            };
            Ok((
                shape_type,
                shape_expr,
                quote! { Dec32 },
                quote! { Dec32ShapeRef },
            ))
        }
    }
}

fn vector_shape_expr(
    kind: VectorKind,
    ui: &Option<UiAttr>,
    span: Span,
) -> Result<(TokenStream2, TokenStream2, TokenStream2, TokenStream2), Error> {
    match kind {
        VectorKind::Vec2 => {
            let shape_type = quote! { lp_data::shape::vec2::StaticVec2Shape };
            let shape_expr = match ui {
                None | Some(UiAttr::VectorRaw) | Some(UiAttr::NumberTextbox) => {
                    quote! { lp_data::shape::vec2::StaticVec2Shape::default() }
                }
                Some(UiAttr::VectorPosition) => {
                    quote! { lp_data::shape::vec2::StaticVec2Shape::new(lp_data::shape::vec2::vec2_meta::Vec2Ui::Position) }
                }
                Some(other) => {
                    return Err(Error::new(
                        span,
                        format!(
                            "ui {:?} not valid for vec2 fields",
                            other_variant_name(other)
                        ),
                    ));
                }
            };
            Ok((
                shape_type,
                shape_expr,
                quote! { Vec2 },
                quote! { Vec2ShapeRef },
            ))
        }
        VectorKind::Vec3 => {
            let shape_type = quote! { lp_data::shape::vec3::StaticVec3Shape };
            let shape_expr = match ui {
                None | Some(UiAttr::VectorRaw) | Some(UiAttr::NumberTextbox) => {
                    quote! { lp_data::shape::vec3::StaticVec3Shape::default() }
                }
                Some(UiAttr::VectorColor) => {
                    quote! { lp_data::shape::vec3::StaticVec3Shape::new(lp_data::shape::vec3::vec3_meta::Vec3Ui::Color) }
                }
                Some(other) => {
                    return Err(Error::new(
                        span,
                        format!(
                            "ui {:?} not valid for vec3 fields",
                            other_variant_name(other)
                        ),
                    ));
                }
            };
            Ok((
                shape_type,
                shape_expr,
                quote! { Vec3 },
                quote! { Vec3ShapeRef },
            ))
        }
        VectorKind::Vec4 => {
            let shape_type = quote! { lp_data::shape::vec4::StaticVec4Shape };
            let shape_expr = match ui {
                None | Some(UiAttr::VectorRaw) | Some(UiAttr::NumberTextbox) => {
                    quote! { lp_data::shape::vec4::StaticVec4Shape::default() }
                }
                Some(other) => {
                    return Err(Error::new(
                        span,
                        format!(
                            "ui {:?} not valid for vec4 fields",
                            other_variant_name(other)
                        ),
                    ));
                }
            };
            Ok((
                shape_type,
                shape_expr,
                quote! { Vec4 },
                quote! { Vec4ShapeRef },
            ))
        }
    }
}

fn bool_shape_expr(ui: &Option<UiAttr>, span: Span) -> Result<(TokenStream2, TokenStream2), Error> {
    let shape_type = quote! { lp_data::shape::bool::StaticBoolShape };
    let shape_expr = match ui {
        None | Some(UiAttr::BoolCheckbox) | Some(UiAttr::NumberTextbox) => {
            quote! { lp_data::shape::bool::StaticBoolShape::default() }
        }
        Some(UiAttr::BoolToggle) => {
            quote! { lp_data::shape::bool::StaticBoolShape::new(lp_data::shape::bool::bool_meta::BoolUi::Toggle) }
        }
        Some(other) => {
            return Err(Error::new(
                span,
                format!(
                    "ui {:?} not valid for bool fields",
                    other_variant_name(other)
                ),
            ));
        }
    };
    Ok((shape_type, shape_expr))
}

fn vector_meta_expr(
    kind: VectorKind,
    ui: &Option<UiAttr>,
    span: Span,
) -> Result<TokenStream2, Error> {
    match kind {
        VectorKind::Vec2 => match ui {
            None | Some(UiAttr::VectorRaw) | Some(UiAttr::NumberTextbox) => {
                Ok(quote! { lp_data::LpTypeMeta::new(lp_data::LpType::vec2()) })
            }
            Some(UiAttr::VectorPosition) => Ok(quote! {
                lp_data::LpTypeMeta::new(lp_data::LpType::Vec2(lp_data::Vec2Type::position()))
            }),
            Some(other) => Err(Error::new(
                span,
                format!(
                    "ui {:?} not valid for vec2 fields",
                    other_variant_name(other)
                ),
            )),
        },
        VectorKind::Vec3 => match ui {
            None | Some(UiAttr::VectorRaw) | Some(UiAttr::NumberTextbox) => {
                Ok(quote! { lp_data::LpTypeMeta::new(lp_data::LpType::vec3()) })
            }
            Some(UiAttr::VectorColor) => Ok(quote! {
                lp_data::LpTypeMeta::new(lp_data::LpType::Vec3(lp_data::Vec3Type::color()))
            }),
            Some(other) => Err(Error::new(
                span,
                format!(
                    "ui {:?} not valid for vec3 fields",
                    other_variant_name(other)
                ),
            )),
        },
        VectorKind::Vec4 => match ui {
            None | Some(UiAttr::VectorRaw) | Some(UiAttr::NumberTextbox) => {
                Ok(quote! { lp_data::LpTypeMeta::new(lp_data::LpType::vec4()) })
            }
            Some(other) => Err(Error::new(
                span,
                format!(
                    "ui {:?} not valid for vec4 fields",
                    other_variant_name(other)
                ),
            )),
        },
    }
}

fn other_variant_name(attr: &UiAttr) -> &'static str {
    match attr {
        UiAttr::NumberSlider { .. } => "slider",
        UiAttr::NumberTextbox => "textbox",
        UiAttr::StringMultiline => "multiline",
        UiAttr::StringSingleLine => "single_line",
        UiAttr::VectorPosition => "position",
        UiAttr::VectorRaw => "raw",
        UiAttr::VectorColor => "color",
        UiAttr::BoolToggle => "toggle",
        UiAttr::BoolCheckbox => "checkbox",
    }
}
