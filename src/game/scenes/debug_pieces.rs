use crate::BLOCK_SCALE;
use crate::State;
use crate::app::*;
use crate::linalg::*;
use crate::game::piece::{ Piece, PIECES };

use super::*;

use crate::game::render::*;

#[derive(Clone, Debug, ImDraw)]
pub struct DebugPiecesScene {
    go_back: bool,
    is_centered: bool,
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
            Piece {
                type_: PIECES[0],
                rot: self.rot,
            },
            app,
            persistent
        );

        let pixel_scale = persistent.pixel_scale;

        let x = x + (pixel_scale.x * BLOCK_SCALE * 6.0) as i32;
        self.draw_debug_piece(
            Vec2i { x , y },
            Piece {
                type_: PIECES[1],
                rot: self.rot,
            },
            app,
            persistent
        );

        let x = x + (pixel_scale.x * BLOCK_SCALE * 6.0) as i32;
        self.draw_debug_piece(
            Vec2i { x , y },
            Piece {
                type_: PIECES[2],
                rot: self.rot,
            },
            app,
            persistent
        );

        let x = x + (pixel_scale.x * BLOCK_SCALE * 6.0) as i32;
        self.draw_debug_piece(
            Vec2i { x , y },
            Piece {
                type_: PIECES[3],
                rot: self.rot,
            },
            app,
            persistent
        );

        let x = 100;
        let y = y + (pixel_scale.y * BLOCK_SCALE * 6.0) as i32;
        self.draw_debug_piece(
            Vec2i { x , y },
            Piece {
                type_: PIECES[4],
                rot: self.rot,
            },
            app,
            persistent
        );

        let x = x + (pixel_scale.x * BLOCK_SCALE * 6.0) as i32;
        self.draw_debug_piece(
            Vec2i { x , y },
            Piece {
                type_: PIECES[5],
                rot: self.rot,
            },
            app,
            persistent
        );

        let x = x + (pixel_scale.x * BLOCK_SCALE * 6.0) as i32;
        self.draw_debug_piece(
            Vec2i { x , y },
            Piece {
                type_: PIECES[6],
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

            Event::KeyDown { scancode: Some(Scancode::L), .. } => {
                self.rot += 1;
            }

            Event::KeyDown { scancode: Some(Scancode::Space), .. } => {
                self.is_centered = !self.is_centered;
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
            is_centered: false,
            rot: 0,
        }
    }

    fn draw_debug_piece(
        &self,
        pos: Vec2i,
        piece: Piece,
        app: &mut App<'_, State>,
        persistent: &mut PersistentData
    ) {
        draw_rect_window(
            pos.into(),
            Vec2 {
                x: persistent.pixel_scale.x * BLOCK_SCALE * 4.0,
                y: persistent.pixel_scale.y * BLOCK_SCALE * 4.0,
            },
            persistent.pixel_scale,
            app,
            persistent
        );

        if self.is_centered {
            draw_piece_centered(
                piece,
                pos.into(),
                WHITE,
                app,
                persistent
            );
        } else {
            draw_piece(
                piece,
                pos.into(),
                WHITE,
                app,
                persistent
            );
        }
    }
}
