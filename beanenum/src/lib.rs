use proc_macro::TokenStream;

#[proc_macro_derive(FromIndexValue)]
pub fn derive_from_index_value(inputs: TokenStream) -> TokenStream {
    TokenStream()
}
