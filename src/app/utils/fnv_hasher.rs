use std::hash::Hasher;

const FNV_OFFSET : u64 = 14695981039346656037;
const FNV_PRIME  : u64 = 1099511628211;

pub struct FNVHasher {
    hash: u64,
}

impl FNVHasher {
    pub fn new() -> Self {
        Self { hash: FNV_OFFSET }
    }

    pub fn cont(hash: u64) -> Self {
        Self { hash }
    }
}

impl Hasher for FNVHasher {
    fn finish(&self) -> u64 { self.hash }
    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes.iter() {
            self.hash ^= *byte as u64;
            self.hash = self.hash.wrapping_mul(FNV_PRIME);
        }
    }
}
