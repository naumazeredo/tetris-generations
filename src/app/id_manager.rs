use super::imgui::imdraw::ImDraw;

#[derive(Copy, Clone, Debug, ImDraw)]
pub struct Id(usize);

impl Id {
    pub fn get(self) -> usize {
        self.0
    }
}

#[derive(Clone, Debug, ImDraw)]
pub struct IdManager {
    last_id: Id,
    free_ids: Vec<Id>,
}

impl IdManager {
    pub fn new() -> Self {
        Self {
            last_id: Id(0usize),
            free_ids: vec![],
        }
    }

    pub fn next(&mut self) -> Id {
        if let Some(id) = self.free_ids.pop() {
            return id;
        }

        let id = self.last_id;
        self.last_id = Id(id.0 + 1);
        id
    }

    // @TODO return result
    pub fn free(&mut self, id: Id) {
        assert!(id.0 < self.last_id.0);

        // @XXX we are not checking for double free
        self.free_ids.push(id);
    }
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
