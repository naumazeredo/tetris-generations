extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::*;

#[proc_macro_derive(ImDraw)]
pub fn imdraw_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse(input).unwrap();
    let name = &ast.ident;

    let fields = match ast.data {
        Data::Struct(DataStruct { fields: Fields::Named(FieldsNamed { named: it, .. } ), .. })
        //| Struct(DataStruct { fields: Fields::Unnamed(FieldsUnnamed { unnamed: it, .. } ), .. })
        => it,

        _ => unimplemented!()
    };

    let expanded_fields = fields.into_iter().enumerate().map(|(index, field)| {
        match field.ident {
            Some(ident) => {
                quote! { self.#ident.imdraw(stringify!(#ident), ui); }
            },
            None => {
                quote! { self.#index.imdraw(stringify!(#index), ui); }
            }
        }
    });

    let gen = quote! {
        impl ImDraw for #name {
            fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
                imgui::TreeNode::new(crate::im_str2!(label)).build(ui, || {
                    let mut id = ui.push_id(label);
                    #(#expanded_fields ;)*
                    id.pop(ui);
                });
            }
        }
    };
    gen.into()
}
