extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::*;

#[proc_macro_derive(ImDraw)]
pub fn imdraw_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse(input).unwrap();
    generate(ast)
}

fn generate(ast: DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let fields = match ast.data {
        | Data::Struct(DataStruct { fields: Fields::Named(FieldsNamed { named: it, .. } ), .. })
        | Data::Struct(DataStruct { fields: Fields::Unnamed(FieldsUnnamed { unnamed: it, .. } ), .. })
        => it,

        _ => unimplemented!()
    };

    let expanded_fields = fields.into_iter().enumerate().map(|(index, field)| {
        match field.ident {
            Some(ident) => {
                quote! { self.#ident.imdraw(stringify!(#ident), ui); }
            },
            None => {
                let field_index = syn::Index::from(index);
                let field_name = format!("[{}]", index);
                quote! {
                    self.#field_index.imdraw(#field_name, ui);
                }
            }
        }
    });

    let gen = quote! {
        impl #impl_generics ImDraw for #name #ty_generics #where_clause {
            fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
                imgui::TreeNode::new(crate::im_str2!(label)).build(ui, || {
                    let id = ui.push_id(label);
                    #(#expanded_fields)*
                    id.pop(ui);
                });
            }
        }
    };
    gen.into()
}
