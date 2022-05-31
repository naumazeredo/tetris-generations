use super::*;

// @Think this whole pixel scale in layout is confusing. Should the player decide on pixel scale
//        of individual components? Should we use a relative pixel scale? How does layout adapt to
//        pixel scale?

#[derive(Debug, ImDraw, Copy, Clone)]
pub struct TetrisLayout {
    playfield_pos: Vec2i,

    hold_piece_window_pos: Vec2i,
    // hold_piece_pixel_scale

    next_pieces_preview_window_pos: [Vec2i; NEXT_PIECES_COUNT],
    next_pieces_preview_pixel_scale: [u8; NEXT_PIECES_COUNT],

    // @TODO move these to RenderStyle
    // visible_rows_count: u8,
    has_grid: bool,
}

impl TetrisGame {
    pub fn new_layout(
        &self,
        app: &App,
        persistent: &mut PersistentData,
    ) -> TetrisLayout {
        let has_grid = true;

        // Playfield rendering
        let playfield_draw_size = get_draw_playfield_size(
            &self.playfield(),
            persistent.pixel_scale, // @TODO change the order of has_grid and pixel_scale?
            has_grid,
        );

        let window_size = app.window_size();
        let playfield_pos = Vec2i {
            x: (window_size.0 as i32 - playfield_draw_size.x) / 2,
            y: (window_size.1 as i32 - playfield_draw_size.y) / 2,
        };

        let hold_window_size = self.hold_piece_window_size(has_grid, persistent);
        let hold_piece_window_pos =
            playfield_pos +
            Vec2i { x: -20, y: 0 } +
            Vec2i { x: -hold_window_size.x, y: 0 };

        // next pieces preview pixel scale

        let secondary_pixel_scale = if persistent.pixel_scale > 1 {
            persistent.pixel_scale
        } else {
            persistent.pixel_scale - 1
        };

        let next_pieces_preview_pixel_scale = [
            persistent.pixel_scale, secondary_pixel_scale,
            secondary_pixel_scale, secondary_pixel_scale,
            secondary_pixel_scale, secondary_pixel_scale,
            secondary_pixel_scale, secondary_pixel_scale,
        ];

        // next pieces preview window position

        let mut next_pieces_preview_window_pos = [Vec2i::new(); NEXT_PIECES_COUNT];

        let mut current_window_pos =
            playfield_pos +
            Vec2i { x: playfield_draw_size.x, y: 0 } +
            Vec2i { x: 20, y: 0 };

        for index in 0..NEXT_PIECES_COUNT {
            next_pieces_preview_window_pos[index] = current_window_pos;

            let next_pieces_preview_window_size = self.next_pieces_preview_window_size(
                has_grid,
                next_pieces_preview_pixel_scale[index as usize]
            );

            current_window_pos += Vec2i { x: 0, y: next_pieces_preview_window_size.y };
        }

        TetrisLayout {
            playfield_pos,
            hold_piece_window_pos,
            next_pieces_preview_window_pos,
            next_pieces_preview_pixel_scale,
            has_grid,
        }
    }

    pub fn update_and_render(
        &mut self,
        layout: TetrisLayout,
        batch: &mut Batch,
        persistent: &mut PersistentData,
    ) {
        self.update_animations();
        self.render(layout, batch, persistent);
    }

    pub fn render(
        &self,
        layout: TetrisLayout,
        batch: &mut Batch,
        persistent: &mut PersistentData,
    ) {
        self.render_playfield(layout.playfield_pos, layout.has_grid, batch, persistent);
        self.render_hold_piece(layout.hold_piece_window_pos, layout.has_grid, batch, persistent);

        for index in 0..self.rules.next_pieces_preview_count {
            self.render_next_pieces_preview(
                layout.next_pieces_preview_window_pos[index as usize],
                index,
                layout.next_pieces_preview_pixel_scale[index as usize],
                layout.has_grid,
                batch,
                persistent
            );
        }
    }

    pub fn render_playfield(
        &self,
        pos: Vec2i,
        has_grid: bool,
        batch: &mut Batch,
        persistent: &mut PersistentData
    ) {
        // playfield
        let playfield_size = get_draw_playfield_size(
            &self.playfield,
            persistent.pixel_scale,
            has_grid,
        );

        // @TODO this should be calculated on update animations
        if let Some(line_clear_animation_type) = self.rules.line_clear_animation_type {
            // Get line clear info
            let lines_to_clear = match self.last_locked_piece {
                None => &[],
                Some(LockedPiece { ref lock_piece_result, .. })
                    => lock_piece_result.get_lines_to_clear_slice(),
            };

            draw_playfield(
                &self.playfield,
                pos,
                persistent.pixel_scale,
                has_grid,
                Some((
                    self.lock_piece_timestamp,
                    self.rules.line_clear_delay,
                    line_clear_animation_type,
                    lines_to_clear,
                    self.timestamp,
                )),
                self.rules.rotation_system,
                batch,
                persistent
            );
        } else {
            draw_playfield(
                &self.playfield,
                pos,
                persistent.pixel_scale,
                has_grid,
                None,
                self.rules.rotation_system,
                batch,
                persistent
            );
        }

        // Clip
        batch.push_clip(pos, playfield_size.into());

        // ghost piece
        if let Some(piece) = self.current_piece {
            let (piece, piece_pos) = piece;
            if self.rules.movement_animation_show_ghost {
                draw_piece_in_playfield(
                    piece,
                    piece_pos,
                    Vec2::new(),
                    Color { r: 1., g: 1., b: 1., a: 0.1 }, // @TODO create ghost color
                    &self.playfield,
                    pos,
                    persistent.pixel_scale,
                    has_grid,
                    batch,
                    persistent
                );
            }

            // render ghost piece
            if self.rules.has_ghost_piece {
                // @TODO cache the ghost piece and only recalculate the position when piece moves
                let mut ghost_piece = piece.clone();
                let mut ghost_piece_pos = piece_pos;

                // Only render if ghost is not on top of the current piece
                // @Maybe improve this and just draw the different pixels. Since there's piece
                //        movement animation, not rendering the ghost in the middle of the piece
                //        movement may be visually weird
                if full_drop_piece(
                    &mut ghost_piece,
                    &mut ghost_piece_pos,
                    &self.playfield,
                ) > 0 {
                    draw_piece_in_playfield(
                        ghost_piece,
                        ghost_piece_pos,
                        Vec2::new(),
                        Color { r: 1., g: 1., b: 1., a: 0.1 }, // @TODO create ghost color
                        &self.playfield,
                        pos,
                        persistent.pixel_scale,
                        has_grid,
                        batch,
                        persistent
                    );
                }
            }

            // render piece
            let movement_animation_delta_grid;
            if self.rules.has_movement_animation {
                movement_animation_delta_grid = self.movement_animation_current_delta_grid;
            } else {
                movement_animation_delta_grid = Vec2::new();
            }

            let color;
            if self.is_locking && self.rules.has_locking_animation {
                let delta_time = self.timestamp - self.locking_animation_timestamp;
                let alpha = 2.0 * std::f32::consts::PI * delta_time as f32;
                let alpha = alpha / (self.rules.locking_animation_duration as f32);
                let alpha = ((1.0 + alpha.sin()) / 2.0) / 2.0 + 0.5;

                color = piece.color().alpha(alpha);
            } else {
                color = piece.color();
            }

            draw_piece_in_playfield(
                piece,
                piece_pos,
                movement_animation_delta_grid,
                color,
                &self.playfield,
                pos,
                persistent.pixel_scale,
                has_grid,
                batch,
                persistent
            );
        }

        // Clip
        batch.pop_clip();
    }

    // @Refactor we have to be more versatile here to allow splitting the next pieces previews
    //           in multiple windows (maybe have a (start, count), or render each one independently
    //           and pass which window borders should be rendered, or have a "window frame"
    //           rendering separate)
    // @Fix this is not counting the window border
    pub fn next_pieces_preview_window_size(
        &self,
        has_grid: bool,
        pixel_scale: u8,
    ) -> Vec2i {
        let size;
        if has_grid {
            size = pixel_scale as i32 * ((1 + BLOCK_SCALE as i32) * 4 + 1);
        } else {
            size = pixel_scale as i32 * BLOCK_SCALE as i32 * 4;
        }

        Vec2i { x: size, y: size }
    }

    pub fn render_next_pieces_preview(
        &self,
        pos: Vec2i,
        index: u8,
        pixel_scale: u8,
        has_grid: bool,
        batch: &mut Batch,
        persistent: &mut PersistentData
    ) {
        // render preview
        if self.rules.next_pieces_preview_count > 0 {
            let window_size = self.next_pieces_preview_window_size(
                has_grid,
                pixel_scale,
            );

            draw_rect_window(
                pos.into(),
                window_size.into(),
                pixel_scale,
                batch,
                persistent
            );

            assert!(self.rules.next_pieces_preview_count <= 8);
            assert!(index < 8);
            draw_piece_centered(
                Piece {
                    variant: self.next_piece_types[index as usize],
                    rot: 0,
                    rotation_system: self.rules.rotation_system,
                },
                pos.into(),
                get_piece_variant_color(self.next_piece_types[index as usize], self.rules.rotation_system),
                pixel_scale,
                has_grid,
                batch,
                persistent
            );

            /*
            assert!(self.rules.next_pieces_preview_count <= 8);
            for i in 0..self.rules.next_pieces_preview_count {
                draw_piece_centered(
                    Piece {
                        variant: self.next_piece_types[i as usize],
                        rot: 0,
                        rotation_system: self.rules.rotation_system,
                    },
                    Vec2 {
                        x: pos.x,
                        y: pos.y + i as f32 * size,
                    },
                    get_piece_variant_color(self.next_piece_types[i as usize], self.rules.rotation_system),
                    self.playfield.has_grid,
                    persistent
                );
            }
            */
        }
    }

    // @Fix this is not counting the window border
    pub fn hold_piece_window_size(
        &self,
        has_grid: bool,
        persistent: &mut PersistentData
    ) -> Vec2i {
        if has_grid {
            let size = persistent.pixel_scale as i32 * ((1 + BLOCK_SCALE as i32) * 4 + 1);
            Vec2i { x: size, y: size }
        } else {
            let size = persistent.pixel_scale as i32 * BLOCK_SCALE as i32 * 4;
            Vec2i { x: size, y: size }
        }
    }

    pub fn render_hold_piece(
        &self,
        pos: Vec2i,
        has_grid: bool,
        batch: &mut Batch,
        persistent: &mut PersistentData
    ) {
        // hold piece
        if self.rules.has_hold_piece {
            let window_size = self.hold_piece_window_size(has_grid, persistent);

            draw_rect_window(
                pos,
                window_size,
                persistent.pixel_scale,
                batch,
                persistent
            );

            if let Some(hold_piece) = self.hold_piece {
                draw_piece_centered(
                    hold_piece,
                    pos,
                    hold_piece.color(),
                    persistent.pixel_scale,
                    has_grid,
                    batch,
                    persistent
                );
            }
        }
    }
}
