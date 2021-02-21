// TODO
// [ ] carry attributes
// [ ] carry generics (+lifetimes)

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::*;

mod entity;
mod containers;

#[proc_macro_attribute]
pub fn gen_entity(container_input: TokenStream, input: TokenStream) -> TokenStream {
    let struct_ast: ItemStruct = parse(input).unwrap();

    let container: Option<Ident> = match parse(container_input) {
        Ok(ident) => Some(ident),
        Err(_) => None,
    };

    entity::generate(struct_ast, container)
        .unwrap_or_else(|err| err.to_compile_error().into())
        .into()
}

#[proc_macro_attribute]
pub fn gen_containers(_: TokenStream, input: TokenStream) -> TokenStream {
    let struct_ast: ItemStruct = parse(input).unwrap();

    containers::generate(struct_ast)
        .unwrap_or_else(|err| err.to_compile_error().into())
        .into()
}
