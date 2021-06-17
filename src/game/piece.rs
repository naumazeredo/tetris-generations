use crate::app::{ImDraw, Color};
use crate::linalg::Vec2i;

pub const PIECES : [PieceType; 7] = [
    PieceType::S,
    PieceType::Z,
    PieceType::J,
    PieceType::L,
    PieceType::O,
    PieceType::I,
    PieceType::T,
];

#[derive(Copy, Clone, Debug, ImDraw)]
pub struct Piece {
    pub type_: PieceType,
    pub rot: i32,
}

impl Piece {
    pub fn blocks(&self) -> &'static [Vec2i] {
        self.type_.blocks(self.rot)
    }

    pub fn min_max_x(self) -> (u8, u8) {
        self.type_.min_max_x(self.rot)
    }

    pub fn min_max_y(self) -> (u8, u8) {
        self.type_.min_max_y(self.rot)
    }

    pub fn color(self) -> Color {
        self.type_.color()
    }
}

#[derive(Copy, Clone, Debug, ImDraw)]
pub enum PieceType { S, Z, J, L, O, I, T }

impl PieceType {
    pub fn blocks(self, rot: i32) -> &'static [Vec2i] {
        //assert!(rot >= 0 && rot < 4);
        let rot = (((rot % 4) + 4) % 4) as usize;

        match self {
            PieceType::S => &S_PIECE.blocks[rot],
            PieceType::Z => &Z_PIECE.blocks[rot],
            PieceType::J => &J_PIECE.blocks[rot],
            PieceType::L => &L_PIECE.blocks[rot],
            PieceType::O => &O_PIECE.blocks[rot],
            PieceType::I => &I_PIECE.blocks[rot],
            PieceType::T => &T_PIECE.blocks[rot],
        }
    }

    pub fn min_max_x(self, rot: i32) -> (u8, u8) {
        //assert!(rot >= 0 && rot < 4);
        let rot = (((rot % 4) + 4) % 4) as usize;

        match self {
            PieceType::S => (S_PIECE.min_x[rot], S_PIECE.max_x[rot]),
            PieceType::Z => (Z_PIECE.min_x[rot], Z_PIECE.max_x[rot]),
            PieceType::J => (J_PIECE.min_x[rot], J_PIECE.max_x[rot]),
            PieceType::L => (L_PIECE.min_x[rot], L_PIECE.max_x[rot]),
            PieceType::O => (O_PIECE.min_x[rot], O_PIECE.max_x[rot]),
            PieceType::I => (I_PIECE.min_x[rot], I_PIECE.max_x[rot]),
            PieceType::T => (T_PIECE.min_x[rot], T_PIECE.max_x[rot]),
        }
    }

    pub fn min_max_y(self, rot: i32) -> (u8, u8) {
        //assert!(rot >= 0 && rot < 4);
        let rot = (((rot % 4) + 4) % 4) as usize;

        match self {
            PieceType::S => (S_PIECE.min_y[rot], S_PIECE.max_y[rot]),
            PieceType::Z => (Z_PIECE.min_y[rot], Z_PIECE.max_y[rot]),
            PieceType::J => (J_PIECE.min_y[rot], J_PIECE.max_y[rot]),
            PieceType::L => (L_PIECE.min_y[rot], L_PIECE.max_y[rot]),
            PieceType::O => (O_PIECE.min_y[rot], O_PIECE.max_y[rot]),
            PieceType::I => (I_PIECE.min_y[rot], I_PIECE.max_y[rot]),
            PieceType::T => (T_PIECE.min_y[rot], T_PIECE.max_y[rot]),
        }
    }

    pub fn color(self) -> Color {
        match self {
            PieceType::S => S_PIECE.color,
            PieceType::Z => Z_PIECE.color,
            PieceType::J => J_PIECE.color,
            PieceType::L => L_PIECE.color,
            PieceType::O => O_PIECE.color,
            PieceType::I => I_PIECE.color,
            PieceType::T => T_PIECE.color,
        }
    }
}

// @TODO manually preprocess all piece datas for each piece positioning rule

// NIT: This could be a very packed struct, but we only have 7 different types and this won't be
// sent over wire or anything, so having an unpacked struct is fine
struct PieceData {
    blocks: [[Vec2i; 4]; 4],
    min_x: [u8; 4],
    max_x: [u8; 4],
    min_y: [u8; 4],
    max_y: [u8; 4],
    color: Color,
}

const S_PIECE : PieceData = PieceData {
    blocks: [
        [Vec2i { x: 1, y: 2 }, Vec2i { x: 2, y: 2 }, Vec2i { x: 0, y: 1 }, Vec2i { x: 1, y: 1 }],
        [Vec2i { x: 1, y: 2 }, Vec2i { x: 1, y: 1 }, Vec2i { x: 2, y: 1 }, Vec2i { x: 2, y: 0 }],
        [Vec2i { x: 1, y: 2 }, Vec2i { x: 2, y: 2 }, Vec2i { x: 0, y: 1 }, Vec2i { x: 1, y: 1 }],
        [Vec2i { x: 1, y: 2 }, Vec2i { x: 1, y: 1 }, Vec2i { x: 2, y: 1 }, Vec2i { x: 2, y: 0 }],
    ],
    min_x: [0, 1, 0, 1],
    max_x: [2, 2, 2, 2],
    min_y: [1, 0, 1, 0],
    max_y: [2, 2, 2, 2],
    color: Color { r: 0.0, g: 0.5, b: 0.0, a: 1.0 },
};

const Z_PIECE : PieceData = PieceData {
    blocks: [
        [Vec2i { x: 0, y: 2 }, Vec2i { x: 1, y: 2 }, Vec2i { x: 1, y: 1 }, Vec2i { x: 2, y: 1 }],
        [Vec2i { x: 1, y: 2 }, Vec2i { x: 0, y: 1 }, Vec2i { x: 1, y: 1 }, Vec2i { x: 0, y: 0 }],
        [Vec2i { x: 0, y: 2 }, Vec2i { x: 1, y: 2 }, Vec2i { x: 1, y: 1 }, Vec2i { x: 2, y: 1 }],
        [Vec2i { x: 1, y: 2 }, Vec2i { x: 0, y: 1 }, Vec2i { x: 1, y: 1 }, Vec2i { x: 0, y: 0 }],
    ],
    min_x: [0, 0, 0, 0],
    max_x: [2, 1, 2, 1],
    min_y: [1, 0, 1, 0],
    max_y: [2, 2, 2, 2],
    color: Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 },
};

const J_PIECE : PieceData = PieceData {
    blocks: [
        [Vec2i { x: 0, y: 2 }, Vec2i { x: 1, y: 2 }, Vec2i { x: 2, y: 2 }, Vec2i { x: 2, y: 1 }],
        [Vec2i { x: 1, y: 3 }, Vec2i { x: 1, y: 2 }, Vec2i { x: 1, y: 1 }, Vec2i { x: 0, y: 1 }],
        [Vec2i { x: 0, y: 3 }, Vec2i { x: 0, y: 2 }, Vec2i { x: 1, y: 2 }, Vec2i { x: 2, y: 2 }],
        [Vec2i { x: 1, y: 3 }, Vec2i { x: 2, y: 3 }, Vec2i { x: 1, y: 2 }, Vec2i { x: 1, y: 1 }],
    ],
    min_x: [0, 0, 0, 1],
    max_x: [2, 1, 2, 2],
    min_y: [1, 1, 2, 1],
    max_y: [2, 3, 3, 3],
    color: Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 },
};

const L_PIECE : PieceData = PieceData {
    blocks: [
        [Vec2i { x: 0, y: 2 }, Vec2i { x: 1, y: 2 }, Vec2i { x: 2, y: 2 }, Vec2i { x: 0, y: 1 }],
        [Vec2i { x: 0, y: 3 }, Vec2i { x: 1, y: 3 }, Vec2i { x: 1, y: 2 }, Vec2i { x: 1, y: 1 }],
        [Vec2i { x: 2, y: 3 }, Vec2i { x: 0, y: 2 }, Vec2i { x: 1, y: 2 }, Vec2i { x: 2, y: 2 }],
        [Vec2i { x: 1, y: 3 }, Vec2i { x: 1, y: 2 }, Vec2i { x: 1, y: 1 }, Vec2i { x: 2, y: 1 }],
    ],
    min_x: [0, 0, 0, 1],
    max_x: [2, 1, 2, 2],
    min_y: [1, 1, 2, 1],
    max_y: [2, 3, 3, 3],
    color: Color { r: 1.0, g: 0.65, b: 0.0, a: 1.0 },
};

const O_PIECE : PieceData = PieceData {
    blocks: [
        [Vec2i { x: 1, y: 2 }, Vec2i { x: 2, y: 2 }, Vec2i { x: 1, y: 1 }, Vec2i { x: 2, y: 1 }],
        [Vec2i { x: 1, y: 2 }, Vec2i { x: 2, y: 2 }, Vec2i { x: 1, y: 1 }, Vec2i { x: 2, y: 1 }],
        [Vec2i { x: 1, y: 2 }, Vec2i { x: 2, y: 2 }, Vec2i { x: 1, y: 1 }, Vec2i { x: 2, y: 1 }],
        [Vec2i { x: 1, y: 2 }, Vec2i { x: 2, y: 2 }, Vec2i { x: 1, y: 1 }, Vec2i { x: 2, y: 1 }],
    ],
    min_x: [1, 1, 1, 1],
    max_x: [2, 2, 2, 2],
    min_y: [1, 1, 1, 1],
    max_y: [2, 2, 2, 2],
    color: Color { r: 1.0, g: 1.0, b: 0.0, a: 1.0 },
};

const I_PIECE : PieceData = PieceData {
    blocks: [
        [Vec2i { x: 0, y: 2 }, Vec2i { x: 1, y: 2 }, Vec2i { x: 2, y: 2 }, Vec2i { x: 3, y: 2 }],
        [Vec2i { x: 2, y: 4 }, Vec2i { x: 2, y: 3 }, Vec2i { x: 2, y: 2 }, Vec2i { x: 2, y: 1 }],
        [Vec2i { x: 0, y: 2 }, Vec2i { x: 1, y: 2 }, Vec2i { x: 2, y: 2 }, Vec2i { x: 3, y: 2 }],
        [Vec2i { x: 2, y: 4 }, Vec2i { x: 2, y: 3 }, Vec2i { x: 2, y: 2 }, Vec2i { x: 2, y: 1 }],
    ],
    min_x: [0, 2, 0, 2],
    max_x: [3, 2, 3, 2],
    min_y: [2, 1, 2, 1],
    max_y: [2, 4, 2, 4],
    color: Color { r: 0.0, g: 1.0, b: 1.0, a: 1.0 },
};

const T_PIECE : PieceData = PieceData {
    blocks: [
        [Vec2i { x: 0, y: 2 }, Vec2i { x: 1, y: 2 }, Vec2i { x: 2, y: 2 }, Vec2i { x: 1, y: 1 }],
        [Vec2i { x: 1, y: 3 }, Vec2i { x: 0, y: 2 }, Vec2i { x: 1, y: 2 }, Vec2i { x: 1, y: 1 }],
        [Vec2i { x: 1, y: 3 }, Vec2i { x: 0, y: 2 }, Vec2i { x: 1, y: 2 }, Vec2i { x: 2, y: 2 }],
        [Vec2i { x: 1, y: 3 }, Vec2i { x: 1, y: 2 }, Vec2i { x: 2, y: 2 }, Vec2i { x: 1, y: 1 }],
    ],
    min_x: [0, 0, 0, 1],
    max_x: [2, 1, 2, 2],
    min_y: [1, 1, 2, 1],
    max_y: [2, 3, 3, 3],
    color: Color { r: 0.5, g: 0.0, b: 0.5, a: 1.0 },
};
