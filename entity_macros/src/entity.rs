use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::*;

pub fn generate(struct_ast: ItemStruct, container: Option<Ident>) -> Result<TokenStream> {
    let vis = &struct_ast.vis;
    let ident = &struct_ast.ident;

    let fields = match struct_ast.fields {
        Fields::Named(FieldsNamed { named, .. } ) => Ok(named),
        _ => Err(Error::new(struct_ast.struct_token.span, "generate entity requires a struct with named fields"))
    }?.into_iter();

    let id_ident = Ident::new(&format!("{}Id", ident), Span::call_site());

    let has_container = container.is_some();

    let animator = if has_container {
        quote!{ animator: Option<Animator>, }
    } else {
        quote!{}
    };

    let struct_gen = quote! {
        #[derive(Copy, Clone, Debug, Default, ImDraw)]
        #vis struct #ident {
            id: #id_ident,
            entity: Entity,
            #animator
            #(#fields),*
        }

        impl IsEntity for #ident {
            type IdType = #id_ident;

            fn new(id: Self::IdType, entity: Entity) -> Self {
                let mut elem = Self::default();
                elem.id = id;
                elem.entity = entity;
                elem
            }

            fn new_animated(id: Self::IdType, entity: Entity, animation_set: AnimationSet) -> Self {
                let mut elem = Self::default();
                elem.id = id;
                elem.entity = entity;
                elem.animator = Some(Animator::new(animation_set));
                elem
            }

            fn id(&self) -> #id_ident { self.id }
            fn entity(&self) -> &Entity { &self.entity }
            fn entity_mut(&mut self) -> &mut Entity { &mut self.entity }
        }

        #[derive(Copy, Clone, Debug, Default, ImDraw)]
        #vis struct #id_ident(Id);

        impl IsId for #id_ident {
            fn new(id: Id) -> Self { Self(id) }
            fn id(self) -> Id { self.0 }
        }
    };

    let container_gen = match container {
        Some(container_ident) => quote! {
            impl EntityAccess for <#ident as IsEntity>::IdType {
                type EntityType = #ident;

                fn create_entity(
                    containers: &mut EntityContainers,
                    transform: Transform,
                    sprite: Sprite
                ) -> <Self::EntityType as IsEntity>::IdType {
                    containers.#container_ident.create_entity(transform, sprite)
                }

                fn create_entity_animated(
                    containers: &mut EntityContainers,
                    transform: Transform,
                    animation_set: AnimationSet
                ) -> <Self::EntityType as IsEntity>::IdType {
                    containers.#container_ident.create_entity_animated(transform, animation_set)
                }

                fn destroy_entity(self, containers: &mut EntityContainers) {
                    containers.#container_ident.destroy_entity(self);
                }

                fn get_entity(self, containers: &EntityContainers) -> Option<&Self::EntityType> {
                    containers.#container_ident.get(self)
                }

                fn get_entity_mut(self, containers: &mut EntityContainers) -> Option<&mut Self::EntityType> {
                    containers.#container_ident.get_mut(self)
                }
            }
        },
        None => quote! {},
    };

    let animation_gen = if has_container {
        quote! {
            impl #ident
            where <#ident as IsEntity>::IdType: EntityAccess
            {
                pub fn play_animation(&mut self, app: &mut App<State>) {
                    self.schedule_animator_task(app);
                }

                pub fn stop_animation(&mut self, app: &mut App<State>) {
                    let animator = self.animator.as_mut().expect("[stop animation] animation not playing");
                    animator.stop(app);
                }

                fn schedule_animator_task(&mut self, app: &mut App<State>) {
                    self.update_sprite(app);

                    let entity_id = self.id();
                    let animator = self.animator.as_mut().expect("[schedule_animator_task] no animator");

                    if animator.is_playing() { animator.stop(app); }
                    animator.play(
                        app,
                        move |id, state, app| {
                            println!("animation task called: {}", id);
                            let entity = state.entity_containers.get_mut(entity_id)
                                .expect("[animator task] no entity with id");

                            let animator = entity.animator.as_mut()
                                .expect("[animator task] entity has no animator");
                            let new_sprite = animator.next_frame(app);

                            // End of animation
                            if new_sprite.is_none() {
                                animator.stop(app);
                                return;
                            }

                            entity.update_sprite(app);
                            entity.schedule_animator_task(app);
                        }
                    );
                }

                fn update_sprite(&mut self, app: &mut App<State>) {
                    let sprite = self.animator
                        .expect("[update_sprite] no animator")
                        .get_current_sprite(app);
                    self.entity_mut().sprite = sprite;
                }
            }
        }
    } else {
        quote!{}
    };

    Ok(
        quote! {
            #struct_gen
            #container_gen
            #animation_gen
        }.into()
    )
}
