mod schema;

use proc_macro::TokenStream;

/// Derive macro for LP schema types.
#[proc_macro_derive(LpSchema, attributes(lp))]
pub fn lp_schema_derive(input: TokenStream) -> TokenStream {
    schema::derive(input)
}
