use super::*;
use crate::app::transform::Transform;

/*
// @XXX Is this better? seems to make the code worse
macro_rules! clip {
    ($batch:ident, $pos:expr, $size:expr, $code:block) => {
        $batch.push_clip($pos, $size);
        $code
        $batch.pop_clip();
    }
}
*/

#[derive(Debug, Default, ImDraw)]
pub struct Batch {
    pub(super) cmds: Vec<Command>,
}

impl Batch {
    pub fn new() -> Self {
        Self { cmds: Vec::new() }
    }

    pub fn queue_draw_solid(
        &mut self,
        transform: &Transform,
        size: Vec2,
        color: Color,
    ) {
        self.cmds.push(
            Command::Draw(DrawCommand {
                program: None,
                texture: None,
                layer: transform.layer,
                color,
                pos: transform.pos,
                scale: transform.scale,
                rot: transform.rot,
                variant: DrawVariant::Solid { size },
            })
        );
    }

    pub fn push_clip(
        &mut self,
        pos: Vec2i,
        size: Vec2i,
    ) {
        assert!(size.x >= 0);
        assert!(size.y >= 0);

        self.cmds.push(
            Command::PushClip {
                min: pos,
                max: pos + size,
                intersect: false,
            }
        );
    }

    pub fn pop_clip(&mut self) {
        self.cmds.push(Command::PopClip);
    }
}

impl App<'_> {
    pub fn queue_draw_solid(
        &mut self,
        transform: &Transform,
        size: Vec2,
        color: Color,
    ) {
        self.renderer.batch.queue_draw_solid(transform, size, color);
    }

    pub fn push_clip(
        &mut self,
        pos: Vec2i,
        size: Vec2i,
    ) {
        self.renderer.batch.push_clip(pos, size);
    }

    pub fn pop_clip(&mut self) {
        self.renderer.batch.pop_clip();
    }
}