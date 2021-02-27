pub mod entity;
pub mod container;

pub use entity::*;
pub use container::*;

use crate::State;
use crate::app::{
    App,
    animations::{Animator, AnimationSet},
    id_manager::Id,
    imgui::ImDraw,
    renderer::Renderer,
    transform::Transform,
};

use entity_macros::*;

#[gen_containers]
pub struct EntityContainers {
    pub my_entity_container: EntityContainer<MyEntity>,
}

#[gen_entity(my_entity_container)]
pub struct MyEntity {
    pub k: i32,
}