mod lp_value;
mod record_value;
mod schema;

use proc_macro::TokenStream;

/// Derive macro for LP schema types.
#[proc_macro_derive(LpSchema, attributes(lp))]
pub fn lp_schema_derive(input: TokenStream) -> TokenStream {
    schema::derive(input)
}

/// Derive macro for LpValue trait.
#[proc_macro_derive(LpValue, attributes(lp))]
pub fn lp_value_derive(input: TokenStream) -> TokenStream {
    lp_value::derive(input)
}

/// Derive macro for RecordValue trait.
#[proc_macro_derive(RecordValue, attributes(lp))]
pub fn record_value_derive(input: TokenStream) -> TokenStream {
    record_value::derive(input)
}
