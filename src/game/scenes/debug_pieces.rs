use crate::BLOCK_SCALE;
use crate::app::*;
use crate::linalg::*;

use crate::game::{
    pieces::{ Piece, PIECES },
    rules::RotationSystem,
};

use super::*;

use crate::game::render::*;

#[derive(Clone, Debug, ImDraw)]
pub struct DebugPiecesScene {
    go_back: bool,
    is_centered: bool,
    rot: i32,

    has_grid: bool,
    rotation_system: RotationSystem,
}

impl SceneTrait for DebugPiecesScene {
    type Scene = Scene;
    type PersistentData = PersistentData;

    fn update(
        &mut self,
        _dt: u64,
        _app: &mut App,
        _persistent: &mut Self::PersistentData
    ) {
        self.rot = ((self.rot % 4) + 4) % 4;
    }

    fn render(
        &mut self,
        app: &mut App,
        persistent: &mut Self::PersistentData
    ) {
        app.queue_draw_text(
            &format!("{:?}", self.rotation_system),
            TransformBuilder::new().pos_xy(10.0, 42.0).layer(1000).build(),
            32.,
            WHITE,
            None,
            None,
        );

        app.queue_draw_text(
            &format!("rot: {}", self.rot),
            TransformBuilder::new().pos_xy(10.0, 84.0).layer(1000).build(),
            32.,
            WHITE,
            None,
            None,
        );

        let x = 100;
        let y = 100;

        draw_piece_window(
            Vec2i { x, y },
            Piece {
                variant: PIECES[0],
                rot: self.rot,
                rotation_system: self.rotation_system,
            },
            persistent.pixel_scale,
            self.is_centered,
            self.has_grid,
            &mut app.batch(),
            persistent
        );

        let pixel_scale = persistent.pixel_scale as i32;

        let x = x + pixel_scale * BLOCK_SCALE as i32 * 6;
        draw_piece_window(
            Vec2i { x , y },
            Piece {
                variant: PIECES[1],
                rot: self.rot,
                rotation_system: self.rotation_system,
            },
            persistent.pixel_scale,
            self.is_centered,
            self.has_grid,
            &mut app.batch(),
            persistent
        );

        let x = x + pixel_scale * BLOCK_SCALE as i32 * 6;
        draw_piece_window(
            Vec2i { x , y },
            Piece {
                variant: PIECES[2],
                rot: self.rot,
                rotation_system: self.rotation_system,
            },
            persistent.pixel_scale,
            self.is_centered,
            self.has_grid,
            &mut app.batch(),
            persistent
        );

        let x = x + pixel_scale * BLOCK_SCALE as i32 * 6;
        draw_piece_window(
            Vec2i { x , y },
            Piece {
                variant: PIECES[3],
                rot: self.rot,
                rotation_system: self.rotation_system,
            },
            persistent.pixel_scale,
            self.is_centered,
            self.has_grid,
            &mut app.batch(),
            persistent
        );

        let x = 100;
        let y = y + pixel_scale * BLOCK_SCALE as i32 * 6;
        draw_piece_window(
            Vec2i { x , y },
            Piece {
                variant: PIECES[4],
                rot: self.rot,
                rotation_system: self.rotation_system,
            },
            persistent.pixel_scale,
            self.is_centered,
            self.has_grid,
            &mut app.batch(),
            persistent
        );

        let x = x + pixel_scale * BLOCK_SCALE as i32 * 6;
        draw_piece_window(
            Vec2i { x , y },
            Piece {
                variant: PIECES[5],
                rot: self.rot,
                rotation_system: self.rotation_system,
            },
            persistent.pixel_scale,
            self.is_centered,
            self.has_grid,
            &mut app.batch(),
            persistent
        );

        let x = x + pixel_scale * BLOCK_SCALE as i32 * 6;
        draw_piece_window(
            Vec2i { x , y },
            Piece {
                variant: PIECES[6],
                rot: self.rot,
                rotation_system: self.rotation_system,
            },
            persistent.pixel_scale,
            self.is_centered,
            self.has_grid,
            &mut app.batch(),
            persistent
        );
    }

    fn handle_input(
        &mut self,
        event: &sdl2::event::Event,
        app: &mut App,
        _persistent: &mut Self::PersistentData,
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

            Event::KeyDown { scancode: Some(Scancode::D), .. } => {
                self.next_rotation_systems();
            }

            Event::KeyDown { scancode: Some(Scancode::A), .. } => {
                self.previous_rotation_systems();
            }

            _ => {}
        }

        false
    }

    fn transition(
        &mut self,
        _app: &mut App,
        _persistent: &mut Self::PersistentData
    ) -> Option<SceneTransition<Self::Scene>> {
        if self.go_back {
            Some(SceneTransition::Pop)
        } else {
            None
        }
    }

    fn on_enter(&mut self, app: &mut App, _persistent: &mut Self::PersistentData) {
        app.pause();
    }

    fn on_exit(&mut self, app: &mut App, _persistent: &mut Self::PersistentData) {
        app.resume();
    }
}

impl DebugPiecesScene {
    pub fn new() -> Self {
        Self {
            go_back: false,
            is_centered: false,
            rot: 0,
            has_grid: true,
            rotation_system: RotationSystem::Original,
        }
    }

    fn next_rotation_systems(&mut self) {
        self.rotation_system = match self.rotation_system {
            RotationSystem::Original => RotationSystem::NRSR,
            RotationSystem::NRSR     => RotationSystem::NRSL,
            RotationSystem::NRSL     => RotationSystem::Sega,
            RotationSystem::Sega     => RotationSystem::ARS,
            RotationSystem::ARS      => RotationSystem::SRS,
            RotationSystem::SRS      => RotationSystem::DTET,
            RotationSystem::DTET     => RotationSystem::Original,
        };
    }

    fn previous_rotation_systems(&mut self) {
        self.rotation_system = match self.rotation_system {
            RotationSystem::Original => RotationSystem::DTET,
            RotationSystem::NRSR     => RotationSystem::Original,
            RotationSystem::NRSL     => RotationSystem::NRSR,
            RotationSystem::Sega     => RotationSystem::NRSL,
            RotationSystem::ARS      => RotationSystem::Sega,
            RotationSystem::SRS      => RotationSystem::ARS,
            RotationSystem::DTET     => RotationSystem::SRS,
        };
    }
}
