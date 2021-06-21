//#![feature(trace_macros)]

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

    let gen = match ast.data {
        | Data::Struct(DataStruct { fields: Fields::Named(FieldsNamed { named: fields, .. } ), .. })
        | Data::Struct(DataStruct { fields: Fields::Unnamed(FieldsUnnamed { unnamed: fields, .. } ), .. })
        => {
            let expanded_fields = fields.into_iter().enumerate().map(|(index, field)| {
                match field.ident {
                    Some(ident) => {
                        quote! { self.#ident.imdraw(stringify!(#ident), ui); }
                    },
                    None => {
                        let field_index = syn::Index::from(index);
                        let field_name = format!(".{}", index);
                        quote! {
                            self.#field_index.imdraw(#field_name, ui);
                        }
                    }
                }
            });

            quote! {
                impl #impl_generics ImDraw for #name #ty_generics #where_clause {
                    fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
                        imgui::TreeNode::new(crate::im_str2!(label)).build(ui, || {
                            let id = ui.push_id(label);
                            #(#expanded_fields)*
                            id.pop(ui);
                        });
                    }
                }
            }
        }

        Data::Enum(DataEnum { variants, .. }) => {
            let expanded_variants = variants.into_iter().map(|variant| {
                let ident = variant.ident;
                match variant.fields {
                    Fields::Unit => {
                        quote! {
                            #name::#ident => {
                                ui.text(format!("{}: {}", label, stringify!(#ident)));
                            }
                        }
                    },

                    Fields::Unnamed(FieldsUnnamed { unnamed: fields, .. } ) => {
                        let (match_fields, fields_operations) : (Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>) =
                            fields
                            .into_iter()
                            .enumerate()
                            .map(|(index, field)| {
                                assert!(field.ident.is_none());
                                let field_indexed_name = format!(".{}", index);
                                let field_name: proc_macro2::TokenStream = format!("__self_{}", index).parse().unwrap();
                                let match_field = format!("ref mut __self_{},", index).parse().unwrap();

                                (
                                    match_field,
                                    quote! { #field_name.imdraw(#field_indexed_name, ui); }
                                )
                            })
                            .unzip();

                        quote! {
                            #name::#ident(#(#match_fields)*) => {
                                let display_label = format!("{}: {}", label, stringify!(#ident));
                                imgui::TreeNode::new(crate::im_str2!(display_label)).build(ui, || {
                                    let id = ui.push_id(label);
                                    #(#fields_operations)*
                                    id.pop(ui);
                                });
                            }
                        }
                    },

                    Fields::Named(FieldsNamed { named: fields, .. } ) => {
                        let (match_fields, fields_operations) : (Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>) =
                            fields
                            .into_iter()
                            .map(|field| {
                                assert!(field.ident.is_some());

                                let field_name = field.ident.unwrap();
                                (
                                    quote! { #field_name, },
                                    quote! { #field_name.imdraw(stringify!(#field_name), ui); }
                                )
                            })
                            .unzip();

                        quote! {
                            #name::#ident { #(#match_fields)* } => {
                                let display_label = format!("{}: {}", label, stringify!(#ident));
                                imgui::TreeNode::new(crate::im_str2!(display_label)).build(ui, || {
                                    let id = ui.push_id(label);
                                    #(#fields_operations)*
                                    id.pop(ui);
                                });
                            }
                        }
                    }
                }
            });

            quote! {
                impl #impl_generics ImDraw for #name #ty_generics #where_clause {
                    fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
                        match self {
                            #(#expanded_variants)*
                        }
                    }
                }
            }
        }

        _ => unimplemented!()
    };

    gen.into()
}
