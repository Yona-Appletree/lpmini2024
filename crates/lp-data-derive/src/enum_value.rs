use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, spanned::Spanned, Attribute, Data, DataEnum, DeriveInput, Error, Ident,
    LitStr, Variant,
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
        Data::Enum(data) => expand_enum(input, data),
        Data::Struct(_) | Data::Union(_) => Err(Error::new(
            input.ident.span(),
            "EnumValue derive only supports enums",
        )),
    }
}

fn expand_enum(input: &DeriveInput, data: &DataEnum) -> Result<TokenStream2, Error> {
    if !input.generics.params.is_empty() {
        return Err(Error::new(
            input.generics.span(),
            "EnumValue derive does not yet support generic parameters",
        ));
    }

    let enum_ident = &input.ident;
    let enum_name = enum_ident.to_string();

    let struct_attrs = StructAttrs::from_attrs(&input.attrs)?;
    let struct_docs = merge_docs(
        extract_doc_comments(&input.attrs),
        struct_attrs.docs.clone(),
    );
    let enum_name_str = struct_attrs
        .name
        .clone()
        .unwrap_or_else(|| enum_name.clone());

    // Check that all variants are unit variants (no fields)
    for variant in &data.variants {
        if !matches!(variant.fields, syn::Fields::Unit) {
            return Err(Error::new(
                variant.span(),
                "EnumValue derive only supports unit variants (no fields). Use Union for variants with fields (future work).",
            ));
        }
    }

    // Generate shape constant
    let variants: Vec<_> = data.variants.iter().collect();
    let shape_constant =
        generate_enum_shape_static(enum_ident, &enum_name_str, struct_docs, &variants)?;

    let shape_const_ident =
        format_ident!("__LP_VALUE_{}_SHAPE", enum_ident.to_string().to_uppercase());

    // Generate variant_index implementation
    let mut variant_matchers = Vec::new();
    for (index, variant) in data.variants.iter().enumerate() {
        let variant_ident = &variant.ident;
        variant_matchers.push(quote! {
            #enum_ident::#variant_ident => #index,
        });
    }

    let tokens = quote! {
        #shape_constant

        impl crate::kind::value::LpValue for #enum_ident {
            fn shape(&self) -> &dyn crate::kind::shape::LpShape {
                &#shape_const_ident
            }
        }

        impl crate::kind::enum_::enum_value::EnumValue for #enum_ident {
            fn shape(&self) -> &dyn crate::kind::enum_::enum_shape::EnumShape {
                &#shape_const_ident
            }

            fn variant_index(&self) -> usize {
                match self {
                    #(#variant_matchers)*
                }
            }
        }
    };

    Ok(tokens)
}

fn generate_enum_shape_static(
    enum_ident: &Ident,
    enum_name: &str,
    enum_docs: Option<String>,
    variants: &[&Variant],
) -> Result<TokenStream2, Error> {
    let mut variant_exprs = Vec::new();

    for variant in variants {
        let variant_ident = &variant.ident;
        let variant_name = variant_ident.to_string();

        let variant_attrs = FieldAttrs::from_attrs(&variant.attrs)?;
        let variant_docs = merge_docs(
            extract_doc_comments(&variant.attrs),
            variant_attrs.docs.clone(),
        );

        let variant_meta = if let Some(docs) = variant_docs {
            let docs_lit = LitStr::new(&docs, Span::call_site());
            quote! {
                crate::kind::enum_::enum_meta::EnumVariantMetaStatic {
                    docs: Some(#docs_lit),
                }
            }
        } else {
            quote! {
                crate::kind::enum_::enum_meta::EnumVariantMetaStatic {
                    docs: None,
                }
            }
        };

        let variant_name_lit = LitStr::new(&variant_name, Span::call_site());
        variant_exprs.push(quote! {
            crate::kind::enum_::enum_static::EnumVariantStatic {
                name: #variant_name_lit,
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

    let enum_name_lit = LitStr::new(enum_name, Span::call_site());
    let enum_meta = if let Some(docs) = enum_docs {
        let docs_lit = LitStr::new(&docs, Span::call_site());
        quote! {
            crate::kind::enum_::enum_meta::EnumMetaStatic {
                name: #enum_name_lit,
                docs: Some(#docs_lit),
            }
        }
    } else {
        quote! {
            crate::kind::enum_::enum_meta::EnumMetaStatic {
                name: #enum_name_lit,
                docs: None,
            }
        }
    };

    let tokens = quote! {
        const #variants_const_ident: &'static [crate::kind::enum_::enum_static::EnumVariantStatic] = &[
            #(#variant_exprs),*
        ];

        const #shape_const_ident: crate::kind::enum_::enum_static::EnumShapeStatic = crate::kind::enum_::enum_static::EnumShapeStatic {
            meta: #enum_meta,
            variants: #variants_const_ident,
        };
    };

    Ok(tokens)
}

// Helper structs and functions (similar to record_value.rs)
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
}

impl FieldAttrs {
    fn from_attrs(attrs: &[Attribute]) -> Result<Self, Error> {
        let docs = extract_doc_comments(attrs);
        Ok(FieldAttrs { docs })
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
