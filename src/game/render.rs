// @TODO move all this to their respective structs
//       All Batch stuff should be behind a trait?

use crate::BLOCK_SCALE;
use crate::app::*;
use crate::linalg::*;

use crate::game::{
    pieces::{get_piece_variant_color, Piece},
    playfield::Playfield,
    rules::{
        RotationSystem,
        line_clear::LineClearAnimationType,
    },
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
    playfield_pos: Vec2i,
    has_grid: bool,
    batch: &mut Batch,
    persistent: &PersistentData
) {
    for block_pos in piece.blocks() {
        draw_block_in_playfield(
            pos + *block_pos,
            delta_pos,
            color,
            playfield,
            playfield_pos,
            has_grid,
            batch,
            persistent
        );
    }
}

pub fn draw_block_in_playfield(
    pos: Vec2i,
    delta_pos: Vec2,
    color: Color,
    playfield: &Playfield,
    playfield_pos: Vec2i,
    has_grid: bool,
    batch: &mut Batch,
    persistent: &PersistentData
) {
    // @TODO be able to draw outside when needed
    if pos.x < 0 || pos.x >= playfield.grid_size.x ||
       pos.y < 0 || pos.y >= playfield.visible_height as i32 {

        return;
    }

    let pixel_scale = persistent.pixel_scale as i32;
    let delta_pos = Vec2i { x: delta_pos.x.round() as i32, y: delta_pos.y.round() as i32 };

    // Vertical position should be corrected since it's from bottom to top.
    // We calculate the bottom position, and have to move up a whole block to have the correct
    // position for y = 0

    let render_pos;
    if has_grid {
        let bottom = playfield_pos.y +
            pixel_scale * ((1 + BLOCK_SCALE as i32) * playfield.visible_height as i32 + 1);

        let block_pos_x = (1 + BLOCK_SCALE as i32) * (pos.x + delta_pos.x) + 1;
        let block_pos_y = (1 + BLOCK_SCALE as i32) * (1 + pos.y + delta_pos.y);

        render_pos = Vec2i {
            x: playfield_pos.x + block_pos_x * pixel_scale,
            y: bottom - block_pos_y * pixel_scale,
        };
    } else {
        let bottom = playfield_pos.y +
            BLOCK_SCALE as i32 * pixel_scale * playfield.visible_height as i32;

        render_pos = Vec2i {
            x: playfield_pos.x + BLOCK_SCALE as i32 * pixel_scale * (pos.x + delta_pos.x),
            y: bottom - BLOCK_SCALE as i32 * pixel_scale * (1 + pos.y + delta_pos.y),
        };
    }

    draw_block(
        render_pos,
        color,
        batch,
        persistent
    );
}

pub fn draw_block(
    pos: Vec2i,
    color: Color,
    batch: &mut Batch,
    persistent: &PersistentData
) {
    let scale = Vec2i { x: persistent.pixel_scale as i32, y: persistent.pixel_scale as i32 };
    batch.queue_draw_sprite(
        &TransformBuilder::new()
            .pos(pos.into())
            .scale(scale.into())
            .layer(10)
            .build(),
        &persistent.sprites.block,
        color
    );
}

pub fn draw_piece(
    piece: Piece,
    pos: Vec2i,
    color: Color,
    has_grid: bool,
    batch: &mut Batch,
    persistent: &PersistentData
) {
    for block_pos in piece.blocks() {
        let block_pos = Vec2i { x: block_pos.x, y: 3 - block_pos.y };

        let draw_pos;
        if has_grid {
            let extra_px = Vec2i { x: 1, y: 1 };
            draw_pos = pos +
                extra_px + block_pos * (1 + BLOCK_SCALE as i32) * persistent.pixel_scale as i32;
        } else {
            draw_pos = pos + block_pos * BLOCK_SCALE as i32 * persistent.pixel_scale as i32;
        }

        draw_block(
            draw_pos,
            color,
            batch,
            persistent
        );
    }
}

pub fn draw_piece_centered(
    piece: Piece,
    pos: Vec2i,
    color: Color,
    has_grid: bool,
    batch: &mut Batch,
    persistent: &PersistentData
) {
    let min_max_x = piece.min_max_x();
    let min_max_y = piece.min_max_y();

    let delta =
        Vec2i {
            x: (min_max_x.0 + min_max_x.1 + 1) as i32,
            y: -((min_max_y.0 + min_max_y.1 + 1) as i32),
        };

    for block_pos in piece.blocks() {
        let block_pos = Vec2i { x: (block_pos.x + 2), y: (1 - block_pos.y) };
        let block_pos =
            block_pos * (1 + BLOCK_SCALE as i32) -
            delta * (1 + BLOCK_SCALE as i32) / 2;

        let draw_pos;
        if has_grid {
            let extra_px = Vec2i { x: 1, y: 1 };
            draw_pos = pos +
                extra_px + block_pos * persistent.pixel_scale as i32;
        } else {
            draw_pos = pos + block_pos * persistent.pixel_scale as i32;
        }

        draw_block(
            draw_pos,
            color,
            batch,
            persistent
        );
    }
}

pub fn get_draw_playfield_size(
    playfield: &Playfield,
    pixel_scale: u8,
    has_grid: bool,
) -> Vec2i {
    let visible_grid_size = Vec2i {
        x: playfield.grid_size.x,
        y: playfield.grid_size.y.min(playfield.visible_height as i32),
    };

    get_draw_playfield_grid_size(visible_grid_size, pixel_scale, has_grid)
}

pub fn get_draw_playfield_grid_size(
    grid_size: Vec2i,
    pixel_scale: u8,
    has_grid: bool,
) -> Vec2i {
    let x;
    if has_grid { x = (1 + BLOCK_SCALE as i32) * grid_size.x + 1; }
    else { x = BLOCK_SCALE as i32 * grid_size.x; }

    let y;
    if has_grid { y = (1 + BLOCK_SCALE as i32) * grid_size.y as i32 + 1; }
    else { y = BLOCK_SCALE as i32 * grid_size.y as i32; }

    pixel_scale as i32 * Vec2i { x, y }
}


pub fn draw_playfield(
    playfield: &Playfield,
    pos: Vec2i,
    has_grid: bool,
    line_clear_animation: Option<(u64, u64, LineClearAnimationType, &[u8], u64)>,
    rotation_system: RotationSystem,
    batch: &mut Batch,
    persistent: &PersistentData
) {
    let size = get_draw_playfield_size(playfield, persistent.pixel_scale, has_grid);

    draw_rect_window(
        pos,
        size,
        persistent.pixel_scale,
        batch,
        persistent
    );

    // blocks

    match line_clear_animation {
        None => {
            // @Refactor cache playfield/draw to framebuffer
            for row in 0..playfield.visible_height as i32 {
                for col in 0..playfield.grid_size.x {
                    if let Some(piece_type) = playfield.block(col, row) {
                        draw_block_in_playfield(
                            Vec2i { x: col, y: row },
                            Vec2::new(),
                            // @Refactor style
                            get_piece_variant_color(piece_type, rotation_system),
                            playfield,
                            pos,
                            has_grid,
                            batch,
                            persistent
                        );
                    }
                }
            }
        },

        Some((lock_timestamp, line_clear_delay, animation_type, lines_to_clear, game_timestamp)) => {
            let mut line_clear_iter = lines_to_clear.iter();
            let mut current_line_to_clear = line_clear_iter.next();

            // @Refactor cache playfield/draw to framebuffer
            for row in 0..playfield.visible_height as i32 {

                // if it's a line to be cleared, we have to apply the animation to it
                if current_line_to_clear.is_some() && *current_line_to_clear.unwrap() == row as u8 {
                    for col in 0..playfield.grid_size.x {
                        let piece_type = playfield.block(col, row).unwrap();

                        if line_clear_animation_should_draw_block(
                            col as u8,
                            //anim_state
                            animation_type,
                            lock_timestamp,
                            line_clear_delay,
                            playfield,
                            game_timestamp,
                        ) {
                            draw_block_in_playfield(
                                Vec2i { x: col, y: row },
                                Vec2::new(),
                                // @Refactor style
                                get_piece_variant_color(piece_type, rotation_system),
                                playfield,
                                pos,
                                has_grid,
                                batch,
                                persistent
                            );
                        }
                    }

                    current_line_to_clear = line_clear_iter.next();
                } else {
                    // otherwise, we just draw the blocks
                    for col in 0..playfield.grid_size.x {
                        if let Some(piece_type) = playfield.block(col, row) {
                            draw_block_in_playfield(
                                Vec2i { x: col, y: row },
                                Vec2::new(),
                                // @Refactor style
                                get_piece_variant_color(piece_type, rotation_system),
                                playfield,
                                pos,
                                has_grid,
                                batch,
                                persistent
                            );
                        }
                    }
                }
            }
        }
    }
}

pub fn draw_rect_window(
    pos: Vec2i,
    size: Vec2i,
    border_size: u8,
    batch: &mut Batch,
    persistent: &PersistentData,
) {

    let border_size = Vec2i { x: border_size as i32, y: border_size as i32 };

    // left
    let rect_pos = pos - border_size;
    let scale = Vec2i {
        x: border_size.x,
        y: 2 * border_size.y + size.y,
    };
    batch.queue_draw_sprite(
        &TransformBuilder::new().pos(rect_pos.into()).scale(scale.into()).build(),
        &persistent.sprites.blank,
        WHITE
    );

    // right
    let rect_pos = pos + Vec2i { x: size.x, y: -border_size.y };
    batch.queue_draw_sprite(
        &TransformBuilder::new().pos(rect_pos.into()).scale(scale.into()).build(),
        &persistent.sprites.blank,
        WHITE
    );

    // top
    let rect_pos = pos - border_size;
    let scale = Vec2i {
        x: 2 * border_size.x + size.x,
        y: border_size.y,
    };
    batch.queue_draw_sprite(
        &TransformBuilder::new().pos(rect_pos.into()).scale(scale.into()).build(),
        &persistent.sprites.blank,
        WHITE
    );

    // bottom
    let rect_pos = pos + Vec2i { x: -border_size.x, y: size.y };
    batch.queue_draw_sprite(
        &TransformBuilder::new().pos(rect_pos.into()).scale(scale.into()).build(),
        &persistent.sprites.blank,
        WHITE
    );

    // bg
    batch.queue_draw_sprite(
        // @TODO fix layer negative not showing
        &TransformBuilder::new().pos(pos.into()).scale(size.into()).layer(0).build(),
        &persistent.sprites.blank,
        BLACK
    );
}

pub fn draw_piece_window(
    pos: Vec2i,
    piece: Piece,
    is_centered: bool,
    has_grid: bool,
    batch: &mut Batch,
    persistent: &mut PersistentData
) {
    let window_size;
    if has_grid {
        let size = persistent.pixel_scale as i32 * ((1 + BLOCK_SCALE as i32) * 4 + 1);
        window_size = Vec2i { x: size, y: size };
    } else {
        let size = persistent.pixel_scale as i32 * BLOCK_SCALE as i32 * 4;
        window_size = Vec2i { x: size, y: size };
    }

    draw_rect_window(
        pos,
        window_size,
        persistent.pixel_scale,
        batch,
        persistent
    );

    if is_centered {
        draw_piece_centered(
            piece,
            pos,
            piece.color(),
            has_grid,
            batch,
            persistent
        );
    } else {
        draw_piece(
            piece,
            pos,
            piece.color(),
            has_grid,
            batch,
            persistent
        );
    }
}

fn line_clear_animation_should_draw_block(
    block_col: u8,
    animation_type: LineClearAnimationType,
    lock_timestamp: u64,
    line_clear_delay: u64,
    playfield: &Playfield,
    game_timestamp: u64,
) -> bool {
    if game_timestamp >= lock_timestamp + line_clear_delay {
        return false;
    }

    match animation_type {
        LineClearAnimationType::Classic => {
            let half_size = ((playfield.grid_size.x + 1) / 2) as u64;
            let animation_duration = game_timestamp - lock_timestamp;
            let step = half_size as u64 * animation_duration / line_clear_delay;
            assert!(step < half_size);

            block_col < (half_size - 1 - step) as u8 || block_col > (half_size + step) as u8
        }
    }
}
