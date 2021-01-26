pub mod animator;
pub mod container;
pub mod traits;
pub mod transform;

pub use animator::*;
pub use container::*;
pub use traits::*;
pub use transform::*;

use std::rc::Rc;
use std::cell::RefCell;

use super::{
    id_manager::{Id, IdManager},
    imgui::imdraw::ImDraw,
    renderer::Sprite,
};

#[derive(Clone, Debug)]
pub struct Entity {
    id: EntityId,
    container: EntityContainer,
}

impl Entity {
    // @TODO return result
    pub fn destroy(&self) {
        self.container.destroy_entity(&self.id);
    }

    pub fn is_active(&self) -> bool {
        match self.container.get_entity_state_by_id(&self.id) {
            EntityState::Visible |
            EntityState::Hidden => { return true; }
            _ => { return false; }
        }
    }

    pub fn set_active(&self, active: bool) {
        self.container.set_entity_active(&self.id, active);
    }
}

impl Renderable for Entity {
    fn is_visible(&self) -> bool {
        todo!();
    }

    fn set_visible(&self, visible: bool) {
        self.container.set_entity_visible(&self.id, visible);
    }

    fn sprite(&self) -> &Sprite {
        todo!();
    }

    fn sprite_mut(&mut self) -> &mut Sprite {
        todo!();
    }
}

// -------
// private
// -------

#[derive(Clone, Debug, ImDraw)]
pub(super) struct EntityId(Rc<RefCell<Option<Id>>>);

impl EntityId {
    fn new(id: Id) -> Self {
        Self(Rc::new(RefCell::new(Some(id))))
    }

    fn destroy(&mut self, id_manager: &mut IdManager) {
        let id = self.0.replace(None)
            .expect("trying to destroy already destroyed entity_id");
        id_manager.free(id);
    }
}

// ------
// ImDraw
// ------

impl ImDraw for Entity {
    fn imdraw(&mut self, label: &str, ui: &imgui::Ui) {
        // @TODO get the data from the container
        ui.text(format!("{}: {:?}", label, *self.id.0.borrow()));
    }
}
