use super::EntityContainers;
pub use crate::app::{
    animation_system::{Animator, AnimationSet},
    id_manager::IsId,
    imgui::imdraw::ImDraw,
    renderer::Sprite,
    transform::Transform,
};

#[derive(Copy, Clone, Debug, Default, ImDraw)]
pub struct Entity {
    pub transform: Transform,
    pub sprite: Sprite,
    pub is_active: bool,
    pub is_visible: bool,
}

pub trait IsEntity {
    //type IdType : IsId + EntityAccess;
    type IdType : IsId;

    fn new(_id: Self::IdType, _entity: Entity) -> Self;
    fn new_animated(_id: Self::IdType, _entity: Entity, _animation_set: AnimationSet) -> Self;
    fn id(&self) -> Self::IdType;
    fn entity(&self) -> &Entity;
    fn entity_mut(&mut self) -> &mut Entity;
}

pub trait EntityAccess {
    type EntityType : IsEntity;

    fn create_entity(
        _containers: &mut EntityContainers,
        _transform: Transform,
        _sprite: Sprite
    ) -> <Self::EntityType as IsEntity>::IdType;

    fn create_entity_animated(
        _containers: &mut EntityContainers,
        _transform: Transform,
        _animation_set: AnimationSet
    ) -> <Self::EntityType as IsEntity>::IdType;

    fn destroy_entity(self, _containers: &mut EntityContainers);
    fn get_entity(self, _containers: &EntityContainers) -> Option<&Self::EntityType>;
    fn get_entity_mut(self, _containers: &mut EntityContainers) -> Option<&mut Self::EntityType>;
}
