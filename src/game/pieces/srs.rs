use crate::app::Color;
use crate::linalg::Vec2i;
use super::{ PieceData, PieceOrientationData };

//https://tetris.fandom.com/wiki/SRS

pub(super) const PIECES_SRS : PieceOrientationData = PieceOrientationData(
    [
        // S
        PieceData {
            blocks: [
                [Vec2i { x: 1, y: 2 }, Vec2i { x: 2, y: 2 }, Vec2i { x: 0, y: 1 }, Vec2i { x: 1, y: 1 }],
                [Vec2i { x: 1, y: 2 }, Vec2i { x: 1, y: 1 }, Vec2i { x: 2, y: 1 }, Vec2i { x: 2, y: 0 }],
                [Vec2i { x: 1, y: 1 }, Vec2i { x: 2, y: 1 }, Vec2i { x: 0, y: 0 }, Vec2i { x: 1, y: 0 }],
                [Vec2i { x: 0, y: 2 }, Vec2i { x: 0, y: 1 }, Vec2i { x: 1, y: 1 }, Vec2i { x: 1, y: 0 }],
            ],
            min_x: [0, 1, 0, 0],
            max_x: [2, 2, 2, 1],
            min_y: [1, 0, 0, 0],
            max_y: [2, 2, 1, 2],
            color: Color { r: 0.0, g: 0.5, b: 0.0, a: 1.0 },
        },

        // Z
        PieceData {
            blocks: [
                [Vec2i { x: 0, y: 2 }, Vec2i { x: 1, y: 2 }, Vec2i { x: 1, y: 1 }, Vec2i { x: 2, y: 1 }],
                [Vec2i { x: 2, y: 2 }, Vec2i { x: 1, y: 1 }, Vec2i { x: 2, y: 1 }, Vec2i { x: 1, y: 0 }],
                [Vec2i { x: 0, y: 1 }, Vec2i { x: 1, y: 1 }, Vec2i { x: 1, y: 0 }, Vec2i { x: 2, y: 0 }],
                [Vec2i { x: 1, y: 2 }, Vec2i { x: 0, y: 1 }, Vec2i { x: 1, y: 1 }, Vec2i { x: 0, y: 0 }],
            ],
            min_x: [0, 1, 0, 0],
            max_x: [2, 2, 2, 1],
            min_y: [1, 0, 0, 0],
            max_y: [2, 2, 1, 2],
            color: Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 },
        },

        // J
        PieceData {
            blocks: [
                [Vec2i { x: 0, y: 2 }, Vec2i { x: 0, y: 1 }, Vec2i { x: 1, y: 1 }, Vec2i { x: 2, y: 1 }],
                [Vec2i { x: 1, y: 2 }, Vec2i { x: 2, y: 2 }, Vec2i { x: 1, y: 1 }, Vec2i { x: 1, y: 0 }],
                [Vec2i { x: 0, y: 1 }, Vec2i { x: 1, y: 1 }, Vec2i { x: 2, y: 1 }, Vec2i { x: 2, y: 0 }],
                [Vec2i { x: 1, y: 2 }, Vec2i { x: 1, y: 1 }, Vec2i { x: 1, y: 0 }, Vec2i { x: 0, y: 0 }],
            ],
            min_x: [0, 1, 0, 0],
            max_x: [2, 2, 2, 1],
            min_y: [1, 0, 0, 0],
            max_y: [2, 2, 1, 2],
            color: Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 },
        },

        // L
        PieceData {
            blocks: [
                [Vec2i { x: 2, y: 2 }, Vec2i { x: 0, y: 1 }, Vec2i { x: 1, y: 1 }, Vec2i { x: 2, y: 1 }],
                [Vec2i { x: 1, y: 2 }, Vec2i { x: 1, y: 1 }, Vec2i { x: 1, y: 0 }, Vec2i { x: 2, y: 0 }],
                [Vec2i { x: 0, y: 1 }, Vec2i { x: 1, y: 1 }, Vec2i { x: 2, y: 1 }, Vec2i { x: 0, y: 0 }],
                [Vec2i { x: 0, y: 2 }, Vec2i { x: 1, y: 2 }, Vec2i { x: 1, y: 1 }, Vec2i { x: 1, y: 0 }],
            ],
            min_x: [0, 1, 0, 0],
            max_x: [2, 2, 2, 1],
            min_y: [1, 0, 0, 0],
            max_y: [2, 2, 1, 2],
            color: Color { r: 1.0, g: 0.65, b: 0.0, a: 1.0 },
        },

        // O
        PieceData {
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
        },

        // I
        PieceData {
            blocks: [
                [Vec2i { x: 0, y: 2 }, Vec2i { x: 1, y: 2 }, Vec2i { x: 2, y: 2 }, Vec2i { x: 3, y: 2 }],
                [Vec2i { x: 2, y: 3 }, Vec2i { x: 2, y: 2 }, Vec2i { x: 2, y: 1 }, Vec2i { x: 2, y: 0 }],
                [Vec2i { x: 0, y: 1 }, Vec2i { x: 1, y: 1 }, Vec2i { x: 2, y: 1 }, Vec2i { x: 3, y: 1 }],
                [Vec2i { x: 1, y: 3 }, Vec2i { x: 1, y: 2 }, Vec2i { x: 1, y: 1 }, Vec2i { x: 1, y: 0 }],
            ],
            min_x: [0, 2, 0, 1],
            max_x: [3, 2, 3, 1],
            min_y: [2, 0, 1, 0],
            max_y: [2, 3, 1, 3],
            color: Color { r: 0.0, g: 1.0, b: 1.0, a: 1.0 },
        },

        // T
        PieceData {
            blocks: [
                [Vec2i { x: 1, y: 2 }, Vec2i { x: 0, y: 1 }, Vec2i { x: 1, y: 1 }, Vec2i { x: 2, y: 1 }],
                [Vec2i { x: 1, y: 2 }, Vec2i { x: 1, y: 1 }, Vec2i { x: 2, y: 1 }, Vec2i { x: 1, y: 0 }],
                [Vec2i { x: 0, y: 1 }, Vec2i { x: 1, y: 1 }, Vec2i { x: 2, y: 1 }, Vec2i { x: 1, y: 0 }],
                [Vec2i { x: 1, y: 2 }, Vec2i { x: 0, y: 1 }, Vec2i { x: 1, y: 1 }, Vec2i { x: 1, y: 0 }],
            ],
            min_x: [0, 1, 0, 0],
            max_x: [2, 2, 2, 1],
            min_y: [1, 0, 0, 0],
            max_y: [2, 2, 1, 2],
            color: Color { r: 0.5, g: 0.0, b: 0.5, a: 1.0 },
        },
    ]
);
