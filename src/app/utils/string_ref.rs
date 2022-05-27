use crate::app::ImDraw;
use std::hash::{Hash, Hasher};
use super::fnv_hasher::FNVHasher;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct StringRef(u64);

// @TODO macro this
impl StringRef {
    pub fn new(s: &str) -> Self {
        let mut hasher = FNVHasher::new();
        s.hash(&mut hasher);
        Self(hasher.finish())
    }
}

impl std::fmt::Display for StringRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "StringRef {}", self.0)
    }
}

impl_imdraw_todo!(StringRef);
