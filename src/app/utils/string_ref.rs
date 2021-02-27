use std::hash::{Hash, Hasher};
use super::fnv_hasher::FNVHasher;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct StringRef(u64);

impl StringRef {
    pub fn new(s: String) -> Self {
        let mut hasher = FNVHasher::new();
        s.hash(&mut hasher);
        Self(hasher.finish())
    }
}
