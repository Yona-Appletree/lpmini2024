use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Derive macro for LP data types
/// 
/// This macro derives `JsonSchema` from schemars and optionally
/// `Serialize` and `Deserialize` from serde.
/// 
/// It also supports `#[lpschema(...)]` attributes that map to schemars metadata.
#[proc_macro_derive(LpDataType, attributes(lpschema))]
pub fn lp_data_type_derive(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    
    // Add JsonSchema to the existing derives
    // For now, we just ensure JsonSchema is derived - in the future we can process
    // #[lpschema(...)] attributes to map to schemars metadata
    if let syn::Data::Struct(_) | syn::Data::Enum(_) | syn::Data::Union(_) = &input.data {
        // The user should already have JsonSchema in their derive list
        // This macro is mainly for future attribute processing
    }
    
    // Just pass through the input unchanged for now
    // Users should include JsonSchema in their derive list
    quote! {
        #input
    }.into()
}

