use crate::BLOCK_SCALE;
use crate::app::*;
use crate::linalg::*;

use crate::game::{
    pieces::{get_piece_type_color, Piece},
    playfield::{Playfield, PLAYFIELD_VISIBLE_HEIGHT},
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
    app: &mut App,
    persistent: &PersistentData
) {
    for block_pos in piece.blocks() {
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
    app: &mut App,
    persistent: &PersistentData
) {
    // @TODO be able to draw outside when needed
    if pos.x < 0 || pos.x >= playfield.grid_size.x ||
       pos.y < 0 || pos.y >= PLAYFIELD_VISIBLE_HEIGHT {

        return;
    }

    let pixel_scale = persistent.pixel_scale;

    let render_pos;

    // Vertical position should be corrected since it's from bottom to top.
    // We calculate the bottom position, and have to move up a whole block to have the correct
    // position for y = 0

    if playfield.has_grid {
        let bottom = playfield.pos.y as f32 +
            pixel_scale as f32 * ((1.0 + BLOCK_SCALE) * PLAYFIELD_VISIBLE_HEIGHT as f32 + 1.0);

        let block_pos_x = (1.0 + BLOCK_SCALE) * (pos.x as f32 + delta_pos.x) + 1.0;
        let block_pos_y = (1.0 + BLOCK_SCALE) * (1.0 + pos.y as f32 + delta_pos.y);

        render_pos = Vec2 {
            x: playfield.pos.x as f32 + block_pos_x * pixel_scale as f32,
            y: bottom - block_pos_y * pixel_scale as f32,
        };
    } else {
        let bottom = playfield.pos.y as f32 +
            BLOCK_SCALE * pixel_scale as f32 * PLAYFIELD_VISIBLE_HEIGHT as f32;

        render_pos = Vec2 {
            x: playfield.pos.x as f32 + BLOCK_SCALE * pixel_scale as f32 * (pos.x as f32 + delta_pos.x),
            y: bottom - BLOCK_SCALE * pixel_scale as f32 * (1.0 + pos.y as f32 + delta_pos.y),
        };
    }

    draw_block(render_pos, color, app, persistent);
}

pub fn draw_block(
    pos: Vec2,
    color: Color,
    app: &mut App,
    persistent: &PersistentData
) {
    let scale = Vec2 { x: persistent.pixel_scale as f32, y: persistent.pixel_scale as f32 };
    app.queue_draw_sprite(
        &TransformBuilder::new()
            .pos(pos)
            .scale(scale)
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
    has_grid: bool,
    app: &mut App,
    persistent: &PersistentData
) {
    for block_pos in piece.blocks() {
        let block_pos = Vec2 { x: block_pos.x as f32, y: (3 - block_pos.y) as f32 };

        let draw_pos;
        if has_grid {
            let extra_px = Vec2 { x: 1.0, y: 1.0 };
            draw_pos = pos + (extra_px + block_pos * (1.0 + BLOCK_SCALE)) * persistent.pixel_scale as f32;
        } else {
            draw_pos = pos + block_pos * BLOCK_SCALE * persistent.pixel_scale as f32;
        }

        draw_block(
            draw_pos,
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
    has_grid: bool,
    app: &mut App,
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

        let draw_pos;
        if has_grid {
            let extra_px = Vec2 { x: 1.0, y: 1.0 };
            draw_pos = pos
                + (extra_px + (block_pos - delta) * (1.0 + BLOCK_SCALE)) * persistent.pixel_scale as f32;
        } else {
            draw_pos = pos + (block_pos - delta) * BLOCK_SCALE * persistent.pixel_scale as f32;
        }

        draw_block(
            draw_pos,
            color,
            app,
            persistent
        );
    }
}

pub fn get_draw_playfield_size(
    playfield: &Playfield,
    pixel_scale: u8,
) -> Vec2 {
    let x;
    if playfield.has_grid { x = (1.0 + BLOCK_SCALE) * playfield.grid_size.x as f32 + 1.0; }
    else { x = BLOCK_SCALE * playfield.grid_size.x as f32; }

    let y;
    if playfield.has_grid { y = (1.0 + BLOCK_SCALE) * PLAYFIELD_VISIBLE_HEIGHT as f32 + 1.0; }
    else { y = BLOCK_SCALE * PLAYFIELD_VISIBLE_HEIGHT as f32; }

    pixel_scale as f32 * Vec2 { x, y }
}

pub fn draw_playfield(
    playfield: &Playfield,
    line_clear_animation: Option<(u64, u64, LineClearAnimationType, &[u8])>,
    rotation_system: RotationSystem,
    app: &mut App,
    persistent: &PersistentData
) {
    let size = get_draw_playfield_size(playfield, persistent.pixel_scale);

    draw_rect_window(
        Vec2::from(playfield.pos),
        size,
        persistent.pixel_scale,
        app,
        persistent
    );

    // blocks

    match line_clear_animation {
        None => {
            // @Refactor cache playfield/draw to framebuffer
            for row in 0..PLAYFIELD_VISIBLE_HEIGHT {
                for col in 0..playfield.grid_size.x {
                    if let Some(piece_type) = playfield.block(col, row) {
                        draw_block_in_playfield(
                            Vec2i { x: col, y: row },
                            Vec2::new(),
                            // @Refactor style
                            get_piece_type_color(piece_type, rotation_system),
                            playfield,
                            app,
                            persistent
                        );
                    }
                }
            }
        },

        Some((lock_timestamp, line_clear_delay, animation_type, lines_to_clear)) => {
            let mut line_clear_iter = lines_to_clear.iter();
            let mut current_line_to_clear = line_clear_iter.next();

            // @Refactor cache playfield/draw to framebuffer
            for row in 0..PLAYFIELD_VISIBLE_HEIGHT {

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
                            app
                        ) {
                            draw_block_in_playfield(
                                Vec2i { x: col, y: row },
                                Vec2::new(),
                                // @Refactor style
                                get_piece_type_color(piece_type, rotation_system),
                                playfield,
                                app,
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
                                get_piece_type_color(piece_type, rotation_system),
                                playfield,
                                app,
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
    pos: Vec2,
    size: Vec2,
    border_size: u8,
    app: &mut App,
    persistent: &PersistentData,
) {
    let border_size = Vec2 { x: border_size as f32, y: border_size as f32 };

    // left
    let rect_pos = pos - border_size;
    let scale = Vec2 {
        x: border_size.x,
        y: 2.0 * border_size.y + size.y,
    };
    app.queue_draw_sprite(
        &TransformBuilder::new().pos(rect_pos).scale(scale).build(),
        &persistent.sprites.blank,
        WHITE
    );

    // right
    let rect_pos = pos + Vec2 { x: size.x, y: -border_size.y };
    app.queue_draw_sprite(
        &TransformBuilder::new().pos(rect_pos).scale(scale).build(),
        &persistent.sprites.blank,
        WHITE
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
        WHITE
    );

    // bottom
    let rect_pos = pos + Vec2 { x: -border_size.x, y: size.y };
    app.queue_draw_sprite(
        &TransformBuilder::new().pos(rect_pos).scale(scale).build(),
        &persistent.sprites.blank,
        WHITE
    );

    // bg
    app.queue_draw_sprite(
        // @TODO fix layer negative not showing
        &TransformBuilder::new().pos(pos).scale(size).layer(0).build(),
        &persistent.sprites.blank,
        BLACK
    );
}

pub fn draw_piece_window(
    pos: Vec2,
    piece: Piece,
    is_centered: bool,
    has_grid: bool,
    app: &mut App,
    persistent: &mut PersistentData
) {
    let window_size;
    if has_grid {
        let size = persistent.pixel_scale as f32 * ((1.0 + BLOCK_SCALE) * 4.0 + 1.0);
        window_size = Vec2 { x: size as f32, y: size as f32 };
    } else {
        let size = persistent.pixel_scale as f32 * BLOCK_SCALE * 4.0;
        window_size = Vec2 { x: size as f32, y: size as f32 };
    }

    draw_rect_window(
        pos.into(),
        window_size,
        persistent.pixel_scale,
        app,
        persistent
    );

    if is_centered {
        draw_piece_centered(
            piece,
            pos.into(),
            piece.color(),
            has_grid,
            app,
            persistent
        );
    } else {
        draw_piece(
            piece,
            pos.into(),
            piece.color(),
            has_grid,
            app,
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
    app: &mut App
) -> bool {
    if app.game_timestamp() >= lock_timestamp + line_clear_delay {
        return false;
    }

    match animation_type {
        LineClearAnimationType::Classic => {
            // @Fix this logic won't be valid for odd sized playfields

            let half_size = (playfield.grid_size.x / 2) as u64;
            let animation_duration = app.game_timestamp() - lock_timestamp;
            let step = half_size as u64 * animation_duration / line_clear_delay;
            assert!(step < half_size);

            block_col < (half_size - 1 - step) as u8 || block_col > (half_size + step) as u8
        }
    }
}
