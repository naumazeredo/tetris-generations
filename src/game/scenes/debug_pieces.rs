use crate::BLOCK_SCALE;
use crate::State;
use crate::app::*;
use crate::linalg::*;
use crate::game::piece::{ Piece, PIECES };

use super::*;

#[derive(Clone, Debug, ImDraw)]
pub struct DebugPiecesScene {
    go_back: bool,
    rot: i32,
}

impl SceneTrait for DebugPiecesScene {
    /*
    fn update(
        &mut self,
        app: &mut App<'_, State>,
        persistent: &mut PersistentData
    ) {
    }
    */

    fn render(
        &mut self,
        app: &mut App<'_, State>,
        persistent: &mut PersistentData
    ) {
        let x = 100;
        let y = 100;

        self.draw_debug_piece(
            Vec2i { x, y },
            &Piece {
                type_: PIECES[0],
                pos: Vec2i { x: 0, y: 0 },
                rot: self.rot,
            },
            app,
            persistent
        );

        let pixel_scale = persistent.pixel_scale;

        let x = x + (pixel_scale.x * BLOCK_SCALE * 6.0) as i32;
        self.draw_debug_piece(
            Vec2i { x , y },
            &Piece {
                type_: PIECES[1],
                pos: Vec2i { x: 0, y: 0 },
                rot: self.rot,
            },
            app,
            persistent
        );

        let x = x + (pixel_scale.x * BLOCK_SCALE * 6.0) as i32;
        self.draw_debug_piece(
            Vec2i { x , y },
            &Piece {
                type_: PIECES[2],
                pos: Vec2i { x: 0, y: 0 },
                rot: self.rot,
            },
            app,
            persistent
        );

        let x = x + (pixel_scale.x * BLOCK_SCALE * 6.0) as i32;
        self.draw_debug_piece(
            Vec2i { x , y },
            &Piece {
                type_: PIECES[3],
                pos: Vec2i { x: 0, y: 0 },
                rot: self.rot,
            },
            app,
            persistent
        );

        let x = 100;
        let y = y + (pixel_scale.y * BLOCK_SCALE * 6.0) as i32;
        self.draw_debug_piece(
            Vec2i { x , y },
            &Piece {
                type_: PIECES[4],
                pos: Vec2i { x: 0, y: 0 },
                rot: self.rot,
            },
            app,
            persistent
        );

        let x = x + (pixel_scale.x * BLOCK_SCALE * 6.0) as i32;
        self.draw_debug_piece(
            Vec2i { x , y },
            &Piece {
                type_: PIECES[5],
                pos: Vec2i { x: 0, y: 0 },
                rot: self.rot,
            },
            app,
            persistent
        );

        let x = x + (pixel_scale.x * BLOCK_SCALE * 6.0) as i32;
        self.draw_debug_piece(
            Vec2i { x , y },
            &Piece {
                type_: PIECES[6],
                pos: Vec2i { x: 0, y: 0 },
                rot: self.rot,
            },
            app,
            persistent
        );

        app.render_queued();
    }

    fn handle_input(
        &mut self,
        app: &mut App<'_, State>,
        _persistent: &mut PersistentData,
        event: &sdl2::event::Event
    ) -> bool {
        use sdl2::event::Event;
        use sdl2::keyboard::Scancode;

        match event {
            Event::KeyDown { scancode: Some(Scancode::F10), .. } => {
                self.go_back = true;
                app.resume();
                return true;
            }

            Event::KeyDown { scancode: Some(Scancode::J), .. } => {
                self.rot -= 1;
            }

            Event::KeyDown { scancode: Some(Scancode::K), .. } => {
                self.rot += 1;
            }

            _ => {}
        }

        false
    }

    fn transition(&mut self) -> Option<SceneTransition> {
        if self.go_back {
            Some(SceneTransition::Pop)
        } else {
            None
        }
    }
}

impl DebugPiecesScene {
    pub fn new() -> Self {
        Self {
            go_back: false,
            rot: 0,
        }
    }

    fn draw_debug_piece(
        &self,
        pos: Vec2i,
        piece: &Piece,
        app: &mut App<'_, State>,
        persistent: &mut PersistentData
    ) {
        let pixel_scale = persistent.pixel_scale;

        for block_pos in piece.type_.blocks(piece.rot) {
            let pos = Vec2 {
                x: pos.x as f32 + BLOCK_SCALE * pixel_scale.x * block_pos.x as f32,
                y: pos.y as f32 + BLOCK_SCALE * pixel_scale.y * (3 - block_pos.y) as f32,
            };

            app.queue_draw_sprite(
                &TransformBuilder::new()
                    .pos(pos)
                    .scale(pixel_scale)
                    .layer(10)
                    .build(),
                &persistent.sprites.block,
                WHITE
            );
        }

        {
            let pos = Vec2::from(pos);
            let scale = BLOCK_SCALE * Vec2 {
                x: pixel_scale.x * 4 as f32,
                y: pixel_scale.y * 4 as f32,
            };
            app.queue_draw_sprite(
                // @TODO fix layer negative not showing
                &TransformBuilder::new().pos(pos).scale(scale).layer(0).build(),
                &persistent.sprites.blank,
                Color { r: 0.2, g: 0.2, b: 0.2, a: 1.0 },
            );
        }
    }
}
