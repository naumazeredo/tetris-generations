use proc_macro::TokenStream;
use quote::quote;
use syn::*;

pub fn generate(struct_ast: ItemStruct) -> Result<TokenStream> {
    let vis = &struct_ast.vis;
    let ident = &struct_ast.ident;

    let fields = match struct_ast.fields {
        Fields::Named(FieldsNamed { named, .. } ) => Ok(named),
        _ => Err(Error::new(struct_ast.struct_token.span, "generate entity requires a struct with named fields"))
    }?.into_iter();

    let fields_news = fields.clone()
        .map(|field| {
            let ident = field.ident.unwrap();
            quote!{ #ident: EntityContainer::new(), }
        });

    let fields_render = fields.clone()
        .map(|field| {
            let ident = field.ident.unwrap();
            quote!{ self.#ident.render(renderer); }
        });

    let struct_gen = quote! {
        #[derive(ImDraw)]
        #vis struct EntityContainers {
            #(#fields)*
        }

        impl #ident {
            pub fn new() -> Self {
                Self {
                    #(#fields_news)*
                }
            }

            pub fn get<I: EntityAccess>(&self, id: I) -> Option<&I::EntityType> {
                id.get_entity(self)
            }

            pub fn get_mut<I: EntityAccess>(&mut self, id: I) -> Option<&mut I::EntityType> {
                id.get_entity_mut(self)
            }

            pub fn create<E: IsEntity>(
                &mut self,
                transform: Transform,
                sprite: Sprite
            ) -> <<E::IdType as EntityAccess>::EntityType as IsEntity>::IdType
            where
                E::IdType: EntityAccess
            {
                E::IdType::create_entity(self, transform, sprite)
            }

            pub fn create_animated<E: IsEntity>(
                &mut self,
                transform: Transform,
                animation_set: AnimationSet
            ) -> <<E::IdType as EntityAccess>::EntityType as IsEntity>::IdType
            where
                E::IdType: EntityAccess
            {
                E::IdType::create_entity_animated(self, transform, animation_set)
            }

            pub fn destroy<I: EntityAccess>(&mut self, id: I) {
                id.destroy_entity(self);
            }

            pub fn render(&self, renderer: &mut Renderer) {
                #(#fields_render)*
            }
        }
    };

    Ok(struct_gen.into())
}
