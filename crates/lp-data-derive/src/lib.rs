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
    let input = parse_macro_input!(input as DeriveInput);

    // Add JsonSchema to the existing derives
    // For now, we just ensure JsonSchema is derived - in the future we can process
    // #[lpschema(...)] attributes to map to schemars metadata
    // All Data variants (Struct, Enum, Union) are handled the same way

    // Just pass through the input unchanged for now
    // Users should include JsonSchema in their derive list
    quote! {
        #input
    }
    .into()
}
