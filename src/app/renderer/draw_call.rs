use super::*;
use crate::linalg::Vec2i;

#[derive(Clone, Debug)]
pub(super) struct DrawCall {
    pub(super) material: MaterialRef,
    pub(super) texture:  TextureRef,
    pub(super) clip:     Option<(Vec2i, Vec2i)>,
    pub(super) start:    usize,
    pub(super) count:    usize,
}
