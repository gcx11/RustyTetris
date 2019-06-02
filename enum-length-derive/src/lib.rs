extern crate proc_macro;
use syn;
use quote::quote;

use proc_macro::TokenStream;

#[proc_macro_derive(EnumVariantCount)]
pub fn derive_enum_variant_count(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let len = match ast.data {
        syn::Data::Enum(enum_item) => { enum_item.variants.len() },
        _ => panic!("EnumVariantCount only works on Enums"),
    };

    let expanded = quote! {
impl #name {
    pub fn variant_count() -> usize {
        return #len
    }
}
};
    expanded.into()
}