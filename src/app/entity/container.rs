// Entity Container

// [ ] tests
// [ ] add cold data with generics
// [ ] add hot data with macro
// [ ] create iterator (and add transform(), sprites(), etc, to entity iterator item)
// [ ] refactor animation system

use std::{
    rc::Rc,
    cell::{RefCell, Ref, RefMut},
};

use super::super::{
    id_manager::IdManager,
    imgui::imdraw::ImDraw,
    renderer::{
        Renderer,
        Sprite,
        color,
        types::*,
    },
};

use super::{
    Entity,
    EntityId,
    transform::Transform,
};

#[derive(Clone, Debug, ImDraw)]
pub struct EntityContainer(Rc<RefCell<EntityContainerInner>>);

impl EntityContainer {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(EntityContainerInner::new())))
    }

    pub fn create_entity(&self, transform: Transform, sprite: Sprite) -> Entity {
        let mut inner = self.0.borrow_mut();

        let id = inner.id_manager.next();

        let index = inner.entities.len();
        let entity_id = EntityId::new(id);

        inner.entities.push(entity_id.clone());
        inner.transforms.push(transform);
        inner.sprites.push(sprite);
        inner.mapping.push(index);

        Entity {
            id: entity_id,
            container: self.clone(),
        }
    }

    pub fn render(&self, renderer: &mut Renderer) {
        let inner = self.0.borrow_mut();

        for i in 0..inner.visible_count {
            renderer.queue_draw_sprite(
                0 as Program,
                color::WHITE,
                &inner.transforms[i],
                &inner.sprites[i]
            );
        }
    }

    pub fn all(&self) -> EntityContainerGuard {
        let r = self.0.borrow();
        let start = 0;
        let end = r.transforms.len();

        EntityContainerGuard { r, start, end }
    }

    pub fn all_mut(&self) -> EntityContainerMutGuard {
        let r = self.0.borrow_mut();
        let start = 0;
        let end = r.transforms.len();

        EntityContainerMutGuard { r, start, end }
    }

    pub fn active(&self) -> EntityContainerGuard {
        let r = self.0.borrow();
        let start = 0;
        let end = r.active_count;

        EntityContainerGuard { r, start, end }
    }

    pub fn active_mut(&self) -> EntityContainerMutGuard {
        let r = self.0.borrow_mut();
        let start = 0;
        let end = r.active_count;

        EntityContainerMutGuard { r, start, end }
    }

    pub fn inactive(&self) -> EntityContainerGuard {
        let r = self.0.borrow();
        let start = r.active_count;
        let end = r.transforms.len();

        EntityContainerGuard { r, start, end }
    }

    pub fn inactive_mut(&self) -> EntityContainerMutGuard {
        let r = self.0.borrow_mut();
        let start = r.active_count;
        let end = r.transforms.len();

        EntityContainerMutGuard { r, start, end }
    }

    pub fn visible(&self) -> EntityContainerGuard {
        let r = self.0.borrow();
        let start = 0;
        let end = r.visible_count;

        EntityContainerGuard { r, start, end }
    }

    pub fn visible_mut(&self) -> EntityContainerMutGuard {
        let r = self.0.borrow_mut();
        let start = 0;
        let end = r.visible_count;

        EntityContainerMutGuard { r, start, end }
    }

    pub fn hidden(&self) -> EntityContainerGuard {
        let r = self.0.borrow();
        let start = r.visible_count;
        let end = r.active_count;

        EntityContainerGuard { r, start, end }
    }

    pub fn hidden_mut(&self) -> EntityContainerMutGuard {
        let r = self.0.borrow_mut();
        let start = r.visible_count;
        let end = r.active_count;

        EntityContainerMutGuard { r, start, end }
    }

    pub(super) fn destroy_entity(&self, entity_id: &EntityId) {
        let id = entity_id.0.borrow().expect("trying to use destroyed entity");

        let mut inner = self.0.borrow_mut();
        let index = inner.mapping[id.get()];

        assert!(index < inner.len());

        let visible_count = inner.visible_count;
        let active_count = inner.active_count;
        let last = inner.len() - 1;

        if index < visible_count {
            swap_and_update_second(&mut inner, index, visible_count-1);
            swap_and_update_second(&mut inner, visible_count-1, active_count-1);
            swap_and_update_second(&mut inner, active_count-1, last);
            inner.visible_count -= 1;
            inner.active_count -= 1;
        } else if index < active_count {
            let active_count = inner.active_count;
            swap_and_update_second(&mut inner, index, active_count-1);
            swap_and_update_second(&mut inner, active_count-1, last);
            inner.active_count -= 1;
        } else {
            swap_and_update_second(&mut inner, index, last);
        }

        inner.transforms.pop();
        inner.sprites.pop();
        let mut entity_id = inner.entities.pop().unwrap();

        entity_id.destroy(&mut inner.id_manager);
    }

    pub(super) fn set_entity_active(&self, entity_id: &EntityId, active: bool) {
        let id = entity_id.0.borrow().expect("trying to use destroyed entity");

        let mut inner = self.0.borrow_mut();
        let index = inner.mapping[id.get()];

        assert!(index < inner.len());

        let active_count = inner.active_count;

        if active {
            if index < active_count {
                // @Maybe not panic and just log?
                panic!("trying to activate an already active entity!");
            } else {
                swap_and_update(&mut inner, index, active_count);
                inner.active_count += 1;
            }
        } else {
            if index < active_count {
                swap_and_update(&mut inner, index, active_count-1);
                inner.active_count -= 1;
            } else {
                // @Maybe not panic and just log?
                panic!("trying to inactivate an already inactive entity!");
            }
        }
    }

    pub(super) fn set_entity_visible(&self, entity_id: &EntityId, visible: bool) {
        let id = entity_id.0.borrow().expect("trying to use destroyed entity");

        let mut inner = self.0.borrow_mut();
        let index = inner.mapping[id.get()];

        assert!(index < inner.len());

        let visible_count = inner.visible_count;
        let active_count = inner.active_count;

        if visible {
            if index < visible_count {
                // @Maybe not panic and just log?
                panic!("trying to make visible an already visible entity!");
            } else if index >= active_count {
                // @Maybe not panic and just log?
                panic!("trying to make visible an inactive entity!");
            } else {
                swap_and_update(&mut inner, index, visible_count);
                inner.visible_count += 1;
            }
        } else {
            if index < visible_count {
                swap_and_update(&mut inner, index, visible_count-1);
                inner.visible_count -= 1;
            } else if index >= active_count {
                // @Maybe not panic and just log?
                panic!("trying to make hidden an inactive entity!");
            } else {
                // @Maybe not panic and just log?
                panic!("trying to make hidden an already hidden entity!");
            }
        }
    }

    pub(super) fn get_entity_state_by_id(&self, entity_id: &EntityId) -> EntityState {
        let id = entity_id.0.borrow().expect("trying to use destroyed entity");

        let inner = self.0.borrow();

        let index = inner.mapping[id.get()];

        assert!(index < inner.transforms.len());

        if index < inner.visible_count { return EntityState::Visible; }
        if index < inner.active_count  { return EntityState::Hidden; }
        return EntityState::Inactive;
    }

    pub(super) fn get_entity_state_by_index(&self, index: usize) -> EntityState {
        let inner = self.0.borrow();

        assert!(index < inner.transforms.len());

        if index < inner.visible_count { return EntityState::Visible; }
        if index < inner.active_count  { return EntityState::Hidden; }
        return EntityState::Inactive;
    }
}

pub struct EntityContainerGuard<'a> {
    r: Ref<'a, EntityContainerInner>,
    start: usize,
    end: usize,
}

pub struct EntityContainerMutGuard<'a> {
    r: RefMut<'a, EntityContainerInner>,
    start: usize,
    end: usize,
}

impl EntityContainerGuard<'_> {
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn transforms(&self) -> &[Transform] {
        &self.r.transforms[self.start..self.end]
    }

    pub fn sprites(&self) -> &[Sprite] {
        &self.r.sprites[self.start..self.end]
    }
}

impl EntityContainerMutGuard<'_> {
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn transforms(&self) -> &[Transform] {
        &self.r.transforms[self.start..self.end]
    }

    pub fn transforms_mut(&mut self) -> &mut [Transform] {
        &mut self.r.transforms[self.start..self.end]
    }

    pub fn sprites(&self) -> &[Sprite] {
        &self.r.sprites[self.start..self.end]
    }

    pub fn sprites_mut(&mut self) -> &mut [Sprite] {
        &mut self.r.sprites[self.start..self.end]
    }
}

// -------
// private
// -------

pub(super) enum EntityState {
    Visible,
    Hidden,
    Inactive,
}

#[derive(Clone, Debug, ImDraw)]
struct EntityContainerInner {
    active_count: usize,
    visible_count: usize,

    entities:   Vec<EntityId>,
    transforms: Vec<Transform>,
    sprites:    Vec<Sprite>,

    mapping: Vec<usize>,
    cold_data: Vec<EntityColdData>,
    id_manager: IdManager,
}

impl EntityContainerInner {
    fn new() -> Self {
        Self {
            active_count: 0,
            visible_count: 0,
            entities: vec![],
            transforms: vec![],
            sprites: vec![],
            mapping: Vec::new(),
            cold_data: Vec::new(),
            id_manager: IdManager::new(),
        }
    }

    fn len(&self) -> usize {
        self.transforms.len()
    }
}

#[derive(Clone, Debug, ImDraw)]
struct EntityColdData {
    //animator: Animator,
}

// -----
// utils
// -----

fn swap_and_update_second(r: &mut RefMut<'_, EntityContainerInner>, first: usize, second: usize) {
    assert!(first < r.len());
    assert!(second < r.len());

    if first == second { return; }

    r.entities.swap(first, second);
    r.transforms.swap(first, second);
    r.sprites.swap(first, second);

    let id = r.entities[first].0.borrow().expect("destroyed entity in container");
    r.mapping[id.get()] = first; // update
}

fn swap_and_update(r: &mut RefMut<'_, EntityContainerInner>, first: usize, second: usize) {
    assert!(first < r.len());
    assert!(second < r.len());

    if first == second { return; }

    r.entities.swap(first, second);
    r.transforms.swap(first, second);
    r.sprites.swap(first, second);

    let id = r.entities[first].0.borrow().expect("destroyed entity in container");
    r.mapping[id.get()] = first; // update

    let id = r.entities[second].0.borrow().expect("destroyed entity in container");
    r.mapping[id.get()] = second; // update
}

// -----
// tests
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_entity_container() {
        let entity_container = EntityContainer::new();
        let entities = entity_container.all();
        entities.len();
    }
}
