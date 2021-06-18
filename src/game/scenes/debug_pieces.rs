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

    has_grid: bool,
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
        let x = 100.0;
        let y = 100.0;

        draw_piece_window(
            Vec2 { x, y },
            Piece {
                type_: PIECES[0],
                rot: self.rot,
            },
            self.is_centered,
            self.has_grid,
            app,
            persistent
        );

        let pixel_scale = persistent.pixel_scale;

        let x = x + pixel_scale as f32 * BLOCK_SCALE * 6.0;
        draw_piece_window(
            Vec2 { x , y },
            Piece {
                type_: PIECES[1],
                rot: self.rot,
            },
            self.is_centered,
            self.has_grid,
            app,
            persistent
        );

        let x = x + pixel_scale as f32 * BLOCK_SCALE * 6.0;
        draw_piece_window(
            Vec2 { x , y },
            Piece {
                type_: PIECES[2],
                rot: self.rot,
            },
            self.is_centered,
            self.has_grid,
            app,
            persistent
        );

        let x = x + pixel_scale as f32 * BLOCK_SCALE * 6.0;
        draw_piece_window(
            Vec2 { x , y },
            Piece {
                type_: PIECES[3],
                rot: self.rot,
            },
            self.is_centered,
            self.has_grid,
            app,
            persistent
        );

        let x = 100.0;
        let y = y + pixel_scale as f32 * BLOCK_SCALE * 6.0;
        draw_piece_window(
            Vec2 { x , y },
            Piece {
                type_: PIECES[4],
                rot: self.rot,
            },
            self.is_centered,
            self.has_grid,
            app,
            persistent
        );

        let x = x + pixel_scale as f32 * BLOCK_SCALE * 6.0;
        draw_piece_window(
            Vec2 { x , y },
            Piece {
                type_: PIECES[5],
                rot: self.rot,
            },
            self.is_centered,
            self.has_grid,
            app,
            persistent
        );

        let x = x + pixel_scale as f32 * BLOCK_SCALE * 6.0;
        draw_piece_window(
            Vec2 { x , y },
            Piece {
                type_: PIECES[6],
                rot: self.rot,
            },
            self.is_centered,
            self.has_grid,
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

            Event::KeyDown { scancode: Some(Scancode::Z), .. } => {
                self.rot -= 1;
            }

            Event::KeyDown { scancode: Some(Scancode::X), .. } => {
                self.rot += 1;
            }

            Event::KeyDown { scancode: Some(Scancode::Space), .. } => {
                self.is_centered = !self.is_centered;
            }

            Event::KeyDown { scancode: Some(Scancode::W), .. } => {
                self.has_grid = !self.has_grid;
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
            has_grid: true,
        }
    }
}
