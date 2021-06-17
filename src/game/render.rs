use crate::BLOCK_SCALE;
use crate::app::*;
use crate::linalg::*;
use crate::State;

use crate::game::{
    piece::Piece,
    playfield::{ Playfield, PLAYFIELD_VISIBLE_HEIGHT },
    scenes::PersistentData,
};

// @Cleanup these functions seems clumsy, too much repetition.

// @Refactor color should be passed by render stack commands
pub fn draw_piece_in_playfield(
    piece: Piece,
    pos: Vec2i,
    delta_pos: Vec2,
    color: Color,
    playfield: &Playfield,
    app: &mut App<'_, State>,
    persistent: &PersistentData
) {
    for block_pos in piece.type_.blocks(piece.rot) {
        draw_block_in_playfield(
            pos + *block_pos,
            delta_pos,
            color,
            playfield,
            app,
            persistent
        );
    }
}

pub fn draw_block_in_playfield(
    pos: Vec2i,
    delta_pos: Vec2,
    color: Color,
    playfield: &Playfield,
    app: &mut App<'_, State>,
    persistent: &PersistentData
) {
    if pos.x < 0 || pos.x >= playfield.grid_size.x ||
       pos.y < 0 || pos.y >= PLAYFIELD_VISIBLE_HEIGHT {

        return;
    }

    let pixel_scale = persistent.pixel_scale;
    let bottom = playfield.pos.y as f32 +
        BLOCK_SCALE * pixel_scale.y * PLAYFIELD_VISIBLE_HEIGHT as f32;

    let pos = Vec2 {
        x: playfield.pos.x as f32 + BLOCK_SCALE * pixel_scale.x * (pos.x as f32 + delta_pos.x),
        y: bottom - BLOCK_SCALE * pixel_scale.y * (pos.y as f32 + 1.0 + delta_pos.y),
    };

    draw_block(pos, color, app, persistent);
}

pub fn draw_block(
    pos: Vec2,
    color: Color,
    app: &mut App<'_, State>,
    persistent: &PersistentData
) {
    app.queue_draw_sprite(
        &TransformBuilder::new()
            .pos(pos)
            .scale(persistent.pixel_scale)
            .layer(10)
            .build(),
        &persistent.sprites.block,
        color
    );
}

pub fn draw_piece(
    piece: Piece,
    pos: Vec2,
    color: Color,
    app: &mut App<'_, State>,
    persistent: &PersistentData
) {
    for block_pos in piece.blocks() {
        let block_pos = Vec2 { x: block_pos.x as f32, y: (3 - block_pos.y) as f32 };
        draw_block(
            pos + block_pos * BLOCK_SCALE * persistent.pixel_scale,
            color,
            app,
            persistent
        );
    }
}

pub fn draw_piece_centered(
    piece: Piece,
    pos: Vec2,
    color: Color,
    app: &mut App<'_, State>,
    persistent: &PersistentData
) {
    let min_max_x = piece.min_max_x();
    let min_max_y = piece.min_max_y();

    let delta =
        Vec2 {
            x: (min_max_x.0 + min_max_x.1 + 1) as f32 / 2.0,
            y: -((min_max_y.0 + min_max_y.1 + 1) as f32 / 2.0),
        };

    for block_pos in piece.blocks() {
        let block_pos = Vec2 { x: (block_pos.x + 2) as f32, y: (1 - block_pos.y) as f32 };
        draw_block(
            pos + (block_pos - delta) * BLOCK_SCALE * persistent.pixel_scale,
            color,
            app,
            persistent
        );
    }
}

pub fn draw_playfield(
    playfield: &Playfield,
    app: &mut App<'_, State>,
    persistent: &PersistentData
) {
    draw_rect_window(
        Vec2::from(playfield.pos),
        Vec2 {
            x: persistent.pixel_scale.x * BLOCK_SCALE * playfield.grid_size.x as f32,
            y: persistent.pixel_scale.y * BLOCK_SCALE * PLAYFIELD_VISIBLE_HEIGHT as f32,
        },
        persistent.pixel_scale,
        app,
        persistent
    );

    // blocks

    // @Refactor cache playfield/draw to framebuffer
    for i in 0..PLAYFIELD_VISIBLE_HEIGHT {
        for j in 0..playfield.grid_size.x {
            if let Some(piece_type) = playfield.block(j, i) {
                draw_block_in_playfield(
                    Vec2i { x: j, y: i },
                    Vec2::new(),
                    piece_type.color(),
                    playfield,
                    app,
                    persistent
                );
            }
        }
    }
}

pub fn draw_rect_window(
    pos: Vec2,
    size: Vec2,
    border_size: Vec2,
    app: &mut App<'_, State>,
    persistent: &PersistentData,
) {
    // left
    let rect_pos = pos - border_size;
    let scale = Vec2 {
        x: border_size.x,
        y: 2.0 * border_size.y + size.y,
    };
    app.queue_draw_sprite(
        &TransformBuilder::new().pos(rect_pos).scale(scale).build(),
        &persistent.sprites.blank,
        BLACK
    );

    // right
    let rect_pos = pos + Vec2 { x: size.x, y: -border_size.y };
    app.queue_draw_sprite(
        &TransformBuilder::new().pos(rect_pos).scale(scale).build(),
        &persistent.sprites.blank,
        BLACK
    );

    // top
    let rect_pos = pos - border_size;
    let scale = Vec2 {
        x: 2.0 * border_size.x + size.x,
        y: border_size.y,
    };
    app.queue_draw_sprite(
        &TransformBuilder::new().pos(rect_pos).scale(scale).build(),
        &persistent.sprites.blank,
        BLACK
    );

    // bottom
    let rect_pos = pos + Vec2 { x: -border_size.x, y: size.y };
    app.queue_draw_sprite(
        &TransformBuilder::new().pos(rect_pos).scale(scale).build(),
        &persistent.sprites.blank,
        BLACK
    );

    // bg
    app.queue_draw_sprite(
        // @TODO fix layer negative not showing
        &TransformBuilder::new().pos(pos).scale(size).layer(0).build(),
        &persistent.sprites.blank,
        Color { r: 0.2, g: 0.2, b: 0.2, a: 1.0 },
    );
}
