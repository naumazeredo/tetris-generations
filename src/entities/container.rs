// Entity Container

use crate::app::{
    animations::AnimationSet,
    id_manager::{IsId, IdGenerator},
    imgui::imdraw::ImDraw,
    renderer::{
        Renderer,
        Sprite,
        color,
        types::*,
    },
    transform::Transform,
};

use super::{
    Entity,
    IsEntity,
};

#[derive(Clone, Debug, ImDraw)]
pub struct EntityContainer<E: IsEntity + ImDraw> {
    // guid: u128,
    id_gen: IdGenerator,
    entities: Vec<Option<E>>,
}

impl<E: IsEntity + ImDraw> EntityContainer<E> {
    pub fn new() -> Self {
        Self {
            id_gen: IdGenerator::new(),
            entities: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.id_gen.len()
    }

    pub fn get(&self, entity_id: E::IdType) -> Option<&E> {
        self.entities[entity_id.id().index()].as_ref()
    }

    pub fn get_mut(&mut self, entity_id: E::IdType) -> Option<&mut E> {
        self.entities[entity_id.id().index()].as_mut()
    }

    pub fn create_entity(&mut self, transform: Transform, sprite: Sprite) -> E::IdType {
        let entity_id = E::IdType::new(self.id_gen.next());

        let entity = E::new(
            entity_id,
            Entity {
                transform,
                sprite,
                is_active: true,
                is_visible: true
            }
        );

        self.entities.push(Some(entity));
        entity_id
    }

    pub fn create_entity_animated(&mut self, transform: Transform, animation_set: AnimationSet) -> E::IdType {
        let entity_id = E::IdType::new(self.id_gen.next());

        let entity = E::new_animated(
            entity_id,
            Entity {
                transform,
                sprite: Sprite::default(),
                is_active: true,
                is_visible: true
            },
            animation_set
        );

        self.entities.push(Some(entity));
        entity_id
    }

    pub fn destroy_entity(&mut self, entity_id: E::IdType) {
        let id = entity_id.id();
        assert!(self.entities[id.index()].is_some());
        self.entities[id.index()].take();
        self.id_gen.free(id);
    }

    pub fn render(&self, renderer: &mut Renderer) {
        let visible_entities = self.entities.iter().flatten()
            .filter(|entity| entity.entity().is_visible)
            .map(|entity| entity.entity());

        for entity in visible_entities {
            renderer.queue_draw_sprite(
                0 as Program,
                color::WHITE,
                &entity.transform,
                &entity.sprite
            );
        }
    }
}
