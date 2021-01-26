use super::super::renderer::Sprite;

pub trait Renderable {
    fn sprite(&self) -> &Sprite;
    fn sprite_mut(&mut self) -> &mut Sprite;
    fn is_visible(&self) -> bool;
    fn set_visible(&self, _visible: bool);
}
