// [ ] rename to id_generator

use super::imgui_wrapper::imdraw::ImDraw;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, ImDraw)]
pub struct Id {
    index: usize,
    generation: u32,
}

impl Id {
    pub fn index(self) -> usize {
        self.index
    }
}

pub trait IsId : Copy + Clone {
    fn new(_id: Id) -> Self;
    fn id(self) -> Id;
}

#[derive(Clone, Debug, ImDraw)]
pub struct IdGenerator {
    ids: Vec<IdGeneratorEntry>,
    free_indexes: Vec<usize>,
}

impl IdGenerator {
    pub fn new() -> Self {
        Self {
            ids: Vec::new(),
            free_indexes: Vec::new(),
        }
    }

    pub fn next(&mut self) -> Id {
        if let Some(index) = self.free_indexes.pop() {
            let mut id = &mut self.ids[index];
            id.is_live = true;
            id.generation += 1;
            return Id { index, generation: id.generation };
        }

        let index = self.ids.len();
        self.ids.push(IdGeneratorEntry { is_live: true, generation: 0 });

        Id { index, generation: 0 }
    }

    pub fn free(&mut self, id: Id) {
        assert!(id.generation == self.ids[id.index].generation);
        assert!(self.ids[id.index].is_live);

        self.ids[id.index].is_live = false;
        self.free_indexes.push(id.index);
    }

    pub fn is_live(&self, id: Id) -> bool {
        self.ids[id.index].is_live
    }

    pub fn len(&self) -> usize {
        self.ids.len() - self.free_indexes.len()
    }
}

// private

#[derive(Copy, Clone, Debug, ImDraw)]
struct IdGeneratorEntry {
    is_live: bool,
    generation: u32,
}

// utils

/*
// @Maybe nightly
impl<T> SliceIndex<[T]> for Id {
    type Output: T;
    pub fn get(self, slice: &[T]) -> Option<&T> { &slice[self.0] }
    pub fn get_mut(self, slice: &mut [T]) -> Option<&mut T> { &mut slice[self.0] }
    //pub fn index
}
*/
