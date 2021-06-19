use crate::app::{ImDraw, Color};
use crate::linalg::Vec2i;

use super::rules::RotationSystem;

mod dtet;
mod nrsl;
mod nrsr;
mod original;
mod sega;
mod srs;

use dtet::*;
use nrsl::*;
use nrsr::*;
use original::*;
use sega::*;
use srs::*;

#[derive(Copy, Clone, Debug, ImDraw)]
pub struct Piece {
    pub type_: PieceType,
    pub rot: i32,
}

impl Piece {
    pub fn blocks(&self, rotation_system: RotationSystem) -> &'static [Vec2i] {
        self.type_.blocks(self.rot, rotation_system)
    }

    pub fn min_max_x(self, rotation_system: RotationSystem) -> (i8, i8) {
        self.type_.min_max_x(self.rot, rotation_system)
    }

    pub fn min_max_y(self, rotation_system: RotationSystem) -> (i8, i8) {
        self.type_.min_max_y(self.rot, rotation_system)
    }

    pub fn color(self, rotation_system: RotationSystem) -> Color {
        self.type_.color(rotation_system)
    }
}

#[derive(Copy, Clone, Debug, ImDraw)]
pub enum PieceType { S, Z, J, L, O, I, T }

impl PieceType {
    pub fn blocks(
        self,
        rot: i32,
        rotation_system: RotationSystem
    ) -> &'static [Vec2i] {
        //assert!(rot >= 0 && rot < 4);
        let rot = (((rot % 4) + 4) % 4) as usize;
        &get_piece_data(self, rotation_system).blocks[rot]
    }

    pub fn min_max_x(
        self,
        rot: i32,
        rotation_system: RotationSystem
    ) -> (i8, i8) {
        //assert!(rot >= 0 && rot < 4);
        let rot = (((rot % 4) + 4) % 4) as usize;
        let piece_data = get_piece_data(self, rotation_system);
        (piece_data.min_x[rot], piece_data.max_x[rot])
    }

    pub fn min_max_y(
        self,
        rot: i32,
        rotation_system: RotationSystem
    ) -> (i8, i8) {
        //assert!(rot >= 0 && rot < 4);
        let rot = (((rot % 4) + 4) % 4) as usize;
        let piece_data = get_piece_data(self, rotation_system);
        (piece_data.min_y[rot], piece_data.max_y[rot])
    }

    // @TODO this should not be here. When styles are implemented we will not use this anymore
    pub fn color(
        self,
        rotation_system: RotationSystem
    ) -> Color {
        get_piece_data(self, rotation_system).color
    }
}

pub const PIECES : [PieceType; 7] = [
    PieceType::S,
    PieceType::Z,
    PieceType::J,
    PieceType::L,
    PieceType::O,
    PieceType::I,
    PieceType::T,
];

fn piece_to_index(piece_type: PieceType) -> usize {
    match piece_type {
        PieceType::S => 0,
        PieceType::Z => 1,
        PieceType::J => 2,
        PieceType::L => 3,
        PieceType::O => 4,
        PieceType::I => 5,
        PieceType::T => 6,
    }
}

fn get_piece_data(
    piece_type: PieceType,
    rotation_system: RotationSystem,
) -> &'static PieceData {

    let orientation_data = match rotation_system {
        RotationSystem::Original => &PIECES_ORIGINAL,
        RotationSystem::NRSR     => &PIECES_NRSR,
        RotationSystem::NRSL     => &PIECES_NRSL,
        RotationSystem::Sega     => &PIECES_SEGA,
        RotationSystem::ARS      => &PIECES_SEGA, // ARS has same piece rotations as Sega
        RotationSystem::SRS      => &PIECES_SRS,
        RotationSystem::DTET     => &PIECES_DTET,
    };

    &(orientation_data.0)[piece_to_index(piece_type)]
}


// NIT: This could be a very packed struct, but we only have 7 different types and this won't be
// sent over wire or anything, so having an unpacked struct is fine
struct PieceData {
    blocks: [[Vec2i; 4]; 4],
    min_x: [i8; 4],
    max_x: [i8; 4],
    min_y: [i8; 4],
    max_y: [i8; 4],
    color: Color,
}

struct PieceOrientationData([PieceData; 7]);
