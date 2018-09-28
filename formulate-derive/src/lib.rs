extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(Formulate)]
pub fn formulate_derive(input: TokenStream) -> TokenStream {
    impl_formulate(&syn::parse(input).unwrap())
}

fn impl_formulate(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let data = match &ast.data {
        syn::Data::Struct(struct_data) => struct_data,
        _ => panic!("not a struct"),
    };
    let fields = match &data.fields {
        syn::Fields::Named(named_fields) => named_fields,
        _ => panic!("not named fields"),
    };
    let names: Vec<String> = fields
        .named
        .iter()
        .filter_map(|field| field.ident.clone().map(|ident| ident.to_string()))
        .collect();
    let gen = quote! {
        impl Formulate for #name {
            fn formulate() -> Vec<&'static str> {
                vec![#(#names),*]
            }
        }
    };
    gen.into()
}
